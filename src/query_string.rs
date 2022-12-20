use percent_encoding::percent_decode;
use serde_json::{Map, Value};
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

fn decode_value(json_str: String) -> Value {
    if json_str.starts_with('{') && json_str.ends_with('}') {
        let values_map: ValuesMap = serde_json::from_str(json_str.as_str()).unwrap();
        let mut result = ValuesMap::new();

        for (k, v) in values_map {
            result.insert(k, decode_value(v.to_string()));
        }

        return Value::from(result);
    }
    if json_str.starts_with('[') && json_str.ends_with(']') {
        let values_array: Value = serde_json::from_str(json_str.as_str()).unwrap();
        let vector_values = values_array
            .as_array()
            .unwrap()
            .iter()
            .map(|el| decode_value(el.to_string()))
            .collect::<Vec<Value>>();
        return Value::from(vector_values);
    }

    let n = json_str.parse::<i64>();
    let f = json_str.parse::<f64>();
    let b = json_str.parse::<bool>();
    let null = Ok::<_, Infallible>(json_str == "null");

    match (n, f, b, null) {
        (Ok(n), _, _, _) => Value::from(n),
        (_, Ok(f), _, _) => Value::from(f),
        (_, _, Ok(b), _) => Value::from(b),
        (_, _, _, Ok(true)) => Value::Null,
        _ => Value::from(json_str.replace('"', "")),
    }
}

pub fn parse_query_string_to_json(bs: &[u8]) -> Value {
    let mut values_map = ValuesMap::new();

    for (key, value) in parse_query_string(bs, '&') {
        values_map.insert(key, decode_value(value));
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
        assert_eq!(
            parse_query_string(b"key=1&key=2&anotherKey=a&yetAnother=z", '&'),
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
        assert_eq!(
            parse_query_string(b"key=1;key=2;anotherKey=a;yetAnother=z", ';'),
            vec![
                (String::from("key"), String::from("1")),
                (String::from("key"), String::from("2")),
                (String::from("anotherKey"), String::from("a")),
                (String::from("yetAnother"), String::from("z")),
            ]
        );
    }

    #[test]
    fn test_handles_url_encoded_ampersand() {
        assert_eq!(
            parse_query_string(b"first=x@test.com&second=aaa", '&'),
            vec![
                (String::from("first"), String::from("x@test.com")),
                (String::from("second"), String::from("aaa")),
            ]
        );
        assert_eq!(
            parse_query_string(b"first=&@A.ac&second=aaa", '&'),
            vec![
                (String::from("first"), String::from("&@A.ac")),
                (String::from("second"), String::from("aaa")),
            ]
        );
        assert_eq!(
            parse_query_string(b"first=a@A.ac&second=aaa", '&'),
            vec![
                (String::from("first"), String::from("a@A.ac")),
                (String::from("second"), String::from("aaa")),
            ]
        );
        assert_eq!(
            parse_query_string(b"first=a@A&.ac&second=aaa", '&'),
            vec![
                (String::from("first"), String::from("a@A&.ac")),
                (String::from("second"), String::from("aaa")),
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
    fn it_parses_nested_objects() {
        assert_eq!(
            parse_query_string_to_json(r#"a={"first": 1, "second": 2, "third": "abc"}"#.as_bytes()),
            json!({"a": {"first": 1, "second": 2, "third": "abc"}})
        );
    }

    #[test]
    fn it_parses_nested_arrays() {
        assert_eq!(
            parse_query_string_to_json(r#"a=[1,2,3,"abc"]"#.as_bytes()),
            json!({"a": [1,2,3,"abc"]})
        );
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
