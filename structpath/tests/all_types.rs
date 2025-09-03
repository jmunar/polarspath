use structpath::{FieldInfo, FieldType, StructPath, StructPathError};

#[derive(Debug, Clone, PartialEq)]
pub enum SampleEnum {
    A,
    B,
    C,
}

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
    f_enum_scalar_required: SampleEnum,
    f_string_scalar_optional: Option<String>,
    f_integer_scalar_optional: Option<i64>,
    f_float_scalar_optional: Option<f64>,
    f_boolean_scalar_optional: Option<bool>,
    #[type_hint = "struct"]
    f_struct_scalar_optional: Option<SampleSubstruct>,
    f_enum_scalar_optional: Option<SampleEnum>,
    f_string_vector_required_elements_required: Vec<String>,
    f_integer_vector_required_elements_required: Vec<i64>,
    f_float_vector_required_elements_required: Vec<f64>,
    f_boolean_vector_required_elements_required: Vec<bool>,
    #[type_hint = "struct"]
    f_struct_vector_required_elements_required: Vec<SampleSubstruct>,
    f_enum_vector_required_elements_required: Vec<SampleEnum>,
    f_string_vector_optional_elements_required: Option<Vec<String>>,
    f_integer_vector_optional_elements_required: Option<Vec<i64>>,
    f_float_vector_optional_elements_required: Option<Vec<f64>>,
    f_boolean_vector_optional_elements_required: Option<Vec<bool>>,
    #[type_hint = "struct"]
    f_struct_vector_optional_elements_required: Option<Vec<SampleSubstruct>>,
    f_enum_vector_optional_elements_required: Option<Vec<SampleEnum>>,
    f_string_vector_required_elements_optional: Vec<Option<String>>,
    f_integer_vector_required_elements_optional: Vec<Option<i64>>,
    f_float_vector_required_elements_optional: Vec<Option<f64>>,
    f_boolean_vector_required_elements_optional: Vec<Option<bool>>,
    #[type_hint = "struct"]
    f_struct_vector_required_elements_optional: Vec<Option<SampleSubstruct>>,
    f_enum_vector_required_elements_optional: Vec<Option<SampleEnum>>,
    f_string_vector_optional_elements_optional: Option<Vec<Option<String>>>,
    f_integer_vector_optional_elements_optional: Option<Vec<Option<i64>>>,
    f_float_vector_optional_elements_optional: Option<Vec<Option<f64>>>,
    f_boolean_vector_optional_elements_optional: Option<Vec<Option<bool>>>,
    #[type_hint = "struct"]
    f_struct_vector_optional_elements_optional: Option<Vec<Option<SampleSubstruct>>>,
    f_enum_vector_optional_elements_optional: Option<Vec<Option<SampleEnum>>>,
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
        f_enum_scalar_required: SampleEnum::A,

        // Scalar optional fields
        f_string_scalar_optional: Some("world".to_string()),
        f_integer_scalar_optional: Some(456),
        f_float_scalar_optional: Some(4.56),
        f_boolean_scalar_optional: Some(false),
        f_struct_scalar_optional: Some(SampleSubstruct {
            subf_string: "sub2".to_string(),
        }),
        f_enum_scalar_optional: Some(SampleEnum::B),

        // Vector fields
        f_string_vector_required_elements_required: vec!["hello".to_string(), "world".to_string()],
        f_integer_vector_required_elements_required: vec![123, 456],
        f_float_vector_required_elements_required: vec![1.23, 4.56],
        f_boolean_vector_required_elements_required: vec![true, false],
        f_struct_vector_required_elements_required: vec![SampleSubstruct {
            subf_string: "sub3".to_string(),
        }],
        f_enum_vector_required_elements_required: vec![SampleEnum::A, SampleEnum::B],

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
        f_enum_vector_optional_elements_required: Some(vec![SampleEnum::A, SampleEnum::B]),

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
        f_enum_vector_required_elements_optional: vec![Some(SampleEnum::A), None],

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
        f_enum_vector_optional_elements_optional: Some(vec![Some(SampleEnum::A), None]),
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
        f_enum_scalar_optional: None,
        f_string_vector_optional_elements_required: None,
        f_integer_vector_optional_elements_required: None,
        f_float_vector_optional_elements_required: None,
        f_boolean_vector_optional_elements_required: None,
        f_struct_vector_optional_elements_required: None,
        f_enum_vector_optional_elements_required: None,
        f_string_vector_optional_elements_optional: None,
        f_integer_vector_optional_elements_optional: None,
        f_float_vector_optional_elements_optional: None,
        f_boolean_vector_optional_elements_optional: None,
        f_struct_vector_optional_elements_optional: None,
        f_enum_vector_optional_elements_optional: None,
        ..sample_struct
    }
}

pub fn sample_substruct_type() -> FieldType {
    FieldType::StructPath(
        "SampleSubstruct".to_string(),
        vec![FieldInfo::new("subf_string", FieldType::String)],
    )
}

#[test]
fn test_field_is_required_struct() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct();

    let t = SampleStruct::get_type("f_string_scalar_required")?;
    assert_eq!(t, FieldType::String);
    let v = sample_struct.get_value("f_string_scalar_required")?;
    assert_eq!(v, "hello".to_string());

    let t = SampleStruct::get_type("f_integer_scalar_required")?;
    assert_eq!(t, FieldType::Integer);
    let v = sample_struct.get_value("f_integer_scalar_required")?;
    assert_eq!(v, 123);

    let t = SampleStruct::get_type("f_float_scalar_required")?;
    assert_eq!(t, FieldType::Float);
    let v = sample_struct.get_value("f_float_scalar_required")?;
    assert_eq!(v, 1.23);

    let t = SampleStruct::get_type("f_boolean_scalar_required")?;
    assert_eq!(t, FieldType::Boolean);
    let v = sample_struct.get_value("f_boolean_scalar_required")?;
    assert_eq!(v, true);

    let t = SampleStruct::get_type("f_struct_scalar_required")?;
    assert_eq!(t, sample_substruct_type());
    let v = sample_struct.get_value("f_struct_scalar_required")?;
    assert_eq!(
        v,
        &SampleSubstruct {
            subf_string: "sub1".to_string()
        }
    );

    let t = SampleStruct::get_type("f_enum_scalar_required")?;
    assert_eq!(t, FieldType::Unknown);
    let v = sample_struct.get_value("f_enum_scalar_required")?;
    assert_eq!(v, &SampleEnum::A);

    Ok(())
}

#[test]
fn test_field_is_some_struct() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct();

    let t = SampleStruct::get_type("f_string_scalar_optional")?;
    assert_eq!(t, FieldType::Option(Box::new(FieldType::String)));
    let v = sample_struct.get_value("f_string_scalar_optional")?;
    assert_eq!(v, Some("world".to_string()));

    let t = SampleStruct::get_type("f_integer_scalar_optional")?;
    assert_eq!(t, FieldType::Option(Box::new(FieldType::Integer)));
    let v = sample_struct.get_value("f_integer_scalar_optional")?;
    assert_eq!(v, Some(456));

    let t = SampleStruct::get_type("f_float_scalar_optional")?;
    assert_eq!(t, FieldType::Option(Box::new(FieldType::Float)));
    let v = sample_struct.get_value("f_float_scalar_optional")?;
    assert_eq!(v, Some(4.56));

    let t = SampleStruct::get_type("f_boolean_scalar_optional")?;
    assert_eq!(t, FieldType::Option(Box::new(FieldType::Boolean)));
    let v = sample_struct.get_value("f_boolean_scalar_optional")?;
    assert_eq!(v, Some(false));

    let t = SampleStruct::get_type("f_struct_scalar_optional")?;
    assert_eq!(t, FieldType::Option(Box::new(sample_substruct_type())));
    let v = sample_struct.get_value("f_struct_scalar_optional")?;
    assert_eq!(
        v,
        Some(&SampleSubstruct {
            subf_string: "sub2".to_string()
        })
    );

    let t = SampleStruct::get_type("f_enum_scalar_optional")?;
    assert_eq!(t, FieldType::Option(Box::new(FieldType::Unknown)));
    let v = sample_struct.get_value("f_enum_scalar_optional")?;
    assert_eq!(v, Some(&SampleEnum::B));

    Ok(())
}

#[test]
fn test_field_is_none_struct() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v = sample_struct.get_value("f_string_scalar_optional")?;
    assert_eq!(v, None::<String>);
    let v = sample_struct.get_value("f_integer_scalar_optional")?;
    assert_eq!(v, None::<i64>);
    let v = sample_struct.get_value("f_float_scalar_optional")?;
    assert_eq!(v, None::<f64>);
    let v = sample_struct.get_value("f_boolean_scalar_optional")?;
    assert_eq!(v, None::<bool>);
    let v = sample_struct.get_value("f_struct_scalar_optional")?;
    assert_eq!(v, None::<&SampleSubstruct>);
    let v = sample_struct.get_value("f_enum_scalar_optional")?;
    assert_eq!(v, None::<&SampleEnum>);

    Ok(())
}

#[test]
fn test_field_is_required_array_with_required_items() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct();

    let t = SampleStruct::get_type("f_string_vector_required_elements_required")?;
    assert_eq!(t, FieldType::Vec(Box::new(FieldType::String)));
    let v = sample_struct.get_value("f_string_vector_required_elements_required")?;
    assert_eq!(v, &vec!["hello".to_string(), "world".to_string()]);

    let t = SampleStruct::get_type("f_integer_vector_required_elements_required")?;
    assert_eq!(t, FieldType::Vec(Box::new(FieldType::Integer)));
    let v = sample_struct.get_value("f_integer_vector_required_elements_required")?;
    assert_eq!(v, &vec![123_i64, 456_i64]);

    let t = SampleStruct::get_type("f_float_vector_required_elements_required")?;
    assert_eq!(t, FieldType::Vec(Box::new(FieldType::Float)));
    let v = sample_struct.get_value("f_float_vector_required_elements_required")?;
    assert_eq!(v, &vec![1.23, 4.56]);

    let t = SampleStruct::get_type("f_boolean_vector_required_elements_required")?;
    assert_eq!(t, FieldType::Vec(Box::new(FieldType::Boolean)));
    let v = sample_struct.get_value("f_boolean_vector_required_elements_required")?;
    assert_eq!(v, &vec![true, false]);

    let t = SampleStruct::get_type("f_struct_vector_required_elements_required")?;
    assert_eq!(t, FieldType::Vec(Box::new(sample_substruct_type())));
    let v = sample_struct.get_value("f_struct_vector_required_elements_required")?;
    assert_eq!(
        v,
        &vec![SampleSubstruct {
            subf_string: "sub3".to_string()
        }],
    );

    let t = SampleStruct::get_type("f_enum_vector_required_elements_required")?;
    assert_eq!(t, FieldType::Vec(Box::new(FieldType::Unknown)));
    let v = sample_struct.get_value("f_enum_vector_required_elements_required")?;
    assert_eq!(v, &vec![SampleEnum::A, SampleEnum::B]);

    Ok(())
}

#[test]
fn test_field_is_some_array_with_required_items() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct();

    let t = SampleStruct::get_type("f_string_vector_optional_elements_required")?;
    assert_eq!(
        t,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::String))))
    );
    let v = sample_struct.get_value("f_string_vector_optional_elements_required")?;
    assert_eq!(v, Some(&vec!["hello".to_string(), "world".to_string()]));

    let t = SampleStruct::get_type("f_integer_vector_optional_elements_required")?;
    assert_eq!(
        t,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Integer))))
    );
    let v = sample_struct.get_value("f_integer_vector_optional_elements_required")?;
    assert_eq!(v, Some(&vec![123_i64, 456_i64]));

    let t = SampleStruct::get_type("f_float_vector_optional_elements_required")?;
    assert_eq!(
        t,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Float))))
    );
    let v = sample_struct.get_value("f_float_vector_optional_elements_required")?;
    assert_eq!(v, Some(&vec![1.23, 4.56]));

    let t = SampleStruct::get_type("f_boolean_vector_optional_elements_required")?;
    assert_eq!(
        t,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Boolean))))
    );
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_required")?;
    assert_eq!(v, Some(&vec![true, false]));

    let t = SampleStruct::get_type("f_struct_vector_optional_elements_required")?;
    assert_eq!(
        t,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(sample_substruct_type()))))
    );
    let v = sample_struct.get_value("f_struct_vector_optional_elements_required")?;
    assert_eq!(
        v,
        Some(&vec![SampleSubstruct {
            subf_string: "sub4".to_string(),
        }])
    );

    let t = SampleStruct::get_type("f_enum_vector_optional_elements_required")?;
    assert_eq!(
        t,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Unknown))))
    );
    let v = sample_struct.get_value("f_enum_vector_optional_elements_required")?;
    assert_eq!(v, Some(&vec![SampleEnum::A, SampleEnum::B]));

    Ok(())
}

#[test]
fn test_field_is_none_array_with_required_items() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v = sample_struct.get_value("f_string_vector_optional_elements_required")?;
    assert_eq!(v, None::<&Vec<String>>);
    let v = sample_struct.get_value("f_integer_vector_optional_elements_required")?;
    assert_eq!(v, None::<&Vec<i64>>);
    let v = sample_struct.get_value("f_float_vector_optional_elements_required")?;
    assert_eq!(v, None::<&Vec<f64>>);
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_required")?;
    assert_eq!(v, None::<&Vec<bool>>);
    let v = sample_struct.get_value("f_struct_vector_optional_elements_required")?;
    assert_eq!(v, None::<&Vec<SampleSubstruct>>);
    let v = sample_struct.get_value("f_enum_vector_optional_elements_required")?;
    assert_eq!(v, None::<&Vec<SampleEnum>>);

    Ok(())
}

#[test]
fn test_field_is_required_array_with_optional_items() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct();

    let t = SampleStruct::get_type("f_string_vector_required_elements_optional")?;
    assert_eq!(
        t,
        FieldType::Vec(Box::new(FieldType::Option(Box::new(FieldType::String))))
    );
    let v = sample_struct.get_value("f_string_vector_required_elements_optional")?;
    assert_eq!(v, &vec![Some("hello".to_string()), None]);

    let t = SampleStruct::get_type("f_integer_vector_required_elements_optional")?;
    assert_eq!(
        t,
        FieldType::Vec(Box::new(FieldType::Option(Box::new(FieldType::Integer))))
    );
    let v = sample_struct.get_value("f_integer_vector_required_elements_optional")?;
    assert_eq!(v, &vec![Some(123_i64), None]);

    let t = SampleStruct::get_type("f_float_vector_required_elements_optional")?;
    assert_eq!(
        t,
        FieldType::Vec(Box::new(FieldType::Option(Box::new(FieldType::Float))))
    );
    let v = sample_struct.get_value("f_float_vector_required_elements_optional")?;
    assert_eq!(v, &vec![Some(1.23), None]);

    let t = SampleStruct::get_type("f_boolean_vector_required_elements_optional")?;
    assert_eq!(
        t,
        FieldType::Vec(Box::new(FieldType::Option(Box::new(FieldType::Boolean))))
    );
    let v = sample_struct.get_value("f_boolean_vector_required_elements_optional")?;
    assert_eq!(v, &vec![Some(true), None]);

    let t = SampleStruct::get_type("f_struct_vector_required_elements_optional")?;
    assert_eq!(
        t,
        FieldType::Vec(Box::new(FieldType::Option(Box::new(
            sample_substruct_type()
        ))))
    );
    let v = sample_struct.get_value("f_struct_vector_required_elements_optional")?;
    assert_eq!(
        v,
        &vec![
            Some(SampleSubstruct {
                subf_string: "sub5".to_string(),
            }),
            None
        ]
    );

    let t = SampleStruct::get_type("f_enum_vector_required_elements_optional")?;
    assert_eq!(
        t,
        FieldType::Vec(Box::new(FieldType::Option(Box::new(FieldType::Unknown))))
    );
    let v = sample_struct.get_value("f_enum_vector_required_elements_optional")?;
    assert_eq!(v, &vec![Some(SampleEnum::A), None]);

    Ok(())
}

#[test]
fn test_field_is_some_array_with_optional_items() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct();

    let t = SampleStruct::get_type("f_string_vector_optional_elements_optional")?;
    assert_eq!(
        t,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Option(
            Box::new(FieldType::String)
        )))))
    );
    let v = sample_struct.get_value("f_string_vector_optional_elements_optional")?;
    assert_eq!(v, Some(&vec![Some("hello".to_string()), None]));

    let t = SampleStruct::get_type("f_integer_vector_optional_elements_optional")?;
    assert_eq!(
        t,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Option(
            Box::new(FieldType::Integer)
        )))))
    );
    let v = sample_struct.get_value("f_integer_vector_optional_elements_optional")?;
    assert_eq!(v, Some(&vec![Some(123_i64), None]));

    let t = SampleStruct::get_type("f_float_vector_optional_elements_optional")?;
    assert_eq!(
        t,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Option(
            Box::new(FieldType::Float)
        )))))
    );
    let v = sample_struct.get_value("f_float_vector_optional_elements_optional")?;
    assert_eq!(v, Some(&vec![Some(1.23), None]));

    let t = SampleStruct::get_type("f_boolean_vector_optional_elements_optional")?;
    assert_eq!(
        t,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Option(
            Box::new(FieldType::Boolean)
        )))))
    );
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_optional")?;
    assert_eq!(v, Some(&vec![Some(true), None]));

    let t = SampleStruct::get_type("f_struct_vector_optional_elements_optional")?;
    assert_eq!(
        t,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Option(
            Box::new(sample_substruct_type())
        )))))
    );
    let v = sample_struct.get_value("f_struct_vector_optional_elements_optional")?;
    assert_eq!(
        v,
        Some(&vec![
            Some(SampleSubstruct {
                subf_string: "sub6".to_string(),
            }),
            None
        ])
    );

    let t = SampleStruct::get_type("f_enum_vector_optional_elements_optional")?;
    assert_eq!(
        t,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Option(
            Box::new(FieldType::Unknown)
        )))))
    );
    let v = sample_struct.get_value("f_enum_vector_optional_elements_optional")?;
    assert_eq!(v, Some(&vec![Some(SampleEnum::A), None]));

    Ok(())
}

#[test]
fn test_field_is_none_array_with_optional_items() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v = sample_struct.get_value("f_string_vector_optional_elements_optional")?;
    assert_eq!(v, None::<&Vec<Option<String>>>);
    let v = sample_struct.get_value("f_integer_vector_optional_elements_optional")?;
    assert_eq!(v, None::<&Vec<Option<i64>>>);
    let v = sample_struct.get_value("f_float_vector_optional_elements_optional")?;
    assert_eq!(v, None::<&Vec<Option<f64>>>);
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_optional")?;
    assert_eq!(v, None::<&Vec<Option<bool>>>);
    let v = sample_struct.get_value("f_struct_vector_optional_elements_optional")?;
    assert_eq!(v, None::<&Vec<Option<SampleSubstruct>>>);
    let v = sample_struct.get_value("f_enum_vector_optional_elements_optional")?;
    assert_eq!(v, None::<&Vec<Option<SampleEnum>>>);

    Ok(())
}

#[test]
fn test_array_index_out_of_bounds() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct();

    let v = sample_struct.get_value("f_string_vector_required_elements_required[2]");
    assert_eq!(v.unwrap_err(), StructPathError::IndexOutOfBounds(2));
    let v = sample_struct.get_value_safe("f_string_vector_required_elements_required[2]")?;
    assert_eq!(v, None::<String>);
    let v = sample_struct.get_value("f_string_vector_optional_elements_required[2]");
    assert_eq!(v.unwrap_err(), StructPathError::IndexOutOfBounds(2));
    let v = sample_struct.get_value_safe("f_string_vector_optional_elements_required[2]")?;
    assert_eq!(v, None::<String>);
    let v = sample_struct.get_value("f_string_vector_required_elements_optional[2]");
    assert_eq!(v.unwrap_err(), StructPathError::IndexOutOfBounds(2));
    let v = sample_struct.get_value_safe("f_string_vector_required_elements_optional[2]")?;
    assert_eq!(v, None::<String>);
    let v = sample_struct.get_value("f_string_vector_optional_elements_optional[2]");
    assert_eq!(v.unwrap_err(), StructPathError::IndexOutOfBounds(2));
    let v = sample_struct.get_value_safe("f_string_vector_optional_elements_optional[2]")?;
    assert_eq!(v, None::<String>);

    Ok(())
}

#[test]
fn test_array_index_is_required_array_with_required_items() -> Result<(), Box<dyn std::error::Error>>
{
    let sample_struct = build_sample_struct();

    let t = SampleStruct::get_type("f_string_vector_required_elements_required[0]")?;
    assert_eq!(t, FieldType::String);
    let v = sample_struct.get_value("f_string_vector_required_elements_required[0]")?;
    assert_eq!(v, "hello".to_string());

    let t = SampleStruct::get_type("f_integer_vector_required_elements_required[0]")?;
    assert_eq!(t, FieldType::Integer);
    let v = sample_struct.get_value("f_integer_vector_required_elements_required[0]")?;
    assert_eq!(v, 123_i64);

    let t = SampleStruct::get_type("f_float_vector_required_elements_required[0]")?;
    assert_eq!(t, FieldType::Float);
    let v = sample_struct.get_value("f_float_vector_required_elements_required[0]")?;
    assert_eq!(v, 1.23);

    let t = SampleStruct::get_type("f_boolean_vector_required_elements_required[0]")?;
    assert_eq!(t, FieldType::Boolean);
    let v = sample_struct.get_value("f_boolean_vector_required_elements_required[0]")?;
    assert_eq!(v, true);

    let t = SampleStruct::get_type("f_struct_vector_required_elements_required[0]")?;
    assert_eq!(t, sample_substruct_type());
    let v = sample_struct.get_value("f_struct_vector_required_elements_required[0]")?;
    assert_eq!(
        v,
        &SampleSubstruct {
            subf_string: "sub3".to_string(),
        }
    );

    let t = SampleStruct::get_type("f_enum_vector_required_elements_required[0]")?;
    assert_eq!(t, FieldType::Unknown);
    let v = sample_struct.get_value("f_enum_vector_required_elements_required[0]")?;
    assert_eq!(v, &SampleEnum::A);

    Ok(())
}

#[test]
fn test_array_index_is_optional_array_with_required_items() -> Result<(), Box<dyn std::error::Error>>
{
    let sample_struct = build_sample_struct();

    let t = SampleStruct::get_type("f_string_vector_optional_elements_required[0]")?;
    assert_eq!(t, FieldType::String);
    let v = sample_struct.get_value("f_string_vector_optional_elements_required[0]")?;
    assert_eq!(v, "hello".to_string());

    let t = SampleStruct::get_type("f_integer_vector_optional_elements_required[0]")?;
    assert_eq!(t, FieldType::Integer);
    let v = sample_struct.get_value("f_integer_vector_optional_elements_required[0]")?;
    assert_eq!(v, 123_i64);

    let t = SampleStruct::get_type("f_float_vector_optional_elements_required[0]")?;
    assert_eq!(t, FieldType::Float);
    let v = sample_struct.get_value("f_float_vector_optional_elements_required[0]")?;
    assert_eq!(v, 1.23);

    let t = SampleStruct::get_type("f_boolean_vector_optional_elements_required[0]")?;
    assert_eq!(t, FieldType::Boolean);
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_required[0]")?;
    assert_eq!(v, true);

    let t = SampleStruct::get_type("f_struct_vector_optional_elements_required[0]")?;
    assert_eq!(t, sample_substruct_type());
    let v = sample_struct.get_value("f_struct_vector_optional_elements_required[0]")?;
    assert_eq!(
        v,
        &SampleSubstruct {
            subf_string: "sub4".to_string(),
        }
    );

    let t = SampleStruct::get_type("f_enum_vector_optional_elements_required[0]")?;
    assert_eq!(t, FieldType::Unknown);
    let v = sample_struct.get_value("f_enum_vector_optional_elements_required[0]")?;
    assert_eq!(v, &SampleEnum::A);

    Ok(())
}

#[test]
fn test_array_index_optional_array_is_none_required_items() -> Result<(), Box<dyn std::error::Error>>
{
    let sample_struct = build_sample_struct_with_null_optionals();

    let v = sample_struct.get_value("f_string_vector_optional_elements_required[0]");
    assert_eq!(v.unwrap_err(), StructPathError::NullValue);

    let v = sample_struct.get_value("f_integer_vector_optional_elements_required[0]");
    assert_eq!(v.unwrap_err(), StructPathError::NullValue);
    let v = sample_struct.get_value("f_float_vector_optional_elements_required[0]");
    assert_eq!(v.unwrap_err(), StructPathError::NullValue);
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_required[0]");
    assert_eq!(v.unwrap_err(), StructPathError::NullValue);
    let v = sample_struct.get_value("f_struct_vector_optional_elements_required[0]");
    assert_eq!(v.unwrap_err(), StructPathError::NullValue);
    let v = sample_struct.get_value("f_enum_vector_optional_elements_required[0]");
    assert_eq!(v.unwrap_err(), StructPathError::NullValue);

    Ok(())
}

#[test]
fn test_array_index_required_array_optional_items() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct();

    let t = SampleStruct::get_type("f_string_vector_required_elements_optional[0]")?;
    assert_eq!(t, FieldType::Option(Box::new(FieldType::String)));
    let v = sample_struct.get_value("f_string_vector_required_elements_optional[0]")?;
    assert_eq!(v, Some("hello".to_string()));
    let v = sample_struct.get_value("f_string_vector_required_elements_optional[1]")?;
    assert_eq!(v, None::<String>);

    let t = SampleStruct::get_type("f_integer_vector_required_elements_optional[0]")?;
    assert_eq!(t, FieldType::Option(Box::new(FieldType::Integer)));
    let v = sample_struct.get_value("f_integer_vector_required_elements_optional[0]")?;
    assert_eq!(v, Some(123_i64));
    let v = sample_struct.get_value("f_integer_vector_required_elements_optional[1]")?;
    assert_eq!(v, None::<&i64>);

    let t = SampleStruct::get_type("f_float_vector_required_elements_optional[0]")?;
    assert_eq!(t, FieldType::Option(Box::new(FieldType::Float)));
    let v = sample_struct.get_value("f_float_vector_required_elements_optional[0]")?;
    assert_eq!(v, Some(1.23));
    let v = sample_struct.get_value("f_float_vector_required_elements_optional[1]")?;
    assert_eq!(v, None::<&f64>);

    let t = SampleStruct::get_type("f_boolean_vector_required_elements_optional[0]")?;
    assert_eq!(t, FieldType::Option(Box::new(FieldType::Boolean)));
    let v = sample_struct.get_value("f_boolean_vector_required_elements_optional[0]")?;
    assert_eq!(v, Some(true));
    let v = sample_struct.get_value("f_boolean_vector_required_elements_optional[1]")?;
    assert_eq!(v, None::<&bool>);

    let t = SampleStruct::get_type("f_struct_vector_required_elements_optional[0]")?;
    assert_eq!(t, FieldType::Option(Box::new(sample_substruct_type())));
    let v = sample_struct.get_value("f_struct_vector_required_elements_optional[0]")?;
    assert_eq!(
        v,
        Some(&SampleSubstruct {
            subf_string: "sub5".to_string(),
        })
    );
    let v = sample_struct.get_value("f_struct_vector_required_elements_optional[1]")?;
    assert_eq!(v, None::<&SampleSubstruct>);

    let t = SampleStruct::get_type("f_enum_vector_required_elements_optional[0]")?;
    assert_eq!(t, FieldType::Option(Box::new(FieldType::Unknown)));
    let v = sample_struct.get_value("f_enum_vector_required_elements_optional[0]")?;
    assert_eq!(v, Some(&SampleEnum::A));
    let v = sample_struct.get_value("f_enum_vector_required_elements_optional[1]")?;
    assert_eq!(v, None::<&SampleEnum>);

    Ok(())
}

#[test]
fn test_get_value_by_index_optional_array_optional_items() -> Result<(), Box<dyn std::error::Error>>
{
    let sample_struct = build_sample_struct();

    let t = SampleStruct::get_type("f_string_vector_optional_elements_optional[0]")?;
    assert_eq!(t, FieldType::Option(Box::new(FieldType::String)));
    let v = sample_struct.get_value("f_string_vector_optional_elements_optional[0]")?;
    assert_eq!(v, Some("hello".to_string()));
    let v = sample_struct.get_value("f_string_vector_optional_elements_optional[1]")?;
    assert_eq!(v, None::<String>);

    let t = SampleStruct::get_type("f_integer_vector_optional_elements_optional[0]")?;
    assert_eq!(t, FieldType::Option(Box::new(FieldType::Integer)));
    let v = sample_struct.get_value("f_integer_vector_optional_elements_optional[0]")?;
    assert_eq!(v, Some(123));
    let v = sample_struct.get_value("f_integer_vector_optional_elements_optional[1]")?;
    assert_eq!(v, None::<&i64>);

    let t = SampleStruct::get_type("f_float_vector_optional_elements_optional[0]")?;
    assert_eq!(t, FieldType::Option(Box::new(FieldType::Float)));
    let v = sample_struct.get_value("f_float_vector_optional_elements_optional[0]")?;
    assert_eq!(v, Some(1.23));
    let v = sample_struct.get_value("f_float_vector_optional_elements_optional[1]")?;
    assert_eq!(v, None::<&f64>);

    let t = SampleStruct::get_type("f_boolean_vector_optional_elements_optional[0]")?;
    assert_eq!(t, FieldType::Option(Box::new(FieldType::Boolean)));
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_optional[0]")?;
    assert_eq!(v, Some(true));
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_optional[1]")?;
    assert_eq!(v, None::<&bool>);

    let t = SampleStruct::get_type("f_struct_vector_optional_elements_optional[0]")?;
    assert_eq!(t, FieldType::Option(Box::new(sample_substruct_type())));
    let v = sample_struct.get_value("f_struct_vector_optional_elements_optional[0]")?;
    assert_eq!(
        v,
        Some(&SampleSubstruct {
            subf_string: "sub6".to_string()
        })
    );
    let v = sample_struct.get_value("f_struct_vector_optional_elements_optional[1]")?;
    assert_eq!(v, None::<&SampleSubstruct>);

    let t = SampleStruct::get_type("f_enum_vector_optional_elements_optional[0]")?;
    assert_eq!(t, FieldType::Option(Box::new(FieldType::Unknown)));
    let v = sample_struct.get_value("f_enum_vector_optional_elements_optional[0]")?;
    assert_eq!(v, Some(&SampleEnum::A));
    let v = sample_struct.get_value("f_enum_vector_optional_elements_optional[1]")?;
    assert_eq!(v, None::<&SampleEnum>);

    Ok(())
}

#[test]
fn test_get_value_by_index_optional_array_is_none_optional_items(
) -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v = sample_struct.get_value("f_string_vector_optional_elements_optional[0]");
    assert_eq!(v.unwrap_err(), StructPathError::NullValue);
    let v = sample_struct.get_value("f_integer_vector_optional_elements_optional[0]");
    assert_eq!(v.unwrap_err(), StructPathError::NullValue);
    let v = sample_struct.get_value("f_float_vector_optional_elements_optional[0]");
    assert_eq!(v.unwrap_err(), StructPathError::NullValue);
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_optional[0]");
    assert_eq!(v.unwrap_err(), StructPathError::NullValue);
    let v = sample_struct.get_value("f_struct_vector_optional_elements_optional[0]");
    assert_eq!(v.unwrap_err(), StructPathError::NullValue);
    let v = sample_struct.get_value("f_enum_vector_optional_elements_optional[0]");
    assert_eq!(v.unwrap_err(), StructPathError::NullValue);

    Ok(())
}

#[test]
fn test_nested_get_value() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct();

    let t = SampleStruct::get_type("f_struct_scalar_required.subf_string")?;
    assert_eq!(t, FieldType::String);
    let v = sample_struct.get_value("f_struct_scalar_required.subf_string")?;
    assert_eq!(v, "sub1".to_string());

    let t = SampleStruct::get_type("f_struct_scalar_optional.subf_string")?;
    assert_eq!(t, FieldType::String);
    let v = sample_struct.get_value("f_struct_scalar_optional.subf_string")?;
    assert_eq!(v, "sub2".to_string());

    let t = SampleStruct::get_type("f_struct_vector_required_elements_required[0].subf_string")?;
    assert_eq!(t, FieldType::String);
    let v = sample_struct.get_value("f_struct_vector_required_elements_required[0].subf_string")?;
    assert_eq!(v, "sub3".to_string());

    let t = SampleStruct::get_type("f_struct_vector_optional_elements_required[0].subf_string")?;
    assert_eq!(t, FieldType::String);
    let v = sample_struct.get_value("f_struct_vector_optional_elements_required[0].subf_string")?;
    assert_eq!(v, "sub4".to_string());

    let t = SampleStruct::get_type("f_struct_vector_required_elements_optional[0].subf_string")?;
    assert_eq!(t, FieldType::String);
    let v = sample_struct.get_value("f_struct_vector_required_elements_optional[0].subf_string")?;
    assert_eq!(v, "sub5".to_string());

    let t = SampleStruct::get_type("f_struct_vector_optional_elements_optional[0].subf_string")?;
    assert_eq!(t, FieldType::String);
    let v = sample_struct.get_value("f_struct_vector_optional_elements_optional[0].subf_string")?;
    assert_eq!(v, "sub6".to_string());

    Ok(())
}

#[test]
fn test_nested_get_value_optional_array_is_none() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v = sample_struct.get_value("f_struct_scalar_optional.subf_string");
    assert_eq!(v.unwrap_err(), StructPathError::NullValue);

    let v = sample_struct.get_value("f_struct_vector_optional_elements_required[0].subf_string");
    assert_eq!(v.unwrap_err(), StructPathError::NullValue);
    let v = sample_struct.get_value("f_struct_vector_optional_elements_optional[0].subf_string");
    assert_eq!(v.unwrap_err(), StructPathError::NullValue);

    Ok(())
}
