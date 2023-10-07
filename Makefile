SHELL := /bin/bash
# =============================================================================
# Variables
# =============================================================================

.DEFAULT_GOAL:=help
.ONESHELL:
USING_PDM       =  $(shell grep "tool.pdm" pyproject.toml && echo "yes")
USING_CARGO	    =  $(shell stat Cargo.toml > /dev/null && echo "yes")
ENV_PREFIX      := $(shell if [ -d .venv ]; then echo ".venv/bin/"; fi)
VENV_EXISTS     := $(shell if [ -d .venv ]; then echo "yes"; fi)
PDM_OPTS        ?=
PDM             ?= pdm $(PDM_OPTS)
CARGO_OPTS      ?=
CARGO           ?= cargo $(CARGO_OPTS)

.EXPORT_ALL_VARIABLES:

.PHONY: help
help: 		   										## Display this help text for Makefile
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

.PHONY: upgrade
upgrade:       										## Upgrade all dependencies to the latest stable versions
	@echo "=> Updating all dependencies"
	@if [ "$(USING_PDM)" ]; then $(PDM) update; fi
	@if [ "$(USING_CARGO)" ]; then $(CARGO) update; fi
	@echo "=> Dependencies Updated"
	@$(ENV_PREFIX)pre-commit autoupdate
	@echo "=> Updated Pre-commit"

# =============================================================================
# Developer Utils
# =============================================================================
.PHONY: install-pdm
install-pdm: 										## Install latest version of PDM
	@curl -sSLO https://pdm.fming.dev/install-pdm.py && \
	@curl -sSL https://pdm.fming.dev/install-pdm.py.sha256 | shasum -a 256 -c - && \
	python3 install-pdm.py

.PHONE: install-rust
install-rust: 										## Install Rust
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

.PHONY: install
install:											## Install the project, dependencies, and pre-commit for local development
	@if ! $(PDM) --version > /dev/null; then echo '=> Installing PDM'; $(MAKE) install-pdm; fi
	@if ! $(CARGO) --version > /dev/null; then echo '=> Installing Rust'; $(MAKE) install-rust; fi
	@if [ "$(VENV_EXISTS)" ]; then echo "=> Removing existing virtual environment"; fi
	@if [ "$(VENV_EXISTS)" ]; then $(MAKE) destroy; fi
	@if [ "$(VENV_EXISTS)" ]; then $(MAKE) clean; fi
	@if [ "$(USING_PDM)" ]; then $(PDM) config venv.in_project true && python3 -m venv --copies .venv && . $(ENV_PREFIX)/activate && $(ENV_PREFIX)/pip install --quiet -U wheel setuptools cython pip; fi
	@if [ "$(USING_PDM)" ]; then $(PDM) install -G:all; fi
	@if [ "$(USING_CARGO)" ]; then $(CARGO) build; fi
	@echo "=> Install complete! Note: If you want to re-install re-run 'make install'"

.PHONY: clean
clean: 												## Cleanup temporary build artifacts
	@echo "=> Cleaning working directory"
	@rm -rf .pytest_cache .ruff_cache .hypothesis build/ -rf dist/ .eggs/
	@cargo clean
	@find . -name '*.egg-info' -exec rm -rf {} +
	@find . -name '*.egg' -exec rm -f {} +
	@find . -name '*.pyc' -exec rm -f {} +
	@find . -name '*.pyo' -exec rm -f {} +
	@find . -name '*~' -exec rm -f {} +
	@find . -name '__pycache__' -exec rm -rf {} +
	@find . -name '.ipynb_checkpoints' -exec rm -rf {} +
	@rm -rf .coverage coverage.xml coverage.json htmlcov/ .pytest_cache tests/.pytest_cache tests/**/.pytest_cache .mypy_cache
	$(MAKE) docs-clean

.PHONY: destroy
destroy: 											## Destroy the virtual environment
	@echo "=> Destroying virtual environment"
	@rm -rf .venv

.PHONY: lock-refresh
lock-refresh:                                        ## Sync lockfiles with requirements files.
	@echo "=> Refreshing lockfiles"
	@pdm update --update-reuse --group :all
	@cargo update
	@echo "=> 🔒 Lockfiles refreshed"

.PHONY: lock
lock:                                               ## Rebuild lockfiles from scratch, updating all dependencies
	@echo "=> Rebuilding lockfiles"
	@pdm update --update-eager --group :all
	@cargo generate-lockfile
	@echo "=> 🔒 Lockfiles built"

.PHONE: build
build:												## Install a release wheel for testing and benchmarking
	@echo "WARN: This currently does not work. Please run 'pdm run maturin develop --release --strip' manually."
	@#pdm run pd

# =============================================================================
# Tests, Linting, Coverage, Benchmarking
# =============================================================================
.PHONY: lint
lint: 												## Runs pre-commit hooks; includes ruff linting, codespell, black
	@echo "=> Running pre-commit process"
	@$(ENV_PREFIX)pre-commit run --all-files
	@echo "=> Linting complete"

.PHONY: coverage
coverage:  											## Run the tests and generate coverage report
	@echo "=> Running tests with coverage"
	@$(ENV_PREFIX)pytest tests --cov=fast_query_parsers.pyi
	@$(ENV_PREFIX)coverage html
	@$(ENV_PREFIX)coverage xml
	@echo "=> Coverage report generated"

.PHONY: test
test:  												## Run the tests
	@echo "=> Running test cases"
	@$(ENV_PREFIX)pytest tests
	@cargo test
	@echo "=> Tests complete"

.PHONY: test-all
test-all: test   									## Run all tests

.PHONY: check-all
check-all: lint test-all coverage 					## Run all linting, tests, and coverage checks

.PHONE: benchmark
benchmark: 										## Run the benchmarks
	@pdm run python benchmarks.py

# =============================================================================
# Docs
# =============================================================================
.PHONY: docs-install
docs-install: 										## Install docs dependencies
	@echo "=> Installing documentation dependencies"
	@$(PDM) install --group docs
	@echo "=> Installed documentation dependencies"

docs-clean: 										## Dump the existing built docs
	@echo "=> Cleaning documentation build assets"
	@rm -rf docs/_build
	@echo "=> Removed existing documentation build assets"

docs-serve: docs-clean 								## Serve the docs locally
	@echo "=> Serving documentation"
	$(ENV_PREFIX)sphinx-autobuild docs docs/_build/ -j auto --watch fast_query_parser.pyi --watch docs --watch tests --watch CONTRIBUTING.rst --watch README.rst --port 8002

docs: docs-clean 									## Dump the existing built docs and rebuild them
	@echo "=> Building documentation"
	@$(ENV_PREFIX)sphinx-build -M html docs docs/_build/ -E -a -j auto --keep-going
