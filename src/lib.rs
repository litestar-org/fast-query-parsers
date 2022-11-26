use pyo3::prelude::*;
use urlencoding::decode_binary;

fn _parse_qsl(qs: &[u8], separator: char) -> Box<[(String, String)]> {
    String::from_utf8(decode_binary(qs).into_owned())
        .unwrap()
        .split(separator)
        .filter(|value| !value.is_empty())
        .map(|value| {
            if value.contains('=') {
                value.split_once('=').unwrap()
            } else {
                (value, "")
            }
        })
        .map(|value| (value.0.to_string(), value.1.to_string().replace('+', " ")))
        .collect::<Vec<(String, String)>>()
        .into_boxed_slice()
}

#[pyfunction]
fn parse_qsl(qs: &[u8], separator: char) -> PyResult<Vec<(String, String)>> {
    Ok(_parse_qsl(qs, separator).to_vec())
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
        let result = _parse_qsl("key=1&key=2&anotherKey=a&yetAnother=z".as_bytes(), '&');
        assert_eq!(
            result,
            vec![
                ("key".to_string(), "1".to_string()),
                ("key".to_string(), "2".to_string()),
                ("anotherKey".to_string(), "a".to_string()),
                ("yetAnother".to_string(), "z".to_string())
            ]
            .into_boxed_slice()
        );
    }

    #[test]
    fn test_semicolon_separator() {
        let result = _parse_qsl("key=1;key=2;anotherKey=a;yetAnother=z".as_bytes(), ';');
        assert_eq!(
            result,
            vec![
                ("key".to_string(), "1".to_string()),
                ("key".to_string(), "2".to_string()),
                ("anotherKey".to_string(), "a".to_string()),
                ("yetAnother".to_string(), "z".to_string())
            ]
            .into_boxed_slice()
        );
    }
}
