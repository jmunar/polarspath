mod sample;
use polars_arrow::array::{Array, PrimitiveArray};
use polars_arrow::scalar::{BooleanScalar, ListScalar, PrimitiveScalar, Utf8Scalar};
use polars_core::prelude::Series;
use polars_structpath::{ArrowBuffer, FromArrow, IntoArrow};
use sample::{sample_struct, sample_struct_null, SampleStruct};

#[test]
fn test_req_struct_to_arrow_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let vals_in = vec![sample_struct(), sample_struct_null(), sample_struct()];
    let mut buffer = Option::<SampleStruct>::new_buffer(3);
    for val in &vals_in {
        buffer.push(val.clone());
    }
    let array_ref = Box::new(buffer.to_arrow().unwrap());
    let vals_out = SampleStruct::from_arrow(array_ref);

    assert_eq!(vals_in.len(), vals_out.len());
    for (val_in, val_out) in vals_in.iter().zip(vals_out.iter()) {
        assert_eq!(val_in, val_out);
    }

    Ok(())
}

#[test]
fn test_opt_struct_to_arrow_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let vals_in = vec![
        Some(sample_struct()),
        Some(sample_struct_null()),
        None,
        Some(sample_struct()),
    ];
    let mut buffer = Option::<SampleStruct>::new_buffer(4);
    for val in &vals_in {
        buffer.push(val.clone());
    }
    let array_ref = Box::new(buffer.to_arrow().unwrap());
    let vals_out = Option::<SampleStruct>::from_arrow(array_ref);

    assert_eq!(vals_in.len(), vals_out.len());
    for (val_in, val_out) in vals_in.iter().zip(vals_out.iter()) {
        assert_eq!(val_in, val_out);
    }

    Ok(())
}

#[test]
fn test_to_arrow_rows() -> Result<(), Box<dyn std::error::Error>> {
    let vals_in = vec![
        Some(sample_struct()),
        Some(sample_struct_null()),
        None,
        Some(sample_struct()),
    ];
    let mut buffer = Option::<SampleStruct>::new_buffer(1);
    for val in vals_in {
        buffer.push(val);
    }
    let array_ref = buffer.to_arrow().unwrap();

    for (idx, row) in array_ref.values_iter().enumerate() {
        let is_valid = array_ref.is_valid(idx);
        println!("Index {}: is_valid={:?}", idx, is_valid);
        // println!("{:?}", row);
        if is_valid {
            let req_string = (**row.get(0).unwrap())
                .as_any()
                .downcast_ref::<Utf8Scalar<i32>>()
                .unwrap()
                .value()
                .unwrap();
            println!("req_string: {:?}", req_string);

            let list_scalar = (**row.get(1).unwrap())
                .as_any()
                .downcast_ref::<ListScalar<i32>>()
                .unwrap();
            let inner_array = list_scalar.values();
            let bytes_array = inner_array
                .as_any()
                .downcast_ref::<PrimitiveArray<u8>>()
                .unwrap();
            let bytes: Vec<u8> = bytes_array.iter().map(|opt| *opt.unwrap()).collect();
            println!("req_bytes: {:?}", bytes);

            let req_i32 = (**row.get(2).unwrap())
                .as_any()
                .downcast_ref::<PrimitiveScalar<i32>>()
                .unwrap()
                .value()
                .unwrap();
            println!("req_i32: {:?}", req_i32);

            let req_i64 = (**row.get(3).unwrap())
                .as_any()
                .downcast_ref::<PrimitiveScalar<i64>>()
                .unwrap()
                .value()
                .unwrap();
            println!("req_i64: {:?}", req_i64);

            let req_u32 = (**row.get(4).unwrap())
                .as_any()
                .downcast_ref::<PrimitiveScalar<u32>>()
                .unwrap()
                .value()
                .unwrap();
            println!("req_u32: {:?}", req_u32);

            let req_u64 = (**row.get(5).unwrap())
                .as_any()
                .downcast_ref::<PrimitiveScalar<u64>>()
                .unwrap()
                .value()
                .unwrap();
            println!("req_u64: {:?}", req_u64);

            let req_f32 = (**row.get(6).unwrap())
                .as_any()
                .downcast_ref::<PrimitiveScalar<f32>>()
                .unwrap()
                .value()
                .unwrap();
            println!("req_f32: {:?}", req_f32);

            let req_f64 = (**row.get(7).unwrap())
                .as_any()
                .downcast_ref::<PrimitiveScalar<f64>>()
                .unwrap()
                .value()
                .unwrap();
            println!("req_f64: {:?}", req_f64);

            let req_bool = (**row.get(8).unwrap())
                .as_any()
                .downcast_ref::<BooleanScalar>()
                .unwrap()
                .value()
                .unwrap();
            println!("req_bool: {:?}", req_bool);
        } else {
            println!("Value is null");
        }
    }

    // Extract by field
    let series: Series = Series::from_arrow("sample_struct".into(), Box::new(array_ref)).unwrap();
    let _field_series = series
        .struct_()
        .unwrap()
        .field_by_name("req_string")
        .unwrap();
    // assert_eq!(_field_series.get(0)?.extract_str(), Some("req_string"));
    // assert_eq!(_field_series.get(1)?.extract_str(), Some("req_string"));
    // assert_eq!(_field_series.get(2)?.extract_str(), None);
    // assert_eq!(_field_series.get(3)?.extract_str(), Some("req_string"));

    Ok(())
}
