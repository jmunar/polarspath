# polars_protobuf

A Rust library that automatically generates [polars_structpath](https://github.com/jmunar/polarspath)
implementations for Protocol Buffer messages, enabling type-safe field access and Polars integration.

## Purpose

`polars_protobuf` provides seamless integration between Protocol Buffers and the `polars_structpath` ecosystem. It enables:

- **Automatic Code Generation**: Automatically applies `StructPath` and `EnumPath` derives to protobuf messages and enums during build time
- **Type-Safe Field Access**: Extract fields from binary protobuf columns in Polars DataFrames using path notation
- **Polars Integration**: Native support for converting protobuf fields to Polars `Series` and `AnyValue` types
- **Python Bindings**: Optional generation of Python extension modules for use with Polars Python API

This crate is used by:
- Build scripts (`build.rs`) in projects that use protobuf messages with Polars
- Python packages that need to extract protobuf fields from Polars DataFrames

## Quick Start

To create a new project using `polars_protobuf`, you can download and run the project generator script:

```bash
curl -sSL https://raw.githubusercontent.com/jmunar/polarspath/main/crates/polars_protobuf/create_polars_protobuf_project.sh | bash -s -- --project-name my_project -p -t
```

The script will:
- Create a new cargo project with all necessary dependencies
- Set up the build configuration (`build.rs`)
- Create a Python package with the same name
- Generate a `pyproject.toml` for Python packaging
- Create a `Makefile` for building the project
- Optionally create sample protobuf messages (`-p`) and tests (`-t`)

## Usage

### In Build Scripts

Add to your `Cargo.toml`:

```toml
[build-dependencies]
polars_protobuf = { version = "*", features = ["build"] }
```

Then in your `build.rs`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    polars_protobuf::build::build_protobuf(polars_protobuf::build::BuildConfig {
        proto_dir: "protobuf/sample".to_string(),
        include_paths: vec!["protobuf/sample".to_string()],
        generate_extensions: Some(polars_protobuf::build::ExtensionConfig {
            python_package_dir: "example_protobuf/example_protobuf".to_string(),
            python_package_name: "example_protobuf".to_string(),
        }),
    })?;
    Ok(())
}
```

### Extracting Values from Protobuf Columns

Once your protobuf messages are generated with `StructPath` support, you can extract fields:

```rust
use polars_core::prelude::{BinaryType, ChunkedArray};
use polars_protobuf::get_value;
use prost::Message;

#[derive(polars_structpath::StructPath, Clone, Message)]
struct Person {
    #[prost(string, tag = "1")]
    name: String,
    #[prost(int64, tag = "2")]
    age: i64,
}

// Assuming you have a ChunkedArray<BinaryType> containing encoded protobuf messages
let binary_column: ChunkedArray<BinaryType> = /* ... */;

// Extract the "name" field from all messages
let name_series = get_value::<Person>(&binary_column, "name", true)?;

// Extract nested fields using path notation
// let parent_name = get_value::<Person>(&binary_column, "parent.name", true)?;
```

### Type Inference

You can also get the Polars data type for a field path:

```rust
use polars_core::prelude::Field;
use polars_protobuf::get_type;

let field = get_type::<Person>(&[], "name")?;
// field.dtype() will be DataType::String
```

## Features

- **`build`**: Enables the build-time code generation functionality (requires `prost-build` and `prost-types`)

## API Reference

### Functions

- **`get_type<T>`**: Get the Polars `Field` type for a given path in a protobuf message type
- **`get_value<T>`**: Extract a field from a `ChunkedArray<BinaryType>` containing encoded protobuf messages

### Build Module (feature = "build")

- **`build_protobuf`**: Main function to build protobuf files with polars_structpath support
- **`BuildConfig`**: Configuration for the build process
- **`ExtensionConfig`**: Configuration for generating Python extension modules

## See Also

- [polars_structpath](../polars_structpath/README.md) - Core polars_structpath library
- [polars_structpath_types](../polars_structpath_types/README.md) - Core types and traits
- [example_protobuf](../../examples/example_protobuf/) - Complete working example
