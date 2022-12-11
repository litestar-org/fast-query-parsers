use pyo3::prelude::*;

fn _parse_qsl(qs: &str, separator: char) -> Vec<(&str, &str)> {
    qs.split(separator)
        .filter(|value| !value.is_empty())
        .map(|value| {
            if value.contains('=') {
                value.split_once('=').unwrap()
            } else {
                (value, "")
            }
        })
        .collect::<Vec<(&str, &str)>>()
}

#[pyfunction]
fn parse_qsl(qs: &str, separator: char) -> PyResult<Vec<(&str, &str)>> {
    Ok(_parse_qsl(qs, separator))
}

#[pymodule]
fn fast_query_parser(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_qsl, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ampersand_separator() {
        let result = _parse_qsl("key=1&key=2&anotherKey=a&yetAnother=z", '&');
        assert_eq!(
            result,
            vec![
                ("key", "1"),
                ("key", "2"),
                ("anotherKey", "a"),
                ("yetAnother", "z")
            ]
        );
    }

    #[test]
    fn test_semicolon_separator() {
        let result = _parse_qsl("key=1;key=2;anotherKey=a;yetAnother=z", ';');
        assert_eq!(
            result,
            vec![
                ("key", "1"),
                ("key", "2"),
                ("anotherKey", "a"),
                ("yetAnother", "z")
            ]
        );
    }
}
