from urllib.parse import parse_qsl as stdlib_parse_qsl
from urllib.parse import urlencode

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
    result = fast_parse_qsl(qs.encode(), "&")
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
    result = fast_parse_qsl(qs.encode(), ";")
    assert result == stdlib_parse_qsl(qs, separator=";", keep_blank_values=True) == expected


@pytest.mark.parametrize(
    "values",
    (
        (("first", "x@test.com"), ("second", "aaa")),
        (("first", "&@A.ac"), ("second", "aaa")),
        (("first", "a@A.ac&"), ("second", "aaa")),
        (("first", "a@A&.ac"), ("second", "aaa")),
    ),
)
def test_query_parsing_of_escaped_values(values: tuple[tuple[str, str], tuple[str, str]]) -> None:
    url_encoded = urlencode(values)
    assert fast_parse_qsl(url_encoded.encode(), "&") == list(values)
