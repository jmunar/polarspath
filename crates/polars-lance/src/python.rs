use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;

use crate::options::LanceScanOptions;

#[pyfunction]
#[pyo3(signature = (path, *, columns=None, n_rows=None, offset=None, filter=None, batch_size=None))]
fn read_lance(
    path: String,
    columns: Option<Vec<String>>,
    n_rows: Option<usize>,
    offset: Option<usize>,
    filter: Option<String>,
    batch_size: Option<usize>,
) -> PyResult<PyDataFrame> {
    let options = LanceScanOptions {
        path,
        columns,
        n_rows,
        offset,
        filter,
        batch_size,
    };
    let df = crate::read_lance(options)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(PyDataFrame(df))
}

#[pymodule]
fn _polars_lance(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_lance, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
