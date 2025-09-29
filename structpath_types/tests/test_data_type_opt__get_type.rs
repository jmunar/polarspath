use structpath_types::{data_type_opt, DataTypeOpt};

mod sample;
use sample::{fields_opt, subfields_opt};

#[test]
fn test_get_type_req_fields() -> Result<(), Box<dyn std::error::Error>> {
    let data_type_opt = DataTypeOpt::Struct(fields_opt());

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

    let t = data_type_opt.get_type("req_enum")?;
    assert_eq!(t, data_type_opt!(Enum([("ITEM", 1)])));

    // Nested
    let t = data_type_opt.get_type("req_struct.subf_string")?;
    assert_eq!(t, data_type_opt!(String));

    Ok(())
}

#[test]
fn test_get_type_opt_fields() -> Result<(), Box<dyn std::error::Error>> {
    let data_type_opt = DataTypeOpt::Struct(fields_opt());

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

    let t = data_type_opt.get_type("opt_enum")?;
    assert_eq!(t, data_type_opt!(Option, Enum([("ITEM", 1)])));

    // Nested
    let t = data_type_opt.get_type("opt_struct.subf_string")?;
    assert_eq!(t, data_type_opt!(Option, String));

    Ok(())
}

#[test]
fn test_get_type_req_vec_req_item_fields() -> Result<(), Box<dyn std::error::Error>> {
    let data_type_opt = DataTypeOpt::Struct(fields_opt());

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

    let t = data_type_opt.get_type("req_vec_req_item_enum")?;
    assert_eq!(t, data_type_opt!(List, Enum([("ITEM", 1)])));

    // Nested
    let t = data_type_opt.get_type("req_vec_req_item_struct[0].subf_string")?;
    assert_eq!(t, data_type_opt!(String));

    Ok(())
}

#[test]
fn test_get_type_opt_vec_req_item_fields() -> Result<(), Box<dyn std::error::Error>> {
    let data_type_opt = DataTypeOpt::Struct(fields_opt());

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

    let t = data_type_opt.get_type("opt_vec_req_item_enum")?;
    assert_eq!(t, data_type_opt!(Option, List, Enum([("ITEM", 1)])));

    // Nested
    let t = data_type_opt.get_type("opt_vec_req_item_struct[0].subf_string")?;
    assert_eq!(t, data_type_opt!(Option, String));

    Ok(())
}

#[test]
fn test_get_type_req_vec_opt_item_fields() -> Result<(), Box<dyn std::error::Error>> {
    let data_type_opt = DataTypeOpt::Struct(fields_opt());

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

    let t = data_type_opt.get_type("req_vec_opt_item_enum")?;
    assert_eq!(t, data_type_opt!(List, Option, Enum([("ITEM", 1)])));

    // Nested
    let t = data_type_opt.get_type("req_vec_opt_item_struct[0].subf_string")?;
    assert_eq!(t, data_type_opt!(Option, String));

    Ok(())
}

#[test]
fn test_get_type_opt_vec_opt_item_fields() -> Result<(), Box<dyn std::error::Error>> {
    let data_type_opt = DataTypeOpt::Struct(fields_opt());

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

    let t = data_type_opt.get_type("opt_vec_opt_item_enum")?;
    assert_eq!(t, data_type_opt!(Option, List, Option, Enum([("ITEM", 1)])));

    // Nested
    let t = data_type_opt.get_type("opt_vec_opt_item_struct[0].subf_string")?;
    assert_eq!(t, data_type_opt!(Option, Option, String));

    Ok(())
}
