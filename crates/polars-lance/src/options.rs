#[derive(Clone, Debug, Default)]
pub struct LanceScanOptions {
    pub path: String,
    /// Column projection: select only these columns.
    pub columns: Option<Vec<String>>,
    /// Maximum number of rows to return.
    pub n_rows: Option<usize>,
    /// Number of rows to skip from the start.
    pub offset: Option<usize>,
    /// SQL-style WHERE clause for filtering rows.
    pub filter: Option<String>,
    /// Number of rows per batch during scanning.
    pub batch_size: Option<usize>,
}
