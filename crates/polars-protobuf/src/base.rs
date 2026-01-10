use crate::ArrowMessage;

use polars_core::prelude::{BinaryType, ChunkedArray, PolarsError, PolarsResult, Series};
use polars_core::utils::rayon::iter::{IntoParallelIterator, ParallelIterator};
use polars_core::POOL;

use polars_arrow::array::{Array, ListArray};
use polars_structpath::{FromArrow, IntoArrow};
use polars_structpath_types::ArrowBuffer;

fn decode_inner<T: ArrowMessage + IntoArrow>(
    ca: &ChunkedArray<BinaryType>,
) -> PolarsResult<Box<dyn Array>>
where
    T::Buffer: ArrowBuffer<Element = T>,
{
    let mut buffer = T::new_buffer(ca.len());

    for opt_bytes in ca.into_iter() {
        match opt_bytes {
            Some(bytes) => {
                let message = T::decode(bytes)
                    .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
                buffer.push(message)
            }
            None => buffer.push_null(),
        }
    }

    Ok(Box::new(buffer.to_arrow()?))
}

fn encode_inner<T: ArrowMessage + FromArrow>(
    array: Box<dyn Array>,
) -> PolarsResult<ListArray<i32>>
{
    let mut buffer = <Option<Vec<u8>>>::new_buffer(array.len());

    let messages = T::from_arrow_opt(array);
    for message in messages {
        match message {
            Some(message) => {
                let bytes = message.encode_to_vec();
                buffer.push(bytes)
            }
            None => buffer.push_null(),
        }
    }

    buffer.to_arrow()
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

pub fn decode<T: ArrowMessage + IntoArrow>(
    ca: &ChunkedArray<BinaryType>,
    spawn_threads: bool,
) -> PolarsResult<Series>
where
    T::Buffer: ArrowBuffer<Element = T>,
{
    let array_refs = if spawn_threads {
        POOL.install(|| -> PolarsResult<Vec<Box<dyn Array>>> {
            let n_threads = POOL.current_num_threads();
            let splits = split_offsets(ca.len(), n_threads);
            splits
                .into_par_iter()
                .map(|(offset, len)| {
                    let sliced = ca.slice(offset as i64, len);
                    decode_inner::<T>(&sliced)
                })
                .collect::<Vec<_>>()
                .into_iter()
                .collect::<PolarsResult<Vec<Box<dyn Array>>>>()
        })?
    } else {
        vec![decode_inner::<T>(ca)?]
    };

    Series::from_arrow_chunks("".into(), array_refs)
}

pub fn encode<T: ArrowMessage + FromArrow>(
    series: &Series,
    _spawn_threads: bool,
) -> PolarsResult<Series>
{
    let chunks = series.chunks();

    let arrays: Vec<ListArray<i32>> = chunks
        .iter()
        .map(|chunk| encode_inner::<T>(chunk.clone()))
        .collect::<PolarsResult<Vec<ListArray<i32>>>>()?;
    
    // Combine all encoded arrays into a single Series
    let array_refs: Vec<Box<dyn Array>> = arrays
        .into_iter()
        .map(|arr| Box::new(arr) as Box<dyn Array>)
        .collect();
    
    Series::from_arrow_chunks("".into(), array_refs)
}