/*
This example benchmarks protobuf encode/decode operations comparing:
1. Sequential processing (manual step-by-step, no parallelism)
2. Polars lazy API (uses encode_expr/decode_expr)
3. Direct API (uses encode_series/decode_series with chunk-level parallelism)

All methods perform the same roundtrip:
  Series (struct) -> encode -> Series (bytes) -> decode -> Series (struct)
*/

use polars_core::prelude::*;
use polars_lazy::prelude::*;
use polars_protobuf::{
    decode_expr, decode_series, encode_expr, encode_series, messages_to_series, ArrowMessage,
};
use polars_structpath::{ArrowBuffer, FromArrow, IntoArrow};

pub mod benchmark {
    include!(concat!(env!("OUT_DIR"), "/examples/benchmark.rs"));
}

fn generate_messages(count: usize) -> Vec<benchmark::SampleMessage> {
    (0..count)
        .map(|i| benchmark::SampleMessage {
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
            f_enum: if i % 2 == 0 {
                benchmark::SampleEnum::UNKNOWN
            } else {
                benchmark::SampleEnum::VARIANT_10
            },
            f_enum_repeated: (0..(i % 4))
                .map(|j| {
                    if j % 2 == 0 {
                        benchmark::SampleEnum::UNKNOWN
                    } else {
                        benchmark::SampleEnum::VARIANT_10
                    }
                })
                .collect(),
            f_submessage: Some(benchmark::SampleSubmessage {
                f_string: format!("f_string{}", i),
                f_integer: i as i64,
            }),
            f_submessage_repeated: (0..(i % 4))
                .map(|j| benchmark::SampleSubmessage {
                    f_string: format!("f_string{}", j),
                    f_integer: j as i64,
                })
                .collect(),
        })
        .collect()
}

fn print_time(label: &str, t0: std::time::Instant) -> f64 {
    let elapsed = t0.elapsed().as_secs_f64();
    println!("    {:<70} {:>10.4} s", label, elapsed);
    elapsed
}

fn print_total(total: f64) {
    println!("    {:-<70} {:->10.4} s", "", "");
    println!("    {:<70} {:>10.4} s", "TOTAL", total);
}

/// Sequential roundtrip: processes messages one at a time using Arrow arrays directly
fn roundtrip_sequential(input_series: &Series) -> PolarsResult<Series> {
    println!("=== Sequential Roundtrip ===");
    let mut total = 0.0;

    // Step 1: Extract messages from input Series
    let t0 = std::time::Instant::now();
    let chunks = input_series.clone().into_chunks();
    let messages = benchmark::SampleMessage::from_arrow(chunks[0].clone());
    total += print_time("1. Extract messages (Series -> Vec<Message>)", t0);

    // Step 2: Encode each message to protobuf bytes (sequential)
    let t0 = std::time::Instant::now();
    let encoded_bytes: Vec<Vec<u8>> = messages.iter().map(|msg| msg.encode_to_vec()).collect();
    total += print_time("2. Encode to bytes (Vec<Message> -> Vec<bytes>)", t0);

    // Step 3: Convert bytes to Series
    let t0 = std::time::Instant::now();
    let bytes_series = {
        let mut buffer = <Vec<u8>>::new_buffer(encoded_bytes.len());
        for bytes in encoded_bytes.clone() {
            buffer.push(bytes);
        }
        let array = buffer.to_arrow()?;
        Series::from_arrow_chunks("encoded".into(), vec![Box::new(array)])?
    };
    total += print_time("3. Bytes to Series (Vec<bytes> -> Series)", t0);

    // Step 4: Extract bytes from Series and decode (sequential)
    let t0 = std::time::Instant::now();
    let decoded_messages: Vec<benchmark::SampleMessage> = encoded_bytes
        .iter()
        .map(|bytes| benchmark::SampleMessage::decode(bytes.as_slice()).unwrap())
        .collect();
    drop(bytes_series); // Ensure we used it
    total += print_time("4. Decode from bytes (Vec<bytes> -> Vec<Message>)", t0);

    // Step 5: Convert messages back to Series
    let t0 = std::time::Instant::now();
    let output_series = messages_to_series(decoded_messages, input_series.name().as_str())?;
    total += print_time("5. Messages to Series (Vec<Message> -> Series)", t0);

    print_total(total);
    Ok(output_series)
}

/// Lazy API roundtrip: uses Polars' lazy API for encode/decode
fn roundtrip_lazy_api(input_series: &Series) -> PolarsResult<Series> {
    println!("=== Lazy API Roundtrip ===");
    let mut total = 0.0;

    let struct_dtype = input_series.dtype().clone();
    let name = input_series.name().clone();

    // Step 1: Create DataFrame from input Series
    let t0 = std::time::Instant::now();
    let df = DataFrame::new(input_series.len(), vec![input_series.clone().into()])?;
    total += print_time("1. Create DataFrame (Series -> DataFrame)", t0);

    // Step 2: Encode using lazy API (struct -> bytes)
    let t0 = std::time::Instant::now();
    let encoded_df = df
        .lazy()
        .select([encode_expr::<benchmark::SampleMessage>(col(name.as_str())).alias("encoded")])
        .collect()?;
    total += print_time("2. Lazy encode (DataFrame[struct] -> DataFrame[bytes])", t0);

    // Step 3: Decode using lazy API (bytes -> struct)
    let t0 = std::time::Instant::now();
    let decoded_df = encoded_df
        .lazy()
        .select([
            decode_expr::<benchmark::SampleMessage>(col("encoded"), struct_dtype)
                .alias(name.as_str()),
        ])
        .collect()?;
    total += print_time("3. Lazy decode (DataFrame[bytes] -> DataFrame[struct])", t0);

    // Step 4: Extract output Series
    let t0 = std::time::Instant::now();
    let output_series = decoded_df
        .column(name.as_str())?
        .as_materialized_series()
        .clone();
    total += print_time("4. Extract Series (DataFrame -> Series)", t0);

    print_total(total);
    Ok(output_series)
}

/// Direct API roundtrip: uses encode_series/decode_series with chunk-level parallelism
fn roundtrip_direct(input_series: &Series) -> PolarsResult<Series> {
    println!("=== Direct API Roundtrip (encode_series/decode_series) ===");
    let mut total = 0.0;

    // Step 1: Encode Series (struct -> bytes)
    let t0 = std::time::Instant::now();
    let encoded_series = encode_series::<benchmark::SampleMessage>(input_series.clone())?;
    total += print_time("1. encode_series (Series[struct] -> Series[bytes])", t0);

    // Step 2: Decode Series (bytes -> struct)
    let t0 = std::time::Instant::now();
    let output_series = decode_series::<benchmark::SampleMessage>(encoded_series)?;
    total += print_time("2. decode_series (Series[bytes] -> Series[struct])", t0);

    print_total(total);
    Ok(output_series)
}

fn verify_roundtrip(input: &Series, output: &Series, method_name: &str) {
    let input_messages: Vec<benchmark::SampleMessage> = input
        .clone()
        .into_chunks()
        .into_iter()
        .flat_map(benchmark::SampleMessage::from_arrow)
        .collect();
    let output_messages: Vec<benchmark::SampleMessage> = output
        .clone()
        .into_chunks()
        .into_iter()
        .flat_map(benchmark::SampleMessage::from_arrow)
        .collect();

    let all_match = input_messages
        .iter()
        .zip(output_messages.iter())
        .all(|(a, b)| a == b);

    if all_match {
        println!("    ✓ {} roundtrip verified\n", method_name);
    } else {
        println!("    ✗ {} roundtrip FAILED\n", method_name);
        for (i, (a, b)) in input_messages
            .iter()
            .zip(output_messages.iter())
            .enumerate()
        {
            if a != b {
                println!("      Mismatch at index {}", i);
                println!("        Input:  {:?}", a);
                println!("        Output: {:?}", b);
            }
        }
    }
}

fn main() -> PolarsResult<()> {
    let num_messages = 100_000;
    println!("Generating {} sample messages...", num_messages);

    let t0 = std::time::Instant::now();
    let messages = generate_messages(num_messages);
    let input_series = messages_to_series(messages, "messages")?;
    println!(
        "Input Series created in {:.4}s (dtype: {:?})\n",
        t0.elapsed().as_secs_f64(),
        input_series.dtype()
    );

    // Run sequential roundtrip (baseline, no parallelism)
    let output_seq = roundtrip_sequential(&input_series)?;
    verify_roundtrip(&input_series, &output_seq, "Sequential");

    // Run lazy API roundtrip (Polars expressions)
    let output_lazy = roundtrip_lazy_api(&input_series)?;
    verify_roundtrip(&input_series, &output_lazy, "Lazy API");

    // Run direct API roundtrip (encode_series/decode_series)
    let output_direct = roundtrip_direct(&input_series)?;
    verify_roundtrip(&input_series, &output_direct, "Direct API");

    Ok(())
}
