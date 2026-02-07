/*
This example demonstrates a roundtrip conversion using the Polars lazy API:
1. Create a Series of protobuf structs
2. Encode structs to bytes using lazy API (struct -> bytes)
3. Decode bytes back to structs using lazy API (bytes -> struct)

This approach is compatible with Polars' streaming engine.
*/

use polars_core::prelude::*;
use polars_lazy::prelude::*;
use polars_protobuf::{decode_expr, encode_expr, messages_to_series};
use polars_structpath::FromArrow;

pub mod benchmark {
    include!(concat!(env!("OUT_DIR"), "/examples/benchmark.rs"));
}

/// Generate sample messages for testing (with strings and enums!)
fn generate_sample_messages(count: usize) -> Vec<benchmark::SampleMessage> {
    (0..count)
        .map(|i| benchmark::SampleMessage {
            f_string: format!("message_{}", i),
            f_integer: i as i64,
            f_double: i as f64 * 1.5,
            f_boolean: i % 2 == 0,
            f_integer_optional: if i % 3 == 0 {
                Some(i as i64 * 10)
            } else {
                None
            },
            f_string_optional: if i % 2 == 0 {
                Some(format!("optional_{}", i))
            } else {
                None
            },
            f_integer_repeated: (0..(i % 3)).map(|j| j as i64).collect(),
            f_string_repeated: (0..(i % 2)).map(|j| format!("rep_{}", j)).collect(),
            f_enum: if i % 2 == 0 {
                benchmark::SampleEnum::UNKNOWN
            } else {
                benchmark::SampleEnum::VARIANT_10
            },
            f_enum_repeated: vec![benchmark::SampleEnum::UNKNOWN],
            f_submessage: Some(benchmark::SampleSubmessage {
                f_string: format!("sub_{}", i),
                f_integer: i as i64,
            }),
            f_submessage_repeated: vec![],
        })
        .collect()
}

fn main() -> PolarsResult<()> {
    println!("=== Polars Lazy API Protobuf Roundtrip Example ===\n");

    // Step 1: Generate sample messages (with strings and enums!)
    let messages = generate_sample_messages(5);
    println!("Step 1: Generated {} sample messages", messages.len());
    for (i, msg) in messages.iter().enumerate() {
        println!(
            "  [{}] f_string={}, f_integer={}, f_enum={:?}",
            i, msg.f_string, msg.f_integer, msg.f_enum
        );
    }

    // Step 2: Convert messages to a Polars Series (struct type)
    let struct_series = messages_to_series(messages.clone(), "messages")?;
    let struct_dtype = struct_series.dtype().clone();
    println!("\nStep 2: Created struct Series");
    println!("  dtype: {:?}", struct_dtype);
    println!("  len: {}", struct_series.len());

    // Step 3: Create a DataFrame with the struct Series
    let df = DataFrame::new(vec![struct_series.into()])?;
    println!("\nStep 3: Created DataFrame");
    println!("{}", df);

    // Step 4: Use lazy API to encode struct -> bytes
    println!("\nStep 4: Encoding structs to bytes using lazy API...");
    let encoded_df = df
        .clone()
        .lazy()
        .select([encode_expr::<benchmark::SampleMessage>(col("messages")).alias("encoded")])
        .collect()?;
    println!("Encoded DataFrame:");
    println!("{}", encoded_df);
    println!("Encoded dtype: {:?}", encoded_df.column("encoded")?.dtype());

    // Step 5: Use lazy API to decode bytes -> struct
    println!("\nStep 5: Decoding bytes back to structs using lazy API...");
    let decoded_df = encoded_df
        .lazy()
        .select([
            decode_expr::<benchmark::SampleMessage>(col("encoded"), struct_dtype.clone())
                .alias("decoded"),
        ])
        .collect()?;
    println!("Decoded DataFrame:");
    println!("{}", decoded_df);

    // Step 6: Verify the roundtrip
    println!("\nStep 6: Verifying roundtrip...");
    let original_series = df.column("messages")?;
    let decoded_series = decoded_df.column("decoded")?;

    // Compare the struct contents
    let original_chunks = original_series
        .as_materialized_series()
        .clone()
        .into_chunks();
    let decoded_chunks = decoded_series
        .as_materialized_series()
        .clone()
        .into_chunks();

    let original_messages = benchmark::SampleMessage::from_arrow(original_chunks[0].clone());
    let decoded_messages = benchmark::SampleMessage::from_arrow(decoded_chunks[0].clone());

    let all_match = original_messages
        .iter()
        .zip(decoded_messages.iter())
        .all(|(orig, dec)| orig == dec);

    if all_match {
        println!(
            "  SUCCESS: All {} messages match after roundtrip!",
            messages.len()
        );
    } else {
        println!("  FAILURE: Some messages don't match after roundtrip");
        for (i, (orig, dec)) in original_messages
            .iter()
            .zip(decoded_messages.iter())
            .enumerate()
        {
            if orig != dec {
                println!("    Mismatch at index {}", i);
                println!("      Original: {:?}", orig);
                println!("      Decoded:  {:?}", dec);
            }
        }
    }

    // Bonus: Demonstrate streaming compatibility
    println!("\n=== Streaming Engine Demonstration ===");
    println!("The lazy API expressions are compatible with Polars streaming engine.");
    println!("For large datasets, you can use:");
    println!("  df.lazy()");
    println!("    .select([encode_expr::<T>(col(\"col\"))])");
    println!("    .with_new_streaming(true)");
    println!("    .collect()");

    Ok(())
}
