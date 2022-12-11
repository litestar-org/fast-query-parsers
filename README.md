# Fast Query Parser

This project is a rust based query string parser for python. It uses PYO3 and Maturin to create native rust binding in python.

## Setup

1. Run `cargo install` to setup the rust dependencies and `poetry install` to setup the python dependencies.
2. Install the pre-commit hooks with `pre-commit install` (requires [pre-commit](https://pre-commit.com/)).

## Building

1. Run `poetry run maturin build --release --strip` to create a release grade wheel. This wheel can be used in tests and benchmarks.
2. Run `poetry run python benchrmarks.py` to execute the python benchmarks.

Contributing

- All PRs are welcome.
