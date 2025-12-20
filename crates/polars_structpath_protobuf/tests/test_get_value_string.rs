use polars_core::prelude::{BinaryType, ChunkedArray, StringType};
use polars_structpath_protobuf::get_value;
use prost::Message;

#[derive(polars_structpath::StructPath, Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    #[prost(string, tag = "1")]
    pub req_string: String,
}

impl Msg {
    fn gen_seq(len: usize) -> ChunkedArray<BinaryType> {
        let samples: Vec<Vec<u8>> = (0..len)
            .map(|i| {
                let message = Msg {
                    req_string: format!("req_string{}", i),
                };
                message.encode_to_vec()
            })
            .collect();
        ChunkedArray::from_iter(samples)
    }
}

#[test]
fn test_get_value_req_string() {
    let samples = Msg::gen_seq(10);
    let result = get_value::<Msg>(&samples, "req_string", true).unwrap();
    let string_series: &ChunkedArray<StringType> = result.str().unwrap();
    for (i, opt_value) in string_series.into_iter().enumerate() {
        let expected = format!("req_string{}", i);
        assert_eq!(opt_value.unwrap(), expected);
    }
}
