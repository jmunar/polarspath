use arrow_array::RecordBatch;
use polars_arrow::ffi::{
    import_array_from_c, import_field_from_c, ArrowArray as PolarsArrowArray,
    ArrowSchema as PolarsArrowSchema,
};
use polars_core::prelude::*;

/// Import a single arrow-rs column into a polars-arrow array via Arrow C Data Interface FFI.
fn import_arrow_column(
    column: &arrow_array::ArrayRef,
) -> PolarsResult<(
    polars_arrow::array::ArrayRef,
    polars_arrow::datatypes::ArrowDataType,
)> {
    let data = column.to_data();

    let (ffi_array, ffi_schema) = arrow_array::ffi::to_ffi(&data)
        .map_err(|e| polars_err!(ComputeError: "Failed to export arrow array to FFI: {}", e))?;

    // SAFETY: Both FFI struct types follow the Arrow C Data Interface specification
    // and have identical C memory layouts (#[repr(C)] with the same fields).
    // The release callback from arrow-rs will properly clean up when polars-arrow drops.
    unsafe {
        let polars_schema: PolarsArrowSchema = std::mem::transmute(ffi_schema);
        let polars_field = import_field_from_c(&polars_schema)?;
        let dtype = polars_field.dtype;

        let polars_ffi_array: PolarsArrowArray = std::mem::transmute(ffi_array);
        let array = import_array_from_c(polars_ffi_array, dtype.clone())?;
        Ok((array, dtype))
    }
}

/// Convert an arrow-rs `RecordBatch` to a polars `DataFrame` via Arrow C Data Interface.
pub fn record_batch_to_df(batch: &RecordBatch) -> PolarsResult<DataFrame> {
    let schema = batch.schema();
    let mut columns = Vec::with_capacity(batch.num_columns());

    for (i, column) in batch.columns().iter().enumerate() {
        let field_name = schema.field(i).name();
        let (polars_array, _dtype) = import_arrow_column(column)?;

        let name = PlSmallStr::from(field_name.as_str());
        let series = Series::from_arrow(name, polars_array)?;
        columns.push(Column::from(series));
    }

    let height = batch.num_rows();
    DataFrame::new(height, columns)
}

/// Convert multiple arrow-rs `RecordBatch`es into a single polars `DataFrame`.
pub fn record_batches_to_df(batches: &[RecordBatch]) -> PolarsResult<DataFrame> {
    if batches.is_empty() {
        return Ok(DataFrame::empty());
    }

    let mut dfs: Vec<DataFrame> = batches
        .iter()
        .map(record_batch_to_df)
        .collect::<PolarsResult<Vec<_>>>()?;

    if dfs.len() == 1 {
        return Ok(dfs.remove(0));
    }

    let first = dfs.remove(0);
    let mut combined = first;
    for df in dfs {
        combined.vstack_mut(&df)?;
    }
    Ok(combined)
}

/// Convert an arrow-rs `Schema` to a polars `Schema` via FFI.
pub fn arrow_schema_to_polars(schema: &arrow::datatypes::Schema) -> PolarsResult<Schema> {
    let mut polars_schema = Schema::with_capacity(schema.fields().len());
    for field in schema.fields() {
        let ffi_schema = arrow_schema::ffi::FFI_ArrowSchema::try_from(field.as_ref())
            .map_err(|e| polars_err!(ComputeError: "Failed to export arrow field to FFI: {}", e))?;

        let polars_field = unsafe {
            let polars_ffi: PolarsArrowSchema = std::mem::transmute(ffi_schema);
            import_field_from_c(&polars_ffi)?
        };

        let name = PlSmallStr::from(polars_field.name.as_str());
        let dtype = DataType::from_arrow(&polars_field.dtype, None);
        polars_schema.with_column(name, dtype);
    }
    Ok(polars_schema)
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow_array::{Float64Array, Int32Array, StringArray};

    #[test]
    fn test_record_batch_to_df() {
        let schema = arrow::datatypes::Schema::new(vec![
            arrow::datatypes::Field::new("id", arrow::datatypes::DataType::Int32, false),
            arrow::datatypes::Field::new("name", arrow::datatypes::DataType::Utf8, true),
            arrow::datatypes::Field::new("score", arrow::datatypes::DataType::Float64, true),
        ]);

        let batch = RecordBatch::try_new(
            std::sync::Arc::new(schema),
            vec![
                std::sync::Arc::new(Int32Array::from(vec![1, 2, 3])),
                std::sync::Arc::new(StringArray::from(vec![Some("alice"), Some("bob"), None])),
                std::sync::Arc::new(Float64Array::from(vec![Some(95.5), None, Some(87.0)])),
            ],
        )
        .unwrap();

        let df = record_batch_to_df(&batch).unwrap();
        assert_eq!(df.shape(), (3, 3));
        let col_names: Vec<&str> = df.get_column_names().iter().map(|s| s.as_str()).collect();
        assert_eq!(col_names, vec!["id", "name", "score"]);
    }

    #[test]
    fn test_record_batches_to_df() {
        let schema = std::sync::Arc::new(arrow::datatypes::Schema::new(vec![
            arrow::datatypes::Field::new("x", arrow::datatypes::DataType::Int32, false),
        ]));

        let batch1 = RecordBatch::try_new(
            schema.clone(),
            vec![std::sync::Arc::new(Int32Array::from(vec![1, 2]))],
        )
        .unwrap();

        let batch2 = RecordBatch::try_new(
            schema,
            vec![std::sync::Arc::new(Int32Array::from(vec![3, 4, 5]))],
        )
        .unwrap();

        let df = record_batches_to_df(&[batch1, batch2]).unwrap();
        assert_eq!(df.shape(), (5, 1));
    }

    #[test]
    fn test_arrow_schema_to_polars() {
        let arrow_schema = arrow::datatypes::Schema::new(vec![
            arrow::datatypes::Field::new("a", arrow::datatypes::DataType::Int32, false),
            arrow::datatypes::Field::new("b", arrow::datatypes::DataType::Utf8, true),
        ]);

        let polars_schema = arrow_schema_to_polars(&arrow_schema).unwrap();
        assert_eq!(polars_schema.len(), 2);
        assert!(polars_schema.contains("a"));
        assert!(polars_schema.contains("b"));
    }
}
