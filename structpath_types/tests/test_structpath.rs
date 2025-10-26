mod sample;
use polars_core::prelude::DataType;
use structpath_types::{data_type_wrapper, HasDataTypeWrapper, Path, StructPath};

#[test]
fn test_fields() {
    let fields = sample::SampleStruct::fields();
    assert_eq!(*fields, sample::fields_polars());
}

#[test]
fn test_data_type() {
    let data_type = sample::SampleStruct::data_type();
    assert_eq!(*data_type, DataType::Struct(sample::fields_polars()));
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
