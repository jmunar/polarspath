mod sample;
use sample::{SampleStruct, SampleSubstruct};

use polars_core::prelude::{AnyValue, Series};
use structpath_types::StructPath;

fn sample_struct() -> SampleStruct {
    SampleStruct {
        req_string: "req_string".to_string(),
        req_i32: 1,
        req_i64: 1,
        req_f64: 1.0,
        req_bool: true,
        req_struct: SampleSubstruct {
            subf_string: "subf_string1".to_string(),
        },
        req_object: "req_object".to_string(),

        opt_string: None,
        opt_i32: None,
        opt_i64: None,
        opt_f64: None,
        opt_bool: None,
        opt_struct: None,
        opt_object: None,

        req_vec_req_item_string: vec!["req_vec_req_item_string".to_string()],
        req_vec_req_item_i32: vec![3],
        req_vec_req_item_i64: vec![3],
        req_vec_req_item_f64: vec![3.0],
        req_vec_req_item_bool: vec![true],
        req_vec_req_item_struct: vec![SampleSubstruct {
            subf_string: "subf_string3".to_string(),
        }],
        req_vec_req_item_object: vec!["req_vec_req_item_object".to_string()],

        opt_vec_req_item_string: None,
        opt_vec_req_item_i32: None,
        opt_vec_req_item_i64: None,
        opt_vec_req_item_f64: None,
        opt_vec_req_item_bool: None,
        opt_vec_req_item_struct: None,
        opt_vec_req_item_object: None,

        req_vec_opt_item_string: vec![None],
        req_vec_opt_item_i32: vec![None],
        req_vec_opt_item_i64: vec![None],
        req_vec_opt_item_f64: vec![None],
        req_vec_opt_item_bool: vec![None],
        req_vec_opt_item_struct: vec![None],
        req_vec_opt_item_object: vec![None],

        opt_vec_opt_item_string: Some(vec![None]),
        opt_vec_opt_item_i32: Some(vec![None]),
        opt_vec_opt_item_i64: Some(vec![None]),
        opt_vec_opt_item_f64: Some(vec![None]),
        opt_vec_opt_item_bool: Some(vec![None]),
        opt_vec_opt_item_struct: Some(vec![None]),
        opt_vec_opt_item_object: Some(vec![None]),
    }
}

#[test]
fn test_field_to_any_value_opt_fields_null() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = sample_struct();

    let any_value = sample_struct.get_value("opt_string")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_i32")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_i64")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_f64")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_bool")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_struct")?;
    assert_eq!(any_value, AnyValue::Null);

    // let any_value = sample_struct.get_value("opt_object")?;
    // assert_eq!(any_value, AnyValue::Null);

    Ok(())
}

#[test]
fn test_field_to_any_value_opt_vec_fields_req_items_null() -> Result<(), Box<dyn std::error::Error>>
{
    let sample_struct = sample_struct();

    let any_value = sample_struct.get_value("opt_vec_req_item_string")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_vec_req_item_i32")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_vec_req_item_i64")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_vec_req_item_f64")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_vec_req_item_bool")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_vec_req_item_struct")?;
    assert_eq!(any_value, AnyValue::Null);

    // Nested
    let any_value = sample_struct.get_value("opt_vec_req_item_struct[0].subf_string")?;
    assert_eq!(any_value, AnyValue::Null);

    Ok(())
}

#[test]
fn test_field_to_any_value_req_vec_fields_opt_items_null() -> Result<(), Box<dyn std::error::Error>>
{
    let sample_struct = sample_struct();

    let any_value = sample_struct.get_value("req_vec_opt_item_string")?;
    assert_eq!(
        any_value,
        AnyValue::List(Series::from_any_values("".into(), &[AnyValue::Null], false).unwrap())
    );

    let any_value = sample_struct.get_value("req_vec_opt_item_i32")?;
    assert_eq!(
        any_value,
        AnyValue::List(Series::from_any_values("".into(), &[AnyValue::Null], false).unwrap())
    );

    let any_value = sample_struct.get_value("req_vec_opt_item_i64")?;
    assert_eq!(
        any_value,
        AnyValue::List(Series::from_any_values("".into(), &[AnyValue::Null], false).unwrap())
    );

    let any_value = sample_struct.get_value("req_vec_opt_item_f64")?;
    assert_eq!(
        any_value,
        AnyValue::List(Series::from_any_values("".into(), &[AnyValue::Null], false).unwrap())
    );

    let any_value = sample_struct.get_value("req_vec_opt_item_bool")?;
    assert_eq!(
        any_value,
        AnyValue::List(Series::from_any_values("".into(), &[AnyValue::Null], false).unwrap())
    );

    let any_value = sample_struct.get_value("req_vec_opt_item_struct")?;
    assert_eq!(
        any_value,
        AnyValue::List(Series::from_any_values("".into(), &[AnyValue::Null], false).unwrap())
    );

    // Nested
    let any_value = sample_struct.get_value("req_vec_opt_item_struct[0].subf_string")?;
    assert_eq!(any_value, AnyValue::Null);

    Ok(())
}
