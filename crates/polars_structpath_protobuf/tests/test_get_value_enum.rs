use polars_core::prelude::{BinaryType, ChunkedArray, DataType, StringType};
use polars_structpath_protobuf::get_value;
use prost::Message;

#[derive(polars_structpath::EnumPath, Clone, PartialEq)]
pub enum MsgEnum {
    UNKNOWN = 0,
    KNOWN = 1,
}

#[derive(polars_structpath::StructPath, Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    #[type_hint("enum", "MsgEnum")]
    #[prost(int32, tag = "1")]
    pub req_enum: i32,
}

impl Msg {
    fn gen_seq(len: usize) -> ChunkedArray<BinaryType> {
        let samples: Vec<Vec<u8>> = (0..len)
            .map(|i| {
                let message = Msg {
                    req_enum: (i % 2) as i32,
                };
                message.encode_to_vec()
            })
            .collect();
        ChunkedArray::from_iter(samples)
    }
}

#[test]
fn test_get_value_req_enum() {
    let samples = Msg::gen_seq(10);
    let result = get_value::<Msg>(&samples, "req_enum", false).unwrap();
    // Cast enum to string to access string values
    let string_series = result.cast(&DataType::String).unwrap();
    let string_series: &ChunkedArray<StringType> = string_series.str().unwrap();
    for (i, opt_value) in string_series.into_iter().enumerate() {
        let expected = if i % 2 == 0 { "UNKNOWN" } else { "KNOWN" };
        assert_eq!(opt_value.unwrap(), expected);
    }
}
