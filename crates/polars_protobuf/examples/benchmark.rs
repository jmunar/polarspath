/*
This example benchmarks the direct access to the protobuf fields vs the use of the
`polars_protobuf` library.
*/

use polars_core::prelude::*;
use polars_protobuf::ArrowMessage;
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

fn main() {
    let messages_in_bytes_raw = benchmark::prost::SampleMessage::gen_seq(100000);
    let messages_in_bytes = ChunkedArray::from_iter(messages_in_bytes_raw.clone());

    println!("Prost decode and arrow roundtrip time");

    let t0 = std::time::Instant::now();
    let messages_in = messages_in_bytes
        .into_iter()
        .map(|sample| benchmark::prost::SampleMessage::decode(sample.unwrap()).unwrap())
        .collect::<Vec<benchmark::prost::SampleMessage>>();
    print_time("Prost decode time:", t0);

    // let messages_in_copy: Vec<_> = messages_in.clone();

    let t0 = std::time::Instant::now();
    let messages_transf = messages_in
        .into_iter()
        .map(<benchmark::SampleMessage as ArrowMessage>::from_prost)
        .collect::<Vec<benchmark::SampleMessage>>();
    print_time("Message transform time:", t0);

    let t0 = std::time::Instant::now();
    let mut buffer = benchmark::SampleMessage::new_buffer(messages_in_bytes.len());
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

    assert_eq!(messages_in_bytes_raw, messages_out_bytes);
}
