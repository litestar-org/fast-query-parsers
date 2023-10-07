============
Contributing
============

All contributions are of course welcome!

Repository Setup
~~~~~~~~~~~~~~~~

1. Run ``cargo build`` to set up the rust dependencies and ``pdm install`` to set up the python dependencies.
2. Install the pre-commit hooks with ``pre-commit install`` (requires `pre-commit <https://pre-commit.com/>`_).

Building
~~~~~~~~

Run ``pdm run maturin develop --release --strip`` to install a release wheel (without debugging info).
This wheel can be used in tests and benchmarks.

Benchmarking
~~~~~~~~~~~~

There are basic benchmarks using pyperf in place.
To run these execute ``pdm run python benchmarks.py``.
