mod sample;
use polars_core::prelude::{DataType, Field};
use structpath_types::{data_type_wrapper, field_type, HasDataTypeWrapper, Path, StructPath};

pub fn fields_polars() -> Vec<Field> {
    Vec::from([
        // Required scalar fields
        field_type!("req_string", String),
        field_type!("req_i32", Int32),
        field_type!("req_i64", Int64),
        field_type!("req_f64", Float64),
        field_type!("req_bool", Boolean),
        field_type!("req_struct", Struct([("subf_string", String)])),
        field_type!("req_enum", Enum([("ITEM", 1)])),
        // Optional scalar fields
        field_type!("opt_string", String),
        field_type!("opt_i32", Int32),
        field_type!("opt_i64", Int64),
        field_type!("opt_f64", Float64),
        field_type!("opt_bool", Boolean),
        field_type!("opt_struct", Struct([("subf_string", String)])),
        field_type!("opt_enum", Enum([("ITEM", 1)])),
        // Required vector fields with required items
        field_type!("req_vec_req_item_string", List(String)),
        field_type!("req_vec_req_item_i32", List(Int32)),
        field_type!("req_vec_req_item_i64", List(Int64)),
        field_type!("req_vec_req_item_f64", List(Float64)),
        field_type!("req_vec_req_item_bool", List(Boolean)),
        field_type!(
            "req_vec_req_item_struct",
            List(Struct([("subf_string", String)]))
        ),
        field_type!("req_vec_req_item_enum", List(Enum([("ITEM", 1)]))),
        // Optional vector fields with required items
        field_type!("opt_vec_req_item_string", List(String)),
        field_type!("opt_vec_req_item_i32", List(Int32)),
        field_type!("opt_vec_req_item_i64", List(Int64)),
        field_type!("opt_vec_req_item_f64", List(Float64)),
        field_type!("opt_vec_req_item_bool", List(Boolean)),
        field_type!(
            "opt_vec_req_item_struct",
            List(Struct([("subf_string", String)]))
        ),
        field_type!("opt_vec_req_item_enum", List(Enum([("ITEM", 1)]))),
        // Required vector fields with optional items
        field_type!("req_vec_opt_item_string", List(String)),
        field_type!("req_vec_opt_item_i32", List(Int32)),
        field_type!("req_vec_opt_item_i64", List(Int64)),
        field_type!("req_vec_opt_item_f64", List(Float64)),
        field_type!("req_vec_opt_item_bool", List(Boolean)),
        field_type!(
            "req_vec_opt_item_struct",
            List(Struct([("subf_string", String)]))
        ),
        field_type!("req_vec_opt_item_enum", List(Enum([("ITEM", 1)]))),
        // Optional vector fields with optional items
        field_type!("opt_vec_opt_item_string", List(String)),
        field_type!("opt_vec_opt_item_i32", List(Int32)),
        field_type!("opt_vec_opt_item_i64", List(Int64)),
        field_type!("opt_vec_opt_item_f64", List(Float64)),
        field_type!("opt_vec_opt_item_bool", List(Boolean)),
        field_type!(
            "opt_vec_opt_item_struct",
            List(Struct([("subf_string", String)]))
        ),
        field_type!("opt_vec_opt_item_enum", List(Enum([("ITEM", 1)]))),
    ])
}

#[test]
fn test_fields() {
    let fields = sample::SampleStruct::fields();
    assert_eq!(*fields, fields_polars());
}

#[test]
fn test_data_type() {
    let data_type = sample::SampleStruct::data_type();
    assert_eq!(*data_type, DataType::Struct(fields_polars()));
}

#[test]
fn test_get_type_by_path() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::from_str("opt_vec_opt_item_struct[0].subf_string")?;
    let t = sample::SampleStruct::get_type_by_path(&path)?;
    assert_eq!(t, data_type_wrapper!(Option(Option(String))));
    Ok(())
}

#[test]
fn test_get_type() -> Result<(), Box<dyn std::error::Error>> {
    let t = sample::SampleStruct::get_type("opt_vec_opt_item_struct[0].subf_string")?;
    assert_eq!(t, data_type_wrapper!(Option(Option(String))));
    Ok(())
}
