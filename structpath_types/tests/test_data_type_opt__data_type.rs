mod sample;
use sample::{fields_opt, fields_polars};

use polars_core::prelude::DataType;
use structpath_types::DataTypeOpt;

#[test]
fn test_to_data_type() -> Result<(), Box<dyn std::error::Error>> {
    let data_type_opt = DataTypeOpt::Struct(fields_opt());
    let data_type = data_type_opt.to_data_type();
    assert_eq!(data_type, DataType::Struct(fields_polars()));
    Ok(())
}
