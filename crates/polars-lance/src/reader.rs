use futures::TryStreamExt;

use crate::interop::record_batches_to_df;
use crate::options::LanceScanOptions;
use crate::runtime::block_on;
use polars_core::prelude::*;

/// Read a Lance dataset into a Polars DataFrame (eager).
///
/// Opens the dataset at the given path, applies projection/filter/limit options,
/// and collects all results into a single DataFrame.
pub fn read_lance(options: LanceScanOptions) -> PolarsResult<DataFrame> {
    block_on(read_lance_async(options))
}

async fn read_lance_async(options: LanceScanOptions) -> PolarsResult<DataFrame> {
    let dataset = lance::Dataset::open(&options.path)
        .await
        .map_err(|e| polars_err!(ComputeError: "Failed to open Lance dataset: {}", e))?;

    let mut scanner = dataset.scan();

    if let Some(ref columns) = options.columns {
        let col_refs: Vec<&str> = columns.iter().map(String::as_str).collect();
        scanner
            .project(&col_refs)
            .map_err(|e| polars_err!(ComputeError: "Failed to project columns: {}", e))?;
    }

    if let Some(ref filter) = options.filter {
        scanner
            .filter(filter)
            .map_err(|e| polars_err!(ComputeError: "Failed to apply filter: {}", e))?;
    }

    let limit = options.n_rows.map(|n| n as i64);
    let offset = options.offset.map(|n| n as i64);
    if limit.is_some() || offset.is_some() {
        scanner
            .limit(limit, offset)
            .map_err(|e| polars_err!(ComputeError: "Failed to set limit/offset: {}", e))?;
    }

    if let Some(batch_size) = options.batch_size {
        scanner.batch_size(batch_size);
    }

    let stream = scanner
        .try_into_stream()
        .await
        .map_err(|e| polars_err!(ComputeError: "Failed to create scan stream: {}", e))?;

    let batches: Vec<_> = stream
        .try_collect()
        .await
        .map_err(|e| polars_err!(ComputeError: "Failed to collect batches: {}", e))?;

    record_batches_to_df(&batches)
}
