use structpath::{StructPathError, Path, PathComponent, StructPath, StructPathTrait, StructValue, Value};

#[derive(StructPath, Clone)]
pub enum SampleEnum {
    A,
    B,
    C,
}

#[derive(StructPath, Clone)]
pub struct SampleSubstruct {
    subf_string: String,
}

#[derive(StructPath, Clone)]
pub struct SampleStruct {
    f_string_scalar_required: String,
    f_integer_scalar_required: i64,
    f_float_scalar_required: f64,
    f_boolean_scalar_required: bool,
    #[type_hint = "enum"]
    f_enum_scalar_required: SampleEnum,
    #[type_hint = "struct"]
    f_struct_scalar_required: SampleSubstruct,

    f_string_scalar_optional: Option<String>,
    f_integer_scalar_optional: Option<i64>,
    f_float_scalar_optional: Option<f64>,
    f_boolean_scalar_optional: Option<bool>,
    #[type_hint = "enum"]
    f_enum_scalar_optional: Option<SampleEnum>,
    #[type_hint = "struct"]
    f_struct_scalar_optional: Option<SampleSubstruct>,

    f_string_vector_required_elements_required: Vec<String>,
    f_integer_vector_required_elements_required: Vec<i64>,
    f_float_vector_required_elements_required: Vec<f64>,
    f_boolean_vector_required_elements_required: Vec<bool>,
    #[type_hint = "enum"]
    f_enum_vector_required_elements_required: Vec<SampleEnum>,
    #[type_hint = "struct"]
    f_struct_vector_required_elements_required: Vec<SampleSubstruct>,

    f_string_vector_optional_elements_required: Option<Vec<String>>,
    f_integer_vector_optional_elements_required: Option<Vec<i64>>,
    f_float_vector_optional_elements_required: Option<Vec<f64>>,
    f_boolean_vector_optional_elements_required: Option<Vec<bool>>,
    #[type_hint = "enum"]
    f_enum_vector_optional_elements_required: Option<Vec<SampleEnum>>,
    #[type_hint = "struct"]
    f_struct_vector_optional_elements_required: Option<Vec<SampleSubstruct>>,

    f_string_vector_required_elements_optional: Vec<Option<String>>,
    f_integer_vector_required_elements_optional: Vec<Option<i64>>,
    f_float_vector_required_elements_optional: Vec<Option<f64>>,
    f_boolean_vector_required_elements_optional: Vec<Option<bool>>,
    #[type_hint = "enum"]
    f_enum_vector_required_elements_optional: Vec<Option<SampleEnum>>,
    #[type_hint = "struct"]
    f_struct_vector_required_elements_optional: Vec<Option<SampleSubstruct>>,

    f_string_vector_optional_elements_optional: Option<Vec<Option<String>>>,
    f_integer_vector_optional_elements_optional: Option<Vec<Option<i64>>>,
    f_float_vector_optional_elements_optional: Option<Vec<Option<f64>>>,
    f_boolean_vector_optional_elements_optional: Option<Vec<Option<bool>>>,
    #[type_hint = "enum"]
    f_enum_vector_optional_elements_optional: Option<Vec<Option<SampleEnum>>>,
    #[type_hint = "struct"]
    f_struct_vector_optional_elements_optional: Option<Vec<Option<SampleSubstruct>>>,
}

pub fn build_sample_struct() -> SampleStruct {
    SampleStruct {
        // Scalar fields
        f_string_scalar_required: "hello".to_string(),
        f_integer_scalar_required: 123,
        f_float_scalar_required: 1.23,
        f_boolean_scalar_required: true,
        f_enum_scalar_required: SampleEnum::A,
        f_struct_scalar_required: SampleSubstruct {
            subf_string: "sub1".to_string(),
        },

        // Scalar optional fields
        f_string_scalar_optional: Some("world".to_string()),
        f_integer_scalar_optional: Some(456),
        f_float_scalar_optional: Some(4.56),
        f_boolean_scalar_optional: Some(false),
        f_enum_scalar_optional: Some(SampleEnum::B),
        f_struct_scalar_optional: Some(SampleSubstruct {
            subf_string: "sub2".to_string(),
        }),

        // Array fields
        f_string_vector_required_elements_required: vec!["hello".to_string(), "world".to_string()],
        f_integer_vector_required_elements_required: vec![123, 456],
        f_float_vector_required_elements_required: vec![1.23, 4.56],
        f_boolean_vector_required_elements_required: vec![true, false],
        f_enum_vector_required_elements_required: vec![SampleEnum::A, SampleEnum::B],
        f_struct_vector_required_elements_required: vec![SampleSubstruct {
            subf_string: "sub3".to_string(),
        }],

        // Optional array with required items
        f_string_vector_optional_elements_required: Some(vec![
            "hello".to_string(),
            "world".to_string(),
        ]),
        f_integer_vector_optional_elements_required: Some(vec![123, 456]),
        f_float_vector_optional_elements_required: Some(vec![1.23, 4.56]),
        f_boolean_vector_optional_elements_required: Some(vec![true, false]),
        f_enum_vector_optional_elements_required: Some(vec![SampleEnum::A, SampleEnum::B]),
        f_struct_vector_optional_elements_required: Some(vec![SampleSubstruct {
            subf_string: "sub4".to_string(),
        }]),

        // Required array with optional items
        f_string_vector_required_elements_optional: vec![Some("hello".to_string()), None],
        f_integer_vector_required_elements_optional: vec![Some(123), None],
        f_float_vector_required_elements_optional: vec![Some(1.23), None],
        f_boolean_vector_required_elements_optional: vec![Some(true), None],
        f_enum_vector_required_elements_optional: vec![Some(SampleEnum::A), None],
        f_struct_vector_required_elements_optional: vec![
            Some(SampleSubstruct {
                subf_string: "sub5".to_string(),
            }),
            None,
        ],

        // Optional array with optional items
        f_string_vector_optional_elements_optional: Some(vec![Some("hello".to_string()), None]),
        f_integer_vector_optional_elements_optional: Some(vec![Some(123), None]),
        f_float_vector_optional_elements_optional: Some(vec![Some(1.23), None]),
        f_boolean_vector_optional_elements_optional: Some(vec![Some(true), None]),
        f_enum_vector_optional_elements_optional: Some(vec![Some(SampleEnum::A), None]),
        f_struct_vector_optional_elements_optional: Some(vec![
            Some(SampleSubstruct {
                subf_string: "sub6".to_string(),
            }),
            None,
        ]),
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
        f_enum_scalar_optional: None,
        f_struct_scalar_optional: None,

        f_string_vector_optional_elements_required: None,
        f_integer_vector_optional_elements_required: None,
        f_float_vector_optional_elements_required: None,
        f_boolean_vector_optional_elements_required: None,
        f_enum_vector_optional_elements_required: None,
        f_struct_vector_optional_elements_required: None,

        f_string_vector_optional_elements_optional: None,
        f_integer_vector_optional_elements_optional: None,
        f_float_vector_optional_elements_optional: None,
        f_boolean_vector_optional_elements_optional: None,
        f_enum_vector_optional_elements_optional: None,
        f_struct_vector_optional_elements_optional: None,

        ..sample_struct
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value_by_field_scalar() {
        let sample_struct = build_sample_struct();
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field("f_string_scalar_required".to_string())],
                })
                .unwrap(),
            Value::String("hello".to_string())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_integer_scalar_required".to_string()
                    )],
                })
                .unwrap(),
            Value::Integer(123)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field("f_float_scalar_required".to_string())],
                })
                .unwrap(),
            Value::Float(1.23)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_boolean_scalar_required".to_string()
                    )],
                })
                .unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field("f_enum_scalar_required".to_string())],
                })
                .unwrap(),
            Value::Enum(SampleEnum::A.clone_box())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field("f_struct_scalar_required".to_string())],
                })
                .unwrap(),
            Value::Struct(
                SampleSubstruct {
                    subf_string: "sub1".to_string()
                }
                .clone_box()
            )
        );
    }

    #[test]
    fn test_get_value_by_field_scalar_optional() {
        let sample_struct = build_sample_struct();

        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field("f_string_scalar_optional".to_string())],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::String("world".to_string())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_integer_scalar_optional".to_string()
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::Integer(456)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field("f_float_scalar_optional".to_string())],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::Float(4.56)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_boolean_scalar_optional".to_string()
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::Boolean(false)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field("f_enum_scalar_optional".to_string())],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::Enum(SampleEnum::B.clone_box())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field("f_struct_scalar_optional".to_string())],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::Struct(
                SampleSubstruct {
                    subf_string: "sub2".to_string()
                }
                .clone_box()
            )
        );
    }

    #[test]
    fn test_get_value_by_field_scalar_optional_none() {
        let sample_struct = build_sample_struct_with_null_optionals();

        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field("f_string_scalar_optional".to_string())],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_integer_scalar_optional".to_string()
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field("f_float_scalar_optional".to_string())],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_boolean_scalar_optional".to_string()
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field("f_enum_scalar_optional".to_string())],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field("f_struct_scalar_optional".to_string())],
                })
                .unwrap(),
            Value::Optional(None)
        );
    }

    #[test]
    fn test_get_value_by_field_required_array_required_items() {
        let sample_struct = build_sample_struct();

        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_string_vector_required_elements_required".to_string()
                    )],
                })
                .unwrap(),
            Value::StringArray(vec!["hello".to_string(), "world".to_string()])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_integer_vector_required_elements_required".to_string()
                    )],
                })
                .unwrap(),
            Value::IntegerArray(vec![123, 456])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_float_vector_required_elements_required".to_string()
                    )],
                })
                .unwrap(),
            Value::FloatArray(vec![1.23, 4.56])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_boolean_vector_required_elements_required".to_string()
                    )],
                })
                .unwrap(),
            Value::BooleanArray(vec![true, false])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_enum_vector_required_elements_required".to_string()
                    )],
                })
                .unwrap(),
            Value::EnumArray(vec![SampleEnum::A.clone_box(), SampleEnum::B.clone_box()])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_struct_vector_required_elements_required".to_string()
                    )],
                })
                .unwrap(),
            Value::StructArray(vec![SampleSubstruct {
                subf_string: "sub3".to_string()
            }
            .clone_box()]),
        );
    }

    #[test]
    fn test_get_value_by_field_optional_array_required_items() {
        let sample_struct = build_sample_struct();

        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_string_vector_optional_elements_required".to_string()
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::StringArray(vec!["hello".to_string(), "world".to_string()])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_integer_vector_optional_elements_required".to_string()
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::IntegerArray(vec![123, 456])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_float_vector_optional_elements_required".to_string()
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::FloatArray(vec![1.23, 4.56])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_boolean_vector_optional_elements_required".to_string()
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::BooleanArray(vec![true, false])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_enum_vector_optional_elements_required".to_string()
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::EnumArray(vec![SampleEnum::A.clone_box(), SampleEnum::B.clone_box()])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_struct_vector_optional_elements_required".to_string()
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::StructArray(vec![SampleSubstruct {
                subf_string: "sub4".to_string()
            }
            .clone_box()])
        );
    }

    #[test]
    fn test_get_value_by_field_optional_array_is_none_required_items() {
        let sample_struct = build_sample_struct_with_null_optionals();

        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_string_vector_optional_elements_required".to_string()
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_integer_vector_optional_elements_required".to_string()
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_float_vector_optional_elements_required".to_string()
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_boolean_vector_optional_elements_required".to_string()
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_enum_vector_optional_elements_required".to_string()
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_struct_vector_optional_elements_required".to_string()
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
    }

    #[test]
    fn test_get_value_by_field_required_array_optional_items() {
        let sample_struct = build_sample_struct();

        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_string_vector_required_elements_optional".to_string()
                    )],
                })
                .unwrap(),
            Value::StringOptArray(vec![Some("hello".to_string()), None])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_integer_vector_required_elements_optional".to_string()
                    )],
                })
                .unwrap(),
            Value::IntegerOptArray(vec![Some(123), None])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_float_vector_required_elements_optional".to_string()
                    )],
                })
                .unwrap(),
            Value::FloatOptArray(vec![Some(1.23), None])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_boolean_vector_required_elements_optional".to_string()
                    )],
                })
                .unwrap(),
            Value::BooleanOptArray(vec![Some(true), None])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_enum_vector_required_elements_optional".to_string()
                    )],
                })
                .unwrap(),
            Value::EnumOptArray(vec![Some(SampleEnum::A.clone_box()), None])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_struct_vector_required_elements_optional".to_string()
                    )],
                })
                .unwrap(),
            Value::StructOptArray(vec![
                Some(
                    SampleSubstruct {
                        subf_string: "sub5".to_string()
                    }
                    .clone_box()
                ),
                None
            ])
        );
    }

    #[test]
    fn test_get_value_by_field_optional_array_optional_items() {
        let sample_struct = build_sample_struct();

        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_string_vector_optional_elements_optional".to_string()
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::StringOptArray(vec![Some("hello".to_string()), None])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_integer_vector_optional_elements_optional".to_string()
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::IntegerOptArray(vec![Some(123), None])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_float_vector_optional_elements_optional".to_string()
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::FloatOptArray(vec![Some(1.23), None])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_boolean_vector_optional_elements_optional".to_string()
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::BooleanOptArray(vec![Some(true), None])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_enum_vector_optional_elements_optional".to_string()
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::EnumOptArray(vec![Some(SampleEnum::A.clone_box()), None])
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_struct_vector_optional_elements_optional".to_string()
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::StructOptArray(vec![
                Some(
                    SampleSubstruct {
                        subf_string: "sub6".to_string()
                    }
                    .clone_box()
                ),
                None
            ])
        );
    }

    #[test]
    fn test_get_value_by_field_optional_array_is_none_optional_items() {
        let sample_struct = build_sample_struct_with_null_optionals();

        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_string_vector_optional_elements_optional".to_string()
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_integer_vector_optional_elements_optional".to_string()
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_float_vector_optional_elements_optional".to_string()
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_boolean_vector_optional_elements_optional".to_string()
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_enum_vector_optional_elements_optional".to_string()
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::Field(
                        "f_struct_vector_optional_elements_optional".to_string()
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
    }

    #[test]
    fn test_get_value_by_index_required_array_required_items() {
        let sample_struct = build_sample_struct();

        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_string_vector_required_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::String("hello".to_string())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_integer_vector_required_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Integer(123)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_float_vector_required_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Float(1.23)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_boolean_vector_required_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_enum_vector_required_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Enum(SampleEnum::A.clone_box())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_struct_vector_required_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Struct(
                SampleSubstruct {
                    subf_string: "sub3".to_string()
                }
                .clone_box()
            )
        );
    }

    #[test]
    fn test_get_value_by_index_optional_array_required_items() {
        let sample_struct = build_sample_struct();

        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_string_vector_optional_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::String("hello".to_string())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_integer_vector_optional_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Integer(123)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_float_vector_optional_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Float(1.23)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_boolean_vector_optional_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_enum_vector_optional_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Enum(SampleEnum::A.clone_box())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_struct_vector_optional_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Struct(
                SampleSubstruct {
                    subf_string: "sub4".to_string()
                }
                .clone_box()
            )
        );
    }

    #[test]
    fn test_get_value_by_index_optional_array_is_none_required_items() {
        let sample_struct = build_sample_struct_with_null_optionals();

        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_string_vector_optional_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_integer_vector_optional_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_float_vector_optional_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_boolean_vector_optional_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_enum_vector_optional_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_struct_vector_optional_elements_required".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
    }

    #[test]
    fn test_get_value_by_index_required_array_optional_items() {
        let sample_struct = build_sample_struct();

        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_string_vector_required_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::String("hello".to_string())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_string_vector_required_elements_optional".to_string(),
                        1
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_integer_vector_required_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::Integer(123)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_integer_vector_required_elements_optional".to_string(),
                        1
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_float_vector_required_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::Float(1.23)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_float_vector_required_elements_optional".to_string(),
                        1
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_boolean_vector_required_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_enum_vector_required_elements_optional".to_string(),
                        1
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_enum_vector_required_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::Enum(SampleEnum::A.clone_box())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_struct_vector_required_elements_optional".to_string(),
                        1
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_struct_vector_required_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::Struct(
                SampleSubstruct {
                    subf_string: "sub5".to_string()
                }
                .clone_box()
            )
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_struct_vector_required_elements_optional".to_string(),
                        1
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
    }

    #[test]
    fn test_get_value_by_index_optional_array_optional_items() {
        let sample_struct = build_sample_struct();

        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_string_vector_optional_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::String("hello".to_string())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_string_vector_optional_elements_optional".to_string(),
                        1
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_integer_vector_optional_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::Integer(123)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_integer_vector_optional_elements_optional".to_string(),
                        1
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_float_vector_optional_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::Float(1.23)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_float_vector_optional_elements_optional".to_string(),
                        1
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_boolean_vector_optional_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::Boolean(true)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_boolean_vector_optional_elements_optional".to_string(),
                        1
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_enum_vector_optional_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::Enum(SampleEnum::A.clone_box())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_enum_vector_optional_elements_optional".to_string(),
                        1
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_struct_vector_optional_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap()
                .into_inner()
                .unwrap(),
            Value::Struct(
                SampleSubstruct {
                    subf_string: "sub6".to_string()
                }
                .clone_box()
            )
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_struct_vector_optional_elements_optional".to_string(),
                        1
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
    }

    #[test]
    fn test_get_value_by_index_optional_array_is_none_optional_items() {
        let sample_struct = build_sample_struct_with_null_optionals();

        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_string_vector_optional_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_integer_vector_optional_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_float_vector_optional_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_boolean_vector_optional_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_enum_vector_optional_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![PathComponent::ArrayIndex(
                        "f_struct_vector_optional_elements_optional".to_string(),
                        0
                    )],
                })
                .unwrap(),
            Value::Optional(None)
        );
    }

    #[test]
    fn test_nested_get_value() {
        let sample_struct = build_sample_struct();
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![
                        PathComponent::Field("f_struct_scalar_required".to_string()),
                        PathComponent::Field("subf_string".to_string()),
                    ],
                })
                .unwrap(),
            Value::String("sub1".to_string())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![
                        PathComponent::Field("f_struct_scalar_optional".to_string()),
                        PathComponent::Field("subf_string".to_string()),
                    ],
                })
                .unwrap(),
            Value::String("sub2".to_string())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![
                        PathComponent::ArrayIndex(
                            "f_struct_vector_required_elements_required".to_string(),
                            0
                        ),
                        PathComponent::Field("subf_string".to_string()),
                    ],
                })
                .unwrap(),
            Value::String("sub3".to_string())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![
                        PathComponent::ArrayIndex(
                            "f_struct_vector_optional_elements_required".to_string(),
                            0
                        ),
                        PathComponent::Field("subf_string".to_string()),
                    ],
                })
                .unwrap(),
            Value::String("sub4".to_string())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![
                        PathComponent::ArrayIndex(
                            "f_struct_vector_required_elements_optional".to_string(),
                            0
                        ),
                        PathComponent::Field("subf_string".to_string()),
                    ],
                })
                .unwrap(),
            Value::String("sub5".to_string())
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![
                        PathComponent::ArrayIndex(
                            "f_struct_vector_optional_elements_optional".to_string(),
                            0
                        ),
                        PathComponent::Field("subf_string".to_string()),
                    ],
                })
                .unwrap(),
            Value::String("sub6".to_string())
        );
    }

    #[test]
    fn test_nested_get_value_optional_array_is_none() {
        let sample_struct = build_sample_struct_with_null_optionals();
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![
                        PathComponent::ArrayIndex(
                            "f_struct_vector_optional_elements_required".to_string(),
                            0
                        ),
                        PathComponent::Field("subf_string".to_string()),
                    ],
                })
                .unwrap(),
            Value::Optional(None)
        );
        assert_eq!(
            sample_struct
                .get_value_by_path(&Path {
                    components: vec![
                        PathComponent::ArrayIndex(
                            "f_struct_vector_optional_elements_optional".to_string(),
                            0
                        ),
                        PathComponent::Field("subf_string".to_string()),
                    ],
                })
                .unwrap(),
            Value::Optional(None)
        );
    }
}
