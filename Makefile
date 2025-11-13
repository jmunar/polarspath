.PHONY: all clean test build format check update-deps

all: format check test examples build-python
all-python: format-python check-python build-python
all-rust: format-rust check-rust test-rust examples-rust

install-uv:
	@curl -LsSf https://astral.sh/uv/install.sh | sh

build-rust:
	@echo "Building Rust workspace..."
	@cargo build --workspace --release

build-python:
	@echo "Building Python protobuf sample package..."
	@mkdir -p structpath_protobuf_example/structpath_protobuf_example/pybindings && \
		protoc \
			-I=structpath_protobuf_example/protobuf/structpath_protobuf_example \
			--python_out=structpath_protobuf_example/structpath_protobuf_example/pybindings \
			structpath_protobuf_example/protobuf/structpath_protobuf_example/*.proto
	@echo "Building Python structpath_protobuf_example package..."
	@cd structpath_protobuf_example && uv run maturin develop --release

build: build-rust build-python

format-rust:
	@echo "Formatting Rust workspace..."
	@cargo fmt --all

format-python:
	@echo "Format fix in Python structpath_protobuf_example..."
	@cd structpath_protobuf_example && uv run ruff format --exclude sample

format: format-rust format-python

check-rust:
	@echo "Running clippy on Rust workspace..."
	@cargo clippy --workspace -- -D warnings

check-python:
	@echo "Format check in Python structpath_protobuf_example..."
	@cd structpath_protobuf_example && uv run ruff check --exclude sample

check: check-rust check-python

update-deps:
	@echo "Updating dependencies for Rust workspace..."
	@cargo update --workspace

test-rust:
	@echo "Running tests for Rust workspace..."
	@cargo test --workspace
	@echo "Running tests for structpath with derive feature..."
	@cargo test -p structpath --features derive

test: test-rust

examples-rust:
	@set -e; \
	workspace_root=$$(pwd); \
	for member in $$(cargo metadata --format-version 1 2>/dev/null | grep '"workspace_members"' | sed 's/.*"workspace_members":\[\([^]]*\)\].*/\1/' | tr ',' '\n' | sed 's/.*path+file:\/\/\([^#]*\).*/\1/' | sed "s|$$workspace_root/||"); do \
		if [ -d "$$member/examples" ]; then \
			crate_name=$$(grep '^name = ' $$member/Cargo.toml 2>/dev/null | head -1 | sed 's/^name = "\(.*\)"/\1/' | tr -d ' '); \
			if [ -n "$$crate_name" ]; then \
				for example in $$member/examples/*.rs; do \
					if [ -f "$$example" ] && [ "$$(basename $$example)" != "readme.rs" ]; then \
						echo "Running example $$example..."; \
						cargo run -p $$crate_name --example $$(basename $$example .rs) || exit 1; \
					fi; \
				done; \
			fi; \
		fi; \
	done

examples: examples-rust

clean-rust:
	@echo "Cleaning Rust workspace..."
	@cargo clean

clean-python:
	@echo "Cleaning Python protobuf sample package..."
	@rm -rf structpath_protobuf_example/structpath_protobuf_example/sample

clean: clean-rust clean-python

help:
	@echo "Available commands:"
	@echo "  make build            - Build all Rust crates and the Python package"
	@echo "  make build-rust       - Build all Rust crates only"
	@echo "  make build-python     - Build the Python package only"
	@echo "  make test             - Run tests in Rust"
	@echo "  make test-rust        - Run tests in Rust only"
	@echo "  make examples         - Run examples in Rust"
	@echo "  make examples-rust    - Run examples in Rust only"
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
