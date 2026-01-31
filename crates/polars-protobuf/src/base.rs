//! Lazy API functions for encoding and decoding protobuf messages with Polars.
//!
//! This module provides `encode_expr` and `decode_expr` functions that work with
//! Polars' lazy API, enabling streaming-compatible protobuf serialization.
//!
//! The encoding and decoding operations are parallelized using Polars' internal
//! thread pool (POOL) for better performance on multi-core systems.

use crate::ArrowMessage;

use polars_arrow::{
    array::{Array, ListArray},
    offset::Offset,
};
use polars_core::{prelude::*, POOL};
use polars_lazy::prelude::*;
use polars_structpath::{ArrowBuffer, FromArrow, IntoArrow};
use rayon::prelude::*;

/// Decodes a ListArray of encoded protobuf messages back into Arrow structs.
///
/// The input is a `ListArray<O>` where each element is a byte array (`Vec<u8>`)
/// containing an encoded protobuf message. The output is an Arrow array containing
/// the decoded messages as structs.
///
/// This function is generic over the offset type `O` (i32 or i64) to handle both
/// `ListArray<i32>` (standard Arrow) and `ListArray<i64>` (Polars internal).
///
/// Decoding is parallelized using Polars' internal thread pool for better performance.
fn decode_inner<O: Offset, T: ArrowMessage + IntoArrow + Send>(
    array: &ListArray<O>,
) -> PolarsResult<Box<dyn Array>>
where
    <T as IntoArrow>::Buffer: ArrowBuffer<Element = T>,
{
    // Extract bytes from each list element (this part must be sequential due to array iteration)
    let byte_slices: Vec<Option<Vec<u8>>> = array
        .iter()
        .map(|opt_array| {
            opt_array.map(|byte_array| {
                let primitive_array = byte_array
                    .as_any()
                    .downcast_ref::<polars_arrow::array::PrimitiveArray<u8>>()
                    .expect("Expected PrimitiveArray<u8>");
                primitive_array.values().as_slice().to_vec()
            })
        })
        .collect();

    // Decode messages in parallel using Polars' thread pool
    let decoded: Vec<Option<T>> = POOL.install(|| {
        byte_slices
            .into_par_iter()
            .map(|opt_bytes| {
                opt_bytes.map(|bytes| {
                    T::decode(bytes.as_slice()).expect("Failed to decode protobuf message")
                })
            })
            .collect()
    });

    // Build the output buffer (sequential, but fast)
    let mut buffer = T::new_buffer(decoded.len());
    for message in decoded {
        match message {
            Some(msg) => buffer.push(msg),
            None => buffer.push_null(),
        }
    }

    Ok(Box::new(buffer.to_arrow()?))
}

/// Encodes Arrow struct arrays into protobuf bytes.
///
/// Takes a boxed Arrow array containing struct data and encodes each element
/// as a protobuf message, returning a `ListArray<i32>` of bytes.
///
/// Encoding is parallelized using Polars' internal thread pool for better performance.
fn encode_inner<T: ArrowMessage + FromArrow + IntoArrow + Send>(
    array: Box<dyn Array>,
) -> PolarsResult<Box<dyn Array>>
where
    <T as IntoArrow>::Buffer: ArrowBuffer<Element = T>,
{
    // Extract messages from array (sequential)
    let messages = T::from_arrow_opt(array);

    // Encode messages to bytes in parallel using Polars' thread pool
    let encoded: Vec<Option<Vec<u8>>> = POOL.install(|| {
        messages
            .into_par_iter()
            .map(|opt_msg| opt_msg.map(|msg| msg.encode_to_vec()))
            .collect()
    });

    // Build the output buffer (sequential, but fast)
    let mut buffer = <Vec<u8>>::new_buffer(encoded.len());
    for bytes in encoded {
        match bytes {
            Some(b) => buffer.push(b),
            None => buffer.push_null(),
        }
    }

    Ok(Box::new(buffer.to_arrow()?))
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
            let name = series.name().clone();

            let encoded_chunks: Vec<Box<dyn Array>> = series
                .into_chunks()
                .into_iter()
                .map(|chunk| encode_inner::<T>(chunk))
                .collect::<PolarsResult<Vec<_>>>()?;

            let encoded_series = Series::from_arrow_chunks(name, encoded_chunks)?;
            Ok(encoded_series.into_column())
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
            let name = series.name().clone();

            let decoded_chunks: Vec<Box<dyn Array>> = series
                .into_chunks()
                .into_iter()
                .map(|chunk| {
                    // Try ListArray<i32> first (what encode_inner produces)
                    if let Some(list_array) = chunk.as_any().downcast_ref::<ListArray<i32>>() {
                        return decode_inner::<i32, T>(list_array);
                    }
                    // Try ListArray<i64> (polars may use this internally)
                    if let Some(list_array) = chunk.as_any().downcast_ref::<ListArray<i64>>() {
                        return decode_inner::<i64, T>(list_array);
                    }
                    Err(PolarsError::ComputeError(
                        format!("Expected ListArray, got {:?}", chunk.dtype()).into(),
                    ))
                })
                .collect::<PolarsResult<Vec<_>>>()?;

            let decoded_series = Series::from_arrow_chunks(name, decoded_chunks)?;
            Ok(decoded_series.into_column())
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
