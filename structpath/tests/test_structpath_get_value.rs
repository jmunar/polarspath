use polars_core::prelude::{AnyValue, DataType, Field, Series};
use structpath::{EnumPath, StructPath};

#[derive(EnumPath, Debug, Clone, PartialEq)]
pub enum SampleEnum {
    ITEM = 1,
}

#[derive(StructPath, Debug, Clone, PartialEq)]
pub struct SampleSubstruct {
    pub subf_string: String,
}

#[derive(StructPath, Debug, Clone, PartialEq)]
pub struct SampleStruct {
    pub req_string: String,
    pub req_i32: i32,
    pub req_i64: i64,
    pub req_u32: u32,
    pub req_u64: u64,
    pub req_f32: f32,
    pub req_f64: f64,
    pub req_bool: bool,
    #[type_hint("struct")]
    pub req_struct: SampleSubstruct,
    #[type_hint("enum")]
    pub req_enum: SampleEnum,
    #[type_hint("enum", "SampleEnum")]
    pub req_enum2: i32,

    pub opt_string: Option<String>,
    pub opt_i32: Option<i32>,
    pub opt_i64: Option<i64>,
    pub opt_u32: Option<u32>,
    pub opt_u64: Option<u64>,
    pub opt_f32: Option<f32>,
    pub opt_f64: Option<f64>,
    pub opt_bool: Option<bool>,
    #[type_hint("struct")]
    pub opt_struct: Option<SampleSubstruct>,
    #[type_hint("enum")]
    pub opt_enum: Option<SampleEnum>,
    #[type_hint("enum", "SampleEnum")]
    pub opt_enum2: Option<i32>,

    pub req_vec_req_item_string: Vec<String>,
    pub req_vec_req_item_i32: Vec<i32>,
    pub req_vec_req_item_i64: Vec<i64>,
    pub req_vec_req_item_u32: Vec<u32>,
    pub req_vec_req_item_u64: Vec<u64>,
    pub req_vec_req_item_f32: Vec<f32>,
    pub req_vec_req_item_f64: Vec<f64>,
    pub req_vec_req_item_bool: Vec<bool>,
    #[type_hint("struct")]
    pub req_vec_req_item_struct: Vec<SampleSubstruct>,
    #[type_hint("enum")]
    pub req_vec_req_item_enum: Vec<SampleEnum>,
    #[type_hint("enum", "SampleEnum")]
    pub req_vec_req_item_enum2: Vec<i32>,

    pub opt_vec_req_item_string: Option<Vec<String>>,
    pub opt_vec_req_item_i32: Option<Vec<i32>>,
    pub opt_vec_req_item_i64: Option<Vec<i64>>,
    pub opt_vec_req_item_u32: Option<Vec<u32>>,
    pub opt_vec_req_item_u64: Option<Vec<u64>>,
    pub opt_vec_req_item_f32: Option<Vec<f32>>,
    pub opt_vec_req_item_f64: Option<Vec<f64>>,
    pub opt_vec_req_item_bool: Option<Vec<bool>>,
    #[type_hint("struct")]
    pub opt_vec_req_item_struct: Option<Vec<SampleSubstruct>>,
    #[type_hint("enum")]
    pub opt_vec_req_item_enum: Option<Vec<SampleEnum>>,
    #[type_hint("enum", "SampleEnum")]
    pub opt_vec_req_item_enum2: Option<Vec<i32>>,

    pub req_vec_opt_item_string: Vec<Option<String>>,
    pub req_vec_opt_item_i32: Vec<Option<i32>>,
    pub req_vec_opt_item_i64: Vec<Option<i64>>,
    pub req_vec_opt_item_u32: Vec<Option<u32>>,
    pub req_vec_opt_item_u64: Vec<Option<u64>>,
    pub req_vec_opt_item_f32: Vec<Option<f32>>,
    pub req_vec_opt_item_f64: Vec<Option<f64>>,
    pub req_vec_opt_item_bool: Vec<Option<bool>>,
    #[type_hint("struct")]
    pub req_vec_opt_item_struct: Vec<Option<SampleSubstruct>>,
    #[type_hint("enum")]
    pub req_vec_opt_item_enum: Vec<Option<SampleEnum>>,
    #[type_hint("enum", "SampleEnum")]
    pub req_vec_opt_item_enum2: Vec<Option<i32>>,

    pub opt_vec_opt_item_string: Option<Vec<Option<String>>>,
    pub opt_vec_opt_item_i32: Option<Vec<Option<i32>>>,
    pub opt_vec_opt_item_i64: Option<Vec<Option<i64>>>,
    pub opt_vec_opt_item_u32: Option<Vec<Option<u32>>>,
    pub opt_vec_opt_item_u64: Option<Vec<Option<u64>>>,
    pub opt_vec_opt_item_f32: Option<Vec<Option<f32>>>,
    pub opt_vec_opt_item_f64: Option<Vec<Option<f64>>>,
    pub opt_vec_opt_item_bool: Option<Vec<Option<bool>>>,
    #[type_hint("struct")]
    pub opt_vec_opt_item_struct: Option<Vec<Option<SampleSubstruct>>>,
    #[type_hint("enum")]
    pub opt_vec_opt_item_enum: Option<Vec<Option<SampleEnum>>>,
    #[type_hint("enum", "SampleEnum")]
    pub opt_vec_opt_item_enum2: Option<Vec<Option<i32>>>,
}

fn sample_struct() -> SampleStruct {
    SampleStruct {
        req_string: "req_string".to_string(),
        req_i32: 1,
        req_i64: 1,
        req_u32: 1,
        req_u64: 1,
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
        opt_u32: Some(2),
        opt_u64: Some(2),
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
        req_vec_req_item_u32: vec![3],
        req_vec_req_item_u64: vec![3],
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
        opt_vec_req_item_u32: Some(vec![4]),
        opt_vec_req_item_u64: Some(vec![4]),
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
        req_vec_opt_item_u32: vec![Some(5)],
        req_vec_opt_item_u64: vec![Some(5)],
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
        opt_vec_opt_item_u32: Some(vec![Some(6)]),
        opt_vec_opt_item_u64: Some(vec![Some(6)]),
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
fn structpath_get_value_req_fields() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = sample_struct();

    let any_value = sample_struct.get_value("req_string")?;
    assert_eq!(any_value, AnyValue::StringOwned("req_string".into()));

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
fn structpath_get_value_opt_fields() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = sample_struct();

    let any_value = sample_struct.get_value("opt_string")?;
    assert_eq!(any_value, AnyValue::StringOwned("opt_string".into()));

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
fn structpath_get_value_req_vec_fields_req_items() -> Result<(), Box<dyn std::error::Error>> {
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

    let any_value = sample_struct.get_value("req_vec_req_item_u32")?;
    assert_eq!(any_value, AnyValue::List(Series::from_iter(vec![3])));

    let any_value = sample_struct.get_value("req_vec_req_item_u64")?;
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
fn structpath_get_value_opt_vec_fields_req_items() -> Result<(), Box<dyn std::error::Error>> {
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
fn structpath_get_value_req_vec_fields_opt_items() -> Result<(), Box<dyn std::error::Error>> {
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
fn structpath_get_value_opt_vec_fields_opt_items() -> Result<(), Box<dyn std::error::Error>> {
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

fn sample_struct_null() -> SampleStruct {
    SampleStruct {
        req_string: "req_string".to_string(),
        req_i32: 1,
        req_i64: 1,
        req_u32: 1,
        req_u64: 1,
        req_f32: 1.0f32,
        req_f64: 1.0,
        req_bool: true,
        req_struct: SampleSubstruct {
            subf_string: "subf_string1".to_string(),
        },
        req_enum: SampleEnum::ITEM,
        req_enum2: 1,

        opt_string: None,
        opt_i32: None,
        opt_i64: None,
        opt_u32: None,
        opt_u64: None,
        opt_f32: None,
        opt_f64: None,
        opt_bool: None,
        opt_struct: None,
        opt_enum: None,
        opt_enum2: None,

        req_vec_req_item_string: vec!["req_vec_req_item_string".to_string()],
        req_vec_req_item_i32: vec![3],
        req_vec_req_item_i64: vec![3],
        req_vec_req_item_u32: vec![3],
        req_vec_req_item_u64: vec![3],
        req_vec_req_item_f32: vec![3.0f32],
        req_vec_req_item_f64: vec![3.0],
        req_vec_req_item_bool: vec![true],
        req_vec_req_item_struct: vec![SampleSubstruct {
            subf_string: "subf_string3".to_string(),
        }],
        req_vec_req_item_enum: vec![SampleEnum::ITEM],
        req_vec_req_item_enum2: vec![1],

        opt_vec_req_item_string: None,
        opt_vec_req_item_i32: None,
        opt_vec_req_item_i64: None,
        opt_vec_req_item_u32: None,
        opt_vec_req_item_u64: None,
        opt_vec_req_item_f32: None,
        opt_vec_req_item_f64: None,
        opt_vec_req_item_bool: None,
        opt_vec_req_item_struct: None,
        opt_vec_req_item_enum: None,
        opt_vec_req_item_enum2: None,

        req_vec_opt_item_string: vec![None],
        req_vec_opt_item_i32: vec![None],
        req_vec_opt_item_i64: vec![None],
        req_vec_opt_item_u32: vec![None],
        req_vec_opt_item_u64: vec![None],
        req_vec_opt_item_f32: vec![None],
        req_vec_opt_item_f64: vec![None],
        req_vec_opt_item_bool: vec![None],
        req_vec_opt_item_struct: vec![None],
        req_vec_opt_item_enum: vec![None],
        req_vec_opt_item_enum2: vec![None],

        opt_vec_opt_item_string: Some(vec![None]),
        opt_vec_opt_item_i32: Some(vec![None]),
        opt_vec_opt_item_i64: Some(vec![None]),
        opt_vec_opt_item_u32: Some(vec![None]),
        opt_vec_opt_item_u64: Some(vec![None]),
        opt_vec_opt_item_f32: Some(vec![None]),
        opt_vec_opt_item_f64: Some(vec![None]),
        opt_vec_opt_item_bool: Some(vec![None]),
        opt_vec_opt_item_struct: Some(vec![None]),
        opt_vec_opt_item_enum: Some(vec![None]),
        opt_vec_opt_item_enum2: Some(vec![None]),
    }
}

#[test]
fn structpath_get_value_opt_fields_null() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = sample_struct_null();

    let any_value = sample_struct.get_value("opt_string")?;
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
fn structpath_get_value_opt_vec_fields_req_items_null() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = sample_struct_null();

    let any_value = sample_struct.get_value("opt_vec_req_item_string")?;
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
fn structpath_get_value_req_vec_fields_opt_items_null() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = sample_struct_null();

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
    assert_eq!(any_value.to_string(), "[null]");

    let any_value = sample_struct.get_value("req_vec_opt_item_enum")?;
    assert_eq!(any_value.to_string(), "[null]");

    let any_value = sample_struct.get_value("req_vec_opt_item_enum2")?;
    assert_eq!(any_value.to_string(), "[null]");

    // Nested
    let any_value = sample_struct.get_value("req_vec_opt_item_struct[0].subf_string")?;
    assert_eq!(any_value, AnyValue::Null);

    Ok(())
}
