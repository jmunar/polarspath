use structpath::prelude::*;

use std::str::FromStr;

// #[derive(StructPath, Debug, Clone)]
// pub enum SampleEnum {
//     A,
//     B,
//     C,
// }

#[derive(StructPath, Debug, Clone, PartialEq)]
pub struct SampleSubstruct {
    subf_string: String,
}

#[derive(StructPath, Debug, Clone, PartialEq)]
pub struct SampleStruct {
    f_string_scalar_required: String,
    f_integer_scalar_required: i64,
    f_float_scalar_required: f64,
    f_boolean_scalar_required: bool,
    #[type_hint = "struct"]
    f_struct_scalar_required: SampleSubstruct,
    // #[type_hint = "enum"]
    // f_enum_scalar_required: SampleEnum,
    f_string_scalar_optional: Option<String>,
    f_integer_scalar_optional: Option<i64>,
    f_float_scalar_optional: Option<f64>,
    f_boolean_scalar_optional: Option<bool>,
    #[type_hint = "struct"]
    f_struct_scalar_optional: Option<SampleSubstruct>,
    // #[type_hint = "enum"]
    // f_enum_scalar_optional: Option<SampleEnum>,
    f_string_vector_required_elements_required: Vec<String>,
    f_integer_vector_required_elements_required: Vec<i64>,
    f_float_vector_required_elements_required: Vec<f64>,
    f_boolean_vector_required_elements_required: Vec<bool>,
    #[type_hint = "struct"]
    f_struct_vector_required_elements_required: Vec<SampleSubstruct>,
    // #[type_hint = "enum"]
    // f_enum_vector_required_elements_required: Vec<SampleEnum>,
    f_string_vector_optional_elements_required: Option<Vec<String>>,
    f_integer_vector_optional_elements_required: Option<Vec<i64>>,
    f_float_vector_optional_elements_required: Option<Vec<f64>>,
    f_boolean_vector_optional_elements_required: Option<Vec<bool>>,
    #[type_hint = "struct"]
    f_struct_vector_optional_elements_required: Option<Vec<SampleSubstruct>>,
    // #[type_hint = "enum"]
    // f_enum_vector_optional_elements_required: Option<Vec<SampleEnum>>,
    f_string_vector_required_elements_optional: Vec<Option<String>>,
    f_integer_vector_required_elements_optional: Vec<Option<i64>>,
    f_float_vector_required_elements_optional: Vec<Option<f64>>,
    f_boolean_vector_required_elements_optional: Vec<Option<bool>>,
    #[type_hint = "struct"]
    f_struct_vector_required_elements_optional: Vec<Option<SampleSubstruct>>,
    // #[type_hint = "enum"]
    // f_enum_vector_required_elements_optional: Vec<Option<SampleEnum>>,
    f_string_vector_optional_elements_optional: Option<Vec<Option<String>>>,
    f_integer_vector_optional_elements_optional: Option<Vec<Option<i64>>>,
    f_float_vector_optional_elements_optional: Option<Vec<Option<f64>>>,
    f_boolean_vector_optional_elements_optional: Option<Vec<Option<bool>>>,
    #[type_hint = "struct"]
    f_struct_vector_optional_elements_optional: Option<Vec<Option<SampleSubstruct>>>,
    // #[type_hint = "enum"]
    // f_enum_vector_optional_elements_optional: Option<Vec<Option<SampleEnum>>>,
}

pub fn build_sample_struct() -> SampleStruct {
    SampleStruct {
        // Scalar fields
        f_string_scalar_required: "hello".to_string(),
        f_integer_scalar_required: 123,
        f_float_scalar_required: 1.23,
        f_boolean_scalar_required: true,
        f_struct_scalar_required: SampleSubstruct {
            subf_string: "sub1".to_string(),
        },
        // f_enum_scalar_required: SampleEnum::A,

        // Scalar optional fields
        f_string_scalar_optional: Some("world".to_string()),
        f_integer_scalar_optional: Some(456),
        f_float_scalar_optional: Some(4.56),
        f_boolean_scalar_optional: Some(false),
        f_struct_scalar_optional: Some(SampleSubstruct {
            subf_string: "sub2".to_string(),
        }),
        // f_enum_scalar_optional: Some(SampleEnum::B),

        // Array fields
        f_string_vector_required_elements_required: vec!["hello".to_string(), "world".to_string()],
        f_integer_vector_required_elements_required: vec![123, 456],
        f_float_vector_required_elements_required: vec![1.23, 4.56],
        f_boolean_vector_required_elements_required: vec![true, false],
        f_struct_vector_required_elements_required: vec![SampleSubstruct {
            subf_string: "sub3".to_string(),
        }],
        // f_enum_vector_required_elements_required: vec![SampleEnum::A, SampleEnum::B],

        // Optional array with required items
        f_string_vector_optional_elements_required: Some(vec![
            "hello".to_string(),
            "world".to_string(),
        ]),
        f_integer_vector_optional_elements_required: Some(vec![123, 456]),
        f_float_vector_optional_elements_required: Some(vec![1.23, 4.56]),
        f_boolean_vector_optional_elements_required: Some(vec![true, false]),
        f_struct_vector_optional_elements_required: Some(vec![SampleSubstruct {
            subf_string: "sub4".to_string(),
        }]),
        // f_enum_vector_optional_elements_required: Some(vec![SampleEnum::A, SampleEnum::B]),

        // Required array with optional items
        f_string_vector_required_elements_optional: vec![Some("hello".to_string()), None],
        f_integer_vector_required_elements_optional: vec![Some(123), None],
        f_float_vector_required_elements_optional: vec![Some(1.23), None],
        f_boolean_vector_required_elements_optional: vec![Some(true), None],
        f_struct_vector_required_elements_optional: vec![
            Some(SampleSubstruct {
                subf_string: "sub5".to_string(),
            }),
            None,
        ],
        // f_enum_vector_required_elements_optional: vec![Some(SampleEnum::A), None],

        // Optional array with optional items
        f_string_vector_optional_elements_optional: Some(vec![Some("hello".to_string()), None]),
        f_integer_vector_optional_elements_optional: Some(vec![Some(123), None]),
        f_float_vector_optional_elements_optional: Some(vec![Some(1.23), None]),
        f_boolean_vector_optional_elements_optional: Some(vec![Some(true), None]),
        f_struct_vector_optional_elements_optional: Some(vec![
            Some(SampleSubstruct {
                subf_string: "sub6".to_string(),
            }),
            None,
        ]),
        // f_enum_vector_optional_elements_optional: Some(vec![Some(SampleEnum::A), None]),
    }
}

pub fn build_sample_struct_with_null_optionals() -> SampleStruct {
    let sample_struct = build_sample_struct();

    // Set optional fields to None to assert that the value is None
    SampleStruct {
        f_string_scalar_optional: None,
        f_integer_scalar_optional: None,
        f_float_scalar_optional: None,
        f_boolean_scalar_optional: None,
        f_struct_scalar_optional: None,
        // f_enum_scalar_optional: None,
        f_string_vector_optional_elements_required: None,
        f_integer_vector_optional_elements_required: None,
        f_float_vector_optional_elements_required: None,
        f_boolean_vector_optional_elements_required: None,
        f_struct_vector_optional_elements_required: None,
        // f_enum_vector_optional_elements_required: None,
        f_string_vector_optional_elements_optional: None,
        f_integer_vector_optional_elements_optional: None,
        f_float_vector_optional_elements_optional: None,
        f_boolean_vector_optional_elements_optional: None,
        f_struct_vector_optional_elements_optional: None,
        // f_enum_vector_optional_elements_optional: None,
        ..sample_struct
    }
}

fn test_get_value_by_field_scalar() {
    let sample_struct = build_sample_struct();
    let v1 = sample_struct.get_value("f_string_scalar_required");
    assert_eq!(v1.unwrap().as_str(), "hello");
    let v2 = sample_struct.get_value("f_integer_scalar_required");
    assert_eq!(v2.unwrap().as_i64(), 123);
    let v3 = sample_struct.get_value("f_float_scalar_required");
    assert_eq!(v3.unwrap().as_f64(), 1.23);
    let v4 = sample_struct.get_value("f_boolean_scalar_required");
    assert_eq!(v4.unwrap().as_bool(), true);
    let v5 = sample_struct.get_value("f_struct_scalar_required");
    assert_eq!(
        v5.unwrap().unbox::<SampleSubstruct>().to_owned(),
        SampleSubstruct {
            subf_string: "sub1".to_string()
        }
    );
}

fn test_get_value_by_field_scalar_optional() {
    let sample_struct = build_sample_struct();

    let v1 = sample_struct.get_value("f_string_scalar_optional");
    assert_eq!(v1.unwrap().unwrap().as_str(), "world");
    let v2 = sample_struct.get_value("f_integer_scalar_optional");
    assert_eq!(v2.unwrap().unwrap().as_i64(), 456);
    let v3 = sample_struct.get_value("f_float_scalar_optional");
    assert_eq!(v3.unwrap().unwrap().as_f64(), 4.56);
    let v4 = sample_struct.get_value("f_boolean_scalar_optional");
    assert_eq!(v4.unwrap().unwrap().as_bool(), false);
    let v5 = sample_struct.get_value("f_struct_scalar_optional");
    assert_eq!(
        v5.unwrap().unwrap().unbox::<SampleSubstruct>().to_owned(),
        SampleSubstruct {
            subf_string: "sub2".to_string()
        }
    );
}

fn test_get_value_by_field_scalar_optional_none() {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v1 = sample_struct.get_value("f_string_scalar_optional");
    assert_eq!(v1.unwrap(), Value::Optional(None));
    let v2 = sample_struct.get_value("f_integer_scalar_optional");
    assert_eq!(v2.unwrap(), Value::Optional(None));
    let v3 = sample_struct.get_value("f_float_scalar_optional");
    assert_eq!(v3.unwrap(), Value::Optional(None));
    let v4 = sample_struct.get_value("f_boolean_scalar_optional");
    assert_eq!(v4.unwrap(), Value::Optional(None));
    let v5 = sample_struct.get_value("f_struct_scalar_optional");
    assert_eq!(v5.unwrap(), Value::Optional(None));
}

fn test_get_value_by_field_required_array_required_items() {
    let sample_struct = build_sample_struct();

    assert_eq!(
        sample_struct
            .get_value("f_string_vector_required_elements_required")
            .unwrap(),
        Value::Array(vec![
            Value::String("hello".to_string()),
            Value::String("world".to_string())
        ])
    );
    assert_eq!(
        sample_struct
            .get_value("f_integer_vector_required_elements_required")
            .unwrap(),
        Value::Array(vec![Value::Integer(123), Value::Integer(456)])
    );
    assert_eq!(
        sample_struct
            .get_value("f_float_vector_required_elements_required")
            .unwrap(),
        Value::Array(vec![Value::Float(1.23), Value::Float(4.56)])
    );
    assert_eq!(
        sample_struct
            .get_value("f_boolean_vector_required_elements_required")
            .unwrap(),
        Value::Array(vec![Value::Boolean(true), Value::Boolean(false)])
    );
    // assert_eq!(
    //     sample_struct.get_value("f_enum_vector_required_elements_required").unwrap(),
    //     Value::BoxableArray(vec![SampleEnum::A.clone_box(), SampleEnum::B.clone_box()])
    // );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_required_elements_required")
            .unwrap(),
        Value::Array(vec![Value::Boxable(
            SampleSubstruct {
                subf_string: "sub3".to_string()
            }
            .clone_box()
        )]),
    );
}

fn test_get_value_by_field_optional_array_required_items() {
    let sample_struct = build_sample_struct();

    assert_eq!(
        sample_struct
            .get_value("f_string_vector_optional_elements_required")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Array(vec![
            Value::String("hello".to_string()),
            Value::String("world".to_string())
        ])
    );
    assert_eq!(
        sample_struct
            .get_value("f_integer_vector_optional_elements_required")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Array(vec![Value::Integer(123), Value::Integer(456)])
    );
    assert_eq!(
        sample_struct
            .get_value("f_float_vector_optional_elements_required")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Array(vec![Value::Float(1.23), Value::Float(4.56)])
    );
    assert_eq!(
        sample_struct
            .get_value("f_boolean_vector_optional_elements_required")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Array(vec![Value::Boolean(true), Value::Boolean(false)])
    );
    // assert_eq!(
    //     sample_struct.get_value("f_enum_vector_optional_elements_required").unwrap().into_inner().unwrap(),
    //     Value::BoxableArray(vec![SampleEnum::A.clone_box(), SampleEnum::B.clone_box()])
    // );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_optional_elements_required")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Array(vec![Value::Boxable(
            SampleSubstruct {
                subf_string: "sub4".to_string()
            }
            .clone_box()
        )])
    );
}

fn test_get_value_by_field_optional_array_is_none_required_items() {
    let sample_struct = build_sample_struct_with_null_optionals();

    assert_eq!(
        sample_struct
            .get_value("f_string_vector_optional_elements_required")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_integer_vector_optional_elements_required")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_float_vector_optional_elements_required")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_boolean_vector_optional_elements_required")
            .unwrap(),
        Value::Optional(None)
    );
    // assert_eq!(
    //     sample_struct.get_value("f_enum_vector_optional_elements_required").unwrap(),
    //     Value::Optional(None)
    // );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_optional_elements_required")
            .unwrap(),
        Value::Optional(None)
    );
}

fn test_get_value_by_field_required_array_optional_items() {
    let sample_struct = build_sample_struct();

    assert_eq!(
        sample_struct
            .get_value("f_string_vector_required_elements_optional")
            .unwrap(),
        Value::Array(vec![
            Value::Optional(Some(Box::new(Value::String("hello".to_string())))),
            Value::Optional(None)
        ])
    );
    assert_eq!(
        sample_struct
            .get_value("f_integer_vector_required_elements_optional")
            .unwrap(),
        Value::Array(vec![
            Value::Optional(Some(Box::new(Value::Integer(123)))),
            Value::Optional(None)
        ])
    );
    assert_eq!(
        sample_struct
            .get_value("f_float_vector_required_elements_optional")
            .unwrap(),
        Value::Array(vec![
            Value::Optional(Some(Box::new(Value::Float(1.23)))),
            Value::Optional(None)
        ])
    );
    assert_eq!(
        sample_struct
            .get_value("f_boolean_vector_required_elements_optional")
            .unwrap(),
        Value::Array(vec![
            Value::Optional(Some(Box::new(Value::Boolean(true)))),
            Value::Optional(None)
        ])
    );
    // assert_eq!(
    //     sample_struct.get_value("f_enum_vector_required_elements_optional").unwrap(),
    //     Value::BoxableOptArray(vec![Some(SampleEnum::A.clone_box()), None])
    // );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_required_elements_optional")
            .unwrap(),
        Value::Array(vec![
            Value::Optional(Some(Box::new(Value::Boxable(
                SampleSubstruct {
                    subf_string: "sub5".to_string()
                }
                .clone_box()
            )))),
            Value::Optional(None)
        ])
    );
}

fn test_get_value_by_field_optional_array_optional_items() {
    let sample_struct = build_sample_struct();

    assert_eq!(
        sample_struct
            .get_value("f_string_vector_optional_elements_optional")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Array(vec![
            Value::Optional(Some(Box::new(Value::String("hello".to_string())))),
            Value::Optional(None)
        ])
    );
    assert_eq!(
        sample_struct
            .get_value("f_integer_vector_optional_elements_optional")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Array(vec![
            Value::Optional(Some(Box::new(Value::Integer(123)))),
            Value::Optional(None)
        ])
    );
    assert_eq!(
        sample_struct
            .get_value("f_float_vector_optional_elements_optional")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Array(vec![
            Value::Optional(Some(Box::new(Value::Float(1.23)))),
            Value::Optional(None)
        ])
    );
    assert_eq!(
        sample_struct
            .get_value("f_boolean_vector_optional_elements_optional")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Array(vec![
            Value::Optional(Some(Box::new(Value::Boolean(true)))),
            Value::Optional(None)
        ])
    );
    // assert_eq!(
    //     sample_struct.get_value("f_enum_vector_optional_elements_optional").unwrap().into_inner().unwrap(),
    //     Value::BoxableOptArray(vec![Some(SampleEnum::A.clone_box()), None])
    // );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_optional_elements_optional")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Array(vec![
            Value::Optional(Some(Box::new(Value::Boxable(
                SampleSubstruct {
                    subf_string: "sub6".to_string()
                }
                .clone_box()
            )))),
            Value::Optional(None),
        ])
    );
}

fn test_get_value_by_field_optional_array_is_none_optional_items() {
    let sample_struct = build_sample_struct_with_null_optionals();

    assert_eq!(
        sample_struct
            .get_value("f_string_vector_optional_elements_optional")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_integer_vector_optional_elements_optional")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_float_vector_optional_elements_optional")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_boolean_vector_optional_elements_optional")
            .unwrap(),
        Value::Optional(None)
    );
    // assert_eq!(
    //     sample_struct.get_value("f_enum_vector_optional_elements_optional").unwrap(),
    //     Value::Optional(None)
    // );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_optional_elements_optional")
            .unwrap(),
        Value::Optional(None)
    );
}

fn test_get_value_by_index_required_array_required_items() {
    let sample_struct = build_sample_struct();

    assert_eq!(
        sample_struct
            .get_value("f_string_vector_required_elements_required[0]")
            .unwrap(),
        Value::String("hello".to_string())
    );
    assert_eq!(
        sample_struct
            .get_value("f_integer_vector_required_elements_required[0]")
            .unwrap(),
        Value::Integer(123)
    );
    assert_eq!(
        sample_struct
            .get_value("f_float_vector_required_elements_required[0]")
            .unwrap(),
        Value::Float(1.23)
    );
    assert_eq!(
        sample_struct
            .get_value("f_boolean_vector_required_elements_required[0]")
            .unwrap(),
        Value::Boolean(true)
    );
    // assert_eq!(
    //     sample_struct.get_value("f_enum_vector_required_elements_required").unwrap(),
    //     Value::Boxable(SampleEnum::A.clone_box())
    // );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_required_elements_required[0]")
            .unwrap(),
        Value::Boxable(
            SampleSubstruct {
                subf_string: "sub3".to_string()
            }
            .clone_box()
        )
    );
}

fn test_get_value_by_index_optional_array_required_items() {
    let sample_struct = build_sample_struct();

    assert_eq!(
        sample_struct
            .get_value("f_string_vector_optional_elements_required[0]")
            .unwrap(),
        Value::String("hello".to_string())
    );
    assert_eq!(
        sample_struct
            .get_value("f_integer_vector_optional_elements_required[0]")
            .unwrap(),
        Value::Integer(123)
    );
    assert_eq!(
        sample_struct
            .get_value("f_float_vector_optional_elements_required[0]")
            .unwrap(),
        Value::Float(1.23)
    );
    assert_eq!(
        sample_struct
            .get_value("f_boolean_vector_optional_elements_required[0]")
            .unwrap(),
        Value::Boolean(true)
    );
    // assert_eq!(
    //     sample_struct.get_value("f_enum_vector_optional_elements_required").unwrap(),
    //     Value::Boxable(SampleEnum::A.clone_box())
    // );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_optional_elements_required[0]")
            .unwrap(),
        Value::Boxable(
            SampleSubstruct {
                subf_string: "sub4".to_string()
            }
            .clone_box()
        )
    );
}

fn test_get_value_by_index_optional_array_is_none_required_items() {
    let sample_struct = build_sample_struct_with_null_optionals();

    assert_eq!(
        sample_struct
            .get_value("f_string_vector_optional_elements_required[0]")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_integer_vector_optional_elements_required[0]")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_float_vector_optional_elements_required[0]")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_boolean_vector_optional_elements_required[0]")
            .unwrap(),
        Value::Optional(None)
    );
    // assert_eq!(
    //     sample_struct.get_value("f_enum_vector_optional_elements_required").unwrap(),
    //     Value::Optional(None)
    // );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_optional_elements_required[0]")
            .unwrap(),
        Value::Optional(None)
    );
}

fn test_get_value_by_index_required_array_optional_items() {
    let sample_struct = build_sample_struct();

    assert_eq!(
        sample_struct
            .get_value("f_string_vector_required_elements_optional[0]")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::String("hello".to_string())
    );
    assert_eq!(
        sample_struct
            .get_value("f_string_vector_required_elements_optional[1]")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_integer_vector_required_elements_optional[0]")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Integer(123)
    );
    assert_eq!(
        sample_struct
            .get_value("f_integer_vector_required_elements_optional[1]")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_float_vector_required_elements_optional[0]")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Float(1.23)
    );
    assert_eq!(
        sample_struct
            .get_value("f_float_vector_required_elements_optional[1]")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_boolean_vector_required_elements_optional[0]")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Boolean(true)
    );
    // assert_eq!(
    //     sample_struct.get_value("f_enum_vector_required_elements_optional[1]").unwrap(),
    //     Value::Optional(None)
    // );
    // assert_eq!(
    //     sample_struct.get_value("f_enum_vector_required_elements_optional[0]").unwrap().into_inner().unwrap(),
    //     Value::Boxable(SampleEnum::A.clone_box())
    // );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_required_elements_optional[1]")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_required_elements_optional[0]")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Boxable(
            SampleSubstruct {
                subf_string: "sub5".to_string()
            }
            .clone_box()
        )
    );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_required_elements_optional[1]")
            .unwrap(),
        Value::Optional(None)
    );
}

fn test_get_value_by_index_optional_array_optional_items() {
    let sample_struct = build_sample_struct();

    assert_eq!(
        sample_struct
            .get_value("f_string_vector_optional_elements_optional[0]")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::String("hello".to_string())
    );
    assert_eq!(
        sample_struct
            .get_value("f_string_vector_optional_elements_optional[1]")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_integer_vector_optional_elements_optional[0]")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Integer(123)
    );
    assert_eq!(
        sample_struct
            .get_value("f_integer_vector_optional_elements_optional[1]")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_float_vector_optional_elements_optional[0]")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Float(1.23)
    );
    assert_eq!(
        sample_struct
            .get_value("f_float_vector_optional_elements_optional[1]")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_boolean_vector_optional_elements_optional[0]")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Boolean(true)
    );
    assert_eq!(
        sample_struct
            .get_value("f_boolean_vector_optional_elements_optional[1]")
            .unwrap(),
        Value::Optional(None)
    );
    // assert_eq!(
    //     sample_struct.get_value("f_enum_vector_optional_elements_optional[0]").unwrap().into_inner().unwrap(),
    //     Value::Boxable(SampleEnum::A.clone_box())
    // );
    // assert_eq!(
    //     sample_struct.get_value("f_enum_vector_optional_elements_optional[1]").unwrap(),
    //     Value::Optional(None)
    // );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_optional_elements_optional[0]")
            .unwrap()
            .into_inner()
            .unwrap(),
        Value::Boxable(
            SampleSubstruct {
                subf_string: "sub6".to_string()
            }
            .clone_box()
        )
    );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_optional_elements_optional[1]")
            .unwrap(),
        Value::Optional(None)
    );
}

fn test_get_value_by_index_optional_array_is_none_optional_items() {
    let sample_struct = build_sample_struct_with_null_optionals();

    assert_eq!(
        sample_struct
            .get_value("f_string_vector_optional_elements_optional[0]")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_integer_vector_optional_elements_optional[0]")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_float_vector_optional_elements_optional[0]")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_boolean_vector_optional_elements_optional[0]")
            .unwrap(),
        Value::Optional(None)
    );
    // assert_eq!(
    //     sample_struct.get_value("f_enum_vector_optional_elements_optional[0]").unwrap(),
    //     Value::Optional(None)
    // );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_optional_elements_optional[0]")
            .unwrap(),
        Value::Optional(None)
    );
}

fn test_nested_get_value() {
    let sample_struct = build_sample_struct();
    assert_eq!(
        sample_struct
            .get_value("f_struct_scalar_required.subf_string")
            .unwrap(),
        Value::String("sub1".to_string())
    );
    assert_eq!(
        sample_struct
            .get_value("f_struct_scalar_optional.subf_string")
            .unwrap(),
        Value::String("sub2".to_string())
    );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_required_elements_required[0].subf_string")
            .unwrap(),
        Value::String("sub3".to_string())
    );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_optional_elements_required[0].subf_string")
            .unwrap(),
        Value::String("sub4".to_string())
    );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_required_elements_optional[0].subf_string")
            .unwrap(),
        Value::String("sub5".to_string())
    );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_optional_elements_optional[0].subf_string")
            .unwrap(),
        Value::String("sub6".to_string())
    );
}

fn test_nested_get_value_optional_array_is_none() {
    let sample_struct = build_sample_struct_with_null_optionals();
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_optional_elements_required[0].subf_string")
            .unwrap(),
        Value::Optional(None)
    );
    assert_eq!(
        sample_struct
            .get_value("f_struct_vector_optional_elements_optional[0].subf_string")
            .unwrap(),
        Value::Optional(None)
    );
}

fn main() {
    test_get_value_by_field_scalar();
    test_get_value_by_field_scalar_optional();
    test_get_value_by_field_scalar_optional_none();
    test_get_value_by_field_required_array_required_items();
    test_get_value_by_field_optional_array_required_items();
    test_get_value_by_field_optional_array_is_none_required_items();
    test_get_value_by_field_required_array_optional_items();
    test_get_value_by_field_optional_array_optional_items();
    test_get_value_by_field_optional_array_is_none_optional_items();
    test_get_value_by_index_required_array_required_items();
    test_get_value_by_index_optional_array_required_items();
    test_get_value_by_index_optional_array_is_none_required_items();
    test_get_value_by_index_required_array_optional_items();
    test_get_value_by_index_optional_array_optional_items();
    test_get_value_by_index_optional_array_is_none_optional_items();
    test_nested_get_value();
    test_nested_get_value_optional_array_is_none();
}
