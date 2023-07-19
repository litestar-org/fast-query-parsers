mod query_string;

use pyo3::prelude::*;
use pythonize::pythonize;

pub use query_string::{parse_query_string as parse_qs, parse_query_string_to_json};

// parse query string into a list of tuples.
#[pyfunction]
#[pyo3(text_signature = "(qs, separator, /)")]
fn parse_query_string(qs: &[u8], separator: char) -> PyResult<Vec<(String, String)>> {
    Ok(parse_qs(qs, separator))
}

// parse query string into a python object.
#[pyfunction]
#[pyo3(signature = (qs, parse_numbers=true, /),text_signature = "(qs, parse_numbers=true, /)")]
fn parse_url_encoded_dict(py: Python, qs: &[u8], parse_numbers: bool) -> PyResult<PyObject> {
    Ok(pythonize(py, &parse_query_string_to_json(qs, parse_numbers)).unwrap())
}

#[pymodule]
fn fast_query_parsers(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_query_string, m)?)?;
    m.add_function(wrap_pyfunction!(parse_url_encoded_dict, m)?)?;

    Ok(())
}
