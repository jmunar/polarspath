use structpath::StructPath;

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
        v5.unwrap().as_struct::<SampleSubstruct>().to_owned(),
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
        v5.unwrap()
            .unwrap()
            .as_struct::<SampleSubstruct>()
            .to_owned(),
        SampleSubstruct {
            subf_string: "sub2".to_string()
        }
    );
}

fn test_get_value_by_field_scalar_optional_none() {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v1 = sample_struct.get_value("f_string_scalar_optional");
    assert_eq!(v1.unwrap().as_option(), None);
    let v2 = sample_struct.get_value("f_integer_scalar_optional");
    assert_eq!(v2.unwrap().as_option(), None);
    let v3 = sample_struct.get_value("f_float_scalar_optional");
    assert_eq!(v3.unwrap().as_option(), None);
    let v4 = sample_struct.get_value("f_boolean_scalar_optional");
    assert_eq!(v4.unwrap().as_option(), None);
    let v5 = sample_struct.get_value("f_struct_scalar_optional");
    assert_eq!(v5.unwrap().as_option(), None);
}

fn test_get_value_by_field_required_array_required_items() {
    let sample_struct = build_sample_struct();

    let v1 = sample_struct.get_value("f_string_vector_required_elements_required");
    assert_eq!(
        v1.unwrap().as_array::<Vec<String>>().to_owned(),
        vec!["hello".to_string(), "world".to_string()]
    );
    let v2 = sample_struct.get_value("f_integer_vector_required_elements_required");
    assert_eq!(
        v2.unwrap().as_array::<Vec<i64>>().to_owned(),
        vec![123, 456]
    );
    let v3 = sample_struct.get_value("f_float_vector_required_elements_required");
    assert_eq!(
        v3.unwrap().as_array::<Vec<f64>>().to_owned(),
        vec![1.23, 4.56]
    );
    let v4 = sample_struct.get_value("f_boolean_vector_required_elements_required");
    assert_eq!(
        v4.unwrap().as_array::<Vec<bool>>().to_owned(),
        vec![true, false]
    );
    let v5 = sample_struct.get_value("f_struct_vector_required_elements_required");
    assert_eq!(
        v5.unwrap().as_array::<Vec<SampleSubstruct>>().to_owned(),
        vec![SampleSubstruct {
            subf_string: "sub3".to_string(),
        }],
    );
}

fn test_get_value_by_field_optional_array_required_items() {
    let sample_struct = build_sample_struct();

    let v1 = sample_struct.get_value("f_string_vector_optional_elements_required");
    assert_eq!(
        v1.unwrap().unwrap().as_array::<Vec<String>>().to_owned(),
        vec!["hello".to_string(), "world".to_string()]
    );
    let v2 = sample_struct.get_value("f_integer_vector_optional_elements_required");
    assert_eq!(
        v2.unwrap().unwrap().as_array::<Vec<i64>>().to_owned(),
        vec![123, 456]
    );
    let v3 = sample_struct.get_value("f_float_vector_optional_elements_required");
    assert_eq!(
        v3.unwrap().unwrap().as_array::<Vec<f64>>().to_owned(),
        vec![1.23, 4.56]
    );
    let v4 = sample_struct.get_value("f_boolean_vector_optional_elements_required");
    assert_eq!(
        v4.unwrap().unwrap().as_array::<Vec<bool>>().to_owned(),
        vec![true, false]
    );
    let v5 = sample_struct.get_value("f_struct_vector_optional_elements_required");
    assert_eq!(
        v5.unwrap()
            .unwrap()
            .as_array::<Vec<SampleSubstruct>>()
            .to_owned(),
        vec![SampleSubstruct {
            subf_string: "sub4".to_string(),
        }],
    );
}

fn test_get_value_by_field_optional_array_is_none_required_items() {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v1 = sample_struct.get_value("f_string_vector_optional_elements_required");
    assert_eq!(v1.unwrap().as_option(), None);
    let v2 = sample_struct.get_value("f_integer_vector_optional_elements_required");
    assert_eq!(v2.unwrap().as_option(), None);
    let v3 = sample_struct.get_value("f_float_vector_optional_elements_required");
    assert_eq!(v3.unwrap().as_option(), None);
    let v4 = sample_struct.get_value("f_boolean_vector_optional_elements_required");
    assert_eq!(v4.unwrap().as_option(), None);
    let v5 = sample_struct.get_value("f_struct_vector_optional_elements_required");
    assert_eq!(v5.unwrap().as_option(), None);
}

fn test_get_value_by_field_required_array_optional_items() {
    let sample_struct = build_sample_struct();

    let v1 = sample_struct.get_value("f_string_vector_required_elements_optional");
    assert_eq!(
        v1.unwrap().as_array::<Vec<Option<String>>>().to_owned(),
        vec![Some("hello".to_string()), None]
    );
    let v2 = sample_struct.get_value("f_integer_vector_required_elements_optional");
    assert_eq!(
        v2.unwrap().as_array::<Vec<Option<i64>>>().to_owned(),
        vec![Some(123), None]
    );
    let v3 = sample_struct.get_value("f_float_vector_required_elements_optional");
    assert_eq!(
        v3.unwrap().as_array::<Vec<Option<f64>>>().to_owned(),
        vec![Some(1.23), None]
    );
    let v4 = sample_struct.get_value("f_boolean_vector_required_elements_optional");
    assert_eq!(
        v4.unwrap().as_array::<Vec<Option<bool>>>().to_owned(),
        vec![Some(true), None]
    );
    let v5 = sample_struct.get_value("f_struct_vector_required_elements_optional");
    assert_eq!(
        v5.unwrap()
            .as_array::<Vec<Option<SampleSubstruct>>>()
            .to_owned(),
        vec![
            Some(SampleSubstruct {
                subf_string: "sub5".to_string(),
            }),
            None
        ]
    );
}

fn test_get_value_by_field_optional_array_optional_items() {
    let sample_struct = build_sample_struct();

    let v1 = sample_struct.get_value("f_string_vector_optional_elements_optional");
    assert_eq!(
        v1.unwrap()
            .unwrap()
            .as_array::<Vec<Option<String>>>()
            .to_owned(),
        vec![Some("hello".to_string()), None]
    );
    let v2 = sample_struct.get_value("f_integer_vector_optional_elements_optional");
    assert_eq!(
        v2.unwrap()
            .unwrap()
            .as_array::<Vec<Option<i64>>>()
            .to_owned(),
        vec![Some(123), None]
    );
    let v3 = sample_struct.get_value("f_float_vector_optional_elements_optional");
    assert_eq!(
        v3.unwrap()
            .unwrap()
            .as_array::<Vec<Option<f64>>>()
            .to_owned(),
        vec![Some(1.23), None]
    );
    let v4 = sample_struct.get_value("f_boolean_vector_optional_elements_optional");
    assert_eq!(
        v4.unwrap()
            .unwrap()
            .as_array::<Vec<Option<bool>>>()
            .to_owned(),
        vec![Some(true), None]
    );
    let v5 = sample_struct.get_value("f_struct_vector_optional_elements_optional");
    assert_eq!(
        v5.unwrap()
            .unwrap()
            .as_array::<Vec<Option<SampleSubstruct>>>()
            .to_owned(),
        vec![
            Some(SampleSubstruct {
                subf_string: "sub6".to_string(),
            }),
            None
        ]
    );
}

fn test_get_value_by_field_optional_array_is_none_optional_items() {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v1 = sample_struct.get_value("f_string_vector_optional_elements_optional");
    assert_eq!(v1.unwrap().as_option(), None);
    let v2 = sample_struct.get_value("f_integer_vector_optional_elements_optional");
    assert_eq!(v2.unwrap().as_option(), None);
    let v3 = sample_struct.get_value("f_float_vector_optional_elements_optional");
    assert_eq!(v3.unwrap().as_option(), None);
    let v4 = sample_struct.get_value("f_boolean_vector_optional_elements_optional");
    assert_eq!(v4.unwrap().as_option(), None);
    let v5 = sample_struct.get_value("f_struct_vector_optional_elements_optional");
    assert_eq!(v5.unwrap().as_option(), None);
}

fn test_get_value_by_index_required_array_required_items() {
    let sample_struct = build_sample_struct();

    let v1 = sample_struct.get_value("f_string_vector_required_elements_required[0]");
    assert_eq!(v1.unwrap().as_str(), "hello");
    let v2 = sample_struct.get_value("f_integer_vector_required_elements_required[0]");
    assert_eq!(v2.unwrap().as_i64(), 123);
    let v3 = sample_struct.get_value("f_float_vector_required_elements_required[0]");
    assert_eq!(v3.unwrap().as_f64(), 1.23);
    let v4 = sample_struct.get_value("f_boolean_vector_required_elements_required[0]");
    assert_eq!(v4.unwrap().as_bool(), true);
    let v5 = sample_struct.get_value("f_struct_vector_required_elements_required[0]");
    assert_eq!(
        v5.unwrap().as_struct::<SampleSubstruct>().to_owned(),
        SampleSubstruct {
            subf_string: "sub3".to_string()
        }
    );
}

fn test_get_value_by_index_optional_array_required_items() {
    let sample_struct = build_sample_struct();

    let v1 = sample_struct.get_value("f_string_vector_optional_elements_required[0]");
    assert_eq!(v1.unwrap().as_str(), "hello");
    let v2 = sample_struct.get_value("f_integer_vector_optional_elements_required[0]");
    assert_eq!(v2.unwrap().as_i64(), 123);
    let v3 = sample_struct.get_value("f_float_vector_optional_elements_required[0]");
    assert_eq!(v3.unwrap().as_f64(), 1.23);
    let v4 = sample_struct.get_value("f_boolean_vector_optional_elements_required[0]");
    assert_eq!(v4.unwrap().as_bool(), true);
    let v5 = sample_struct.get_value("f_struct_vector_optional_elements_required[0]");
    assert_eq!(
        v5.unwrap().as_struct::<SampleSubstruct>().to_owned(),
        SampleSubstruct {
            subf_string: "sub4".to_string()
        }
    );
}

fn test_get_value_by_index_optional_array_is_none_required_items() {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v1 = sample_struct.get_value("f_string_vector_optional_elements_required[0]");
    assert_eq!(v1.unwrap().as_option(), None);
    let v2 = sample_struct.get_value("f_integer_vector_optional_elements_required[0]");
    assert_eq!(v2.unwrap().as_option(), None);
    let v3 = sample_struct.get_value("f_float_vector_optional_elements_required[0]");
    assert_eq!(v3.unwrap().as_option(), None);
    let v4 = sample_struct.get_value("f_boolean_vector_optional_elements_required[0]");
    assert_eq!(v4.unwrap().as_option(), None);
    let v5 = sample_struct.get_value("f_struct_vector_optional_elements_required[0]");
    assert_eq!(v5.unwrap().as_option(), None);
}

fn test_get_value_by_index_required_array_optional_items() {
    let sample_struct = build_sample_struct();

    let v1 = sample_struct.get_value("f_string_vector_required_elements_optional[0]");
    assert_eq!(v1.unwrap().as_option().unwrap().as_str(), "hello");
    let v2 = sample_struct.get_value("f_string_vector_required_elements_optional[1]");
    assert_eq!(v2.unwrap().as_option(), None);
    let v3 = sample_struct.get_value("f_integer_vector_required_elements_optional[0]");
    assert_eq!(v3.unwrap().as_option().unwrap().as_i64(), 123);
    let v4 = sample_struct.get_value("f_integer_vector_required_elements_optional[1]");
    assert_eq!(v4.unwrap().as_option(), None);
    let v5 = sample_struct.get_value("f_float_vector_required_elements_optional[0]");
    assert_eq!(v5.unwrap().as_option().unwrap().as_f64(), 1.23);
    let v6 = sample_struct.get_value("f_float_vector_required_elements_optional[1]");
    assert_eq!(v6.unwrap().as_option(), None);
    let v7 = sample_struct.get_value("f_boolean_vector_required_elements_optional[0]");
    assert_eq!(v7.unwrap().as_option().unwrap().as_bool(), true);
    let v8 = sample_struct.get_value("f_boolean_vector_required_elements_optional[1]");
    assert_eq!(v8.unwrap().as_option(), None);
    let v9 = sample_struct.get_value("f_struct_vector_required_elements_optional[0]");
    assert_eq!(
        v9.unwrap()
            .as_option()
            .unwrap()
            .as_struct::<SampleSubstruct>()
            .to_owned(),
        SampleSubstruct {
            subf_string: "sub5".to_string()
        }
    );
    let v10 = sample_struct.get_value("f_struct_vector_required_elements_optional[1]");
    assert_eq!(v10.unwrap().as_option(), None);
}

fn test_get_value_by_index_optional_array_optional_items() {
    let sample_struct = build_sample_struct();

    let v1 = sample_struct.get_value("f_string_vector_optional_elements_optional[0]");
    assert_eq!(v1.unwrap().as_option().unwrap().as_str(), "hello");
    let v2 = sample_struct.get_value("f_string_vector_optional_elements_optional[1]");
    assert_eq!(v2.unwrap().as_option(), None);
    let v3 = sample_struct.get_value("f_integer_vector_optional_elements_optional[0]");
    assert_eq!(v3.unwrap().as_option().unwrap().as_i64(), 123);
    let v4 = sample_struct.get_value("f_integer_vector_optional_elements_optional[1]");
    assert_eq!(v4.unwrap().as_option(), None);
    let v5 = sample_struct.get_value("f_float_vector_optional_elements_optional[0]");
    assert_eq!(v5.unwrap().as_option().unwrap().as_f64(), 1.23);
    let v6 = sample_struct.get_value("f_float_vector_optional_elements_optional[1]");
    assert_eq!(v6.unwrap().as_option(), None);
    let v7 = sample_struct.get_value("f_boolean_vector_optional_elements_optional[0]");
    assert_eq!(v7.unwrap().as_option().unwrap().as_bool(), true);
    let v8 = sample_struct.get_value("f_boolean_vector_optional_elements_optional[1]");
    assert_eq!(v8.unwrap().as_option(), None);
    let v9 = sample_struct.get_value("f_struct_vector_optional_elements_optional[0]");
    assert_eq!(
        v9.unwrap()
            .as_option()
            .unwrap()
            .as_struct::<SampleSubstruct>()
            .to_owned(),
        SampleSubstruct {
            subf_string: "sub6".to_string()
        }
    );
    let v10 = sample_struct.get_value("f_struct_vector_optional_elements_optional[1]");
    assert_eq!(v10.unwrap().as_option(), None);
}

fn test_get_value_by_index_optional_array_is_none_optional_items() {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v1 = sample_struct.get_value("f_string_vector_optional_elements_optional[0]");
    assert_eq!(v1.unwrap().as_option(), None);
    let v2 = sample_struct.get_value("f_integer_vector_optional_elements_optional[0]");
    assert_eq!(v2.unwrap().as_option(), None);
    let v3 = sample_struct.get_value("f_float_vector_optional_elements_optional[0]");
    assert_eq!(v3.unwrap().as_option(), None);
    let v4 = sample_struct.get_value("f_boolean_vector_optional_elements_optional[0]");
    assert_eq!(v4.unwrap().as_option(), None);
    let v5 = sample_struct.get_value("f_struct_vector_optional_elements_optional[0]");
    assert_eq!(v5.unwrap().as_option(), None);
}

fn test_nested_get_value() {
    let sample_struct = build_sample_struct();

    let v1 = sample_struct.get_value("f_struct_scalar_required.subf_string");
    assert_eq!(v1.unwrap().as_str(), "sub1");
    let v2 = sample_struct.get_value("f_struct_scalar_optional.subf_string");
    assert_eq!(v2.unwrap().as_str(), "sub2");
    let v3 = sample_struct.get_value("f_struct_vector_required_elements_required[0].subf_string");
    assert_eq!(v3.unwrap().as_str(), "sub3");
    let v4 = sample_struct.get_value("f_struct_vector_optional_elements_required[0].subf_string");
    assert_eq!(v4.unwrap().as_str(), "sub4");
    let v5 = sample_struct.get_value("f_struct_vector_required_elements_optional[0].subf_string");
    assert_eq!(v5.unwrap().as_str(), "sub5");
    let v6 = sample_struct.get_value("f_struct_vector_optional_elements_optional[0].subf_string");
    assert_eq!(v6.unwrap().as_str(), "sub6");
}

fn test_nested_get_value_optional_array_is_none() {
    let sample_struct = build_sample_struct_with_null_optionals();

    let v1 = sample_struct.get_value("f_struct_vector_optional_elements_required[0].subf_string");
    assert_eq!(v1.unwrap().as_option(), None);
    let v2 = sample_struct.get_value("f_struct_vector_optional_elements_optional[0].subf_string");
    assert_eq!(v2.unwrap().as_option(), None);
}
