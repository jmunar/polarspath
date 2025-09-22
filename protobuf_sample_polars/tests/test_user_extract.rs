use polars_core::prelude::{BinaryType, ChunkedArray, StringType};
use prost::Message;
use protobuf_sample_polars::get_value;

#[derive(structpath::StructPath, Clone, PartialEq, ::prost::Message)]
pub struct SimpleString {
    #[prost(string, tag = "1")]
    pub name: String,
}

impl SimpleString {
    fn gen_seq(len: usize) -> ChunkedArray<BinaryType> {
        let samples: Vec<Vec<u8>> = (0..len)
            .map(|i| {
                let message = SimpleString {
                    name: format!("name{}", i),
                };
                message.encode_to_vec()
            })
            .collect();
        ChunkedArray::from_iter(samples)
    }
}

#[test]
fn test_simple_string() {
    let samples = SimpleString::gen_seq(10);
    let result = get_value::<SimpleString>(&samples, "name").unwrap();
    let string_series: &ChunkedArray<StringType> = result.str().unwrap();
    for (i, opt_value) in string_series.into_iter().enumerate() {
        let expected = format!("name{}", i);
        assert_eq!(opt_value.unwrap(), expected);
    }
}

#[derive(structpath::StructPath, Clone, PartialEq, ::prost::Message)]
pub struct SimpleStrings {
    #[prost(string, repeated, tag = "1")]
    pub names: Vec<String>,
}

impl SimpleStrings {
    fn gen_seq(len: usize) -> ChunkedArray<BinaryType> {
        let samples: Vec<Vec<u8>> = (0..len)
            .map(|i| {
                let message = SimpleStrings {
                    names: (0..i).map(|j| format!("name{}_{}", i, j)).collect(),
                };
                message.encode_to_vec()
            })
            .collect();
        ChunkedArray::from_iter(samples)
    }
}

#[test]
fn test_simple_strings() {
    let samples = SimpleStrings::gen_seq(10);
    let result = get_value::<SimpleStrings>(&samples, "names").unwrap();

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
            let expected = format!("name{}_{}", i, j);
            let actual = string_ca.get(j).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
