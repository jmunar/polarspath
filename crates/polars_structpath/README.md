# polars_structpath

The main user-facing library for dynamically accessing nested Rust structures using path notation, with seamless integration to Polars DataFrames.

## Purpose

`polars_structpath` is the primary entry point for the `polars_structpath` ecosystem. It provides:

- **Unified API**: A single crate that re-exports all necessary types, traits, and derive macros
- **Path-Based Access**: Dynamic access to nested struct fields using intuitive path notation like `"parent.name"` or `"parents[0].age"`
- **Polars Integration**: Native support for Polars `AnyValue` and `DataType` types
- **Derive Macros**: Procedural macros for automatically implementing `StructPath` and `EnumPath` traits

This crate wraps and re-exports functionality from:
- `polars_structpath_types`: Core types and traits
- `polars_structpath_derive`: Derive macro implementations (optional, enabled via `derive` feature)

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
polars_structpath = { version = "*", features = ["derive"] }
```

Then use it in your code:

```rust
use polars_core::prelude::{AnyValue, DataType};
use polars_structpath::StructPath;

#[derive(StructPath, Debug, Clone)]
struct Parent {
    name: String,
    age: i64,
}

#[derive(StructPath, Debug, Clone)]
struct User {
    name: String,
    age: i64,
    #[type_hint("struct")]
    parents: Vec<Parent>,
}

fn main() {
    let user = User {
        name: "John".to_string(),
        age: 32,
        parents: vec![Parent {
            name: "Joseph".to_string(),
            age: 65,
        }],
    };

    // Access nested values using path notation
    let father_name = user.get_value("parents[0].name").unwrap();
    assert_eq!(father_name, AnyValue::String("Joseph"));

    // Get type information
    let name_type = User::get_type("name").unwrap().polars;
    assert_eq!(name_type, DataType::String);
}
```

## Path Syntax

Paths support the following syntax:

- **Simple fields**: `"name"`
- **Nested fields**: `"parent.name"`
- **Array indices**: `"parents[0]"`
- **Nested array access**: `"parents[0].name"`

## Features

- **`derive`** (default): Enables the `StructPath` and `EnumPath` derive macros
- **`std`** (default): Standard library support

## Supported Types

The library supports:

- **Scalar Types**: `String`, `i32`, `i64`, `f64`, `bool`
- **Nested Structures**: Any struct implementing the `StructPath` trait
- **Optional Types**: `Option<T>` for all types above
- **Vectors**: `Vec<T>` for all supported types
- **Enums**: Enums implementing the `EnumPath` trait

## See Also

- [Main README](../../README.md) - Overview of the entire polars_structpath ecosystem
- [polars_structpath_types](../polars_structpath_types/README.md) - Core types and traits
- [polars_structpath_derive](../polars_structpath_derive/README.md) - Derive macro implementation
- [polars_protobuf](../polars_protobuf/README.md) - Protocol Buffers integration

