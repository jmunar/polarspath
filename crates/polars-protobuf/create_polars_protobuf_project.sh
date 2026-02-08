#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to display usage
show_usage() {
    echo -e "$(cat <<EOF
${GREEN}polars-protobuf Project Generator${NC}

Creates a complete Rust + Python project for working with Protocol Buffer messages
using polars-structpath. The generated project includes:

${YELLOW}What This Script Does:${NC}
  1. Creates a Rust library that auto-generates type-safe structs from .proto files
  2. Generates Arrow/Polars integration via polars-structpath derive macros
  3. Sets up Python bindings with maturin for cross-language interop
  4. Configures the build system to regenerate code on .proto changes

${YELLOW}Project Structure Created:${NC}
  project_name/
  ├── Cargo.toml              # Rust dependencies (polars-protobuf, prost, pyo3)
  ├── build.rs                # Build script that processes .proto files
  ├── src/lib.rs              # Main Rust library (includes generated code)
  ├── protobuf/               # Directory for your .proto files
  │   └── project_name/       # Package-specific proto directory
  │       └── *.proto         # Your protobuf schemas here
  ├── project_name/           # Python package directory
  │   ├── __init__.py         # Auto-generated Python module init
  │   └── pybindings/         # Auto-generated protobuf Python bindings
  ├── pyproject.toml          # Python package config (maturin build)
  └── Makefile                # Build commands for Python package

${YELLOW}Generated Code:${NC}
  During 'cargo build', the build.rs script generates:
  - Rust structs with #[derive(StructPath)] for each protobuf message
  - Rust enums with #[derive(EnumPath)] for each protobuf enum
  - ArrowMessage trait implementations for encode/decode
  - Python bindings via maturin/pyo3
  - Python __init__.py with type stubs and utilities

${YELLOW}Usage:${NC}
  $0 [OPTIONS]

${YELLOW}Options:${NC}
  -n, --project-name NAME     Project name (required, e.g., example_protobuf)
                              Must be a valid Rust crate name (lowercase, underscores)
  -p, --sample-proto          Create sample person.proto with examples of:
                              - Basic types (string, int, bool)
                              - Optional fields
                              - Nested messages
                              - Repeated fields
                              - Enums
  -h, --help                  Show this help message

${YELLOW}Examples:${NC}
  # Create minimal project (add your own .proto files)
  $0 --project-name my_messages

  # Create project with sample person.proto for learning
  $0 -n my_messages --sample-proto

${YELLOW}Next Steps After Creation:${NC}
  1. cd project_name
  2. Add your .proto files to protobuf/project_name/ (or use sample)
  3. cargo build                  # Generates Rust code
  4. make build-python            # Builds Python package
  5. cargo test                   # Run Rust tests
  6. Check generated code in target/debug/build/.../out/

${YELLOW}Integration Examples:${NC}
  Rust: use project_name::package_name::MyMessage;
  Python: from project_name import encode_MyMessage, decode_MyMessage

For more information, see: https://github.com/your-repo/polars-protobuf
EOF
)"
}

# Initialize variables with defaults
PROJECT_NAME=""
CREATE_SAMPLE_PROTO="n"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -n|--project-name)
            PROJECT_NAME="$2"
            shift 2
            ;;
        -p|--sample-proto)
            CREATE_SAMPLE_PROTO="y"
            shift
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo ""
            show_usage
            exit 1
            ;;
    esac
done

# Validate required arguments
if [ -z "$PROJECT_NAME" ]; then
    echo -e "${RED}Error: Project name is required${NC}"
    echo ""
    show_usage
    exit 1
fi

# Validate project name (basic check for valid Rust crate name)
if [[ ! "$PROJECT_NAME" =~ ^[a-z][a-z0-9_]*$ ]]; then
    echo -e "${RED}Error: Project name must be a valid Rust crate name (lowercase letters, numbers, underscores)${NC}"
    exit 1
fi

echo -e "${GREEN}polars-protobuf Project Generator${NC}"
echo "=================================="
echo ""

echo ""
echo -e "${GREEN}Setting up project: ${PROJECT_NAME}${NC}"
echo ""

# Convert project name to snake_case for package name (already should be)
PACKAGE_NAME=$(echo "$PROJECT_NAME" | tr '[:upper:]' '[:lower:]' | tr '-' '_')
PYTHON_PACKAGE_NAME="$PACKAGE_NAME"

# Create or enter project directory
if [ -d "$PROJECT_NAME" ]; then
    echo -e "${YELLOW}Directory '$PROJECT_NAME' already exists, using existing directory${NC}"
    cd "$PROJECT_NAME"
else
    echo "Creating cargo project..."
    cargo new --lib "$PROJECT_NAME"
    cd "$PROJECT_NAME"
fi

# Create protobuf directory structure
echo "Creating protobuf directory structure..."
mkdir -p "protobuf/$PACKAGE_NAME"

# Create Cargo.toml
if [ -f "Cargo.toml" ]; then
    echo -e "${YELLOW}Cargo.toml already exists, skipping${NC}"
else
    echo "Configuring Cargo.toml..."
    cat > Cargo.toml <<EOF
[package]
name = "$PROJECT_NAME"
edition = "2021"
description = "Protocol Buffer messages with polars-structpath support"
license = "MIT OR Apache-2.0"

[lib]
crate-type = ["cdylib", "rlib"]

[features]
default = []
extension-module = ["pyo3", "pyo3/extension-module"]

[dependencies]
prost = "*"
polars-structpath = { version = "*", features = ["derive"] }
polars-protobuf = { version = "*" }
pyo3 = { version = "*", optional = true, features = ["abi3-py38"] }
pyo3-polars = { version = "*" }

[build-dependencies]
polars-protobuf = { version = "*", features = ["build"] }
prost-build = "*"
prost-types = "*"
EOF
fi

# Create build.rs
if [ -f "build.rs" ]; then
    echo -e "${YELLOW}build.rs already exists, skipping${NC}"
else
    echo "Creating build.rs..."
    cat > build.rs <<EOF
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let proto_dir = "protobuf/$PACKAGE_NAME";

    polars_protobuf::build::BuildConfig::from_proto_dir(out_dir, proto_dir)?
        .with_python(PathBuf::from("$PACKAGE_NAME"), "_${PACKAGE_NAME}_rust")
        .build()?;

    println!("cargo:rerun-if-changed={}", proto_dir);
    Ok(())
}
EOF
fi

# Create src/lib.rs
if [ -f "src/lib.rs" ]; then
    echo -e "${YELLOW}src/lib.rs already exists, skipping${NC}"
else
    echo "Creating src/lib.rs..."
    cat > src/lib.rs <<EOF
pub mod $PACKAGE_NAME {
    include!(concat!(env!("OUT_DIR"), "/$PACKAGE_NAME.rs"));
}

#[cfg(feature = "extension-module")]
use pyo3::prelude::*;

#[cfg(feature = "extension-module")]
#[pymodule]
fn _${PACKAGE_NAME}_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
EOF
fi

# Create sample protobuf message if requested
if [[ "$CREATE_SAMPLE_PROTO" =~ ^[Yy]$ ]]; then
    echo "Creating sample protobuf message..."
    cat > "protobuf/$PACKAGE_NAME/person.proto" <<EOF
syntax = "proto3";

package $PACKAGE_NAME;

// Address as a top-level message (not nested)
message Address {
  string street = 1;
  string city = 2;
  int32 zip_code = 3;
}

// Status as a top-level enum (not nested)
enum Status {
  UNKNOWN = 0;
  ACTIVE = 1;
  INACTIVE = 2;
}

message Person {
  string name = 1;
  int64 age = 2;
  optional string email = 3;
  bool is_active = 4;
  Address address = 5;
  repeated string tags = 6;
  Status status = 7;
  repeated Address previous_addresses = 8;
}
EOF
fi

# Create Python package directory (always overwrite)
echo "Creating Python package structure..."
mkdir -p "$PYTHON_PACKAGE_NAME"

# Create __init__.py placeholder for the Python package
# This will be overwritten by build.rs during cargo build
echo "Creating $PYTHON_PACKAGE_NAME/__init__.py..."
cat > "$PYTHON_PACKAGE_NAME/__init__.py" <<EOF
# Auto-generated Python package initialization
# This file is regenerated during cargo build
EOF

# Create pyproject.toml
if [ -f "pyproject.toml" ]; then
    echo -e "${YELLOW}pyproject.toml already exists, skipping${NC}"
else
    echo "Creating pyproject.toml..."
    cat > pyproject.toml <<EOF
[project]
name = "$PYTHON_PACKAGE_NAME"
version = "0.1.0"
description = "Python package for $PYTHON_PACKAGE_NAME"
requires-python = ">=3.10"
classifiers = [
    "Programming Language :: Rust",
    "Programming Language :: Python :: Implementation :: CPython",
    "Programming Language :: Python :: Implementation :: PyPy",
]
dependencies = [
    "polars",
    "protobuf",
    "pyarrow",
]

[dependency-groups]
dev = [
    "bandit",
    "maturin",
    "notebook",
    "pytest",
    "ruff",
]

[tool.hatch.build.targets.sdist]
include = ["$PYTHON_PACKAGE_NAME"]

[tool.hatch.build.targets.wheel]
include = ["$PYTHON_PACKAGE_NAME"]

[tool.isort]
profile = "black"
line_length = 88

[build-system]
requires = ["maturin>=1.0,<2.0", "polars>=1.3.0"]
build-backend = "maturin"

[tool.maturin]
bindings = "pyo3"
features = ["extension-module"]
module-name = "$PYTHON_PACKAGE_NAME._${PYTHON_PACKAGE_NAME}_rust"
EOF
fi

# Create Makefile
if [ -f "Makefile" ]; then
    echo -e "${YELLOW}Makefile already exists, skipping${NC}"
else
    echo "Creating Makefile..."
    cat > Makefile <<'MAKEEOF'
.PHONY: build clean help

RELEASE ?=

help:
	@echo "Available commands:"
	@echo "  make install-uv              - Download and install uv"
	@echo "  make build                   - Build both Python bindings and Python package (debug)"
	@echo "  make build RELEASE=1         - Build both Python bindings and Python package (release)"
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
	@uv run maturin develop --uv $(if $(RELEASE),--release,)

build-python-bindings:
	@echo "Building protobuf Python bindings..."
MAKEEOF
    cat >> Makefile <<EOF
	@mkdir -p $PYTHON_PACKAGE_NAME/pybindings && \\
		protoc \\
			-I=protobuf/$PACKAGE_NAME \\
			--python_out=$PYTHON_PACKAGE_NAME/pybindings \\
			protobuf/$PACKAGE_NAME/*.proto

build: build-python-bindings build-python

clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf target
	@rm -rf $PYTHON_PACKAGE_NAME/*.so
	@rm -rf $PYTHON_PACKAGE_NAME/__pycache__
	@rm -rf $PYTHON_PACKAGE_NAME/*/__pycache__
EOF
fi

echo ""
echo -e "${GREEN}Project setup completed successfully!${NC}"
echo ""
echo "Next steps:"
echo "  1. Add your .proto files to protobuf/$PACKAGE_NAME/"
echo "  2. Run 'cargo build' to build the Rust project"
echo "  3. Run 'make build-python' to build the Python package"
echo "  4. Run 'cargo test' to run tests"
echo ""
echo -e "${YELLOW}Note: Make sure to add polars-protobuf dependencies to your Cargo.toml${NC}"
echo "      You may need to use path dependencies or publish to crates.io"
echo ""

