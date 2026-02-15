use std::any::Any;
use std::sync::Arc;

use polars_core::prelude::*;
use polars_lazy::prelude::*;

use crate::interop::arrow_schema_to_polars;
use crate::options::LanceScanOptions;
use crate::reader::read_lance;
use crate::runtime::block_on;

struct LanceScan {
    options: LanceScanOptions,
}

impl AnonymousScan for LanceScan {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn scan(&self, scan_opts: AnonymousScanArgs) -> PolarsResult<DataFrame> {
        let mut options = self.options.clone();

        // Merge projection pushdown columns
        if let Some(with_columns) = &scan_opts.with_columns {
            let cols: Vec<String> = with_columns.iter().map(|s| s.to_string()).collect();
            options.columns = Some(cols);
        }

        // Merge n_rows limit
        if let Some(n_rows) = scan_opts.n_rows {
            options.n_rows = Some(match options.n_rows {
                Some(existing) => existing.min(n_rows),
                None => n_rows,
            });
        }

        read_lance(options)
    }

    fn schema(&self, _infer_schema_length: Option<usize>) -> PolarsResult<SchemaRef> {
        let schema = block_on(async {
            let dataset = lance::Dataset::open(&self.options.path).await.map_err(
                |e| polars_err!(ComputeError: "Failed to open Lance dataset for schema: {}", e),
            )?;

            let lance_schema = dataset.schema();
            let arrow_schema: arrow::datatypes::Schema = lance_schema.into();
            arrow_schema_to_polars(&arrow_schema)
        })?;

        Ok(Arc::new(schema))
    }

    fn allows_projection_pushdown(&self) -> bool {
        true
    }

    fn allows_predicate_pushdown(&self) -> bool {
        false
    }
}

/// Create a lazy scan over a Lance dataset.
///
/// Returns a `LazyFrame` backed by the `AnonymousScan` trait, enabling
/// projection pushdown for efficient column selection.
pub fn scan_lance(options: LanceScanOptions) -> PolarsResult<LazyFrame> {
    let scan = LanceScan { options };
    let args = ScanArgsAnonymous {
        name: "LANCE",
        ..ScanArgsAnonymous::default()
    };
    LazyFrame::anonymous_scan(Arc::new(scan), args)
}
