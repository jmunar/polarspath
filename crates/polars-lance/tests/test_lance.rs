use std::sync::Arc;

use arrow_array::{Float64Array, Int32Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use polars_lance::{read_lance, scan_lance, LanceScanOptions};

/// Helper: create a temp Lance dataset and return its path.
fn create_test_dataset() -> (tempfile::TempDir, String) {
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new("name", DataType::Utf8, true),
        Field::new("score", DataType::Float64, true),
    ]));

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(Int32Array::from(vec![1, 2, 3, 4, 5])),
            Arc::new(StringArray::from(vec![
                Some("alice"),
                Some("bob"),
                Some("charlie"),
                None,
                Some("eve"),
            ])),
            Arc::new(Float64Array::from(vec![
                Some(95.5),
                Some(87.0),
                None,
                Some(72.3),
                Some(91.0),
            ])),
        ],
    )
    .unwrap();

    let tmpdir = tempfile::tempdir().unwrap();
    let path = tmpdir.path().join("test_dataset");
    let path_str = path.to_str().unwrap().to_string();

    let reader = arrow_array::RecordBatchIterator::new(vec![Ok(batch)], schema);

    polars_lance::__test_utils::block_on(async {
        lance::Dataset::write(reader, &path_str, None)
            .await
            .unwrap();
    });

    (tmpdir, path_str)
}

#[test]
fn test_read_lance_basic() {
    let (_tmpdir, path) = create_test_dataset();

    let df = read_lance(LanceScanOptions {
        path,
        ..Default::default()
    })
    .unwrap();

    assert_eq!(df.shape(), (5, 3));
    let col_names: Vec<&str> = df.get_column_names().iter().map(|s| s.as_str()).collect();
    assert_eq!(col_names, vec!["id", "name", "score"]);
}

#[test]
fn test_read_lance_column_projection() {
    let (_tmpdir, path) = create_test_dataset();

    let df = read_lance(LanceScanOptions {
        path,
        columns: Some(vec!["id".to_string(), "name".to_string()]),
        ..Default::default()
    })
    .unwrap();

    assert_eq!(df.shape(), (5, 2));
    let col_names: Vec<&str> = df.get_column_names().iter().map(|s| s.as_str()).collect();
    assert_eq!(col_names, vec!["id", "name"]);
}

#[test]
fn test_read_lance_row_limit() {
    let (_tmpdir, path) = create_test_dataset();

    let df = read_lance(LanceScanOptions {
        path,
        n_rows: Some(3),
        ..Default::default()
    })
    .unwrap();

    assert_eq!(df.shape().0, 3);
}

#[test]
fn test_read_lance_offset() {
    let (_tmpdir, path) = create_test_dataset();

    let df = read_lance(LanceScanOptions {
        path,
        offset: Some(2),
        n_rows: Some(2),
        ..Default::default()
    })
    .unwrap();

    assert_eq!(df.shape().0, 2);
}

#[test]
fn test_read_lance_filter() {
    let (_tmpdir, path) = create_test_dataset();

    let df = read_lance(LanceScanOptions {
        path,
        filter: Some("id > 3".to_string()),
        ..Default::default()
    })
    .unwrap();

    assert_eq!(df.shape().0, 2);
}

#[test]
fn test_scan_lance_basic() {
    let (_tmpdir, path) = create_test_dataset();

    let lf = scan_lance(LanceScanOptions {
        path,
        ..Default::default()
    })
    .unwrap();

    let df = lf.collect().unwrap();
    assert_eq!(df.shape(), (5, 3));
}

#[test]
fn test_scan_lance_with_select() {
    let (_tmpdir, path) = create_test_dataset();

    let lf = scan_lance(LanceScanOptions {
        path,
        ..Default::default()
    })
    .unwrap();

    let df = lf
        .select([
            polars_lazy::prelude::col("id"),
            polars_lazy::prelude::col("score"),
        ])
        .collect()
        .unwrap();

    assert_eq!(df.shape(), (5, 2));
    let col_names: Vec<&str> = df.get_column_names().iter().map(|s| s.as_str()).collect();
    assert_eq!(col_names, vec!["id", "score"]);
}
