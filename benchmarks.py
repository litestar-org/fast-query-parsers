from collections import defaultdict
from contextlib import suppress
from json import JSONDecodeError, loads
from typing import Any, DefaultDict, Dict, List
from urllib.parse import parse_qs as stdlib_parse_qs
from urllib.parse import parse_qsl as stdlib_parse_qsl
from urllib.parse import urlencode

import pyperf
from fast_query_parsers import parse_query_string, parse_url_encoded_dict

url_encoded_query = urlencode(
    [
        ("value", "10"),
        ("value", "12"),
        ("veggies", '["tomato", "potato", "aubergine"]'),
        ("nested", '{"some_key": "some_value"}'),
        ("calories", "122.53"),
        ("healthy", True),
        ("polluting", False),
    ]
).encode()


def parse_url_encoded_form_data(encoded_data: bytes) -> Dict[str, Any]:
    """Parse an url encoded form data dict.
    Args:
        encoded_data: The encoded byte string.
        encoding: The encoding used.
    Returns:
        A parsed dict.
    """
    decoded_dict: DefaultDict[str, List[Any]] = defaultdict(list)
    for k, v in stdlib_parse_qsl(encoded_data.decode(), keep_blank_values=True):
        with suppress(JSONDecodeError):
            v = loads(v) if isinstance(v, str) else v
        decoded_dict[k].append(v)
    return {k: v if len(v) > 1 else v[0] for k, v in decoded_dict.items()}


def bench_qsl(runner: pyperf.Runner):
    runner.bench_func(
        "stdlib parse_qsl parsing query string",
        lambda: stdlib_parse_qsl(b"key=1&key=2&key=3&another=a&zorg=5=".decode(), keep_blank_values=True),
    )
    runner.bench_func(
        "parse_query_string parsing query string",
        lambda: parse_query_string(b"key=1&key=2&key=3&another=a&zorg=5=", "&"),
    )
    runner.bench_func(
        "stdlib parse_qsl parsing urlencoded query string",
        lambda: stdlib_parse_qsl(url_encoded_query.decode(), keep_blank_values=True),
    )
    runner.bench_func("parse_query_string urlencoded query string", lambda: parse_query_string(url_encoded_query, "&"))


def bench_qs(runner: pyperf.Runner):
    runner.bench_func(
        "stdlib parse_qs parsing url-encoded values into dict",
        lambda: stdlib_parse_qs(url_encoded_query.decode()),
    )
    runner.bench_func(
        "python parse_url_encoded_form_data parsing url-encoded values into dict",
        lambda: parse_url_encoded_form_data(url_encoded_query),
    )
    runner.bench_func(
        "parse_url_encoded_dict parsing url-encoded values into dict", lambda: parse_url_encoded_dict(url_encoded_query)
    )


if __name__ == "__main__":
    runner = pyperf.Runner()

    bench_qsl(runner)
    bench_qs(runner)
