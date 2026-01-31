/*
This example benchmarks the protobuf encode/decode operations using the Polars lazy API.
*/

use polars_core::prelude::*;
use polars_lazy::prelude::*;
use polars_protobuf::{decode_expr, encode_expr, messages_to_series, ArrowMessage};
use polars_structpath::{ArrowBuffer, FromArrow, IntoArrow};
use prost::Message;

pub mod benchmark {
    include!(concat!(env!("OUT_DIR"), "/examples/benchmark.rs"));
}

impl benchmark::prost::SampleMessage {
    fn gen_seq(len: usize) -> Vec<Vec<u8>> {
        (0..len)
            .map(|i| {
                let message = benchmark::prost::SampleMessage {
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
                    f_enum: if i % 2 == 0 { 0 } else { 10 },
                    f_enum_repeated: (0..(i % 4))
                        .map(|j| if j % 2 == 0 { 0 } else { 10 })
                        .collect(),
                    f_submessage: Some(benchmark::prost::SampleSubmessage {
                        f_string: format!("f_string{}", i),
                        f_integer: i as i64,
                    }),
                    f_submessage_repeated: (0..(i % 4))
                        .map(|j| benchmark::prost::SampleSubmessage {
                            f_string: format!("f_string{}", j),
                            f_integer: j as i64,
                        })
                        .collect(),
                };
                message.encode_to_vec()
            })
            .collect()
    }
}

fn print_time(label: &str, t0: std::time::Instant) {
    let t1 = std::time::Instant::now();
    println!("    {:<42} {:>10.4} s", label, (t1 - t0).as_secs_f64());
}

fn roundtrip_stepwise(messages_in_bytes: Vec<Vec<u8>>) {
    println!("Prost->Polars stepwise roundtrip");
    let messages_in_bytes_chunked = ChunkedArray::from_iter(messages_in_bytes.clone());

    let t0 = std::time::Instant::now();
    let messages_in = messages_in_bytes_chunked
        .into_iter()
        .map(|sample| benchmark::prost::SampleMessage::decode(sample.unwrap()).unwrap())
        .collect::<Vec<benchmark::prost::SampleMessage>>();
    print_time("Prost decode time:", t0);

    let t0 = std::time::Instant::now();
    let messages_transf = messages_in
        .into_iter()
        .map(<benchmark::SampleMessage as ArrowMessage>::from_prost)
        .collect::<Vec<benchmark::SampleMessage>>();
    print_time("Message transform time:", t0);

    let t0 = std::time::Instant::now();
    let mut buffer = benchmark::SampleMessage::new_buffer(messages_in_bytes_chunked.len());
    for message in messages_transf {
        buffer.push(message);
    }
    print_time("Buffer push time:", t0);

    let t0 = std::time::Instant::now();
    let array = buffer.to_arrow().unwrap();
    print_time("Buffer to arrow time:", t0);

    let t0 = std::time::Instant::now();
    let messages_transf_out = benchmark::SampleMessage::from_arrow(Box::new(array));
    print_time("Arrow to messages time:", t0);

    let t0 = std::time::Instant::now();
    let messages_out = messages_transf_out
        .into_iter()
        .map(|message| message.to_prost())
        .collect::<Vec<benchmark::prost::SampleMessage>>();
    print_time("Messages to prost time:", t0);

    let t0 = std::time::Instant::now();
    let messages_out_bytes = messages_out
        .iter()
        .map(|message| message.encode_to_vec())
        .collect::<Vec<Vec<u8>>>();
    print_time("Messages to prost bytes time:", t0);

    assert_eq!(messages_in_bytes, messages_out_bytes);
}

fn lazy_api_roundtrip(messages_in_bytes: Vec<Vec<u8>>) -> PolarsResult<()> {
    println!("Prost->Polars lazy API roundtrip");

    // Step 1: Decode prost bytes to ArrowMessage structs
    let t0 = std::time::Instant::now();
    let messages: Vec<benchmark::SampleMessage> = messages_in_bytes
        .iter()
        .map(|bytes| {
            let prost_msg = benchmark::prost::SampleMessage::decode(bytes.as_slice()).unwrap();
            benchmark::SampleMessage::from_prost(prost_msg)
        })
        .collect();
    print_time("Prost decode + transform time:", t0);

    // Step 2: Convert to Polars Series
    let t0 = std::time::Instant::now();
    let struct_series = messages_to_series(messages, "messages")?;
    let struct_dtype = struct_series.dtype().clone();
    print_time("Messages to series time:", t0);

    // Step 3: Create DataFrame
    let df = DataFrame::new(vec![struct_series.into()])?;

    // Step 4: Encode using lazy API
    let t0 = std::time::Instant::now();
    let encoded_df = df
        .clone()
        .lazy()
        .select([encode_expr::<benchmark::SampleMessage>(col("messages")).alias("encoded")])
        .collect()?;
    print_time("Lazy encode time:", t0);

    // Step 5: Decode using lazy API
    let t0 = std::time::Instant::now();
    let decoded_df = encoded_df
        .lazy()
        .select([
            decode_expr::<benchmark::SampleMessage>(col("encoded"), struct_dtype).alias("decoded"),
        ])
        .collect()?;
    print_time("Lazy decode time:", t0);

    // Step 6: Extract messages from decoded DataFrame
    let t0 = std::time::Instant::now();
    let decoded_chunks = decoded_df
        .column("decoded")?
        .as_materialized_series()
        .clone()
        .into_chunks();
    let decoded_messages = benchmark::SampleMessage::from_arrow(decoded_chunks[0].clone());
    print_time("Arrow to messages time:", t0);

    // Step 7: Convert back to prost bytes
    let t0 = std::time::Instant::now();
    let messages_out_bytes: Vec<Vec<u8>> = decoded_messages
        .into_iter()
        .map(|msg| msg.to_prost().encode_to_vec())
        .collect();
    print_time("Messages to prost bytes time:", t0);

    assert_eq!(messages_in_bytes, messages_out_bytes);
    Ok(())
}

fn main() -> PolarsResult<()> {
    let messages_in_bytes = benchmark::prost::SampleMessage::gen_seq(100000);

    roundtrip_stepwise(messages_in_bytes.clone());
    println!();
    lazy_api_roundtrip(messages_in_bytes)?;

    Ok(())
}
