from typing import Any

def parse_query_string(qs: bytes, separator: str) -> list[tuple[str, str]]: ...

"""Parse a query string into a list of tuples.

Args:
    qs: a query byte-string.
    separator: A separator symbol.

Returns:
    A list of string/string tuples.
"""

def parse_url_encoded_dict(qs: bytes, parse_numbers: bool = False) -> dict[str, Any]: ...

"""Parse a query string into a dictionary of values.

Args:
    qs: a query byte-string.
    parse_numbers: If true, parse numbers into ints and floats otherwise keep them as strings.

Returns:
    A string keyed dictionary of parsed values.
"""

__all__ = ("parse_query_string", "parse_url_encoded_dict")
