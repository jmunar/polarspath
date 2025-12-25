mod sample;
use sample::{sample_struct_null, SampleEnum};

use polars_core::prelude::{AnyValue, Series};
use polars_structpath_types::{HasDataTypeWrapper, StructPath};

#[test]
fn test_field_to_any_value_opt_fields_null() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = sample_struct_null();

    let any_value = sample_struct.get_value("opt_string")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_bytes")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_i32")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_i64")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_u32")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_u64")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_f32")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_f64")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_bool")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_struct")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_enum")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_enum2")?;
    assert_eq!(any_value, AnyValue::Null);

    Ok(())
}

#[test]
fn test_field_to_any_value_opt_vec_fields_req_items_null() -> Result<(), Box<dyn std::error::Error>>
{
    let sample_struct = sample_struct_null();

    let any_value = sample_struct.get_value("opt_vec_req_item_string")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_vec_req_item_bytes")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_vec_req_item_i32")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_vec_req_item_i64")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_vec_req_item_u32")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_vec_req_item_u64")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_vec_req_item_f32")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_vec_req_item_f64")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_vec_req_item_bool")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_vec_req_item_struct")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_vec_req_item_enum")?;
    assert_eq!(any_value, AnyValue::Null);

    let any_value = sample_struct.get_value("opt_vec_req_item_enum2")?;
    assert_eq!(any_value, AnyValue::Null);

    // Nested
    let any_value = sample_struct.get_value("opt_vec_req_item_struct[0].subf_string")?;
    assert_eq!(any_value, AnyValue::Null);

    Ok(())
}

#[test]
fn test_field_to_any_value_req_vec_fields_opt_items_null() -> Result<(), Box<dyn std::error::Error>>
{
    let sample_struct = sample_struct_null();

    let any_value = sample_struct.get_value("req_vec_opt_item_string")?;
    assert_eq!(
        any_value,
        AnyValue::List(Series::from_any_values("".into(), &[AnyValue::Null], false).unwrap())
    );

    let any_value = sample_struct.get_value("req_vec_opt_item_bytes")?;
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

    let any_value = sample_struct.get_value("req_vec_opt_item_u32")?;
    assert_eq!(
        any_value,
        AnyValue::List(Series::from_any_values("".into(), &[AnyValue::Null], false).unwrap())
    );

    let any_value = sample_struct.get_value("req_vec_opt_item_u64")?;
    assert_eq!(
        any_value,
        AnyValue::List(Series::from_any_values("".into(), &[AnyValue::Null], false).unwrap())
    );

    let any_value = sample_struct.get_value("req_vec_opt_item_f32")?;
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

    let any_value = sample_struct.get_value("req_vec_opt_item_enum")?;
    assert_eq!(
        any_value,
        AnyValue::List(
            Series::from_any_values_and_dtype(
                "".into(),
                &[AnyValue::Null],
                SampleEnum::data_type(),
                false
            )
            .unwrap()
        )
    );

    let any_value = sample_struct.get_value("req_vec_opt_item_enum2")?;
    assert_eq!(
        any_value,
        AnyValue::List(
            Series::from_any_values_and_dtype(
                "".into(),
                &[AnyValue::Null],
                SampleEnum::data_type(),
                false
            )
            .unwrap()
        )
    );

    // Nested
    let any_value = sample_struct.get_value("req_vec_opt_item_struct[0].subf_string")?;
    assert_eq!(any_value, AnyValue::Null);

    Ok(())
}
