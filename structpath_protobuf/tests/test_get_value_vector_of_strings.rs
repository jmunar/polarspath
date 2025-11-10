use polars_core::prelude::{BinaryType, ChunkedArray};
use prost::Message;
use structpath_protobuf::get_value;

#[derive(structpath::StructPath, Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    #[prost(string, repeated, tag = "1")]
    pub req_vec_req_item_string: Vec<String>,
}

impl Msg {
    fn gen_seq(len: usize) -> ChunkedArray<BinaryType> {
        let samples: Vec<Vec<u8>> = (0..len)
            .map(|i| {
                let message = Msg {
                    req_vec_req_item_string: (0..i).map(|j| format!("item{}_{}", i, j)).collect(),
                };
                message.encode_to_vec()
            })
            .collect();
        ChunkedArray::from_iter(samples)
    }
}

#[test]
fn test_get_value_req_vec_req_item_string() {
    let samples = Msg::gen_seq(10);
    let result = get_value::<Msg>(&samples, "req_vec_req_item_string", true).unwrap();

    // Test that we can properly access the list elements as expected
    let series_as_any = result.as_any();
    let list_series = series_as_any
        .downcast_ref::<ChunkedArray<polars_core::datatypes::ListType>>()
        .unwrap();

    // Verify that we have the right number of items and basic structure
    assert_eq!(list_series.len(), 10);

    // For each list item, verify its contents
    for i in 0..10 {
        let list_item = list_series.get_as_series(i).unwrap();
        let string_ca = list_item.str().unwrap();

        // Verify correct number of elements in each sub-list
        assert_eq!(string_ca.len(), i);

        // Verify each string value in the sub-list
        for j in 0..i {
            let expected = format!("item{}_{}", i, j);
            let actual = string_ca.get(j).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
