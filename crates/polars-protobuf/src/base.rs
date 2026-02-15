//! Lazy API functions for encoding and decoding protobuf messages with Polars.
//!
//! This module provides `encode_expr` and `decode_expr` functions that work with
//! Polars' lazy API, enabling streaming-compatible protobuf serialization.
//!
//! Encoding and decoding operations use chunk-level parallelization via Polars'
//! internal thread pool (POOL). The input Series is split into chunks that are
//! processed independently in parallel, enabling efficient multi-core utilization
//! for both Arrow conversions and protobuf serialization.

use crate::ArrowMessage;

use polars_arrow::{
    array::{Array, BinaryViewArray, ListArray},
    offset::Offset,
};
use polars_core::{prelude::*, POOL};
use polars_lazy::prelude::*;
use polars_structpath::{ArrowBuffer, FromArrow, IntoArrow};
use rayon::prelude::*;

/// Decodes a ListArray of encoded protobuf messages back into Arrow structs (sequential).
///
/// The input is a `ListArray<O>` where each element is a byte array (`Vec<u8>`)
/// containing an encoded protobuf message. The output is an Arrow array containing
/// the decoded messages as structs.
///
/// This function is generic over the offset type `O` (i32 or i64) to handle both
/// `ListArray<i32>` (standard Arrow) and `ListArray<i64>` (Polars internal).
///
/// This function is sequential; chunk-level parallelism is handled by `decode_series`.
fn decode_inner_list<O: Offset, T: ArrowMessage + IntoArrow>(
    array: &ListArray<O>,
) -> PolarsResult<Box<dyn Array>>
where
    <T as IntoArrow>::Buffer: ArrowBuffer<Element = T>,
{
    let mut buffer = T::new_buffer(array.len());
    for opt_array in array.iter() {
        match opt_array {
            Some(byte_array) => {
                let primitive_array = byte_array
                    .as_any()
                    .downcast_ref::<polars_arrow::array::PrimitiveArray<u8>>()
                    .expect("Expected PrimitiveArray<u8>");
                let bytes = primitive_array.values().as_slice();
                buffer.push(T::decode(bytes).expect("Failed to decode protobuf message"));
            }
            None => buffer.push_null(),
        }
    }
    Ok(Box::new(buffer.to_arrow()?))
}

/// Decodes a BinaryViewArray of encoded protobuf messages back into Arrow structs (sequential).
///
/// The input is a `BinaryViewArray` where each element is a byte slice containing
/// an encoded protobuf message. This is the format produced by Python's
/// `SerializeToString()` when stored in a Polars Binary column.
///
/// This function is sequential; chunk-level parallelism is handled by `decode_series`.
fn decode_inner_binary<T: ArrowMessage + IntoArrow>(
    array: &BinaryViewArray,
) -> PolarsResult<Box<dyn Array>>
where
    <T as IntoArrow>::Buffer: ArrowBuffer<Element = T>,
{
    let mut buffer = T::new_buffer(array.len());
    for opt_bytes in array.iter() {
        match opt_bytes {
            Some(bytes) => {
                buffer.push(T::decode(bytes).expect("Failed to decode protobuf message"));
            }
            None => buffer.push_null(),
        }
    }
    Ok(Box::new(buffer.to_arrow()?))
}

/// Encodes Arrow struct arrays into protobuf bytes (sequential).
///
/// Takes a boxed Arrow array containing struct data and encodes each element
/// as a protobuf message, returning a `ListArray<i32>` of bytes.
///
/// This function is sequential; chunk-level parallelism is handled by `encode_series`.
fn encode_inner<T: ArrowMessage + FromArrow + IntoArrow>(
    array: Box<dyn Array>,
) -> PolarsResult<Box<dyn Array>>
where
    <T as IntoArrow>::Buffer: ArrowBuffer<Element = T>,
{
    let messages = T::from_arrow_opt(array);
    let mut buffer = <Vec<u8>>::new_buffer(messages.len());
    for opt_msg in messages {
        match opt_msg {
            Some(msg) => buffer.push(msg.encode_to_vec()),
            None => buffer.push_null(),
        }
    }
    Ok(Box::new(buffer.to_arrow()?))
}

/// Minimum chunk size for parallel processing. Chunks smaller than this are not
/// worth the overhead of parallel dispatch.
const MIN_CHUNK_SIZE: usize = 10_000;

/// Splits a Series into slices for parallel processing.
///
/// The number of slices is determined by the thread pool size, targeting ~2 slices
/// per thread for work-stealing headroom, with a minimum chunk size to avoid overhead.
fn split_series(series: &Series) -> Vec<Series> {
    let len = series.len();
    if len == 0 {
        return vec![series.clone()];
    }
    let num_threads = POOL.current_num_threads();
    let num_chunks = (num_threads * 2).min(len / MIN_CHUNK_SIZE).max(1);
    let chunk_size = len.div_ceil(num_chunks);
    (0..len)
        .step_by(chunk_size)
        .map(|offset| {
            let size = chunk_size.min(len - offset);
            series.slice(offset as i64, size)
        })
        .collect()
}

/// Encodes a Series of struct data into protobuf bytes.
///
/// Takes a Series containing struct data and encodes each element as a protobuf
/// message, returning a Series of `List(UInt8)` containing the encoded bytes.
///
/// The Series is split into chunks that are processed in parallel using Polars'
/// internal thread pool, parallelizing both Arrow conversions and protobuf encoding.
///
/// This is the core encode operation used by both `encode_expr` (lazy API) and
/// generated pyo3-polars plugin functions.
///
/// # Type Parameters
///
/// * `T` - The message type implementing `ArrowMessage`, `FromArrow`, and `IntoArrow`
///
/// # Arguments
///
/// * `series` - The input Series containing struct data to encode
///
/// # Returns
///
/// A `PolarsResult<Series>` containing `List(UInt8)` with encoded protobuf bytes
pub fn encode_series<T: ArrowMessage + FromArrow + IntoArrow + Send>(
    series: Series,
) -> PolarsResult<Series>
where
    <T as IntoArrow>::Buffer: ArrowBuffer<Element = T>,
{
    let name = series.name().clone();
    let slices = split_series(&series);
    let encoded_chunks: PolarsResult<Vec<Box<dyn Array>>> = POOL.install(|| {
        slices
            .into_par_iter()
            .map(|slice| {
                // Each slice has exactly one chunk (produced by Series::slice)
                let array = slice.into_chunks().into_iter().next().unwrap();
                encode_inner::<T>(array)
            })
            .collect()
    });
    Series::from_arrow_chunks(name, encoded_chunks?)
}

/// Decodes a single Arrow array chunk by detecting its type and dispatching
/// to the appropriate decoder.
fn decode_chunk<T: ArrowMessage + IntoArrow>(chunk: Box<dyn Array>) -> PolarsResult<Box<dyn Array>>
where
    <T as IntoArrow>::Buffer: ArrowBuffer<Element = T>,
{
    // Try BinaryViewArray first (Python's SerializeToString produces Binary)
    if let Some(binary_array) = chunk.as_any().downcast_ref::<BinaryViewArray>() {
        return decode_inner_binary::<T>(binary_array);
    }
    // Try ListArray<i32> (what encode_inner produces)
    if let Some(list_array) = chunk.as_any().downcast_ref::<ListArray<i32>>() {
        return decode_inner_list::<i32, T>(list_array);
    }
    // Try ListArray<i64> (polars may use this internally)
    if let Some(list_array) = chunk.as_any().downcast_ref::<ListArray<i64>>() {
        return decode_inner_list::<i64, T>(list_array);
    }
    Err(PolarsError::ComputeError(
        format!(
            "Expected BinaryViewArray or ListArray, got {:?}",
            chunk.dtype()
        )
        .into(),
    ))
}

/// Decodes a Series of protobuf bytes back into struct data.
///
/// Takes a Series containing encoded protobuf bytes (as `BinaryView`, `List(UInt8)`)
/// and decodes each element back into struct data.
///
/// The Series is split into chunks that are processed in parallel using Polars'
/// internal thread pool, parallelizing both protobuf decoding and Arrow conversions.
///
/// This is the core decode operation used by both `decode_expr` (lazy API) and
/// generated pyo3-polars plugin functions.
///
/// # Type Parameters
///
/// * `T` - The message type implementing `ArrowMessage` and `IntoArrow`
///
/// # Arguments
///
/// * `series` - The input Series containing encoded protobuf bytes
///
/// # Returns
///
/// A `PolarsResult<Series>` containing struct data with decoded protobuf messages
pub fn decode_series<T: ArrowMessage + IntoArrow + Send>(series: Series) -> PolarsResult<Series>
where
    <T as IntoArrow>::Buffer: ArrowBuffer<Element = T>,
{
    let name = series.name().clone();
    let slices = split_series(&series);
    let decoded_chunks: PolarsResult<Vec<Box<dyn Array>>> = POOL.install(|| {
        slices
            .into_par_iter()
            .map(|slice| {
                // Each slice has exactly one chunk (produced by Series::slice)
                let array = slice.into_chunks().into_iter().next().unwrap();
                decode_chunk::<T>(array)
            })
            .collect()
    });
    Series::from_arrow_chunks(name, decoded_chunks?)
}

/// Creates a lazy expression that encodes struct data to protobuf bytes.
///
/// This function wraps an input expression and returns a new expression that,
/// when evaluated, encodes the struct column as protobuf binary data.
///
/// # Type Parameters
///
/// * `T` - The message type implementing `ArrowMessage`, `FromArrow`, and `IntoArrow`
///
/// # Arguments
///
/// * `expr_in` - The input expression selecting the struct column to encode
///
/// # Returns
///
/// An `Expr` that produces a `List(UInt8)` column containing encoded protobuf bytes
///
/// # Example
///
/// ```ignore
/// use polars_protobuf::encode_expr;
/// use polars_lazy::prelude::*;
///
/// let encoded_df = df
///     .lazy()
///     .select([encode_expr::<MyMessage>(col("messages")).alias("encoded")])
///     .collect()?;
/// ```
pub fn encode_expr<T: ArrowMessage + FromArrow + IntoArrow + Send + 'static>(expr_in: Expr) -> Expr
where
    <T as IntoArrow>::Buffer: ArrowBuffer<Element = T>,
{
    expr_in.map(
        |column| {
            let series = column.as_materialized_series().clone();
            Ok(encode_series::<T>(series)?.into_column())
        },
        |_, field| {
            let list_dtype = DataType::List(Box::new(DataType::UInt8));
            Ok(Field::new(field.name().clone(), list_dtype))
        },
    )
}

/// Creates a lazy expression that decodes protobuf bytes back to struct data.
///
/// This function wraps an input expression and returns a new expression that,
/// when evaluated, decodes the binary protobuf data back to struct columns.
///
/// # Type Parameters
///
/// * `T` - The message type implementing `ArrowMessage` and `IntoArrow`
///
/// # Arguments
///
/// * `expr_in` - The input expression selecting the binary column to decode
/// * `output_dtype` - The expected output `DataType` (must be a Struct type matching `T`)
///
/// # Returns
///
/// An `Expr` that produces a struct column containing decoded protobuf messages
///
/// # Example
///
/// ```ignore
/// use polars_protobuf::decode_expr;
/// use polars_lazy::prelude::*;
///
/// let decoded_df = encoded_df
///     .lazy()
///     .select([decode_expr::<MyMessage>(col("encoded"), struct_dtype).alias("decoded")])
///     .collect()?;
/// ```
pub fn decode_expr<T: ArrowMessage + IntoArrow + Send + 'static>(
    expr_in: Expr,
    output_dtype: DataType,
) -> Expr
where
    <T as IntoArrow>::Buffer: ArrowBuffer<Element = T>,
{
    expr_in.map(
        |column| {
            let series = column.as_materialized_series().clone();
            Ok(decode_series::<T>(series)?.into_column())
        },
        move |_, field| Ok(Field::new(field.name().clone(), output_dtype.clone())),
    )
}

/// Converts a vector of `ArrowMessage` structs into a Polars Series.
///
/// This is a convenience function for creating a Series from Rust structs
/// that implement the `ArrowMessage` trait.
///
/// # Type Parameters
///
/// * `T` - The message type implementing `ArrowMessage` and `IntoArrow`
///
/// # Arguments
///
/// * `messages` - Vector of messages to convert
/// * `name` - Name for the resulting Series
///
/// # Returns
///
/// A `PolarsResult<Series>` containing the messages as a struct column
///
/// # Example
///
/// ```ignore
/// use polars_protobuf::messages_to_series;
///
/// let messages = vec![MyMessage { field: "value".into() }];
/// let series = messages_to_series(messages, "my_column")?;
/// ```
pub fn messages_to_series<T: ArrowMessage + IntoArrow>(
    messages: Vec<T>,
    name: &str,
) -> PolarsResult<Series>
where
    <T as IntoArrow>::Buffer: ArrowBuffer<Element = T>,
{
    let mut buffer = T::new_buffer(messages.len());
    for message in messages {
        buffer.push(message);
    }
    let array = buffer.to_arrow()?;
    Series::from_arrow_chunks(name.into(), vec![Box::new(array)])
}
