use structpath_types::{data_type_wrapper, HasDataTypeWrapper};

mod sample;
use sample::SampleStruct;

#[test]
fn test_field_type() -> Result<(), Box<dyn std::error::Error>> {
    let sample_type = SampleStruct::data_type_wrapper();

    let t = sample_type.field_type("req_string")?;
    assert_eq!(t, data_type_wrapper!(String));
    let t = sample_type.field_type("req_i64")?;
    assert_eq!(t, data_type_wrapper!(Int64));
    let t = sample_type.field_type("req_f64")?;
    assert_eq!(t, data_type_wrapper!(Float64));
    let t = sample_type.field_type("req_bool")?;
    assert_eq!(t, data_type_wrapper!(Boolean));
    let t = sample_type.field_type("req_struct")?;
    assert_eq!(t, data_type_wrapper!(Struct([("subf_string", String)])));
    let t = sample_type.field_type("req_enum")?;
    assert_eq!(t, data_type_wrapper!(Enum([("ITEM", 1)])));

    let t = sample_type.field_type("opt_string")?;
    assert_eq!(t, data_type_wrapper!(Option(String)));
    let t = sample_type.field_type("opt_i64")?;
    assert_eq!(t, data_type_wrapper!(Option(Int64)));
    let t = sample_type.field_type("opt_f64")?;
    assert_eq!(t, data_type_wrapper!(Option(Float64)));
    let t = sample_type.field_type("opt_bool")?;
    assert_eq!(t, data_type_wrapper!(Option(Boolean)));
    let t = sample_type.field_type("opt_struct")?;
    assert_eq!(
        t,
        data_type_wrapper!(Option(Struct([("subf_string", String)])))
    );
    let t = sample_type.field_type("opt_enum")?;
    assert_eq!(t, data_type_wrapper!(Option(Enum([("ITEM", 1)]))));

    let t = sample_type.field_type("req_vec_req_item_string")?;
    assert_eq!(t, data_type_wrapper!(List(String)));
    let t = sample_type.field_type("req_vec_req_item_i64")?;
    assert_eq!(t, data_type_wrapper!(List(Int64)));
    let t = sample_type.field_type("req_vec_req_item_f64")?;
    assert_eq!(t, data_type_wrapper!(List(Float64)));
    let t = sample_type.field_type("req_vec_req_item_bool")?;
    assert_eq!(t, data_type_wrapper!(List(Boolean)));
    let t = sample_type.field_type("req_vec_req_item_struct")?;
    assert_eq!(
        t,
        data_type_wrapper!(List(Struct([("subf_string", String)])))
    );
    let t = sample_type.field_type("req_vec_req_item_enum")?;
    assert_eq!(t, data_type_wrapper!(List(Enum([("ITEM", 1)]))));

    let t = sample_type.field_type("opt_vec_req_item_string")?;
    assert_eq!(t, data_type_wrapper!(Option(List(String))));
    let t = sample_type.field_type("opt_vec_req_item_i64")?;
    assert_eq!(t, data_type_wrapper!(Option(List(Int64))));
    let t = sample_type.field_type("opt_vec_req_item_f64")?;
    assert_eq!(t, data_type_wrapper!(Option(List(Float64))));
    let t = sample_type.field_type("opt_vec_req_item_bool")?;
    assert_eq!(t, data_type_wrapper!(Option(List(Boolean))));
    let t = sample_type.field_type("opt_vec_req_item_struct")?;
    assert_eq!(
        t,
        data_type_wrapper!(Option(List(Struct([("subf_string", String)]))))
    );
    let t = sample_type.field_type("opt_vec_req_item_enum")?;
    assert_eq!(t, data_type_wrapper!(Option(List(Enum([("ITEM", 1)])))));

    let t = sample_type.field_type("req_vec_opt_item_string")?;
    assert_eq!(t, data_type_wrapper!(List(Option(String))));
    let t = sample_type.field_type("req_vec_opt_item_i64")?;
    assert_eq!(t, data_type_wrapper!(List(Option(Int64))));
    let t = sample_type.field_type("req_vec_opt_item_f64")?;
    assert_eq!(t, data_type_wrapper!(List(Option(Float64))));
    let t = sample_type.field_type("req_vec_opt_item_bool")?;
    assert_eq!(t, data_type_wrapper!(List(Option(Boolean))));
    let t = sample_type.field_type("req_vec_opt_item_struct")?;
    assert_eq!(
        t,
        data_type_wrapper!(List(Option(Struct([("subf_string", String)]))))
    );
    let t = sample_type.field_type("req_vec_opt_item_enum")?;
    assert_eq!(t, data_type_wrapper!(List(Option(Enum([("ITEM", 1)])))));

    let t = sample_type.field_type("opt_vec_opt_item_string")?;
    assert_eq!(t, data_type_wrapper!(Option(List(Option(String)))));
    let t = sample_type.field_type("opt_vec_opt_item_i64")?;
    assert_eq!(t, data_type_wrapper!(Option(List(Option(Int64)))));
    let t = sample_type.field_type("opt_vec_opt_item_f64")?;
    assert_eq!(t, data_type_wrapper!(Option(List(Option(Float64)))));
    let t = sample_type.field_type("opt_vec_opt_item_bool")?;
    assert_eq!(t, data_type_wrapper!(Option(List(Option(Boolean)))));
    let t = sample_type.field_type("opt_vec_opt_item_struct")?;
    assert_eq!(
        t,
        data_type_wrapper!(Option(List(Option(Struct([("subf_string", String)])))))
    );
    let t = sample_type.field_type("opt_vec_opt_item_enum")?;
    assert_eq!(
        t,
        data_type_wrapper!(Option(List(Option(Enum([("ITEM", 1)])))))
    );

    Ok(())
}
