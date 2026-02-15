.PHONY: build build-python clean help

help:
	@echo "Available commands:"
	@echo "  make install-uv                - Download and install uv"
	@echo "  make build RELEASE=0|1         - Build both Python bindings and Python package (RELEASE required)"
	@echo "  make build-python RELEASE=0|1  - Build Python package (RELEASE required)"
	@echo "  make build-python-bindings     - Add protobuf Python bindings to Python package"
	@echo "  make clean                     - Clean build artifacts"

install-uv:
	@if command -v uv >/dev/null 2>&1; then \
		echo "uv is already installed"; \
	else \
		echo "Installing uv..."; \
		curl -LsSf https://astral.sh/uv/install.sh | sh; \
	fi

build-python:
ifndef RELEASE
	$(error RELEASE is required. Usage: make build-python RELEASE=0 or RELEASE=1)
endif
ifneq ($(RELEASE),0)
ifneq ($(RELEASE),1)
	$(error RELEASE must be 0 or 1. Got: $(RELEASE))
endif
endif
	@echo "Building Python environment and package..."
	@uv sync
	@uv run maturin develop --uv $(if $(filter 1,$(RELEASE)),--release,)

build-python-bindings:
	@echo "Building protobuf Python bindings..."
	@mkdir -p example_protobuf/pybindings && \
		protoc \
			-I=protobuf/example_protobuf \
			--python_out=example_protobuf/pybindings \
			protobuf/example_protobuf/*.proto

build: build-python-bindings
ifndef RELEASE
	$(error RELEASE is required. Usage: make build RELEASE=0 or RELEASE=1)
endif
ifneq ($(RELEASE),0)
ifneq ($(RELEASE),1)
	$(error RELEASE must be 0 or 1. Got: $(RELEASE))
endif
endif
	@$(MAKE) build-python RELEASE=$(RELEASE)

clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf target
	@rm -rf example_protobuf/*.so
	@rm -rf example_protobuf/__pycache__
	@rm -rf example_protobuf/*/__pycache__
