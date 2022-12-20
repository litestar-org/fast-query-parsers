use serde_json::{Map, Value};
use std::collections::HashMap;
use std::convert::Infallible;
use urlencoding::decode;

type ValuesMap = Map<String, Value>;

pub fn parse_query_string(qs: &[u8], separator: char) -> Vec<(String, String)> {
    String::from_utf8(qs.to_vec())
        .unwrap()
        .replace('+', " ")
        .split(separator)
        .filter(|value| !value.is_empty())
        .map(|value| value.split_once('=').unwrap_or((value, "")))
        .map(|value| {
            (
                decode(value.0).unwrap().to_string(),
                decode(value.1).unwrap().to_string(),
            )
        })
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

    let json_integer = json_str.parse::<i64>();
    let json_float = json_str.parse::<f64>();
    let json_boolean = json_str.parse::<bool>();
    let json_null = Ok::<_, Infallible>(json_str == "null");
    let python_true = Ok::<_, Infallible>(json_str == "True");
    let python_false = Ok::<_, Infallible>(json_str == "False");

    match (
        json_integer,
        json_float,
        json_boolean,
        json_null,
        python_true,
        python_false,
    ) {
        (Ok(json_integer), _, _, _, _, _) => Value::from(json_integer),
        (_, Ok(json_float), _, _, _, _) => Value::from(json_float),
        (_, _, Ok(json_boolean), _, _, _) => Value::from(json_boolean),
        (_, _, _, Ok(true), _, _) => Value::Null,
        (_, _, _, _, Ok(true), _) => Value::from(true),
        (_, _, _, _, _, Ok(true)) => Value::from(false),
        _ => Value::from(json_str.replace('"', "")),
    }
}

pub fn parse_query_string_to_json(bs: &[u8]) -> Value {
    let mut values_map = ValuesMap::new();
    let mut array_map: HashMap<String, Vec<String>> = HashMap::new();

    for (key, value) in parse_query_string(bs, '&') {
        array_map.entry(key).or_default().push(value)
    }

    for (key, value) in array_map.into_iter() {
        if value.len() == 1 {
            values_map.insert(key, decode_value(value[0].to_owned()));
        } else {
            values_map.insert(key, decode_value(format!("[{}]", value.join(","))));
        }
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
            parse_query_string(b"first=%26%40A.ac&second=aaa", '&'),
            vec![
                (String::from("first"), String::from("&@A.ac")),
                (String::from("second"), String::from("aaa")),
            ]
        );
        assert_eq!(
            parse_query_string(b"first=a%40A.ac&second=aaa", '&'),
            vec![
                (String::from("first"), String::from("a@A.ac")),
                (String::from("second"), String::from("aaa")),
            ]
        );
        assert_eq!(
            parse_query_string(b"first=a%40A%26.ac&second=aaa", '&'),
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

    #[test]
    fn it_parses_a_list_of_values() {
        assert_eq!(
            parse_query_string_to_json(b"a=1&a=2&a=3"),
            json!({ "a": [1,2,3] })
        );
    }
}