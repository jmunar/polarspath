use polars_core::prelude::{
    AnyValue, BinaryType, ChunkedArray, Field, PolarsError, PolarsResult, Series,
};
use polars_core::utils::rayon::iter::{IntoParallelIterator, ParallelIterator};
use polars_core::POOL;
use prost::Message;

use structpath::StructPath;

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

pub fn get_value<T>(
    ca: &ChunkedArray<BinaryType>,
    path: &str,
    parallel: bool,
) -> PolarsResult<Series>
where
    T: StructPath + Message + Default,
{
    let dtype = T::get_type(path)
        .map_err(|e| PolarsError::StructFieldNotFound(e.to_string().into()))?
        .polars;

    let any_values = if parallel {
        get_value_inner::<T>(ca, path)?
    } else {
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
    };

    Series::from_any_values_and_dtype("".into(), &any_values, &dtype, true)
}
