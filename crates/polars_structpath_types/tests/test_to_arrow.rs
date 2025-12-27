mod sample;
use polars_core::prelude::Series;
use polars_structpath_types::{ArrowBuffer, HasArrowBuffer};
use sample::{sample_struct, sample_struct_null, SampleStruct};

#[test]
fn test_to_arrow() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = SampleStruct::new_buffer(1);
    buffer.push(sample_struct());
    buffer.push(sample_struct_null());
    buffer.push_null();
    buffer.push(sample_struct());
    let array_ref = buffer.to_arrow().unwrap();
    println!("{:?}", array_ref);
    let _series = Series::from_arrow("sample_struct".into(), array_ref).unwrap();
    // println!("{:?}", series);
    Ok(())
}
