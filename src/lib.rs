mod query_string;

use pyo3::prelude::*;
use pyo3::types::PyAny;
use pythonize::pythonize;

pub use query_string::{parse_query_string as parse_qs, parse_query_string_to_json};

// parse query string into a list of tuples.
#[pyfunction]
#[pyo3(text_signature = "(qs, separator)")]
fn parse_query_string(qs: &[u8], separator: char) -> Vec<(String, String)> {
    parse_qs(qs, separator)
}

// parse query string into a python object.
#[pyfunction]
#[pyo3(signature = (qs, parse_numbers=true), text_signature = "(qs, parse_numbers=true)")]
fn parse_url_encoded_dict(py: Python, qs: &[u8], parse_numbers: bool) -> Py<PyAny> {
    pythonize(py, &parse_query_string_to_json(qs, parse_numbers))
        .unwrap()
        .into()
}

#[pyo3::prelude::pymodule(gil_used = false)]
mod fast_query_parsers {
    #[pymodule_export]
    use super::parse_query_string;

    #[pymodule_export]
    use super::parse_url_encoded_dict;
}
