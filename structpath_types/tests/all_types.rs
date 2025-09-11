use indexmap::IndexMap;
use polars_core::prelude::{DataType, Field};
use structpath_types::{data_type_opt, field_type, field_type_opt, DataTypeOpt};

fn subfields_opt() -> IndexMap<String, DataTypeOpt> {
    IndexMap::from([("subf_string".into(), DataTypeOpt::String)])
}

fn sample_data_type_opt_struct() -> DataTypeOpt {
    DataTypeOpt::Struct(IndexMap::from([
        // Required scalar fields
        field_type_opt!("req_string", String),
        field_type_opt!("req_i64", Int64),
        field_type_opt!("req_f64", Float64),
        field_type_opt!("req_bool", Boolean),
        field_type_opt!("req_struct", Struct(subfields_opt())),
        field_type_opt!("req_object", Object("object")),
        // Optional scalar fields
        field_type_opt!("opt_string", Option, String),
        field_type_opt!("opt_i64", Option, Int64),
        field_type_opt!("opt_f64", Option, Float64),
        field_type_opt!("opt_bool", Option, Boolean),
        field_type_opt!("opt_struct", Option, Struct(subfields_opt())),
        field_type_opt!("opt_object", Option, Object("object")),
        // Required vector fields with required items
        field_type_opt!("req_vec_req_item_string", List, String),
        field_type_opt!("req_vec_req_item_i64", List, Int64),
        field_type_opt!("req_vec_req_item_f64", List, Float64),
        field_type_opt!("req_vec_req_item_bool", List, Boolean),
        field_type_opt!("req_vec_req_item_struct", List, Struct(subfields_opt())),
        field_type_opt!("req_vec_req_item_object", List, Object("object")),
        // Optional vector fields with required items
        field_type_opt!("opt_vec_req_item_string", Option, List, String),
        field_type_opt!("opt_vec_req_item_i64", Option, List, Int64),
        field_type_opt!("opt_vec_req_item_f64", Option, List, Float64),
        field_type_opt!("opt_vec_req_item_bool", Option, List, Boolean),
        field_type_opt!(
            "opt_vec_req_item_struct",
            Option,
            List,
            Struct(subfields_opt())
        ),
        field_type_opt!("opt_vec_req_item_object", Option, List, Object("object")),
        // Optional vector fields with required items
        field_type_opt!("opt_vec_req_item_string", Option, List, String),
        field_type_opt!("opt_vec_req_item_i64", Option, List, Int64),
        field_type_opt!("opt_vec_req_item_f64", Option, List, Float64),
        field_type_opt!("opt_vec_req_item_bool", Option, List, Boolean),
        field_type_opt!(
            "opt_vec_req_item_struct",
            Option,
            List,
            Struct(subfields_opt())
        ),
        field_type_opt!("opt_vec_req_item_object", Option, List, Object("object")),
        // Required vector fields with optional items
        field_type_opt!("req_vec_opt_item_string", List, Option, String),
        field_type_opt!("req_vec_opt_item_i64", List, Option, Int64),
        field_type_opt!("req_vec_opt_item_f64", List, Option, Float64),
        field_type_opt!("req_vec_opt_item_bool", List, Option, Boolean),
        field_type_opt!(
            "req_vec_opt_item_struct",
            List,
            Option,
            Struct(subfields_opt())
        ),
        field_type_opt!("req_vec_opt_item_object", List, Option, Object("object")),
        // Optional vector fields with optional items
        field_type_opt!("opt_vec_opt_item_string", Option, List, Option, String),
        field_type_opt!("opt_vec_opt_item_i64", Option, List, Option, Int64),
        field_type_opt!("opt_vec_opt_item_f64", Option, List, Option, Float64),
        field_type_opt!("opt_vec_opt_item_bool", Option, List, Option, Boolean),
        field_type_opt!(
            "opt_vec_opt_item_struct",
            Option,
            List,
            Option,
            Struct(subfields_opt())
        ),
        field_type_opt!(
            "opt_vec_opt_item_object",
            Option,
            List,
            Option,
            Object("object")
        ),
    ]))
}

fn subfields() -> Vec<Field> {
    Vec::from([Field::new("subf_string".into(), DataType::String)])
}

fn sample_data_type_struct() -> DataType {
    DataType::Struct(Vec::from([
        // Required scalar fields
        field_type!("req_string", String),
        field_type!("req_i64", Int64),
        field_type!("req_f64", Float64),
        field_type!("req_bool", Boolean),
        field_type!("req_struct", Struct(subfields())),
        field_type!("req_object", Object("object")),
        // Optional scalar fields
        field_type!("opt_string", String),
        field_type!("opt_i64", Int64),
        field_type!("opt_f64", Float64),
        field_type!("opt_bool", Boolean),
        field_type!("opt_struct", Struct(subfields())),
        field_type!("opt_object", Object("object")),
        // Required vector fields with required items
        field_type!("req_vec_req_item_string", List, String),
        field_type!("req_vec_req_item_i64", List, Int64),
        field_type!("req_vec_req_item_f64", List, Float64),
        field_type!("req_vec_req_item_bool", List, Boolean),
        field_type!("req_vec_req_item_struct", List, Struct(subfields())),
        field_type!("req_vec_req_item_object", List, Object("object")),
        // Optional vector fields with required items
        field_type!("opt_vec_req_item_string", List, String),
        field_type!("opt_vec_req_item_i64", List, Int64),
        field_type!("opt_vec_req_item_f64", List, Float64),
        field_type!("opt_vec_req_item_bool", List, Boolean),
        field_type!("opt_vec_req_item_struct", List, Struct(subfields())),
        field_type!("opt_vec_req_item_object", List, Object("object")),
        // Required vector fields with optional items
        field_type!("req_vec_opt_item_string", List, String),
        field_type!("req_vec_opt_item_i64", List, Int64),
        field_type!("req_vec_opt_item_f64", List, Float64),
        field_type!("req_vec_opt_item_bool", List, Boolean),
        field_type!("req_vec_opt_item_struct", List, Struct(subfields())),
        field_type!("req_vec_opt_item_object", List, Object("object")),
        // Optional vector fields with optional items
        field_type!("opt_vec_opt_item_string", List, String),
        field_type!("opt_vec_opt_item_i64", List, Int64),
        field_type!("opt_vec_opt_item_f64", List, Float64),
        field_type!("opt_vec_opt_item_bool", List, Boolean),
        field_type!("opt_vec_opt_item_struct", List, Struct(subfields())),
        field_type!("opt_vec_opt_item_object", List, Object("object")),
    ]))
}

#[test]
fn test_to_data_type() -> Result<(), Box<dyn std::error::Error>> {
    let data_type_opt = sample_data_type_opt_struct();
    let data_type = data_type_opt.to_data_type();
    assert_eq!(data_type, sample_data_type_struct());
    Ok(())
}

#[test]
fn test_get_type_req_fields() -> Result<(), Box<dyn std::error::Error>> {
    let data_type_opt = sample_data_type_opt_struct();

    let t = data_type_opt.get_type("req_string")?;
    assert_eq!(t, data_type_opt!(String));

    let t = data_type_opt.get_type("req_i64")?;
    assert_eq!(t, data_type_opt!(Int64));

    let t = data_type_opt.get_type("req_f64")?;
    assert_eq!(t, data_type_opt!(Float64));

    let t = data_type_opt.get_type("req_bool")?;
    assert_eq!(t, data_type_opt!(Boolean));

    let t = data_type_opt.get_type("req_struct")?;
    assert_eq!(t, data_type_opt!(Struct(subfields_opt())));

    let t = data_type_opt.get_type("req_object")?;
    assert_eq!(t, data_type_opt!(Object("object")));

    // Nested
    let t = data_type_opt.get_type("req_struct.subf_string")?;
    assert_eq!(t, data_type_opt!(String));

    Ok(())
}

#[test]
fn test_get_type_opt_fields() -> Result<(), Box<dyn std::error::Error>> {
    let data_type_opt = sample_data_type_opt_struct();

    let t = data_type_opt.get_type("opt_string")?;
    assert_eq!(t, data_type_opt!(Option, String));

    let t = data_type_opt.get_type("opt_i64")?;
    assert_eq!(t, data_type_opt!(Option, Int64));

    let t = data_type_opt.get_type("opt_f64")?;
    assert_eq!(t, data_type_opt!(Option, Float64));

    let t = data_type_opt.get_type("opt_bool")?;
    assert_eq!(t, data_type_opt!(Option, Boolean));

    let t = data_type_opt.get_type("opt_struct")?;
    assert_eq!(t, data_type_opt!(Option, Struct(subfields_opt())));

    let t = data_type_opt.get_type("opt_object")?;
    assert_eq!(t, data_type_opt!(Option, Object("object")));

    // Nested
    let t = data_type_opt.get_type("opt_struct.subf_string")?;
    assert_eq!(t, data_type_opt!(Option, String));

    Ok(())
}

#[test]
fn test_get_type_req_vec_req_item_fields() -> Result<(), Box<dyn std::error::Error>> {
    let data_type_opt = sample_data_type_opt_struct();

    let t = data_type_opt.get_type("req_vec_req_item_string")?;
    assert_eq!(t, data_type_opt!(List, String));

    let t = data_type_opt.get_type("req_vec_req_item_i64")?;
    assert_eq!(t, data_type_opt!(List, Int64));

    let t = data_type_opt.get_type("req_vec_req_item_f64")?;
    assert_eq!(t, data_type_opt!(List, Float64));

    let t = data_type_opt.get_type("req_vec_req_item_bool")?;
    assert_eq!(t, data_type_opt!(List, Boolean));

    let t = data_type_opt.get_type("req_vec_req_item_struct")?;
    assert_eq!(t, data_type_opt!(List, Struct(subfields_opt())));

    let t = data_type_opt.get_type("req_vec_req_item_object")?;
    assert_eq!(t, data_type_opt!(List, Object("object")));

    // Nested
    let t = data_type_opt.get_type("req_vec_req_item_struct[0].subf_string")?;
    assert_eq!(t, data_type_opt!(String));

    Ok(())
}

#[test]
fn test_get_type_opt_vec_req_item_fields() -> Result<(), Box<dyn std::error::Error>> {
    let data_type_opt = sample_data_type_opt_struct();

    let t = data_type_opt.get_type("opt_vec_req_item_string")?;
    assert_eq!(t, data_type_opt!(Option, List, String));

    let t = data_type_opt.get_type("opt_vec_req_item_i64")?;
    assert_eq!(t, data_type_opt!(Option, List, Int64));

    let t = data_type_opt.get_type("opt_vec_req_item_f64")?;
    assert_eq!(t, data_type_opt!(Option, List, Float64));

    let t = data_type_opt.get_type("opt_vec_req_item_bool")?;
    assert_eq!(t, data_type_opt!(Option, List, Boolean));

    let t = data_type_opt.get_type("opt_vec_req_item_struct")?;
    assert_eq!(t, data_type_opt!(Option, List, Struct(subfields_opt())));

    let t = data_type_opt.get_type("opt_vec_req_item_object")?;
    assert_eq!(t, data_type_opt!(Option, List, Object("object")));

    // Nested
    let t = data_type_opt.get_type("opt_vec_req_item_struct[0].subf_string")?;
    assert_eq!(t, data_type_opt!(Option, String));

    Ok(())
}

#[test]
fn test_get_type_req_vec_opt_item_fields() -> Result<(), Box<dyn std::error::Error>> {
    let data_type_opt = sample_data_type_opt_struct();

    let t = data_type_opt.get_type("req_vec_opt_item_string")?;
    assert_eq!(t, data_type_opt!(List, Option, String));

    let t = data_type_opt.get_type("req_vec_opt_item_i64")?;
    assert_eq!(t, data_type_opt!(List, Option, Int64));

    let t = data_type_opt.get_type("req_vec_opt_item_f64")?;
    assert_eq!(t, data_type_opt!(List, Option, Float64));

    let t = data_type_opt.get_type("req_vec_opt_item_bool")?;
    assert_eq!(t, data_type_opt!(List, Option, Boolean));

    let t = data_type_opt.get_type("req_vec_opt_item_struct")?;
    assert_eq!(t, data_type_opt!(List, Option, Struct(subfields_opt())));

    let t = data_type_opt.get_type("req_vec_opt_item_object")?;
    assert_eq!(t, data_type_opt!(List, Option, Object("object")));

    // Nested
    let t = data_type_opt.get_type("req_vec_opt_item_struct[0].subf_string")?;
    assert_eq!(t, data_type_opt!(Option, String));

    Ok(())
}

#[test]
fn test_get_type_opt_vec_opt_item_fields() -> Result<(), Box<dyn std::error::Error>> {
    let data_type_opt = sample_data_type_opt_struct();

    let t = data_type_opt.get_type("opt_vec_opt_item_string")?;
    assert_eq!(t, data_type_opt!(Option, List, Option, String));

    let t = data_type_opt.get_type("opt_vec_opt_item_i64")?;
    assert_eq!(t, data_type_opt!(Option, List, Option, Int64));

    let t = data_type_opt.get_type("opt_vec_opt_item_f64")?;
    assert_eq!(t, data_type_opt!(Option, List, Option, Float64));

    let t = data_type_opt.get_type("opt_vec_opt_item_bool")?;
    assert_eq!(t, data_type_opt!(Option, List, Option, Boolean));

    let t = data_type_opt.get_type("opt_vec_opt_item_struct")?;
    assert_eq!(
        t,
        data_type_opt!(Option, List, Option, Struct(subfields_opt()))
    );

    let t = data_type_opt.get_type("opt_vec_opt_item_object")?;
    assert_eq!(t, data_type_opt!(Option, List, Option, Object("object")));

    // Nested
    let t = data_type_opt.get_type("opt_vec_opt_item_struct[0].subf_string")?;
    assert_eq!(t, data_type_opt!(Option, Option, String));

    Ok(())
}
