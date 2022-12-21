from typing import List, Tuple
from urllib.parse import parse_qsl as stdlib_parse_qsl
from urllib.parse import urlencode

import pytest
from starlite_http_utils import parse_qsl as fast_parse_qsl


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
def test_parse_qsl_semicolon_separator(qs: str, expected: List[Tuple[str, str]]) -> None:
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
def test_query_parsing_of_escaped_values(values: Tuple[Tuple[str, str], Tuple[str, str]]) -> None:
    url_encoded = urlencode(values)
    assert fast_parse_qsl(url_encoded.encode(), "&") == list(values)


def test_parses_non_ascii_text() -> None:
    assert fast_parse_qsl("arabic_text=اختبار اللغة العربية".encode(), "&") == [("arabic_text", "اختبار اللغة العربية")]
