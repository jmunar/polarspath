.PHONY: all clean test build format check update-deps

CARGOS := structpath_types structpath_derive structpath protobuf_sample protobuf_sample_polars

all: format check test build
all-rust: format-rust check-rust test-rust build-rust

install-uv:
	@curl -LsSf https://astral.sh/uv/install.sh | sh

build-rust:
	@set -e; for cargo in $(CARGOS); do \
		echo "Building Rust $$cargo..."; \
		(cd $$cargo && cargo build --release) || exit 1; \
	done

build-python:
	@echo "Building Python protobuf sample package..."
	@mkdir -p protobuf_sample_polars/protobuf_sample_polars/sample && \
		protoc \
			-I=protobuf_sample/protobuf/sample \
			--python_out=protobuf_sample_polars/protobuf_sample_polars/sample \
			protobuf_sample/protobuf/sample/*.proto
	@echo "Building Python protobuf_sample_polars package..."
	@cd protobuf_sample_polars && uv run maturin develop --release

build: build-rust build-python

format-rust:
	@set -e; for cargo in $(CARGOS); do \
		echo "Format fix in Rust $$cargo..."; \
		(cd $$cargo && cargo fmt) || exit 1; \
	done

format-python:
	@echo "Format fix in Python protobuf_sample_polars..."
	@cd protobuf_sample_polars && uv run ruff format --exclude sample

format: format-rust format-python

check-rust:
	@set -e; for cargo in $(CARGOS); do \
		echo "Format check in Rust $$cargo..."; \
		(cd $$cargo && cargo clippy -- -D warnings) || exit 1; \
	done

check-python:
	@echo "Format check in Python protobuf_sample_polars..."
	@cd protobuf_sample_polars && uv run ruff check --exclude sample

check: check-rust check-python

update-deps:
	@set -e; for cargo in $(CARGOS); do \
		echo "Updating dependencies for Rust $$cargo..."; \
		(cd $$cargo && cargo update) || exit 1; \
	done

test-rust:
	@set -e; for cargo in $(CARGOS); do \
		echo "Running tests for Rust $$cargo..."; \
		(cd $$cargo && cargo test) || exit 1; \
		if [ "$$cargo" = "structpath" ]; then \
			echo "Running tests for Rust $$cargo with derive feature..."; \
			(cd $$cargo && cargo test --features derive) || exit 1; \
		fi \
	done

test: test-rust

clean-rust:
	@set -e; for cargo in $(CARGOS); do \
		echo "Cleaning Rust $$cargo..."; \
		(cd $$cargo && cargo clean) || exit 1; \
		rm -rf $$cargo/target; \
	done

clean-python:
	@echo "Cleaning Python protobuf sample package..."
	@rm -rf protobuf_sample_polars/protobuf_sample_polars/sample

clean: clean-rust clean-python

help:
	@echo "Available commands:"
	@echo "  make build            - Build all Rust crates and the Python package"
	@echo "  make build-rust       - Build all Rust crates only"
	@echo "  make build-python     - Build the Python package only"
	@echo "  make test             - Run tests in all Rust crates"
	@echo "  make test-rust        - Run tests in all Rust crates only"
	@echo "  make format           - Format Rust and Python code"
	@echo "  make format-rust      - Format Rust code only"
	@echo "  make format-python    - Format Python code only"
	@echo "  make check            - Run all linters and checks for Rust and Python"
	@echo "  make check-rust       - Run clippy linter on all Rust crates only"
	@echo "  make check-python     - Run all Python linters and checks only"
	@echo "  make update-deps      - Update dependencies to latest versions in all Rust crates"
	@echo "  make clean            - Clean build artifacts for all crates"
	@echo "  make clean-rust       - Clean Rust build artifacts only"
	@echo "  make clean-python     - Clean Python build artifacts only"
	@echo "  make help             - Show this help message" 
