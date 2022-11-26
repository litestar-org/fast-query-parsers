from urllib.parse import parse_qsl as stdlib_parse_qsl

import pyperf
from fast_query_parser import parse_qsl as fast_parse_qsl


def main():
    runner = pyperf.Runner()
    runner.bench_func(
        "benchmark stdlib query parser",
        lambda: stdlib_parse_qsl("key=1&key=2&key=3&another=a&zorg=5=", keep_blank_values=True),
    )
    runner.bench_func(
        "benchmark fast query parser", lambda: fast_parse_qsl(b"key=1&key=2&key=3&another=a&zorg=5=", "&")
    )


if __name__ == "__main__":
    main()
