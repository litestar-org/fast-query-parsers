from urllib.parse import parse_qsl as stdlib_parse_qsl
from urllib.parse import urlencode

import pyperf
from fast_query_parser import parse_qsl as fast_parse_qsl


def main():
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
    )

    runner = pyperf.Runner()
    runner.bench_func(
        "stdlib non-urlencoded-qs parsing",
        lambda: stdlib_parse_qsl("key=1&key=2&key=3&another=a&zorg=5=", keep_blank_values=True),
    )
    runner.bench_func(
        "fast_parse_qsl non-urlencoded-qs parsing", lambda: fast_parse_qsl("key=1&key=2&key=3&another=a&zorg=5=", "&")
    )
    runner.bench_func(
        "stdlib urlencoded-qs parsing",
        lambda: stdlib_parse_qsl(url_encoded_query, keep_blank_values=True),
    )
    runner.bench_func("parse_url_encoded urlencoded-qs parsing", lambda: fast_parse_qsl(url_encoded_query, "&"))


if __name__ == "__main__":
    main()
