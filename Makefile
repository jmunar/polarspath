.PHONY: all clean test format check update-deps

all: format check test examples
all-python: format-python check-python
all-rust: format-rust check-rust test-rust examples-rust

install-uv:
	@curl -LsSf https://astral.sh/uv/install.sh | sh

build-python:
ifndef RELEASE
	$(error RELEASE is required. Usage: make build-python RELEASE=0 or RELEASE=1)
endif
ifneq ($(RELEASE),0)
ifneq ($(RELEASE),1)
	$(error RELEASE must be 0 or 1. Got: $(RELEASE))
endif
endif
	@echo "Re-creating example_protobuf..."
	@cd examples && ../crates/polars-protobuf/create_polars_protobuf_project.sh --project-name example_protobuf --sample-proto
	@echo "Building Python example_protobuf package..."
	@cd examples/example_protobuf && make build RELEASE=$(RELEASE)

format-rust:
	@echo "Formatting Rust workspace..."
	@cargo fmt --all

format-python:
	@echo "Format fix in Python example_protobuf..."
	@cd examples/example_protobuf && uv run ruff format --exclude example_protobuf/pybindings

format: format-rust format-python

check-rust:
	@echo "Running clippy on Rust workspace..."
	@cargo clippy --workspace -- -D warnings

check-python:
	@echo "Format check in Python example_protobuf..."
	@cd examples/example_protobuf && uv run ruff check --exclude example_protobuf/pybindings

check: check-rust check-python

update-deps:
	@echo "Updating dependencies for Rust workspace..."
	@cargo update --workspace

test-rust:
	@echo "Running tests for Rust workspace..."
	@cargo test --workspace

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
	@rm -rf examples/example_protobuf/example_protobuf/sample

clean: clean-rust clean-python

help:
	@echo "Available commands:"
	@echo ""
	@echo "Main targets:"
	@echo "  make all                      - Run format, check, test, examples (no build)"
	@echo "  make all-python               - Run Python format and check only (no build)"
	@echo "  make all-rust                 - Run Rust format, check, test, and examples"
	@echo ""
	@echo "Build targets:"
	@echo "  make build-python RELEASE=0|1 - Build the Python package (RELEASE required: 0=debug, 1=release)"
	@echo ""
	@echo "Test targets:"
	@echo "  make test                     - Run tests in Rust"
	@echo "  make test-rust                - Run tests in Rust only"
	@echo ""
	@echo "Example targets:"
	@echo "  make examples                 - Run examples in Rust"
	@echo "  make examples-rust            - Run examples in Rust only"
	@echo ""
	@echo "Format targets:"
	@echo "  make format                   - Format Rust and Python code"
	@echo "  make format-rust              - Format Rust code only"
	@echo "  make format-python            - Format Python code only"
	@echo ""
	@echo "Check targets:"
	@echo "  make check                    - Run all linters and checks for Rust and Python"
	@echo "  make check-rust               - Run clippy linter on all Rust crates only"
	@echo "  make check-python             - Run all Python linters and checks only"
	@echo ""
	@echo "Utility targets:"
	@echo "  make update-deps              - Update dependencies to latest versions in all Rust crates"
	@echo "  make clean                    - Clean build artifacts for all crates"
	@echo "  make clean-rust               - Clean Rust build artifacts only"
	@echo "  make clean-python             - Clean Python build artifacts only"
	@echo "  make help                     - Show this help message" 
