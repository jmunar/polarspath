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
    -t, --sample-tests          Create sample tests (optional)
    -w, --use-workspace         Use workspace dependencies in Cargo.toml (optional)
    -h, --help                  Show this help message

Examples:
    $0 --project-name my_project
    $0 -n my_project --sample-proto --sample-tests
    $0 -n my_project -b /path/to/projects --use-workspace
EOF
}

# Initialize variables with defaults
USE_WORKSPACE=false
PROJECT_NAME=""
CREATE_SAMPLE_PROTO="n"
CREATE_SAMPLE_TESTS="n"

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
        -t|--sample-tests)
            CREATE_SAMPLE_TESTS="y"
            shift
            ;;
        -w|--use-workspace)
            USE_WORKSPACE=true
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

echo -e "${GREEN}polars_protobuf Project Generator${NC}"
echo "=================================="
echo ""

# Check if project already exists
if [ -d "$PROJECT_NAME" ]; then
    echo -e "${RED}Error: Directory '$PROJECT_NAME' already exists${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}Creating project: ${PROJECT_NAME}${NC}"
echo ""

# Convert project name to snake_case for package name (already should be)
PACKAGE_NAME=$(echo "$PROJECT_NAME" | tr '[:upper:]' '[:lower:]' | tr '-' '_')
PYTHON_PACKAGE_NAME="$PACKAGE_NAME"

# Create cargo project
echo "Creating cargo project..."
cargo new --lib "$PROJECT_NAME"
cd "$PROJECT_NAME"

# Create protobuf directory structure
echo "Creating protobuf directory structure..."
mkdir -p "protobuf/$PACKAGE_NAME"

# Create Cargo.toml
echo "Configuring Cargo.toml..."
if [ "$USE_WORKSPACE" = true ]; then
    cat > Cargo.toml <<EOF
[package]
name = "$PROJECT_NAME"
edition.workspace = true
description = "Protocol Buffer messages with polars_structpath support"
license = "MIT OR Apache-2.0"

[lib]
crate-type = ["cdylib", "rlib"]

[features]
default = []
extension-module = ["pyo3", "pyo3/extension-module", "pyo3-polars", "serde"]

[dependencies]
prost = { workspace = true }
polars_structpath = { workspace = true, features = ["derive"] }
polars_protobuf = { workspace = true }
pyo3 = { workspace = true, optional = true }
pyo3-polars = { workspace = true, optional = true }
serde = { workspace = true, optional = true }

[build-dependencies]
polars_protobuf = { workspace = true, features = ["build"] }
prost-build = { workspace = true }
prost-types = { workspace = true }
EOF
else
    cat > Cargo.toml <<EOF
[package]
name = "$PROJECT_NAME"
edition = "2021"
description = "Protocol Buffer messages with polars_structpath support"
license = "MIT OR Apache-2.0"

[lib]
crate-type = ["cdylib", "rlib"]

[features]
default = []
extension-module = ["pyo3", "pyo3/extension-module", "pyo3-polars", "serde"]

[dependencies]
prost = "*"
polars_structpath = { version = "*", features = ["derive"] }
polars_protobuf = { version = "*" }
pyo3 = { version = "*", optional = true, features = ["abi3-py38"] }
pyo3-polars = { version = "*", optional = true, features = ["derive"] }
serde = { version = "*", optional = true, features = ["derive"] }

[build-dependencies]
polars_protobuf = { version = "*", features = ["build"] }
prost-build = "*"
prost-types = "*"
EOF
fi

# Create build.rs
echo "Creating build.rs..."
cat > build.rs <<EOF
use polars_protobuf::build::{build_protobuf, BuildConfig, ExtensionConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_protobuf(BuildConfig {
        proto_dir: "protobuf/$PACKAGE_NAME".to_string(),
        include_paths: vec!["protobuf/$PACKAGE_NAME".to_string()],
        generate_extensions: Some(ExtensionConfig {
            python_package_dir: "$PYTHON_PACKAGE_NAME".to_string(),
            python_package_name: "$PYTHON_PACKAGE_NAME".to_string(),
        }),
    })?;

    println!("cargo:rerun-if-changed=protobuf/$PACKAGE_NAME");
    Ok(())
}
EOF

# Create src/lib.rs
echo "Creating src/lib.rs..."
cat > src/lib.rs <<EOF
pub mod $PACKAGE_NAME {
    include!(concat!(env!("OUT_DIR"), "/$PACKAGE_NAME.rs"));
}

#[cfg(feature = "extension-module")]
include!(concat!(env!("OUT_DIR"), "/extension_generated.rs"));
EOF

# Create sample protobuf message if requested
if [[ "$CREATE_SAMPLE_PROTO" =~ ^[Yy]$ ]]; then
    echo "Creating sample protobuf message..."
    cat > "protobuf/$PACKAGE_NAME/person.proto" <<EOF
syntax = "proto3";

package $PACKAGE_NAME;

message Person {
  string name = 1;
  int64 age = 2;
  optional string email = 3;
  bool is_active = 4;

  message Address {
    string street = 1;
    string city = 2;
    int32 zip_code = 3;
  }

  enum Status {
    UNKNOWN = 0;
    ACTIVE = 1;
    INACTIVE = 2;
  }

  Address address = 5;
  repeated string tags = 6;
  Status status = 7;
  repeated Address previous_addresses = 8;
}
EOF
fi

# Create sample tests if requested
if [[ "$CREATE_SAMPLE_TESTS" =~ ^[Yy]$ ]]; then
    echo "Creating sample tests..."
    mkdir -p tests

    if [[ "$CREATE_SAMPLE_PROTO" =~ ^[Yy]$ ]]; then
        cat > tests/test_person.rs <<EOF
use polars_structpath::polars_core::prelude::{AnyValue, DataType};
use polars_structpath::{data_type_wrapper, HasDataTypeWrapper, StructPath};
use ${PROJECT_NAME}::$PACKAGE_NAME;

#[test]
fn test_get_type_person() -> Result<(), Box<dyn std::error::Error>> {
    let person_type = $PACKAGE_NAME::Person::data_type();
    assert!(matches!(person_type, DataType::Struct(_)));
    Ok(())
}

#[test]
fn test_get_type_fields() -> Result<(), Box<dyn std::error::Error>> {
    let name_type = $PACKAGE_NAME::Person::get_type("name")?;
    assert_eq!(name_type, data_type_wrapper!(String));

    let age_type = $PACKAGE_NAME::Person::get_type("age")?;
    assert_eq!(age_type, data_type_wrapper!(Int64));

    let email_type = $PACKAGE_NAME::Person::get_type("email")?;
    assert_eq!(email_type, data_type_wrapper!(Option(String)));

    let street_type = $PACKAGE_NAME::Person::get_type("address.street")?;
    assert_eq!(street_type, data_type_wrapper!(Option(String)));

    let tag_type = $PACKAGE_NAME::Person::get_type("tags")?;
    assert_eq!(tag_type, data_type_wrapper!(List(String)));

    let tag0_type = $PACKAGE_NAME::Person::get_type("tags[0]")?;
    assert_eq!(tag0_type, data_type_wrapper!(String));

    Ok(())
}

#[test]
fn test_get_value_person() -> Result<(), Box<dyn std::error::Error>> {
    let mut person = $PACKAGE_NAME::Person::default();
    person.name = "Alice".to_string();
    person.age = 30;
    person.email = Some("alice@example.com".to_string());
    person.is_active = true;

    person.address = Some($PACKAGE_NAME::person::Address {
        street: "123 Main St".to_string(),
        city: "Springfield".to_string(),
        zip_code: 12345,
    });

    person.tags.push("premium".to_string());
    person.tags.push("verified".to_string());

    person.status = 1; // ACTIVE

    let name = person.get_value("name")?;
    assert_eq!(name, AnyValue::String("Alice"));

    let age = person.get_value("age")?;
    assert_eq!(age, AnyValue::Int64(30));

    let email = person.get_value("email")?;
    assert_eq!(email, AnyValue::String("alice@example.com"));

    let street = person.get_value("address.street")?;
    assert_eq!(street, AnyValue::String("123 Main St"));

    let tag0 = person.get_value("tags[0]")?;
    assert_eq!(tag0, AnyValue::String("premium"));

    Ok(())
}

#[test]
fn test_get_value_nested_array() -> Result<(), Box<dyn std::error::Error>> {
    let mut person = $PACKAGE_NAME::Person::default();

    person
        .previous_addresses
        .push(example_protobuf::person::Address {
            street: "456 Old St".to_string(),
            city: "Oldtown".to_string(),
            zip_code: 54321,
        });

    person
        .previous_addresses
        .push(example_protobuf::person::Address {
            street: "789 New St".to_string(),
            city: "Newtown".to_string(),
            zip_code: 98765,
        });

    let first_old_street = person.get_value("previous_addresses[0].street")?;
    assert_eq!(first_old_street, AnyValue::String("456 Old St"));

    let second_old_city = person.get_value("previous_addresses[1].city")?;
    assert_eq!(second_old_city, AnyValue::String("Newtown"));

    Ok(())
}
EOF
    else
        cat > tests/test_basic.rs <<EOF
use ${PROJECT_NAME}::$PACKAGE_NAME;
use polars_structpath::polars_core::prelude::DataType;
use polars_structpath::StructPath;

#[test]
fn test_module_loaded() {
    // Basic test to ensure the module is loaded correctly
    // Add your own tests here once you create protobuf messages
    assert!(true);
}
EOF
    fi
fi

# Create Python package directory
echo "Creating Python package structure..."
mkdir -p "$PYTHON_PACKAGE_NAME"

# Create pyproject.toml
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
EOF

# Create Makefile
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

echo ""
echo -e "${GREEN}Project created successfully!${NC}"
echo ""
echo "Next steps:"
echo "  1. Add your .proto files to protobuf/$PACKAGE_NAME/"
echo "  2. Run 'cargo build' to build the Rust project"
echo "  3. Run 'make build-python' to build the Python package"
echo "  4. Run 'cargo test' to run tests"
echo ""
echo -e "${YELLOW}Note: Make sure to add polars_protobuf dependencies to your Cargo.toml${NC}"
echo "      You may need to use path dependencies or publish to crates.io"
echo ""

