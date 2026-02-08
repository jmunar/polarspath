#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to display usage
show_usage() {
    cat <<EOF
Usage: $0 [OPTIONS]

Options:
    -n, --project-name NAME     Project name (required, e.g., example_protobuf)
    -p, --sample-proto          Create a sample protobuf message (optional)
    -h, --help                  Show this help message

Examples:
    $0 --project-name my_project
    $0 -n my_project --sample-proto
EOF
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
    cat > build.rs <<'BUILDRS'
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
BUILDRS
cat >> build.rs <<EOF
    let proto_dir = "protobuf/$PACKAGE_NAME";
EOF
cat >> build.rs <<'BUILDRS'

    // Discover all .proto files
    let proto_files: Vec<String> = fs::read_dir(proto_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "proto") {
                Some(path.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();

    if proto_files.is_empty() {
        println!("cargo:warning=No .proto files found in {}", proto_dir);
        return Ok(());
    }

    let protos: Vec<&str> = proto_files.iter().map(|s| s.as_str()).collect();
    let includes: &[&str] = &[proto_dir];
    let build_config = polars_protobuf::build::BuildConfig::new(out_dir, &protos, includes);
BUILDRS
cat >> build.rs <<EOF
    build_config
        .with_python(PathBuf::from("$PACKAGE_NAME"), "_${PACKAGE_NAME}_rust")
        .build()?;
EOF
cat >> build.rs <<'BUILDRS'

    println!("cargo:rerun-if-changed={}", proto_dir);
    Ok(())
}
BUILDRS
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
requires-python = ">=3.8"
classifiers = [
    "Programming Language :: Rust",
    "Programming Language :: Python :: Implementation :: CPython",
    "Programming Language :: Python :: Implementation :: PyPy",
]
dependencies = [
    "polars",
    "protobuf==5.29.3",
    "pyarrow",
]

[dependency-groups]
dev = [
    "bandit",
    "maturin",
    "notebook",
    "pytest",
    "ruff>=0.12.7",
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
    cat > Makefile <<EOF
.PHONY: build clean help

help:
	@echo "Available commands:"
	@echo "  make install-uv              - Download and install uv"
	@echo "  make build                   - Build both Python bindings and Python package"
	@echo "  make build-python            - Build Python package"
	@echo "  make build-python-bindings   - Add protobuf Python bindings to Python package"
	@echo "  make clean                   - Clean build artifacts"

install-uv:
	@if command -v uv >/dev/null 2>&1; then \\
		echo "uv is already installed"; \\
	else \\
		echo "Installing uv..."; \\
		curl -LsSf https://astral.sh/uv/install.sh | sh; \\
	fi

build-python:
	@echo "Building Python environment and package..."
	@uv sync

build-python-bindings:
	@echo "Building protobuf Python bindings..."
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

