# Starlite HTTP Utils

This library includes python utilities that use rust based native bindings. The purpose of these utilities is to offer
superior performance to the native python implementations.

## Repository Setup

1. Run `cargo install` to setup the rust dependencies and `poetry install` to setup the python dependencies.
2. Install the pre-commit hooks with `pre-commit install` (requires [pre-commit](https://pre-commit.com/)).

## Building

Run `poetry run maturin build --release --strip` to create a release wheel (without debugging info). This wheel can be used in tests and benchmarks.

## Benchmarking

There are basic benchmarks using pyperf in place. To run these execute `poetry run python benchrmarks.py`.

Note: the benchmarks for `parse_qs` are not accurate because this lib's `parse_qs` also parses strings into their respective
types, whereas the stdlib version just creates a dict of values.

## Contributing

- All PRs are welcome.
