from urllib.parse import parse_qs as stdlib_parse_qs
from urllib.parse import parse_qsl as stdlib_parse_qsl
from urllib.parse import urlencode

import pyperf

from fast_query_parser import parse_qs as fast_parse_qs
from fast_query_parser import parse_qsl as fast_parse_qsl

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


def bench_qsl(runner: pyperf.Runner):
    runner.bench_func(
        "stdlib non-urlencoded-qs parsing bytes intro tuple of strings",
        lambda: stdlib_parse_qsl(b"key=1&key=2&key=3&another=a&zorg=5=".decode(), keep_blank_values=True),
    )
    runner.bench_func(
        "stdlib non-urlencoded-qs parsing string into tuple of strings",
        lambda: stdlib_parse_qsl(
            "key=1&key=2&key=3&another=a&zorg=5=",
            keep_blank_values=True,
        ),
    )
    runner.bench_func(
        "fast_parse_qsl non-urlencoded-qs parsing bytes into tuple of strings",
        lambda: fast_parse_qsl(b"key=1&key=2&key=3&another=a&zorg=5=", "&"),
    )
    runner.bench_func(
        "stdlib urlencoded bytes parsing",
        lambda: stdlib_parse_qsl(url_encoded_query.decode(), keep_blank_values=True),
    )
    runner.bench_func("parse_url_encoded urlencoded bytes parsing", lambda: fast_parse_qsl(url_encoded_query, "&"))


def bench_qs(runner: pyperf.Runner):
    runner.bench_func(
        "stdlib parse_qs parsing into dict",
        lambda: stdlib_parse_qs(url_encoded_query.decode()),
    )
    runner.bench_func("fast_parse_qs parsing into dict", lambda: fast_parse_qs(url_encoded_query))


if __name__ == "__main__":
    runner = pyperf.Runner()

    bench_qsl(runner)
    bench_qs(runner)
