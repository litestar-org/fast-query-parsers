<!-- markdownlint-disable -->
<p align="center">
  <img src="https://github.com/litestar-org/branding/blob/473f54621e55cde9acbb6fcab7fc03036173eb3d/assets/Branding%20-%20SVG%20-%20Transparent/Logo%20-%20Banner%20-%20Inline%20-%20Light.svg#gh-light-mode-only" alt="Litestar Logo - Light" width="100%" height="auto" />
  <img src="https://github.com/litestar-org/branding/blob/473f54621e55cde9acbb6fcab7fc03036173eb3d/assets/Branding%20-%20SVG%20-%20Transparent/Logo%20-%20Banner%20-%20Inline%20-%20Dark.svg#gh-dark-mode-only" alt="Litestar Logo - Dark" width="100%" height="auto" />
</p>
<!-- markdownlint-restore -->

<div align="center">

<!-- prettier-ignore-start -->

| Project   |     | Status                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
|-----------|:----|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| CI/CD     |     | [![Publish](https://github.com/litestar-org/fast-query-parsers/actions/workflows/publish.yaml/badge.svg)](https://github.com/litestar-org/fast-query-parsers/actions/workflows/publish.yaml) [![CI](https://github.com/litestar-org/fast-query-parsers/actions/workflows/ci.yaml/badge.svg)](https://github.com/litestar-org/fast-query-parsers/actions/workflows/ci.yaml)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Package   |     | [![PyPI - Version](https://img.shields.io/pypi/v/fast-query-parsers?labelColor=202235&color=edb641&logo=python&logoColor=edb641)](https://badge.fury.io/py/litestar) ![PyPI - Support Python Versions](https://img.shields.io/pypi/pyversions/fast-query-parsers?labelColor=202235&color=edb641&logo=python&logoColor=edb641) ![PyPI - Downloads](https://img.shields.io/pypi/dm/fast-query-parsers?logo=python&label=fast-query-parsers%20downloads&labelColor=202235&color=edb641&logoColor=edb641)                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Community |     | [![Reddit](https://img.shields.io/reddit/subreddit-subscribers/litestarapi?label=r%2FLitestar&logo=reddit&labelColor=202235&color=edb641&logoColor=edb641)](https://reddit.com/r/litestarapi) [![Discord](https://img.shields.io/discord/919193495116337154?labelColor=202235&color=edb641&label=chat%20on%20discord&logo=discord&logoColor=edb641)](https://discord.gg/X3FJqy8d2j) [![Matrix](https://img.shields.io/badge/chat%20on%20Matrix-bridged-202235?labelColor=202235&color=edb641&logo=matrix&logoColor=edb641)](https://matrix.to/#/#litestar:matrix.org) [![Medium](https://img.shields.io/badge/Medium-202235?labelColor=202235&color=edb641&logo=medium&logoColor=edb641)](https://blog.litestar.dev) [![Twitter](https://img.shields.io/twitter/follow/LitestarAPI?labelColor=202235&color=edb641&logo=twitter&logoColor=edb641&style=flat)](https://twitter.com/LitestarAPI) [![Blog](https://img.shields.io/badge/Blog-litestar.dev-202235?logo=blogger&labelColor=202235&color=edb641&logoColor=edb641)](https://blog.litestar.dev) |
| Meta      |     | [![Litestar Project](https://img.shields.io/badge/Litestar%20Org-%E2%AD%90%20Fast%20Query%20Parsers-202235.svg?logo=python&labelColor=202235&color=edb641&logoColor=edb641)](https://github.com/litestar-org/fast-query-parsers) [![License - MIT](https://img.shields.io/badge/license-MIT-202235.svg?logo=python&labelColor=202235&color=edb641&logoColor=edb641)](https://spdx.org/licenses/) [![Litestar Sponsors](https://img.shields.io/badge/Sponsor-%E2%9D%A4-%23edb641.svg?&logo=github&logoColor=edb641&labelColor=202235)](https://github.com/sponsors/litestar-org) [![linting - Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/charliermarsh/ruff/main/assets/badge/v2.json&labelColor=202235)](https://github.com/astral-sh/ruff) [![code style - Black](https://img.shields.io/badge/code%20style-black-000000.svg?logo=python&labelColor=202235&logoColor=edb641)](https://github.com/psf/black)                                                                                                         |

<!-- prettier-ignore-end -->
</div>

# Fast Query Parsers

This library includes ultra-fast Rust based query string and urlencoded parsers. These parsers are used
by [`Litestar`](https://github.com/litestar-org/litestar), but are developed separately - and can of course be used separately.

## Installation

```shell
pip install fast-query-parsers
```

## Usage

The library exposes two function `parse_query_string` and `parse_url_encoded_dict`.

### `parse_query_string`

This function is used to parse a query string into a list of key/value tuples.

```python
from fast_query_parsers import parse_query_string

result = parse_query_string(b"value=1&value=2&type=dollar&country=US", "&")
# [("value", "1"), ("value", "2"), ("type", "dollar"), ("country", "US")]
```

The first argument to this function is a byte string that includes the query string to be parsed, the second argument is
the separator used.

#### Benchmarks

Query string parsing is more than x5 times faster than the standard library:

```text
stdlib parse_qsl parsing query string: Mean +- std dev: 2.86 us +- 0.03 us
.....................
parse_query_string parsing query string: Mean +- std dev: 916 ns +- 13 ns
.....................
stdlib parse_qsl parsing urlencoded query string: Mean +- std dev: 8.30 us +- 0.10 us
.....................
parse_query_string urlencoded query string: Mean +- std dev: 1.50 us +- 0.03 us
```

### `parse_url_encoded_dict`

This function is used to parse a url-encoded form data dictionary and parse it into the python equivalent of JSON types.

```python
from urllib.parse import urlencode

from fast_query_parsers import parse_url_encoded_dict

encoded = urlencode(
    [
        ("value", "10"),
        ("value", "12"),
        ("veggies", '["tomato", "potato", "aubergine"]'),
        ("nested", '{"some_key": "some_value"}'),
        ("calories", "122.53"),
        ("healthy", "true"),
        ("polluting", "false"),
        ("json", "null"),
    ]
).encode()

result = parse_url_encoded_dict(encoded, parse_numbers=True)

# result == {
#     "value": [10, 12],
#     "veggies": ["tomato", "potato", "aubergine"],
#     "nested": {"some_key": "some_value"},
#     "calories": 122.53,
#     "healthy": True,
#     "polluting": False,
#     "json": None,
# }
```

This function handles type conversions correctly - unlike the standard library function `parse_qs`. Additionally, it
does not nest all values inside lists.

Note: the second argument passed to `parse_url_encoded_dict` dictates whether numbers should be parsed. If `True`,
the value will be parsed into an int or float as appropriate, otherwise it will be kept as a string.
By default, the value of this arg is `True`.

#### Benchmarks

Url Encoded parsing is more than x2 times faster than the standard library, without accounting for parsing of values:

```text
stdlib parse_qs parsing url-encoded values into dict: Mean +- std dev: 8.99 us +- 0.09 us
.....................
parse_url_encoded_dict parse url-encoded values into dict: Mean +- std dev: 3.77 us +- 0.08 us
```

To actually mimic the parsing done by `parse_url_encoded_dict` we will need a utility along these lines:

```python
from collections import defaultdict
from contextlib import suppress
from json import loads, JSONDecodeError
from typing import Any, DefaultDict, Dict, List
from urllib.parse import parse_qsl


def parse_url_encoded_form_data(encoded_data: bytes) -> Dict[str, Any]:
    """Parse an url encoded form data into dict of parsed values"""
    decoded_dict: DefaultDict[str, List[Any]] = defaultdict(list)
    for k, v in parse_qsl(encoded_data.decode(), keep_blank_values=True):
        with suppress(JSONDecodeError):
            v = loads(v) if isinstance(v, str) else v
        decoded_dict[k].append(v)
    return {k: v if len(v) > 1 else v[0] for k, v in decoded_dict.items()}
```

With the above, the benchmarks looks like so:

```text
python parse_url_encoded_form_data parsing url-encoded values into dict: Mean +- std dev: 19.7 us +- 0.1 us
.....................
parse_url_encoded_dict parsing url-encoded values into dict: Mean +- std dev: 3.69 us +- 0.03 us
```

## Contributing

All contributions are of course welcome!

### Repository Setup

1. Run `cargo build` to set up the rust dependencies and `pdm install` to set up the python dependencies.
2. Install the pre-commit hooks with `pre-commit install` (requires [pre-commit](https://pre-commit.com/)).

### Building

Run `pdm run maturin develop --release --strip` to install a release wheel (without debugging info). This wheel can be
used in tests and benchmarks.

### Benchmarking

There are basic benchmarks using pyperf in place. To run these execute `pdm run python benchmarks.py`.
