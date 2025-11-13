.PHONY: install-uv build-python build-rust clean help

help:
	@echo "Available commands:"
	@echo "  make install-uv     - Download and install uv"
	@echo "  make build-python   - Build Python environment and package"
	@echo "  make build-rust     - Build Rust project"
	@echo "  make build          - Build both Rust and Python"
	@echo "  make clean          - Clean build artifacts"

install-uv:
	@if command -v uv >/dev/null 2>&1; then 		echo "uv is already installed"; 	else 		echo "Installing uv..."; 		curl -LsSf https://astral.sh/uv/install.sh | sh; 	fi

build-rust:
	@echo "Building Rust project..."
	@cargo build --release

build-python: install-uv
	@echo "Building Python environment and package..."
	@uv sync
	@uv run maturin develop --release

build: build-rust build-python

clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf target
	@rm -rf structpath_protobuf_example/*.so
	@rm -rf structpath_protobuf_example/__pycache__
	@rm -rf structpath_protobuf_example/*/__pycache__
