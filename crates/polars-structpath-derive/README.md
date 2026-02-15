# polars-structpath-derive

This crate provides procedural derive macros for the `polars-structpath` ecosystem. It automatically generates Arrow buffer implementations for Rust structs and enums, enabling seamless conversion to and from Apache Arrow arrays for Polars integration.

## Purpose

`polars-structpath-derive` is a procedural macro crate that generates code for:

- **StructPath derive macro**: Automatically generates Arrow buffer implementations for structs using `impl_struct_buffer!` from `polars-structpath`
- **EnumPath derive macro**: Automatically generates Arrow buffer implementations for enums using `impl_enum_buffer!` from `polars-structpath`

This crate is used by:
- `polars-structpath`: The main user-facing library that re-exports these derive macros
- End users: Who apply `#[derive(StructPath)]` or `#[derive(EnumPath)]` to their structs and enums

## Usage

For usage examples, see [polars-structpath](../polars-structpath/README.md).

## See Also

- [Main README](../../README.md) - Overview of the entire `polarspath` ecosystem
- [polars-structpath](../polars-structpath/README.md) - User-facing API documentation
- [polars-protobuf](../polars-protobuf/README.md) - Protocol Buffers integration

