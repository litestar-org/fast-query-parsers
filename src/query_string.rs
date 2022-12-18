use percent_encoding::percent_decode;
use serde_json::{Map, Number, Value};
use std::convert::Infallible;

type ValuesMap = Map<String, Value>;

fn decode_url_encoded(url_encoded: &[u8]) -> String {
    percent_decode(url_encoded).decode_utf8_lossy().to_string()
}

pub fn parse_query_string(qs: &[u8], separator: char) -> Vec<(String, String)> {
    decode_url_encoded(qs)
        .replace('+', " ")
        .split(separator)
        .filter(|value| !value.is_empty())
        .map(|value| value.split_once('=').unwrap_or((value, "")))
        .map(|value| (value.0.to_owned(), value.1.to_owned()))
        .collect::<Vec<(String, String)>>()
}

pub fn parse_query_string_to_json(bs: &[u8]) -> Value {
    let mut values_map = ValuesMap::new();

    for (key, value) in parse_query_string(bs, '&') {
        let decoded_value = {
            let n = value.parse::<i64>();
            let f = value.parse::<f64>();
            let b = value.parse::<bool>();
            let null = Ok::<_, Infallible>(value == "null");
            match (n, f, b, null) {
                (Ok(n), _, _, _) => Value::Number(Number::from(n)),
                (_, Ok(f), _, _) => Value::Number(Number::from_f64(f).unwrap()),
                (_, _, Ok(b), _) => Value::Bool(b),
                (_, _, _, Ok(true)) => Value::Null,
                _ => Value::String(value),
            }
        };
        values_map.insert(key, decoded_value);
    }

    values_map.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::{json, to_string, Value};

    fn eq_str(value: Value, string: &str) {
        assert_eq!(&to_string(&value).unwrap(), string)
    }

    #[test]
    fn test_ampersand_separator() {
        let result = parse_query_string(b"key=1&key=2&anotherKey=a&yetAnother=z", '&');
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
        let result = parse_query_string(b"key=1;key=2;anotherKey=a;yetAnother=z", ';');
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
    fn it_parses_simple_string() {
        eq_str(parse_query_string_to_json(b"0=foo"), r#"{"0":"foo"}"#);
    }

    #[test]
    fn it_transforms_standalone_keys() {
        eq_str(
            parse_query_string_to_json(b"foo=bar&baz"),
            r#"{"baz":"","foo":"bar"}"#,
        );
    }

    #[test]
    fn it_doesnt_produce_empty_keys() {
        assert_eq!(parse_query_string_to_json(b"_r=1&"), json!({"_r": 1}));
    }

    #[test]
    fn it_parses_plus_sign() {
        eq_str(
            parse_query_string_to_json(b"a=b%20c+d%2B"),
            r#"{"a":"b c d+"}"#,
        );
    }

    #[test]
    fn it_parses_numbers() {
        assert_eq!(parse_query_string_to_json(b"a=1"), json!({"a": 1}));
        assert_eq!(parse_query_string_to_json(b"a=1.1"), json!({"a": 1.1}));
        assert_eq!(parse_query_string_to_json(b"a=1.1e1"), json!({"a": 11.0}));
        assert_eq!(parse_query_string_to_json(b"a=1.1e-1"), json!({"a": 0.11}));
    }

    #[test]
    fn it_parses_booleans() {
        assert_eq!(parse_query_string_to_json(b"a=true"), json!({"a": true}));
        assert_eq!(parse_query_string_to_json(b"a=false"), json!({"a": false}));
    }

    #[test]
    fn it_parses_null() {
        assert_eq!(parse_query_string_to_json(b"a=null"), json!({ "a": null }));
    }

    #[test]
    fn it_parses_empty_string() {
        assert_eq!(parse_query_string_to_json(b"a="), json!({ "a": "" }));
    }
}
