mod sample;
use sample::{sample_struct, SampleEnum};

use polars_core::prelude::{AnyValue, DataType, Field, Series};
use polars_structpath_types::{EnumPath, StructPath};

#[test]
fn test_field_to_any_value_req_fields() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = sample_struct();

    let any_value = sample_struct.get_value("req_string")?;
    assert_eq!(any_value, AnyValue::StringOwned("req_string".into()));

    let any_value = sample_struct.get_value("req_bytes")?;
    assert_eq!(any_value, AnyValue::BinaryOwned(b"req_bytes".to_vec()));

    let any_value = sample_struct.get_value("req_i32")?;
    assert_eq!(any_value, AnyValue::Int32(1));

    let any_value = sample_struct.get_value("req_i64")?;
    assert_eq!(any_value, AnyValue::Int64(1));

    let any_value = sample_struct.get_value("req_u32")?;
    assert_eq!(any_value, AnyValue::UInt32(1));

    let any_value = sample_struct.get_value("req_u64")?;
    assert_eq!(any_value, AnyValue::UInt64(1));

    let any_value = sample_struct.get_value("req_f32")?;
    assert_eq!(any_value, AnyValue::Float32(1.0f32));

    let any_value = sample_struct.get_value("req_f64")?;
    assert_eq!(any_value, AnyValue::Float64(1.0));

    let any_value = sample_struct.get_value("req_bool")?;
    assert_eq!(any_value, AnyValue::Boolean(true));

    let any_value = sample_struct.get_value("req_struct")?;
    assert_eq!(
        any_value,
        AnyValue::StructOwned(Box::new((
            vec![AnyValue::StringOwned("subf_string1".into())],
            Vec::from([Field::new("subf_string".into(), DataType::String)])
        )))
    );

    let any_value = sample_struct.get_value("req_enum")?;
    assert_eq!(any_value, AnyValue::Enum(0, SampleEnum::mapping()));

    let any_value = sample_struct.get_value("req_enum2")?;
    assert_eq!(any_value, AnyValue::Enum(0, SampleEnum::mapping()));

    Ok(())
}

#[test]
fn test_field_to_any_value_opt_fields() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = sample_struct();

    let any_value = sample_struct.get_value("opt_string")?;
    assert_eq!(any_value, AnyValue::StringOwned("opt_string".into()));

    let any_value = sample_struct.get_value("opt_bytes")?;
    assert_eq!(any_value, AnyValue::BinaryOwned(b"opt_bytes".to_vec()));

    let any_value = sample_struct.get_value("opt_i32")?;
    assert_eq!(any_value, AnyValue::Int32(2));

    let any_value = sample_struct.get_value("opt_i64")?;
    assert_eq!(any_value, AnyValue::Int64(2));

    let any_value = sample_struct.get_value("opt_u32")?;
    assert_eq!(any_value, AnyValue::UInt32(2));

    let any_value = sample_struct.get_value("opt_u64")?;
    assert_eq!(any_value, AnyValue::UInt64(2));

    let any_value = sample_struct.get_value("opt_f32")?;
    assert_eq!(any_value, AnyValue::Float32(2.0f32));

    let any_value = sample_struct.get_value("opt_f64")?;
    assert_eq!(any_value, AnyValue::Float64(2.0));

    let any_value = sample_struct.get_value("opt_bool")?;
    assert_eq!(any_value, AnyValue::Boolean(false));

    let any_value = sample_struct.get_value("opt_struct")?;
    assert_eq!(
        any_value,
        AnyValue::StructOwned(Box::new((
            vec![AnyValue::StringOwned("subf_string2".into())],
            Vec::from([Field::new("subf_string".into(), DataType::String)])
        )))
    );

    let any_value = sample_struct.get_value("opt_enum")?;
    assert_eq!(any_value, AnyValue::Enum(0, SampleEnum::mapping()));

    let any_value = sample_struct.get_value("opt_enum2")?;
    assert_eq!(any_value, AnyValue::Enum(0, SampleEnum::mapping()));

    Ok(())
}

#[test]
fn test_field_to_any_value_req_vec_fields_req_items() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = sample_struct();

    let any_value = sample_struct.get_value("req_vec_req_item_string")?;
    assert_eq!(
        any_value,
        AnyValue::List(Series::from_iter(vec![
            "req_vec_req_item_string".to_string()
        ]))
    );

    let any_value = sample_struct.get_value("req_vec_req_item_bytes")?;
    assert_eq!(
        any_value,
        AnyValue::List(
            Series::from_any_values_and_dtype(
                "".into(),
                &[AnyValue::BinaryOwned(b"req_vec_req_item_bytes".to_vec())],
                &polars_core::prelude::DataType::Binary,
                true
            )
            .unwrap()
        )
    );

    let any_value = sample_struct.get_value("req_vec_req_item_i32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![3])));

    let any_value = sample_struct.get_value("req_vec_req_item_i64")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![3])));

    let any_value = sample_struct.get_value("req_vec_req_item_u32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![3u32])));

    let any_value = sample_struct.get_value("req_vec_req_item_u64")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![3u64])));

    let any_value = sample_struct.get_value("req_vec_req_item_f32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![3.0f32])));

    let any_value = sample_struct.get_value("req_vec_req_item_f64")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![3.0])));

    let any_value = sample_struct.get_value("req_vec_req_item_bool")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![true])));

    let any_value = sample_struct.get_value("req_vec_req_item_struct")?;
    assert_eq!(any_value.to_string(), "[{\"subf_string3\"}]");

    let any_value = sample_struct.get_value("req_vec_req_item_enum")?;
    assert_eq!(any_value.to_string(), "[\"ITEM\"]");

    let any_value = sample_struct.get_value("req_vec_req_item_enum2")?;
    assert_eq!(any_value.to_string(), "[\"ITEM\"]");

    // Nested
    let any_value = sample_struct.get_value("req_vec_req_item_struct[0].subf_string")?;
    assert_eq!(any_value, AnyValue::StringOwned("subf_string3".into()));

    Ok(())
}

#[test]
fn test_field_to_any_value_opt_vec_fields_req_items() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = sample_struct();

    let any_value = sample_struct.get_value("opt_vec_req_item_string")?;
    assert_eq!(
        any_value,
        AnyValue::List(Series::from_iter(vec![
            "opt_vec_req_item_string".to_string()
        ]))
    );

    let any_value = sample_struct.get_value("opt_vec_req_item_bytes")?;
    assert_eq!(
        any_value,
        AnyValue::List(
            Series::from_any_values_and_dtype(
                "".into(),
                &[AnyValue::BinaryOwned(b"opt_vec_req_item_bytes".to_vec())],
                &polars_core::prelude::DataType::Binary,
                true
            )
            .unwrap()
        )
    );

    let any_value = sample_struct.get_value("opt_vec_req_item_i32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![4])));

    let any_value = sample_struct.get_value("opt_vec_req_item_i64")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![4])));

    let any_value = sample_struct.get_value("opt_vec_req_item_u32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![4u32])));

    let any_value = sample_struct.get_value("opt_vec_req_item_u64")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![4u64])));

    let any_value = sample_struct.get_value("opt_vec_req_item_f32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![4.0f32])));

    let any_value = sample_struct.get_value("opt_vec_req_item_f64")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![4.0])));

    let any_value = sample_struct.get_value("opt_vec_req_item_bool")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![false])));

    let any_value = sample_struct.get_value("opt_vec_req_item_struct")?;
    assert_eq!(any_value.to_string(), "[{\"subf_string4\"}]");

    let any_value = sample_struct.get_value("opt_vec_req_item_enum")?;
    assert_eq!(any_value.to_string(), "[\"ITEM\"]");

    let any_value = sample_struct.get_value("opt_vec_req_item_enum2")?;
    assert_eq!(any_value.to_string(), "[\"ITEM\"]");

    // Nested
    let any_value = sample_struct.get_value("opt_vec_req_item_struct[0].subf_string")?;
    assert_eq!(any_value, AnyValue::StringOwned("subf_string4".into()));

    Ok(())
}

#[test]
fn test_field_to_any_value_req_vec_fields_opt_items() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = sample_struct();

    let any_value = sample_struct.get_value("req_vec_opt_item_string")?;
    assert_eq!(
        any_value,
        AnyValue::List(Series::from_iter(vec![
            "req_vec_opt_item_string".to_string()
        ]))
    );

    let any_value = sample_struct.get_value("req_vec_opt_item_bytes")?;
    assert_eq!(
        any_value,
        AnyValue::List(
            Series::from_any_values_and_dtype(
                "".into(),
                &[AnyValue::BinaryOwned(b"req_vec_opt_item_bytes".to_vec())],
                &polars_core::prelude::DataType::Binary,
                false
            )
            .unwrap()
        )
    );

    let any_value = sample_struct.get_value("req_vec_opt_item_i32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![5])));

    let any_value = sample_struct.get_value("req_vec_opt_item_i64")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![5])));

    let any_value = sample_struct.get_value("req_vec_opt_item_u32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![5u32])));

    let any_value = sample_struct.get_value("req_vec_opt_item_u64")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![5u64])));

    let any_value = sample_struct.get_value("req_vec_opt_item_f32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![5.0f32])));

    let any_value = sample_struct.get_value("req_vec_opt_item_f64")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![5.0])));

    let any_value = sample_struct.get_value("req_vec_opt_item_bool")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![true])));

    let any_value = sample_struct.get_value("req_vec_opt_item_struct")?;
    assert_eq!(any_value.to_string(), "[{\"subf_string5\"}]");

    let any_value = sample_struct.get_value("req_vec_opt_item_enum")?;
    assert_eq!(any_value.to_string(), "[\"ITEM\"]");

    let any_value = sample_struct.get_value("req_vec_opt_item_enum2")?;
    assert_eq!(any_value.to_string(), "[\"ITEM\"]");

    // Nested
    let any_value = sample_struct.get_value("req_vec_opt_item_struct[0].subf_string")?;
    assert_eq!(any_value, AnyValue::StringOwned("subf_string5".into()));

    Ok(())
}

#[test]
fn test_field_to_any_value_opt_vec_fields_opt_items() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = sample_struct();

    let any_value = sample_struct.get_value("opt_vec_opt_item_string")?;
    assert_eq!(
        any_value,
        AnyValue::List(Series::from_iter(vec![
            "opt_vec_opt_item_string".to_string()
        ]))
    );

    let any_value = sample_struct.get_value("opt_vec_opt_item_bytes")?;
    assert_eq!(
        any_value,
        AnyValue::List(
            Series::from_any_values_and_dtype(
                "".into(),
                &[AnyValue::BinaryOwned(b"opt_vec_opt_item_bytes".to_vec())],
                &polars_core::prelude::DataType::Binary,
                false
            )
            .unwrap()
        )
    );

    let any_value = sample_struct.get_value("opt_vec_opt_item_i32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![6])));

    let any_value = sample_struct.get_value("opt_vec_opt_item_i64")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![6])));

    let any_value = sample_struct.get_value("opt_vec_opt_item_u32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![6u32])));

    let any_value = sample_struct.get_value("opt_vec_opt_item_u64")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![6u64])));

    let any_value = sample_struct.get_value("opt_vec_opt_item_f32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![6.0f32])));

    let any_value = sample_struct.get_value("opt_vec_opt_item_f64")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![6.0])));

    let any_value = sample_struct.get_value("opt_vec_opt_item_bool")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![false])));

    let any_value = sample_struct.get_value("opt_vec_opt_item_struct")?;
    assert_eq!(any_value.to_string(), "[{\"subf_string6\"}]");

    let any_value = sample_struct.get_value("opt_vec_opt_item_enum")?;
    assert_eq!(any_value.to_string(), "[\"ITEM\"]");

    let any_value = sample_struct.get_value("opt_vec_opt_item_enum2")?;
    assert_eq!(any_value.to_string(), "[\"ITEM\"]");

    // Nested
    let any_value = sample_struct.get_value("opt_vec_opt_item_struct[0].subf_string")?;
    assert_eq!(any_value, AnyValue::StringOwned("subf_string6".into()));

    Ok(())
}
