use polars_core::prelude::{
    AnyValue, BinaryType, ChunkedArray, Field, PolarsError, PolarsResult, Series,
};
use polars_core::utils::rayon::iter::{IntoParallelIterator, ParallelIterator};
use polars_core::POOL;
use prost::Message;

use polars_structpath::StructPath;

/// Get the Polars `Field` type for a given path in a protobuf message type.
///
/// This function uses the `StructPath` trait to determine the data type of a field
/// at the specified path within a protobuf message type. The result is returned as
/// a Polars `Field` that can be used for type inference in Polars operations.
///
/// # Arguments
///
/// * `input_fields` - Input fields from Polars (currently only used to extract the field name)
/// * `path` - The path to the field (e.g., `"name"`, `"parent.name"`, `"items[0].value"`)
///
/// # Type Parameters
///
/// * `T` - The protobuf message type that implements both `StructPath` and `Message`
///
/// # Returns
///
/// Returns a `PolarsResult<Field>` containing the field name and data type.
///
/// # Errors
///
/// Returns `PolarsError::StructFieldNotFound` if the path is invalid or the field doesn't exist.
///
/// # Example
///
/// ```rust,no_run
/// use polars_core::prelude::Field;
/// use polars_protobuf::get_type;
/// use prost::Message;
///
/// #[derive(polars_structpath::StructPath, Message)]
/// struct Person {
///     #[prost(string, tag = "1")]
///     name: String,
/// }
///
/// let field = get_type::<Person>(&[], "name").unwrap();
/// // field.dtype() will be DataType::String
/// ```
pub fn get_type<T>(input_fields: &[Field], path: &str) -> PolarsResult<Field>
where
    T: StructPath,
{
    let data_type_wrapper =
        T::get_type(path).map_err(|e| PolarsError::StructFieldNotFound(e.to_string().into()))?;
    let data_type = data_type_wrapper.polars;
    let name = input_fields
        .first()
        .map(|f| f.name().clone())
        .unwrap_or_else(|| "".into());
    Ok(Field::new(name, data_type))
}

fn get_value_inner<T>(
    ca: &ChunkedArray<BinaryType>,
    path: &str,
) -> PolarsResult<Vec<AnyValue<'static>>>
where
    T: StructPath + Message + Default,
{
    ca.into_iter()
        .map(|opt_bytes| match opt_bytes {
            Some(bytes) => {
                let message = T::decode(bytes)
                    .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                let value = message
                    .get_value(path)
                    .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                Ok(value.into_static())
            }
            None => Ok(AnyValue::Null),
        })
        .collect()
}

fn split_offsets(len: usize, n: usize) -> Vec<(usize, usize)> {
    if n == 1 {
        vec![(0, len)]
    } else {
        let chunk_size = len / n;

        (0..n)
            .map(|partition| {
                let offset = partition * chunk_size;
                let len = if partition == (n - 1) {
                    len - offset
                } else {
                    chunk_size
                };
                (partition * chunk_size, len)
            })
            .collect()
    }
}

/// Extract a field from a `ChunkedArray<BinaryType>` containing encoded protobuf messages.
///
/// This function decodes each binary protobuf message in the chunked array, extracts the
/// field at the specified path, and returns a Polars `Series` containing the extracted values.
/// The function supports parallel processing for improved performance on large datasets.
///
/// # Arguments
///
/// * `ca` - A `ChunkedArray<BinaryType>` containing encoded protobuf messages
/// * `path` - The path to the field to extract (e.g., `"name"`, `"parent.name"`, `"items[0].value"`)
/// * `spawn_threads` - Whether to use parallel processing. If `true`, the function will use
///   Polars' thread pool to process chunks in parallel
///
/// # Type Parameters
///
/// * `T` - The protobuf message type that implements `StructPath`, `Message`, and `Default`
///
/// # Returns
///
/// Returns a `PolarsResult<Series>` containing the extracted field values. The series will have
/// the appropriate data type based on the field type (e.g., `String`, `Int64`, `Float64`, etc.).
/// Null values are represented as `AnyValue::Null` for missing or unset fields.
///
/// # Errors
///
/// Returns `PolarsError::StructFieldNotFound` if the path is invalid.
/// Returns `PolarsError::ComputeError` if a protobuf message fails to decode or if field
/// extraction fails.
///
/// # Example
///
/// ```no_run
/// use polars_core::prelude::{BinaryType, ChunkedArray};
/// use polars_protobuf::get_value;
/// use prost::Message;
///
/// #[derive(polars_structpath::StructPath, Clone, Message)]
/// struct Person {
///     #[prost(string, tag = "1")]
///     name: String,
///     #[prost(int64, tag = "2")]
///     age: i64,
/// }
///
/// fn example() -> Result<(), Box<dyn std::error::Error>> {
///     // Assuming you have a ChunkedArray<BinaryType> containing encoded Person messages
///     let binary_column: ChunkedArray<BinaryType> = todo!();
///
///     // Extract the "name" field
///     let name_series = get_value::<Person>(&binary_column, "name", true)?;
///
///     // Extract the "age" field
///     let age_series = get_value::<Person>(&binary_column, "age", true)?;
///     Ok(())
/// }
/// ```
pub fn get_value<T>(
    ca: &ChunkedArray<BinaryType>,
    path: &str,
    spawn_threads: bool,
) -> PolarsResult<Series>
where
    T: StructPath + Message + Default,
{
    let dtype = T::get_type(path)
        .map_err(|e| PolarsError::StructFieldNotFound(e.to_string().into()))?
        .polars;

    let any_values = if spawn_threads {
        POOL.install(|| -> PolarsResult<Vec<AnyValue<'static>>> {
            let n_threads = POOL.current_num_threads();
            let splits = split_offsets(ca.len(), n_threads);

            Ok(splits
                .into_par_iter()
                .map(|(offset, len)| {
                    let sliced = ca.slice(offset as i64, len);
                    get_value_inner::<T>(&sliced, path)
                })
                .collect::<PolarsResult<Vec<Vec<AnyValue<'static>>>>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<AnyValue<'static>>>())
        })?
    } else {
        get_value_inner::<T>(ca, path)?
    };

    Series::from_any_values_and_dtype("".into(), &any_values, &dtype, true)
}
