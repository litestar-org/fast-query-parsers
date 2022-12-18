use pyo3::prelude::*;
use urlencoding::decode;

fn _parse_qsl(qs: &str, separator: char) -> Vec<(String, String)> {
    decode(qs)
        .unwrap_or_default()
        .split(separator)
        .filter(|value| !value.is_empty())
        .map(|value| value.split_once('=').unwrap_or((value, "")))
        .map(|value| (value.0.to_owned(), value.1.replace('+', " ")))
        .collect::<Vec<(String, String)>>()
}

#[pyfunction]
fn parse_qsl(qs: &str, separator: char) -> PyResult<Vec<(String, String)>> {
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
                (String::from("key"), String::from("1")),
                (String::from("key"), String::from("2")),
                (String::from("anotherKey"), String::from("a")),
                (String::from("yetAnother"), String::from("z")),
            ]
        );
    }

    #[test]
    fn test_semicolon_separator() {
        let result = _parse_qsl("key=1;key=2;anotherKey=a;yetAnother=z", ';');
        assert_eq!(
            result,
            vec![
                (String::from("key"), String::from("1")),
                (String::from("key"), String::from("2")),
                (String::from("anotherKey"), String::from("a")),
                (String::from("yetAnother"), String::from("z")),
            ]
        );
    }
}
