.PHONY: build clean help

help:
	@echo "Available commands:"
	@echo "  make install-uv              - Download and install uv"
	@echo "  make build                   - Build both Python bindings and Python package"
	@echo "  make build-python            - Build Python package"
	@echo "  make build-python-bindings   - Add protobuf Python bindings to Python package"
	@echo "  make clean                   - Clean build artifacts"

install-uv:
	@if command -v uv >/dev/null 2>&1; then \
		echo "uv is already installed"; \
	else \
		echo "Installing uv..."; \
		curl -LsSf https://astral.sh/uv/install.sh | sh; \
	fi

build-python:
	@echo "Building Python environment and package..."
	@uv sync

build-python-bindings:
	@echo "Building protobuf Python bindings..."
	@mkdir -p example_protobuf/pybindings && \
		protoc \
			-I=protobuf/example_protobuf \
			--python_out=example_protobuf/pybindings \
			protobuf/example_protobuf/*.proto

build: build-python-bindings build-python

clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf target
	@rm -rf example_protobuf/*.so
	@rm -rf example_protobuf/__pycache__
	@rm -rf example_protobuf/*/__pycache__
