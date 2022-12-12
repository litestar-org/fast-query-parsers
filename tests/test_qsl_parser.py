from urllib.parse import parse_qsl as stdlib_parse_qsl

import pytest
from fast_query_parser import parse_qsl as fast_parse_qsl


@pytest.mark.parametrize(
    "qs, expected",
    [
        ("", []),
        ("&", []),
        ("&&", []),
        ("=", [("", "")]),
        ("=a", [("", "a")]),
        ("a", [("a", "")]),
        ("a=", [("a", "")]),
        ("&a=b", [("a", "b")]),
        ("a=a+b&b=b+c", [("a", "a b"), ("b", "b c")]),
        ("a=1&a=2", [("a", "1"), ("a", "2")]),
        ("", []),
        ("&", []),
        ("&&", []),
        ("=", [("", "")]),
        ("=a", [("", "a")]),
        ("a", [("a", "")]),
        ("a=", [("a", "")]),
        ("&a=b", [("a", "b")]),
        ("a=a+b&b=b+c", [("a", "a b"), ("b", "b c")]),
        ("a=1&a=2", [("a", "1"), ("a", "2")]),
        (";a=b", [(";a", "b")]),
        ("a=a+b;b=b+c", [("a", "a b;b=b c")]),
        (";a=b", [(";a", "b")]),
        ("a=a+b;b=b+c", [("a", "a b;b=b c")]),
    ],
)
def test_parse_qsl_standard_separator(qs: str, expected: list[tuple[str, str]]) -> None:
    result = fast_parse_qsl(qs, "&")
    assert result == stdlib_parse_qsl(qs, keep_blank_values=True) == expected


@pytest.mark.parametrize(
    "qs, expected",
    [
        (";", []),
        (";;", []),
        (";a=b", [("a", "b")]),
        ("a=a+b;b=b+c", [("a", "a b"), ("b", "b c")]),
        ("a=1;a=2", [("a", "1"), ("a", "2")]),
        (";", []),
        (";;", []),
        (";a=b", [("a", "b")]),
        ("a=a+b;b=b+c", [("a", "a b"), ("b", "b c")]),
        ("a=1;a=2", [("a", "1"), ("a", "2")]),
    ],
)
def test_parse_qsl_semicolon_separator(qs: str, expected: list[tuple[str, str]]) -> None:
    result = fast_parse_qsl(qs, ";")
    assert result == stdlib_parse_qsl(qs, separator=";", keep_blank_values=True) == expected
