mod sample;
use sample::{SampleEnum, SampleStruct, SampleSubstruct};

use polars_core::prelude::{AnyValue, DataType, Field, Series};
use structpath_types::{EnumPath, StructPath};

fn sample_struct() -> SampleStruct {
    SampleStruct {
        req_string: "req_string".to_string(),
        req_i32: 1,
        req_i64: 1,
        req_f32: 1.0f32,
        req_f64: 1.0,
        req_bool: true,
        req_struct: SampleSubstruct {
            subf_string: "subf_string1".to_string(),
        },
        req_enum: SampleEnum::ITEM,
        req_enum2: 1,

        opt_string: Some("opt_string".to_string()),
        opt_i32: Some(2),
        opt_i64: Some(2),
        opt_f32: Some(2.0f32),
        opt_f64: Some(2.0),
        opt_bool: Some(false),
        opt_struct: Some(SampleSubstruct {
            subf_string: "subf_string2".to_string(),
        }),
        opt_enum: Some(SampleEnum::ITEM),
        opt_enum2: Some(1),

        req_vec_req_item_string: vec!["req_vec_req_item_string".to_string()],
        req_vec_req_item_i32: vec![3],
        req_vec_req_item_i64: vec![3],
        req_vec_req_item_f32: vec![3.0f32],
        req_vec_req_item_f64: vec![3.0],
        req_vec_req_item_bool: vec![true],
        req_vec_req_item_struct: vec![SampleSubstruct {
            subf_string: "subf_string3".to_string(),
        }],
        req_vec_req_item_enum: vec![SampleEnum::ITEM],
        req_vec_req_item_enum2: vec![1],

        opt_vec_req_item_string: Some(vec!["opt_vec_req_item_string".to_string()]),
        opt_vec_req_item_i32: Some(vec![4]),
        opt_vec_req_item_i64: Some(vec![4]),
        opt_vec_req_item_f32: Some(vec![4.0f32]),
        opt_vec_req_item_f64: Some(vec![4.0]),
        opt_vec_req_item_bool: Some(vec![false]),
        opt_vec_req_item_struct: Some(vec![SampleSubstruct {
            subf_string: "subf_string4".to_string(),
        }]),
        opt_vec_req_item_enum: Some(vec![SampleEnum::ITEM]),
        opt_vec_req_item_enum2: Some(vec![1]),

        req_vec_opt_item_string: vec![Some("req_vec_opt_item_string".to_string())],
        req_vec_opt_item_i32: vec![Some(5)],
        req_vec_opt_item_i64: vec![Some(5)],
        req_vec_opt_item_f32: vec![Some(5.0f32)],
        req_vec_opt_item_f64: vec![Some(5.0)],
        req_vec_opt_item_bool: vec![Some(true)],
        req_vec_opt_item_struct: vec![Some(SampleSubstruct {
            subf_string: "subf_string5".to_string(),
        })],
        req_vec_opt_item_enum: vec![Some(SampleEnum::ITEM)],
        req_vec_opt_item_enum2: vec![Some(1)],

        opt_vec_opt_item_string: Some(vec![Some("opt_vec_opt_item_string".to_string())]),
        opt_vec_opt_item_i32: Some(vec![Some(6)]),
        opt_vec_opt_item_i64: Some(vec![Some(6)]),
        opt_vec_opt_item_f32: Some(vec![Some(6.0f32)]),
        opt_vec_opt_item_f64: Some(vec![Some(6.0)]),
        opt_vec_opt_item_bool: Some(vec![Some(false)]),
        opt_vec_opt_item_struct: Some(vec![Some(SampleSubstruct {
            subf_string: "subf_string6".to_string(),
        })]),
        opt_vec_opt_item_enum: Some(vec![Some(SampleEnum::ITEM)]),
        opt_vec_opt_item_enum2: Some(vec![Some(1)]),
    }
}

#[test]
fn test_field_to_any_value_req_fields() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = sample_struct();

    let any_value = sample_struct.get_value("req_string")?;
    assert_eq!(any_value, AnyValue::StringOwned("req_string".into()));

    let any_value = sample_struct.get_value("req_i32")?;
    assert_eq!(any_value, AnyValue::Int32(1));

    let any_value = sample_struct.get_value("req_i64")?;
    assert_eq!(any_value, AnyValue::Int64(1));

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

    let any_value = sample_struct.get_value("opt_i32")?;
    assert_eq!(any_value, AnyValue::Int32(2));

    let any_value = sample_struct.get_value("opt_i64")?;
    assert_eq!(any_value, AnyValue::Int64(2));

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

    let any_value = sample_struct.get_value("req_vec_req_item_i32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![3])));

    let any_value = sample_struct.get_value("req_vec_req_item_i64")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![3])));

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

    let any_value = sample_struct.get_value("opt_vec_req_item_i32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![4])));

    let any_value = sample_struct.get_value("opt_vec_req_item_i64")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![4])));

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

    let any_value = sample_struct.get_value("req_vec_opt_item_i32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![5])));

    let any_value = sample_struct.get_value("req_vec_opt_item_i64")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![5])));

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

    let any_value = sample_struct.get_value("opt_vec_opt_item_i32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![6])));

    let any_value = sample_struct.get_value("opt_vec_opt_item_i64")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![6])));

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
