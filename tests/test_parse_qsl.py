from typing import List, Tuple
from urllib.parse import parse_qsl, urlencode

import pytest
from fast_query_parsers import parse_query_string


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
def test_parse_qsl_standard_separator(qs: str, expected: List[Tuple[str, str]]) -> None:
    result = parse_query_string(qs.encode(), "&")
    assert result == parse_qsl(qs, keep_blank_values=True) == expected


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
def test_parse_qsl_semicolon_separator(qs: str, expected: List[Tuple[str, str]]) -> None:
    result = parse_query_string(qs.encode(), ";")
    assert result == parse_qsl(qs, separator=";", keep_blank_values=True) == expected


@pytest.mark.parametrize(
    "values",
    (
        (("first", "x@test.com"), ("second", "aaa")),
        (("first", "&@A.ac"), ("second", "aaa")),
        (("first", "a@A.ac&"), ("second", "aaa")),
        (("first", "a@A&.ac"), ("second", "aaa")),
    ),
)
def test_query_parsing_of_escaped_values(values: Tuple[Tuple[str, str], Tuple[str, str]]) -> None:
    url_encoded = urlencode(values)
    assert parse_query_string(url_encoded.encode(), "&") == list(values)


def test_parses_non_ascii_text() -> None:
    assert parse_query_string("arabic_text=اختبار اللغة العربية".encode(), "&") == [
        ("arabic_text", "اختبار اللغة العربية")
    ]
