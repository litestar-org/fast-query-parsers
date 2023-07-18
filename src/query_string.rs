use std::convert::Infallible;

use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::FxHashMap;
use serde_json::{from_str, Value};
use urlencoding::decode;

lazy_static! {
    static ref PARENTHESES_RE: Regex = Regex::new(r"(^\[.*\]$|^\{.*\}$)").unwrap();
}

#[inline]
pub fn parse_query_string(qs: &[u8], separator: char) -> Vec<(String, String)> {
    String::from_utf8(qs.to_vec())
        .unwrap_or_default()
        .replace('+', " ")
        .split(separator)
        .filter_map(|value| {
            if !value.is_empty() {
                return match decode(value).unwrap_or_default().split_once('=') {
                    Some((key, value)) => Some((key.to_owned(), value.to_owned())),
                    None => Some((value.to_owned(), String::from(""))),
                };
            }
            None
        })
        .collect::<Vec<(String, String)>>()
}

#[inline]
fn decode_value(json_str: String, parse_numbers: bool) -> Value {
    if PARENTHESES_RE.is_match(json_str.as_str()) {
        let result: Value = match from_str(json_str.as_str()) {
            Ok(value) => value,
            Err(_) => match from_str(json_str.replace('\'', "\"").as_str()) {
                Ok(normalized) => normalized,
                Err(_) => Value::Null,
            },
        };
        return result;
    }

    let normalized = json_str.replace('"', "");
    let json_boolean = normalized.parse::<bool>();
    let json_null = Ok::<_, Infallible>(normalized == "null");

    if parse_numbers {
        let json_integer = normalized.parse::<i64>();
        let json_float = normalized.parse::<f64>();
        return match (json_integer, json_float, json_boolean, json_null) {
            (Ok(json_integer), _, _, _) => Value::from(json_integer),
            (_, Ok(json_float), _, _) => Value::from(json_float),
            (_, _, Ok(json_boolean), _) => Value::from(json_boolean),
            (_, _, _, Ok(true)) => Value::Null,
            _ => Value::from(normalized),
        };
    }

    match (json_boolean, json_null) {
        (Ok(json_boolean), _) => Value::from(json_boolean),
        (_, Ok(true)) => Value::Null,
        _ => Value::from(normalized),
    }
}

#[inline]
pub fn parse_query_string_to_json(bs: &[u8], parse_numbers: bool) -> Value {
    let mut array_map: FxHashMap<String, Vec<Value>> = FxHashMap::default();

    for (key, value) in parse_query_string(bs, '&') {
        match array_map.get_mut(&key) {
            Some(entry) => {
                entry.push(decode_value(value, parse_numbers));
            }
            None => {
                array_map.insert(key, vec![decode_value(value, parse_numbers)]);
            }
        }
    }

    array_map
        .iter()
        .map(|(key, value)| {
            if value.len() == 1 {
                (key, value[0].to_owned())
            } else {
                (key, Value::Array(value.to_owned()))
            }
        })
        .collect::<Value>()
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_string, Value};

    use super::*;

    fn eq_str(value: Value, string: &str) {
        assert_eq!(&to_string(&value).unwrap_or_default(), string)
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
    fn parse_query_string_to_json_parses_simple_string() {
        eq_str(parse_query_string_to_json(b"0=foo", true), r#"{"0":"foo"}"#);
    }

    #[test]
    fn parse_query_string_to_json_transforms_standalone_keys() {
        eq_str(
            parse_query_string_to_json(b"foo=bar&baz", true),
            r#"{"baz":"","foo":"bar"}"#,
        );
    }

    #[test]
    fn parse_query_string_to_json_doesnt_produce_empty_keys() {
        assert_eq!(parse_query_string_to_json(b"_r=1&", true), json!({"_r": 1}));
    }

    #[test]
    fn parse_query_string_to_json_parses_plus_sign() {
        eq_str(
            parse_query_string_to_json(b"a=b%20c+d%2B", true),
            r#"{"a":"b c d+"}"#,
        );
    }

    #[test]
    fn parse_query_string_to_json_parses_numbers_if_parse_numbers_is_true() {
        assert_eq!(parse_query_string_to_json(b"a=1", true), json!({"a": 1}));
        assert_eq!(
            parse_query_string_to_json(b"a=1.1", true),
            json!({"a": 1.1})
        );
        assert_eq!(
            parse_query_string_to_json(b"a=1.1e1", true),
            json!({"a": 11.0})
        );
        assert_eq!(
            parse_query_string_to_json(b"a=1.1e-1", true),
            json!({"a": 0.11})
        );
    }

    #[test]
    fn parse_query_string_to_json_parses_numbers_to_string_if_parse_numbers_is_false() {
        assert_eq!(parse_query_string_to_json(b"a=1", false), json!({"a": "1"}));
        assert_eq!(
            parse_query_string_to_json(b"a=1.1", false),
            json!({"a": "1.1"})
        );
        assert_eq!(
            parse_query_string_to_json(b"a=1.1e1", false),
            json!({"a": "1.1e1"})
        );
        assert_eq!(
            parse_query_string_to_json(b"a=1.1e-1", false),
            json!({"a": "1.1e-1"})
        );
    }

    #[test]
    fn parse_query_string_to_json_parses_booleans() {
        assert_eq!(
            parse_query_string_to_json(b"a=true", false),
            json!({"a": true})
        );
        assert_eq!(
            parse_query_string_to_json(b"a=false", false),
            json!({"a": false})
        );
    }

    #[test]
    fn parse_query_string_to_json_parses_nested_objects() {
        assert_eq!(
            parse_query_string_to_json(
                r#"a={"first": 1, "second": 2, "third": "abc"}"#.as_bytes(),
                true
            ),
            json!({"a": {"first": 1, "second": 2, "third": "abc"}})
        );
    }

    #[test]
    fn parse_query_string_to_json_parses_nested_arrays() {
        assert_eq!(
            parse_query_string_to_json(r#"a=[1,2,3,"abc"]"#.as_bytes(), true),
            json!({"a": [1,2,3,"abc"]})
        );
    }

    #[test]
    fn parse_query_string_to_json_parses_null() {
        assert_eq!(
            parse_query_string_to_json(b"a=null", true),
            json!({ "a": null })
        );
    }

    #[test]
    fn parse_query_string_to_json_parses_empty_string() {
        assert_eq!(parse_query_string_to_json(b"a=", true), json!({ "a": "" }));
    }

    #[test]
    fn parse_query_string_to_json_parses_a_list_of_values() {
        assert_eq!(
            parse_query_string_to_json(b"a=1&a=2&a=3", true),
            json!({ "a": [1,2,3] })
        );
    }

    #[test]
    fn parse_query_string_to_json_parses_random_values() {
        let result = parse_query_string_to_json(b"_id=637ca2c6a8178b1d6aab4140&index=0&guid=92d50031-11ee-4756-af59-cd47a45082e7&isActive=false&balance=%242%2C627.33&picture=http%3A%2F%2Fplacehold.it%2F32x32&age=36&eyeColor=blue&name=Colette+Suarez&gender=female&company=ZENTILITY&email=colettesuarez%40zentility.com&phone=%2B1+%28841%29+509-2669&address=400+Polar+Street%2C+Emory%2C+Palau%2C+3376&about=Deserunt+nostrud+quis+enim+fugiat+labore+labore+sint+deserunt+aliquip+est+fugiat+mollit+commodo.+Labore+pariatur+laboris+ut+irure+voluptate+aliqua+non+ex+enim.+Dolor+ea+mollit+dolore+anim+eu+velit+labore+aliquip+laborum+irure+duis+aliqua+sunt+sint.+Ex+elit+ea+irure+nisi+qui+exercitation+ullamco+occaecat+eu+culpa+magna+quis+dolor+dolor.+Officia+nostrud+consectetur+exercitation+consequat+qui+est+dolore+cillum+dolor+minim+tempor.%0D%0A&registered=2015-12-11T05%3A34%3A25+-01%3A00&latitude=-14.326509&longitude=-32.417451&tags=qui&tags=occaecat&tags=quis&tags=minim&tags=aliquip&tags=sunt&tags=pariatur&friends=%7B%27id%27%3A+0%2C+%27name%27%3A+%27Flora+Phelps%27%7D&friends=%7B%27id%27%3A+1%2C+%27name%27%3A+%27Coffey+Warner%27%7D&friends=%7B%27id%27%3A+2%2C+%27name%27%3A+%27Lyons+Mccall%27%7D&greeting=Hello%2C+Colette+Suarez%21+You+have+4+unread+messages.&favoriteFruit=banana", true);
        assert_eq!(
            result,
            json!({
                    "_id": "637ca2c6a8178b1d6aab4140",
                    "about": "Deserunt nostrud quis enim fugiat labore labore sint deserunt aliquip est fugiat mollit commodo. Labore pariatur laboris ut irure voluptate aliqua non ex enim. Dolor ea mollit dolore anim eu velit labore aliquip laborum irure duis aliqua sunt sint. Ex elit ea irure nisi qui exercitation ullamco occaecat eu culpa magna quis dolor dolor. Officia nostrud consectetur exercitation consequat qui est dolore cillum dolor minim tempor.\r\n",
                    "address": "400 Polar Street, Emory, Palau, 3376",
                    "age": 36,
                    "balance": "$2,627.33",
                    "company": "ZENTILITY",
                    "email": "colettesuarez@zentility.com",
                    "eyeColor": "blue",
                    "favoriteFruit": "banana",
                    "friends": [
                        {
                            "id":0,
                            "name": "Flora Phelps"
                        },
                        {
                            "id":1,
                            "name": "Coffey Warner"
                        },
                        {
                            "id":2,
                            "name": "Lyons Mccall"
                        }
                    ],
                    "gender": "female",
                    "greeting": "Hello, Colette Suarez! You have 4 unread messages.",
                    "guid": "92d50031-11ee-4756-af59-cd47a45082e7",
                    "index": 0,
                    "isActive": false,
                    "latitude": -14.326509,
                    "longitude": -32.417451,
                    "name": "Colette Suarez",
                    "phone":"+1 (841) 509-2669",
                    "picture":"http://placehold.it/32x32",
                    "registered":"2015-12-11T05:34:25 -01:00",
                    "tags": [
                        "qui",
                        "occaecat",
                        "quis",
                        "minim",
                        "aliquip",
                        "sunt",
                        "pariatur"
                    ]
                }
            )
        )
    }
}
