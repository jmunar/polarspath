/*
This example benchmarks the direct access to the protobuf fields vs the use of the
protobuf_sample_polars library.
*/

use polars_core::prelude::{
    AnyValue, BinaryType, BooleanType, ChunkedArray, Float64Type, Int64Type, IntoSeries, ListType,
    Series, StringType,
};
use prost::Message;
use protobuf_sample_polars::extract_impl;

#[derive(structpath::StructPath, Clone, PartialEq, Message)]
pub struct SampleMessage {
    #[prost(string, tag = "1")]
    pub f_string: String,
    #[prost(int64, tag = "2")]
    pub f_integer: i64,
    #[prost(double, tag = "3")]
    pub f_double: f64,
    #[prost(bool, tag = "4")]
    pub f_boolean: bool,

    #[prost(int64, optional, tag = "5")]
    pub f_integer_optional: Option<i64>,
    #[prost(string, optional, tag = "6")]
    pub f_string_optional: Option<String>,

    #[prost(int64, repeated, tag = "7")]
    pub f_integer_repeated: Vec<i64>,
    #[prost(string, repeated, tag = "8")]
    pub f_string_repeated: Vec<String>,
}

impl SampleMessage {
    fn gen_seq(len: usize) -> ChunkedArray<BinaryType> {
        let samples: Vec<Vec<u8>> = (0..len)
            .map(|i| {
                let message = SampleMessage {
                    f_string: format!("f_string{}", i),
                    f_integer: i as i64,
                    f_double: i as f64,
                    f_boolean: i % 2 == 0,
                    f_integer_optional: if i % 2 == 0 { Some(i as i64) } else { None },
                    f_string_optional: if i % 2 == 0 {
                        Some(format!("f_string{}", i))
                    } else {
                        None
                    },
                    f_integer_repeated: (0..(i % 4)).map(|j| j as i64).collect(),
                    f_string_repeated: (0..(i % 4)).map(|j| format!("f_string{}", j)).collect(),
                };
                message.encode_to_vec()
            })
            .collect();
        ChunkedArray::from_iter(samples)
    }
}

fn benchmark_prost_decode(samples: &ChunkedArray<BinaryType>) {
    println!("Prost decode time");
    let t0 = std::time::Instant::now();
    samples.into_iter().for_each(|bytes| {
        let _ = SampleMessage::decode(bytes.unwrap()).unwrap();
    });
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken:              {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );
}

fn benchmark_f_string(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_string");
    let t0 = std::time::Instant::now();
    let result_path = extract_impl::<SampleMessage>(&samples, "f_string").unwrap();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (structpath): {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    let t0 = std::time::Instant::now();
    let result_direct = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            Some(message.f_string)
        })
        .collect::<ChunkedArray<StringType>>()
        .into_series();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (direct):     {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    assert_eq!(result_path, result_direct);
}

fn benchmark_f_integer(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_integer");
    let t0 = std::time::Instant::now();
    let result_path = extract_impl::<SampleMessage>(&samples, "f_integer").unwrap();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (structpath): {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    let t0 = std::time::Instant::now();
    let result_direct = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            Some(message.f_integer)
        })
        .collect::<ChunkedArray<Int64Type>>()
        .into_series();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (direct):     {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    assert_eq!(result_path, result_direct);
}

fn benchmark_f_double(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_double");
    let t0 = std::time::Instant::now();
    let result_path = extract_impl::<SampleMessage>(&samples, "f_double").unwrap();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (structpath): {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    let t0 = std::time::Instant::now();
    let result_direct = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            Some(message.f_double)
        })
        .collect::<ChunkedArray<Float64Type>>()
        .into_series();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (direct):     {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    assert_eq!(result_path, result_direct);
}

fn benchmark_f_boolean(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_boolean");
    let t0 = std::time::Instant::now();
    let result_path = extract_impl::<SampleMessage>(&samples, "f_boolean").unwrap();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (structpath): {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    let t0 = std::time::Instant::now();
    let result_direct = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            Some(message.f_boolean)
        })
        .collect::<ChunkedArray<BooleanType>>()
        .into_series();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (direct):     {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    assert_eq!(result_path, result_direct);
}

fn benchmark_f_integer_optional(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_integer_optional");
    let t0 = std::time::Instant::now();
    let result_path = extract_impl::<SampleMessage>(&samples, "f_integer_optional").unwrap();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (structpath): {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    let t0 = std::time::Instant::now();
    let result_direct = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            message.f_integer_optional
        })
        .collect::<ChunkedArray<Int64Type>>()
        .into_series();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (direct):     {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    assert_eq!(result_path, result_direct);
}

fn benchmark_f_string_optional(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_string_optional");
    let t0 = std::time::Instant::now();
    let result_path = extract_impl::<SampleMessage>(&samples, "f_string_optional").unwrap();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (structpath): {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    let t0 = std::time::Instant::now();
    let result_direct = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            message.f_string_optional
        })
        .collect::<ChunkedArray<StringType>>()
        .into_series();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (direct):     {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    assert_eq!(result_path, result_direct);
}

fn benchmark_f_integer_repeated(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_integer_repeated");
    let t0 = std::time::Instant::now();
    let result_path = extract_impl::<SampleMessage>(&samples, "f_integer_repeated").unwrap();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (structpath): {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    let t0 = std::time::Instant::now();
    let result_direct = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            let vec_values: ChunkedArray<Int64Type> =
                message.f_integer_repeated.into_iter().map(Some).collect();
            Some(vec_values.into_series())
        })
        .collect::<ChunkedArray<ListType>>()
        .into_series();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (direct):     {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    // Optimized version using AnyValue to avoid intermediate allocations
    let t0 = std::time::Instant::now();
    let any_values = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            // Create ChunkedArray directly from Vec<i64> without intermediate Series
            let inner_ca: ChunkedArray<Int64Type> =
                message.f_integer_repeated.into_iter().map(Some).collect();
            AnyValue::List(inner_ca.into_series())
        })
        .collect::<Vec<AnyValue>>();

    let result_optimized = Series::from_any_values("".into(), &any_values, true).unwrap();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (any value):  {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    assert_eq!(result_path, result_direct);
    assert_eq!(result_path, result_optimized);
}

fn benchmark_f_string_repeated(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_string_repeated");
    let t0 = std::time::Instant::now();
    let result_path = extract_impl::<SampleMessage>(&samples, "f_string_repeated").unwrap();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (structpath): {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    let t0 = std::time::Instant::now();
    let result_direct = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            let vec_values: ChunkedArray<StringType> =
                message.f_string_repeated.into_iter().map(Some).collect();
            Some(vec_values.into_series())
        })
        .collect::<ChunkedArray<ListType>>()
        .into_series();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (direct):     {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    // Optimized version using AnyValue to avoid intermediate allocations
    let t0 = std::time::Instant::now();
    let any_values = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            // Create ChunkedArray directly from Vec<String> without intermediate Series
            let inner_ca: ChunkedArray<StringType> =
                message.f_string_repeated.into_iter().map(Some).collect();
            AnyValue::List(inner_ca.into_series())
        })
        .collect::<Vec<AnyValue>>();

    let result_optimized = Series::from_any_values("".into(), &any_values, true).unwrap();
    let t1 = std::time::Instant::now();
    println!(
        "    Time taken (any value):  {:>8.4} s",
        (t1 - t0).as_secs_f64()
    );

    assert_eq!(result_path, result_direct);
    assert_eq!(result_path, result_optimized);
}

fn main() {
    let samples = SampleMessage::gen_seq(100000);
    benchmark_prost_decode(&samples);
    benchmark_f_string(&samples);
    benchmark_f_integer(&samples);
    benchmark_f_double(&samples);
    benchmark_f_boolean(&samples);
    benchmark_f_integer_optional(&samples);
    benchmark_f_string_optional(&samples);
    benchmark_f_integer_repeated(&samples);
    benchmark_f_string_repeated(&samples);
}
