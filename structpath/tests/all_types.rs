use structpath::{FieldType, FromValue, StructPath};

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

#[test]
fn test_get_type_by_field() -> Result<(), Box<dyn std::error::Error>> {
    let v = SampleStruct::get_type("f_string_scalar_required")?;
    assert_eq!(v, FieldType::String);
    let v = SampleStruct::get_type("f_integer_scalar_required")?;
    assert_eq!(v, FieldType::Integer);
    let v = SampleStruct::get_type("f_float_scalar_required")?;
    assert_eq!(v, FieldType::Float);
    let v = SampleStruct::get_type("f_boolean_scalar_required")?;
    assert_eq!(v, FieldType::Boolean);
    let v = SampleStruct::get_type("f_struct_scalar_required")?;
    assert_eq!(v, FieldType::StructPath("SampleSubstruct".to_string()));
    let v = SampleStruct::get_type("f_enum_scalar_required")?;
    assert_eq!(v, FieldType::Unknown);

    let v = SampleStruct::get_type("f_string_scalar_optional")?;
    assert_eq!(v, FieldType::Option(Box::new(FieldType::String)));
    let v = SampleStruct::get_type("f_integer_scalar_optional")?;
    assert_eq!(v, FieldType::Option(Box::new(FieldType::Integer)));
    let v = SampleStruct::get_type("f_float_scalar_optional")?;
    assert_eq!(v, FieldType::Option(Box::new(FieldType::Float)));
    let v = SampleStruct::get_type("f_boolean_scalar_optional")?;
    assert_eq!(v, FieldType::Option(Box::new(FieldType::Boolean)));
    let v = SampleStruct::get_type("f_struct_scalar_optional")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::StructPath(
            "SampleSubstruct".to_string()
        )))
    );
    let v = SampleStruct::get_type("f_enum_scalar_optional")?;
    assert_eq!(v, FieldType::Option(Box::new(FieldType::Unknown)));

    let v = SampleStruct::get_type("f_string_vector_required_elements_required")?;
    assert_eq!(v, FieldType::Vec(Box::new(FieldType::String)));
    let v = SampleStruct::get_type("f_integer_vector_required_elements_required")?;
    assert_eq!(v, FieldType::Vec(Box::new(FieldType::Integer)));
    let v = SampleStruct::get_type("f_float_vector_required_elements_required")?;
    assert_eq!(v, FieldType::Vec(Box::new(FieldType::Float)));
    let v = SampleStruct::get_type("f_boolean_vector_required_elements_required")?;
    assert_eq!(v, FieldType::Vec(Box::new(FieldType::Boolean)));
    let v = SampleStruct::get_type("f_struct_vector_required_elements_required")?;
    assert_eq!(
        v,
        FieldType::Vec(Box::new(FieldType::StructPath(
            "SampleSubstruct".to_string()
        )))
    );
    let v = SampleStruct::get_type("f_enum_vector_required_elements_required")?;
    assert_eq!(v, FieldType::Vec(Box::new(FieldType::Unknown)));

    let v = SampleStruct::get_type("f_string_vector_optional_elements_required")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::String))))
    );
    let v = SampleStruct::get_type("f_integer_vector_optional_elements_required")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Integer))))
    );
    let v = SampleStruct::get_type("f_float_vector_optional_elements_required")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Float))))
    );
    let v = SampleStruct::get_type("f_boolean_vector_optional_elements_required")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Boolean))))
    );
    let v = SampleStruct::get_type("f_struct_vector_optional_elements_required")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::StructPath(
            "SampleSubstruct".to_string()
        )))))
    );
    let v = SampleStruct::get_type("f_enum_vector_optional_elements_required")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Unknown))))
    );

    let v = SampleStruct::get_type("f_string_vector_required_elements_optional")?;
    assert_eq!(
        v,
        FieldType::Vec(Box::new(FieldType::Option(Box::new(FieldType::String))))
    );
    let v = SampleStruct::get_type("f_integer_vector_required_elements_optional")?;
    assert_eq!(
        v,
        FieldType::Vec(Box::new(FieldType::Option(Box::new(FieldType::Integer))))
    );
    let v = SampleStruct::get_type("f_float_vector_required_elements_optional")?;
    assert_eq!(
        v,
        FieldType::Vec(Box::new(FieldType::Option(Box::new(FieldType::Float))))
    );
    let v = SampleStruct::get_type("f_boolean_vector_required_elements_optional")?;
    assert_eq!(
        v,
        FieldType::Vec(Box::new(FieldType::Option(Box::new(FieldType::Boolean))))
    );
    let v = SampleStruct::get_type("f_struct_vector_required_elements_optional")?;
    assert_eq!(
        v,
        FieldType::Vec(Box::new(FieldType::Option(Box::new(
            FieldType::StructPath("SampleSubstruct".to_string())
        ))))
    );
    let v = SampleStruct::get_type("f_enum_vector_required_elements_optional")?;
    assert_eq!(
        v,
        FieldType::Vec(Box::new(FieldType::Option(Box::new(FieldType::Unknown))))
    );

    let v = SampleStruct::get_type("f_string_vector_optional_elements_optional")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Option(
            Box::new(FieldType::String)
        )))))
    );
    let v = SampleStruct::get_type("f_integer_vector_optional_elements_optional")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Option(
            Box::new(FieldType::Integer)
        )))))
    );
    let v = SampleStruct::get_type("f_float_vector_optional_elements_optional")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Option(
            Box::new(FieldType::Float)
        )))))
    );
    let v = SampleStruct::get_type("f_boolean_vector_optional_elements_optional")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Option(
            Box::new(FieldType::Boolean)
        )))))
    );
    let v = SampleStruct::get_type("f_struct_vector_optional_elements_optional")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Option(
            Box::new(FieldType::StructPath("SampleSubstruct".to_string()))
        )))))
    );
    let v = SampleStruct::get_type("f_enum_vector_optional_elements_optional")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Vec(Box::new(FieldType::Option(
            Box::new(FieldType::Unknown)
        )))))
    );

    Ok(())
}

#[test]
fn test_get_type_by_index() -> Result<(), Box<dyn std::error::Error>> {
    let v = SampleStruct::get_type("f_string_vector_required_elements_required[0]")?;
    assert_eq!(v, FieldType::String);
    let v = SampleStruct::get_type("f_integer_vector_required_elements_required[0]")?;
    assert_eq!(v, FieldType::Integer);
    let v = SampleStruct::get_type("f_float_vector_required_elements_required[0]")?;
    assert_eq!(v, FieldType::Float);
    let v = SampleStruct::get_type("f_boolean_vector_required_elements_required[0]")?;
    assert_eq!(v, FieldType::Boolean);
    let v = SampleStruct::get_type("f_struct_vector_required_elements_required[0]")?;
    assert_eq!(v, FieldType::StructPath("SampleSubstruct".to_string()));
    let v = SampleStruct::get_type("f_enum_vector_required_elements_required[0]")?;
    assert_eq!(v, FieldType::Unknown);

    let v = SampleStruct::get_type("f_string_vector_optional_elements_required[0]")?;
    assert_eq!(v, FieldType::Option(Box::new(FieldType::String)));
    let v = SampleStruct::get_type("f_integer_vector_optional_elements_required[0]")?;
    assert_eq!(v, FieldType::Option(Box::new(FieldType::Integer)));
    let v = SampleStruct::get_type("f_float_vector_optional_elements_required[0]")?;
    assert_eq!(v, FieldType::Option(Box::new(FieldType::Float)));
    let v = SampleStruct::get_type("f_boolean_vector_optional_elements_required[0]")?;
    assert_eq!(v, FieldType::Option(Box::new(FieldType::Boolean)));
    let v = SampleStruct::get_type("f_struct_vector_optional_elements_required[0]")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::StructPath(
            "SampleSubstruct".to_string()
        )))
    );
    let v = SampleStruct::get_type("f_enum_vector_optional_elements_required[0]")?;
    assert_eq!(v, FieldType::Option(Box::new(FieldType::Unknown)));

    let v = SampleStruct::get_type("f_string_vector_required_elements_optional[0]")?;
    assert_eq!(v, FieldType::Option(Box::new(FieldType::String)));
    let v = SampleStruct::get_type("f_integer_vector_required_elements_optional[0]")?;
    assert_eq!(v, FieldType::Option(Box::new(FieldType::Integer)));
    let v = SampleStruct::get_type("f_float_vector_required_elements_optional[0]")?;
    assert_eq!(v, FieldType::Option(Box::new(FieldType::Float)));
    let v = SampleStruct::get_type("f_boolean_vector_required_elements_optional[0]")?;
    assert_eq!(v, FieldType::Option(Box::new(FieldType::Boolean)));
    let v = SampleStruct::get_type("f_struct_vector_required_elements_optional[0]")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::StructPath(
            "SampleSubstruct".to_string()
        )))
    );
    let v = SampleStruct::get_type("f_enum_vector_required_elements_optional[0]")?;
    assert_eq!(v, FieldType::Option(Box::new(FieldType::Unknown)));

    let v = SampleStruct::get_type("f_string_vector_optional_elements_optional[0]")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Option(Box::new(FieldType::String))))
    );
    let v = SampleStruct::get_type("f_integer_vector_optional_elements_optional[0]")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Option(Box::new(FieldType::Integer))))
    );
    let v = SampleStruct::get_type("f_float_vector_optional_elements_optional[0]")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Option(Box::new(FieldType::Float))))
    );
    let v = SampleStruct::get_type("f_boolean_vector_optional_elements_optional[0]")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Option(Box::new(FieldType::Boolean))))
    );
    let v = SampleStruct::get_type("f_struct_vector_optional_elements_optional[0]")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Option(Box::new(
            FieldType::StructPath("SampleSubstruct".to_string())
        ))))
    );
    let v = SampleStruct::get_type("f_enum_vector_optional_elements_optional[0]")?;
    assert_eq!(
        v,
        FieldType::Option(Box::new(FieldType::Option(Box::new(FieldType::Unknown))))
    );

    Ok(())
}

#[test]
fn test_nested_get_type() -> Result<(), Box<dyn std::error::Error>> {
    let v = SampleStruct::get_type("f_struct_scalar_required.subf_string")?;
    assert_eq!(v, FieldType::String);
    let v = SampleStruct::get_type("f_struct_scalar_optional.subf_string")?;
    assert_eq!(v, FieldType::String);
    let v = SampleStruct::get_type("f_struct_vector_required_elements_required[0].subf_string")?;
    assert_eq!(v, FieldType::String);
    let v = SampleStruct::get_type("f_struct_vector_optional_elements_required[0].subf_string")?;
    assert_eq!(v, FieldType::String);
    let v = SampleStruct::get_type("f_struct_vector_required_elements_optional[0].subf_string")?;
    assert_eq!(v, FieldType::String);
    let v = SampleStruct::get_type("f_struct_vector_optional_elements_optional[0].subf_string")?;
    assert_eq!(v, FieldType::String);

    Ok(())
}

#[test]
fn test_get_value_by_field_scalar() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct();
    let v = sample_struct.get_value("f_string_scalar_required")?;
    assert_eq!(v, "hello");
    let v = sample_struct.get_value("f_integer_scalar_required")?;
    assert_eq!(v, 123);
    let v = sample_struct.get_value("f_float_scalar_required")?;
    assert_eq!(v, 1.23);
    let v = sample_struct.get_value("f_boolean_scalar_required")?;
    assert_eq!(bool::from_value(v), true);
    let v = sample_struct.get_value("f_struct_scalar_required")?;
    assert_eq!(
        <&SampleSubstruct>::from_value(&v),
        &SampleSubstruct {
            subf_string: "sub1".to_string()
        }
    );
    let v = sample_struct.get_value("f_enum_scalar_required")?;
    assert_eq!(<&SampleEnum>::from_value(&v), &SampleEnum::A);

    Ok(())
}

#[test]
fn test_get_value_by_field_scalar_optional() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct();

    let v = sample_struct.get_value("f_string_scalar_optional")?;
    assert_eq!(<Option<&str>>::from_value(&v), Some("world"));
    let v = sample_struct.get_value("f_integer_scalar_optional")?;
    assert_eq!(<Option<i64>>::from_value(v), Some(456));
    let v = sample_struct.get_value("f_float_scalar_optional")?;
    assert_eq!(<Option<f64>>::from_value(v), Some(4.56));
    let v = sample_struct.get_value("f_boolean_scalar_optional")?;
    assert_eq!(<Option<bool>>::from_value(v), Some(false));
    let v = sample_struct.get_value("f_struct_scalar_optional")?;
    assert_eq!(
        <Option<&SampleSubstruct>>::from_value(&v),
        Some(&SampleSubstruct {
            subf_string: "sub2".to_string()
        })
    );
    let v = sample_struct.get_value("f_enum_scalar_optional")?;
    assert_eq!(<Option<&SampleEnum>>::from_value(&v), Some(&SampleEnum::B));

    Ok(())
}

#[test]
fn test_get_value_by_field_scalar_optional_none() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v = sample_struct.get_value("f_string_scalar_optional")?;
    assert_eq!(<Option<&str>>::from_value(&v), None);
    let v = sample_struct.get_value("f_integer_scalar_optional")?;
    assert_eq!(<Option<i64>>::from_value(v), None);
    let v = sample_struct.get_value("f_float_scalar_optional")?;
    assert_eq!(<Option<f64>>::from_value(v), None);
    let v = sample_struct.get_value("f_boolean_scalar_optional")?;
    assert_eq!(<Option<bool>>::from_value(v), None);
    let v = sample_struct.get_value("f_struct_scalar_optional")?;
    assert_eq!(<Option<&SampleSubstruct>>::from_value(&v), None);
    let v = sample_struct.get_value("f_enum_scalar_optional")?;
    assert_eq!(<Option<&SampleEnum>>::from_value(&v), None);

    Ok(())
}

#[test]
fn test_get_value_by_field_required_array_required_items() -> Result<(), Box<dyn std::error::Error>>
{
    let sample_struct = build_sample_struct();

    let v = sample_struct.get_value("f_string_vector_required_elements_required")?;
    assert_eq!(
        <&Vec<String>>::from_value(&v),
        &vec!["hello".to_string(), "world".to_string()]
    );
    let v = sample_struct.get_value("f_integer_vector_required_elements_required")?;
    assert_eq!(<&Vec<i64>>::from_value(&v), &vec![123, 456]);
    let v = sample_struct.get_value("f_float_vector_required_elements_required")?;
    assert_eq!(<&Vec<f64>>::from_value(&v), &vec![1.23, 4.56]);
    let v = sample_struct.get_value("f_boolean_vector_required_elements_required")?;
    assert_eq!(<&Vec<bool>>::from_value(&v), &vec![true, false]);
    let v = sample_struct.get_value("f_struct_vector_required_elements_required")?;
    assert_eq!(
        <&Vec<SampleSubstruct>>::from_value(&v),
        &vec![SampleSubstruct {
            subf_string: "sub3".to_string()
        }],
    );
    let v = sample_struct.get_value("f_enum_vector_required_elements_required")?;
    assert_eq!(
        <&Vec<SampleEnum>>::from_value(&v),
        &vec![SampleEnum::A, SampleEnum::B]
    );

    Ok(())
}

#[test]
fn test_get_value_by_field_optional_array_required_items() -> Result<(), Box<dyn std::error::Error>>
{
    let sample_struct = build_sample_struct();

    let v = sample_struct.get_value("f_string_vector_optional_elements_required")?;
    assert_eq!(
        <Option<&Vec<String>>>::from_value(&v),
        Some(&vec!["hello".to_string(), "world".to_string()])
    );
    let v = sample_struct.get_value("f_integer_vector_optional_elements_required")?;
    assert_eq!(<Option<&Vec<i64>>>::from_value(&v), Some(&vec![123, 456]));
    let v = sample_struct.get_value("f_float_vector_optional_elements_required")?;
    assert_eq!(<Option<&Vec<f64>>>::from_value(&v), Some(&vec![1.23, 4.56]));
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_required")?;
    assert_eq!(
        <Option<&Vec<bool>>>::from_value(&v),
        Some(&vec![true, false])
    );
    let v = sample_struct.get_value("f_struct_vector_optional_elements_required")?;
    assert_eq!(
        <Option<&Vec<SampleSubstruct>>>::from_value(&v),
        Some(&vec![SampleSubstruct {
            subf_string: "sub4".to_string(),
        }]),
    );
    let v = sample_struct.get_value("f_enum_vector_optional_elements_required")?;
    assert_eq!(
        <Option<&Vec<SampleEnum>>>::from_value(&v),
        Some(&vec![SampleEnum::A, SampleEnum::B])
    );

    Ok(())
}

#[test]
fn test_get_value_by_field_optional_array_is_none_required_items(
) -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v = sample_struct.get_value("f_string_vector_optional_elements_required")?;
    assert_eq!(<Option<&Vec<String>>>::from_value(&v), None);
    let v = sample_struct.get_value("f_integer_vector_optional_elements_required")?;
    assert_eq!(<Option<&Vec<i64>>>::from_value(&v), None);
    let v = sample_struct.get_value("f_float_vector_optional_elements_required")?;
    assert_eq!(<Option<&Vec<f64>>>::from_value(&v), None);
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_required")?;
    assert_eq!(<Option<&Vec<bool>>>::from_value(&v), None);
    let v = sample_struct.get_value("f_struct_vector_optional_elements_required")?;
    assert_eq!(<Option<&Vec<SampleSubstruct>>>::from_value(&v), None);
    let v = sample_struct.get_value("f_enum_vector_optional_elements_required")?;
    assert_eq!(<Option<&Vec<SampleEnum>>>::from_value(&v), None);

    Ok(())
}

#[test]
fn test_get_value_by_field_required_array_optional_items() -> Result<(), Box<dyn std::error::Error>>
{
    let sample_struct = build_sample_struct();

    let v = sample_struct.get_value("f_string_vector_required_elements_optional")?;
    assert_eq!(
        <&Vec<Option<String>>>::from_value(&v),
        &vec![Some("hello".to_string()), None]
    );
    let v = sample_struct.get_value("f_integer_vector_required_elements_optional")?;
    assert_eq!(<&Vec<Option<i64>>>::from_value(&v), &vec![Some(123), None]);
    let v = sample_struct.get_value("f_float_vector_required_elements_optional")?;
    assert_eq!(<&Vec<Option<f64>>>::from_value(&v), &vec![Some(1.23), None]);
    let v = sample_struct.get_value("f_boolean_vector_required_elements_optional")?;
    assert_eq!(
        <&Vec<Option<bool>>>::from_value(&v),
        &vec![Some(true), None]
    );
    let v = sample_struct.get_value("f_struct_vector_required_elements_optional")?;
    assert_eq!(
        <&Vec<Option<SampleSubstruct>>>::from_value(&v),
        &vec![
            Some(SampleSubstruct {
                subf_string: "sub5".to_string(),
            }),
            None
        ]
    );
    let v = sample_struct.get_value("f_enum_vector_required_elements_optional")?;
    assert_eq!(
        <&Vec<Option<SampleEnum>>>::from_value(&v),
        &vec![Some(SampleEnum::A), None]
    );

    Ok(())
}

#[test]
fn test_get_value_by_field_optional_array_optional_items() -> Result<(), Box<dyn std::error::Error>>
{
    let sample_struct = build_sample_struct();

    let v = sample_struct.get_value("f_string_vector_optional_elements_optional")?;
    assert_eq!(
        <Option<&Vec<Option<String>>>>::from_value(&v),
        Some(&vec![Some("hello".to_string()), None])
    );
    let v = sample_struct.get_value("f_integer_vector_optional_elements_optional")?;
    assert_eq!(
        <Option<&Vec<Option<i64>>>>::from_value(&v),
        Some(&vec![Some(123), None])
    );
    let v = sample_struct.get_value("f_float_vector_optional_elements_optional")?;
    assert_eq!(
        <Option<&Vec<Option<f64>>>>::from_value(&v),
        Some(&vec![Some(1.23), None])
    );
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_optional")?;
    assert_eq!(
        <Option<&Vec<Option<bool>>>>::from_value(&v),
        Some(&vec![Some(true), None])
    );
    let v = sample_struct.get_value("f_struct_vector_optional_elements_optional")?;
    assert_eq!(
        <Option<&Vec<Option<SampleSubstruct>>>>::from_value(&v),
        Some(&vec![
            Some(SampleSubstruct {
                subf_string: "sub6".to_string(),
            }),
            None
        ])
    );
    let v = sample_struct.get_value("f_enum_vector_optional_elements_optional")?;
    assert_eq!(
        <Option<&Vec<Option<SampleEnum>>>>::from_value(&v),
        Some(&vec![Some(SampleEnum::A), None])
    );

    Ok(())
}

#[test]
fn test_get_value_by_field_optional_array_is_none_optional_items(
) -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v = sample_struct.get_value("f_string_vector_optional_elements_optional")?;
    assert_eq!(<Option<&Vec<Option<String>>>>::from_value(&v), None);
    let v = sample_struct.get_value("f_integer_vector_optional_elements_optional")?;
    assert_eq!(<Option<&Vec<Option<i64>>>>::from_value(&v), None);
    let v = sample_struct.get_value("f_float_vector_optional_elements_optional")?;
    assert_eq!(<Option<&Vec<Option<f64>>>>::from_value(&v), None);
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_optional")?;
    assert_eq!(<Option<&Vec<Option<bool>>>>::from_value(&v), None);
    let v = sample_struct.get_value("f_struct_vector_optional_elements_optional")?;
    assert_eq!(
        <Option<&Vec<Option<SampleSubstruct>>>>::from_value(&v),
        None
    );
    let v = sample_struct.get_value("f_enum_vector_optional_elements_optional")?;
    assert_eq!(<Option<&Vec<Option<SampleEnum>>>>::from_value(&v), None);

    Ok(())
}

#[test]
fn test_get_value_by_index_required_array_required_items() -> Result<(), Box<dyn std::error::Error>>
{
    let sample_struct = build_sample_struct();

    let v = sample_struct.get_value("f_string_vector_required_elements_required[0]")?;
    assert_eq!(String::from_value(v), "hello");
    let v = sample_struct.get_value("f_integer_vector_required_elements_required[0]")?;
    assert_eq!(i64::from_value(v), 123);
    let v = sample_struct.get_value("f_float_vector_required_elements_required[0]")?;
    assert_eq!(f64::from_value(v), 1.23);
    let v = sample_struct.get_value("f_boolean_vector_required_elements_required[0]")?;
    assert_eq!(bool::from_value(v), true);
    let v = sample_struct.get_value("f_struct_vector_required_elements_required[0]")?;
    assert_eq!(
        <&SampleSubstruct>::from_value(&v),
        &SampleSubstruct {
            subf_string: "sub3".to_string()
        }
    );
    let v = sample_struct.get_value("f_enum_vector_required_elements_required[0]")?;
    assert_eq!(<&SampleEnum>::from_value(&v), &SampleEnum::A);

    Ok(())
}

#[test]
fn test_get_value_by_index_optional_array_required_items() -> Result<(), Box<dyn std::error::Error>>
{
    let sample_struct = build_sample_struct();

    let v = sample_struct.get_value("f_string_vector_optional_elements_required[0]")?;
    assert_eq!(String::from_value(v), "hello".to_string());
    let v = sample_struct.get_value("f_integer_vector_optional_elements_required[0]")?;
    assert_eq!(i64::from_value(v), 123);
    let v = sample_struct.get_value("f_float_vector_optional_elements_required[0]")?;
    assert_eq!(f64::from_value(v), 1.23);
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_required[0]")?;
    assert_eq!(bool::from_value(v), true);
    let v = sample_struct.get_value("f_struct_vector_optional_elements_required[0]")?;
    assert_eq!(
        <&SampleSubstruct>::from_value(&v),
        &SampleSubstruct {
            subf_string: "sub4".to_string()
        }
    );
    let v = sample_struct.get_value("f_enum_vector_optional_elements_required[0]")?;
    assert_eq!(<&SampleEnum>::from_value(&v), &SampleEnum::A);

    Ok(())
}

#[test]
fn test_get_value_by_index_optional_array_is_none_required_items(
) -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v = sample_struct.get_value("f_string_vector_optional_elements_required[0]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_integer_vector_optional_elements_required[0]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_float_vector_optional_elements_required[0]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_required[0]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_struct_vector_optional_elements_required[0]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_enum_vector_optional_elements_required[0]")?;
    assert_eq!(v.as_option(), None);

    Ok(())
}

#[test]
fn test_get_value_by_index_required_array_optional_items() -> Result<(), Box<dyn std::error::Error>>
{
    let sample_struct = build_sample_struct();

    let v = sample_struct.get_value("f_string_vector_required_elements_optional[0]")?;
    assert_eq!(String::from_value(v), "hello");
    let v = sample_struct.get_value("f_string_vector_required_elements_optional[1]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_integer_vector_required_elements_optional[0]")?;
    assert_eq!(i64::from_value(v), 123);
    let v = sample_struct.get_value("f_integer_vector_required_elements_optional[1]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_float_vector_required_elements_optional[0]")?;
    assert_eq!(f64::from_value(v), 1.23);
    let v = sample_struct.get_value("f_float_vector_required_elements_optional[1]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_boolean_vector_required_elements_optional[0]")?;
    assert_eq!(bool::from_value(v), true);
    let v = sample_struct.get_value("f_boolean_vector_required_elements_optional[1]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_struct_vector_required_elements_optional[0]")?;
    assert_eq!(
        <&SampleSubstruct>::from_value(&v),
        &SampleSubstruct {
            subf_string: "sub5".to_string()
        }
    );
    let v = sample_struct.get_value("f_struct_vector_required_elements_optional[1]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_enum_vector_required_elements_optional[0]")?;
    assert_eq!(
        <&SampleEnum>::from_value(&v.as_option().unwrap()),
        &SampleEnum::A
    );
    let v = sample_struct.get_value("f_enum_vector_required_elements_optional[1]")?;
    assert_eq!(v.as_option(), None);

    Ok(())
}

#[test]
fn test_get_value_by_index_optional_array_optional_items() -> Result<(), Box<dyn std::error::Error>>
{
    let sample_struct = build_sample_struct();

    let v = sample_struct.get_value("f_string_vector_optional_elements_optional[0]")?;
    assert_eq!(String::from_value(v), "hello");
    let v = sample_struct.get_value("f_string_vector_optional_elements_optional[1]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_integer_vector_optional_elements_optional[0]")?;
    assert_eq!(i64::from_value(v), 123);
    let v = sample_struct.get_value("f_integer_vector_optional_elements_optional[1]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_float_vector_optional_elements_optional[0]")?;
    assert_eq!(f64::from_value(v), 1.23);
    let v = sample_struct.get_value("f_float_vector_optional_elements_optional[1]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_optional[0]")?;
    assert_eq!(bool::from_value(v), true);
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_optional[1]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_struct_vector_optional_elements_optional[0]")?;
    assert_eq!(
        <&SampleSubstruct>::from_value(&v),
        &SampleSubstruct {
            subf_string: "sub6".to_string()
        }
    );
    let v = sample_struct.get_value("f_struct_vector_optional_elements_optional[1]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_enum_vector_optional_elements_optional[0]")?;
    assert_eq!(<&SampleEnum>::from_value(&v), &SampleEnum::A);
    let v = sample_struct.get_value("f_enum_vector_optional_elements_optional[1]")?;
    assert_eq!(v.as_option(), None);

    Ok(())
}

#[test]
fn test_get_value_by_index_optional_array_is_none_optional_items(
) -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v = sample_struct.get_value("f_string_vector_optional_elements_optional[0]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_integer_vector_optional_elements_optional[0]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_float_vector_optional_elements_optional[0]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_boolean_vector_optional_elements_optional[0]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_struct_vector_optional_elements_optional[0]")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_enum_vector_optional_elements_optional[0]")?;
    assert_eq!(v.as_option(), None);

    Ok(())
}

#[test]
fn test_nested_get_value() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct();

    let v = sample_struct.get_value("f_struct_scalar_required.subf_string")?;
    assert_eq!(String::from_value(v), "sub1");
    let v = sample_struct.get_value("f_struct_scalar_optional.subf_string")?;
    assert_eq!(String::from_value(v), "sub2");
    let v = sample_struct.get_value("f_struct_vector_required_elements_required[0].subf_string")?;
    assert_eq!(String::from_value(v), "sub3");
    let v = sample_struct.get_value("f_struct_vector_optional_elements_required[0].subf_string")?;
    assert_eq!(String::from_value(v), "sub4");
    let v = sample_struct.get_value("f_struct_vector_required_elements_optional[0].subf_string")?;
    assert_eq!(String::from_value(v), "sub5");
    let v = sample_struct.get_value("f_struct_vector_optional_elements_optional[0].subf_string")?;
    assert_eq!(String::from_value(v), "sub6");

    Ok(())
}

#[test]
fn test_nested_get_value_optional_array_is_none() -> Result<(), Box<dyn std::error::Error>> {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v = sample_struct.get_value("f_struct_scalar_optional.subf_string")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_struct_vector_optional_elements_required[0].subf_string")?;
    assert_eq!(v.as_option(), None);
    let v = sample_struct.get_value("f_struct_vector_optional_elements_optional[0].subf_string")?;
    assert_eq!(v.as_option(), None);

    Ok(())
}
