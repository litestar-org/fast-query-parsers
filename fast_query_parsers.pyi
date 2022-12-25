from typing import Any, Dict, List, Tuple

def parse_query_string(qs: bytes, separator: str) -> List[Tuple[str, str]]: ...

"""Parse a query string into a list of tuples.

Args:
    qs: a query byte-string.
    separator: A seperator symbol.

Returns:
    A list of string/string tuples.
"""

def parse_url_encoded_dict(qs: bytes) -> Dict[str, Any]: ...

"""Parse a query string into a dictionary of values.

Args:
    qs: a query byte-string.

Returns:
    A string keyed dictionary of parsed values.
"""

__all__ = ("parse_query_string", "parse_url_encoded_dict")
