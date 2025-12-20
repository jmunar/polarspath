# polars_structpath_derive

This crate provides procedural derive macros for the `polars_structpath` ecosystem. It automatically generates implementations of the `StructPath` and `EnumPath` traits, enabling dynamic path-based access to nested Rust structures.

## Purpose

`polars_structpath_derive` is a procedural macro crate that generates code for:

- **StructPath derive macro**: Automatically implements the `StructPath` trait for structs, enabling path-based field access
- **EnumPath derive macro**: Automatically implements the `EnumPath` trait for enums, enabling path-based enum value access with case conversion support

This crate is used by:
- `polars_structpath`: The main user-facing library that re-exports these derive macros
- End users: Who apply `#[derive(StructPath)]` or `#[derive(EnumPath)]` to their structs and enums

## Usage

### StructPath

Apply the `#[derive(StructPath)]` attribute to structs with named fields:

```rust
use polars_structpath::StructPath;

#[derive(StructPath)]
struct User {
    name: String,
    age: i64,
    #[type_hint("struct")]
    parent: Option<Parent>,
}

#[derive(StructPath)]
struct Parent {
    name: String,
}
```

The `#[type_hint]` attribute can be used to provide type information for complex types like nested structs, enums, or collections.

### EnumPath

Apply the `#[derive(EnumPath)]` attribute to enums:

```rust
use polars_structpath::EnumPath;

#[derive(EnumPath)]
#[enum_path(camel_case_to_upper_snake_case)]
enum Status {
    Active,
    Inactive,
}
```

The `#[enum_path]` attribute supports case conversion functions for mapping enum variant names to string representations.

## See Also

- [Main README](../../README.md) - Overview of the entire polars_structpath ecosystem
- [polars_structpath](../polars_structpath/README.md) - User-facing API documentation
- [polars_structpath_types](../polars_structpath_types/README.md) - Core types and traits
- [polars_protobuf](../polars_protobuf/README.md) - Protocol Buffers integration

