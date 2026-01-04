# polars_structpath_derive

This crate provides procedural derive macros for the `polars_structpath` ecosystem. It automatically generates Arrow buffer implementations for Rust structs and enums, enabling seamless conversion to and from Apache Arrow arrays for Polars integration.

## Purpose

`polars_structpath_derive` is a procedural macro crate that generates code for:

- **StructPath derive macro**: Automatically generates Arrow buffer implementations for structs using `impl_struct_buffer!` from `polars_structpath_types`
- **EnumPath derive macro**: Automatically generates Arrow buffer implementations for enums using `impl_enum_buffer!` from `polars_structpath_types`

This crate is used by:
- `polars_structpath`: The main user-facing library that re-exports these derive macros
- End users: Who apply `#[derive(StructPath)]` or `#[derive(EnumPath)]` to their structs and enums

## Usage

For usage examples, see [polars_structpath](../polars_structpath/README.md).

## See Also

- [Main README](../../README.md) - Overview of the entire polars_structpath ecosystem
- [polars_structpath](../polars_structpath/README.md) - User-facing API documentation
- [polars_structpath_types](../polars_structpath_types/README.md) - Core types and traits
- [polars_protobuf](../polars_protobuf/README.md) - Protocol Buffers integration

