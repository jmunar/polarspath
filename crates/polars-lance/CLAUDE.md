# polars-lance

Lance dataset integration for Polars DataFrames.

## Overview

Provides `read_lance` (eager) and `scan_lance` (lazy) for reading [Lance](https://lancedb.github.io/lance/) datasets into Polars DataFrames/LazyFrames.

## Architecture

```
src/
├── lib.rs        # Module structure, public API (read_lance, scan_lance, LanceScanOptions)
├── interop.rs    # Arrow-rs ↔ polars-arrow bridge via Arrow C Data Interface (FFI)
├── runtime.rs    # Shared tokio runtime (OnceLock) for async→sync bridging
├── options.rs    # LanceScanOptions struct (path, columns, n_rows, offset, filter, batch_size)
├── reader.rs     # read_lance() — eager read into DataFrame
├── scan.rs       # scan_lance() — lazy scan via AnonymousScan with projection pushdown
└── python.rs     # pyo3 bindings (gated behind `extension-module` feature)
```

## Arrow Interop (interop.rs)

The central challenge: Lance uses `arrow` crate (arrow-rs), Polars uses `polars-arrow` (arrow2 fork). These are different Arrow implementations.

**Bridge strategy**: Arrow C Data Interface (FFI) with `std::mem::transmute` between the two crates' `#[repr(C)]` FFI structs (`FFI_ArrowArray` / `ArrowArray`, `FFI_ArrowSchema` / `ArrowSchema`), which share identical memory layouts per the Arrow C Data Interface spec.

**Key functions**:
- `import_arrow_column(column) -> (polars ArrayRef, ArrowDataType)` — single column conversion via FFI
- `record_batch_to_df(batch) -> DataFrame` — full RecordBatch conversion
- `record_batches_to_df(batches) -> DataFrame` — multiple batches, vstacked
- `arrow_schema_to_polars(schema) -> Schema` — schema conversion via FFI

**Important**: Use `Series::from_arrow()` (not `from_chunks_and_dtype_unchecked`) to build Series from imported arrays. This handles type mapping correctly (e.g., arrow-rs `Utf8` → polars `String`/`Utf8View`).

## Async Bridging (runtime.rs)

Lance is fully async (tokio). Polars is sync. A shared `OnceLock<Runtime>` tokio multi-thread runtime is created once and used via `block_on()` to bridge async Lance operations into sync Polars calls.

## Lazy Scan (scan.rs)

Implements `AnonymousScan` trait for `LazyFrame::anonymous_scan()`:
- `allows_projection_pushdown() -> true` — Polars pushes column selection down
- `allows_predicate_pushdown() -> false` — kept simple for v1
- `scan()` merges `AnonymousScanArgs` (with_columns, n_rows) with user `LanceScanOptions`
- `schema()` opens the dataset to read its Arrow schema, converts to polars Schema

## Lance API Usage (reader.rs)

```rust
let dataset = Dataset::open(&path).await?;
let mut scanner = dataset.scan();
scanner.project(&columns)?;         // column projection
scanner.filter("id > 10")?;         // SQL-style WHERE
scanner.limit(Some(100), Some(10))?; // limit/offset (i64)
scanner.batch_size(1024);            // rows per batch
let stream = scanner.try_into_stream().await?;
let batches: Vec<RecordBatch> = stream.try_collect().await?;
```

## Version Constraints

**Lance is pinned to 0.39** due to a chrono version conflict:
- `polars-arrow 0.53` requires `chrono <= 0.4.41`
- `lance >= 1.0` requires `chrono >= 0.4.42` (via datafusion 51+)
- chrono 0.4.42 changed parsing behavior for `%.3f`/`%.6f`/`%.9f` specifiers, which broke polars assumptions

**Upgrade path**: Once polars releases chrono 0.4.42 support (tracked in [pola-rs/polars#26075](https://github.com/pola-rs/polars/pull/26075)), update to:
- `lance = "2"`, `arrow = "57.3"`, `arrow-schema = "57.3"`, `arrow-array = "57.3"`

The Lance API surface used here (`Dataset::open`, `Scanner`, `Dataset::write`) is stable across 0.39 → 2.0.

## Python Bindings

`pyo3-polars` must be **optional** (`extension-module` feature only). If it's a default dependency, it enables the `python` feature on polars-plan, which adds `PythonScan`/`PythonDataset` enum variants that `polars-mem-engine` doesn't handle — causing compilation errors.

## Testing

- **Unit tests** in `interop.rs` — test FFI conversion of RecordBatch/Schema without touching disk
- **Integration tests** in `tests/test_lance.rs` — create temp Lance datasets via `Dataset::write()`, test roundtrips, projection, limit, offset, filter, and lazy scan. Uses `__test_utils::block_on` to run async dataset writes.

## Polars 0.53 API Notes

- `DataFrame::new(height: usize, columns: Vec<Column>)` — takes height + Column (not Series)
- `Column::from(series)` — convert Series → Column
- `DataType::from_arrow(dt, md: Option<&Metadata>)` — second arg is metadata, not bool
- `Series::from_arrow(name, array)` — safe conversion handling type mapping (Utf8→Utf8View)
