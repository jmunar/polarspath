/*
This example benchmarks the direct access to the protobuf fields vs the use of the
`polars_protobuf` library.
*/

use polars_core::prelude::*;
use polars_structpath::{ArrowBuffer, FromArrow, IntoArrow, EnumPath, StructPath};
use prost::Message;

pub mod benchmark {
    include!(concat!(env!("OUT_DIR"), "/benchmark.rs"));
}

impl benchmark::SampleMessage {
    fn gen_seq(len: usize) -> Vec<Vec<u8>> {
        (0..len)
            .map(|i| {
                let message = benchmark::SampleMessage {
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
                    f_enum_repeated: (0..(i % 4)).map(|j| if j % 2 == 0 { 0 } else { 10 }).collect(),
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
                };
                message.encode_to_vec()
            })
            .collect()
    }
}

#[derive(EnumPath, Debug, Clone, PartialEq)]
pub enum SampleEnumArrow {
    UNKNOWN = 0,
    #[allow(clippy::upper_case_acronyms, non_camel_case_types)]
    VARIANT_10 = 10,
}

// impl_enum_buffer!(SampleEnumArrow, [(UNKNOWN, 0), (VARIANT_10, 10)]);

#[derive(StructPath, Debug, Clone, PartialEq)]
pub struct SampleSubmessageArrow {
    f_string: String,
    f_integer: i64,
}

impl SampleSubmessageArrow {
    fn from_prost(message: benchmark::SampleSubmessage) -> Self {
        Self {
            f_string: message.f_string,
            f_integer: message.f_integer,
        }
    }

    fn to_prost(self) -> benchmark::SampleSubmessage {
        benchmark::SampleSubmessage {
            f_string: self.f_string,
            f_integer: self.f_integer,
        }
    }
}

#[derive(StructPath, Debug, Clone, PartialEq)]
pub struct SampleMessageArrow {
    f_string: String,
    f_integer: i64,
    f_double: f64,
    f_boolean: bool,
    f_integer_optional: Option<i64>,
    f_string_optional: Option<String>,
    f_integer_repeated: Vec<i64>,
    f_string_repeated: Vec<String>,
    f_enum: SampleEnumArrow,
    f_enum_repeated: Vec<SampleEnumArrow>,
    f_submessage: Option<SampleSubmessageArrow>,
    f_submessage_repeated: Vec<SampleSubmessageArrow>,
}

impl SampleMessageArrow {
    fn from_prost(message: benchmark::SampleMessage) -> Self {
        Self {
            f_string: message.f_string,
            f_integer: message.f_integer,
            f_double: message.f_double,
            f_boolean: message.f_boolean,
            f_integer_optional: message.f_integer_optional,
            f_string_optional: message.f_string_optional,
            f_integer_repeated: message.f_integer_repeated,
            f_string_repeated: message.f_string_repeated,
            f_enum: SampleEnumArrow::from_rust_idx(message.f_enum),
            f_enum_repeated: message.f_enum_repeated.into_iter().map(|enum_value| SampleEnumArrow::from_rust_idx(enum_value)).collect(),
            f_submessage: message.f_submessage.map(|submessage| SampleSubmessageArrow::from_prost(submessage)),
            f_submessage_repeated: message.f_submessage_repeated.into_iter().map(|submessage| SampleSubmessageArrow::from_prost(submessage)).collect(),
        }
    }

    fn to_prost(self) -> benchmark::SampleMessage {
        benchmark::SampleMessage {
            f_string: self.f_string,
            f_integer: self.f_integer,
            f_double: self.f_double,
            f_boolean: self.f_boolean,
            f_integer_optional: self.f_integer_optional,
            f_string_optional: self.f_string_optional,
            f_integer_repeated: self.f_integer_repeated,
            f_string_repeated: self.f_string_repeated,
            f_enum: self.f_enum as i32,
            f_enum_repeated: self.f_enum_repeated.into_iter().map(|enum_value| enum_value as i32).collect(),
            f_submessage: self.f_submessage.map(|submessage| submessage.to_prost()),
            f_submessage_repeated: self.f_submessage_repeated.into_iter().map(|submessage| submessage.to_prost()).collect(),
        }
    }
}

fn print_time(label: &str, t0: std::time::Instant) {
    let t1 = std::time::Instant::now();
    println!("    {:<42} {:>10.4} s", label, (t1 - t0).as_secs_f64());
}

fn main() {
    let messages_in_bytes_raw = benchmark::SampleMessage::gen_seq(100000);
    let messages_in_bytes = ChunkedArray::from_iter(messages_in_bytes_raw.clone());

    println!("Prost decode and arrow roundtrip time");

    let t0 = std::time::Instant::now();
    let messages_in = messages_in_bytes
        .into_iter()
        .map(|sample| benchmark::SampleMessage::decode(sample.unwrap()).unwrap())
        .collect::<Vec<benchmark::SampleMessage>>();
    print_time("Prost decode time:", t0);

    // let messages_in_copy: Vec<_> = messages_in.clone();

    let t0 = std::time::Instant::now();
    let messages_transf = messages_in.into_iter().map(SampleMessageArrow::from_prost)
        .collect::<Vec<SampleMessageArrow>>();
    print_time("Message transform time:", t0);

    let t0 = std::time::Instant::now();
    let mut buffer = SampleMessageArrow::new_buffer(messages_in_bytes.len());
    for message in messages_transf {
        buffer.push(message);
    }
    print_time("Buffer push time:", t0);

    let t0 = std::time::Instant::now();
    let array = buffer.to_arrow().unwrap();
    print_time("Buffer to arrow time:", t0);

    let t0 = std::time::Instant::now();
    let messages_transf_out = SampleMessageArrow::from_arrow(Box::new(array));
    print_time("Arrow to messages time:", t0);

    let t0 = std::time::Instant::now();
    let messages_out = messages_transf_out.into_iter().map(|message| message.to_prost()).collect::<Vec<benchmark::SampleMessage>>();
    print_time("Messages to prost time:", t0);

    let t0 = std::time::Instant::now();
    let messages_out_bytes = messages_out.iter().map(|message| message.encode_to_vec()).collect::<Vec<Vec<u8>>>();
    print_time("Messages to prost bytes time:", t0);

    assert_eq!(messages_in_bytes_raw, messages_out_bytes);
}
