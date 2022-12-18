mod query_string;

use pyo3::prelude::*;
use pythonize::pythonize;

pub use query_string::{parse_query_string, parse_query_string_to_json};

#[pyfunction]
#[pyo3(text_signature = "(qs, separator, /)")]
fn parse_qsl(qs: &[u8], separator: char) -> PyResult<Vec<(String, String)>> {
    Ok(parse_query_string(qs, separator))
}

#[pyfunction]
#[pyo3(text_signature = "(qs, /)")]
fn parse_qs(qs: &[u8]) -> PyResult<PyObject> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    Ok(pythonize(py, &parse_query_string_to_json(qs)).unwrap())
}

#[pymodule]
fn fast_query_parser(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_qsl, m)?)?;
    m.add_function(wrap_pyfunction!(parse_qs, m)?)?;

    Ok(())
}
