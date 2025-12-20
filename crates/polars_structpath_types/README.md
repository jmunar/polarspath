# polars_structpath_types

This crate provides the core types and traits that form the foundation of the `polars_structpath` ecosystem. It contains the non-derive implementation components that enable dynamic path-based access to nested Rust structures with seamless integration to Polars DataFrames.

## Purpose

`polars_structpath_types` is a foundational library that defines:

- **Core Types**: Data structures for representing paths, data types, and type metadata
- **Core Traits**: Interfaces for path-based data access (`StructPath`, `EnumPath`, `HasDataTypeWrapper`)
- **Type System**: A wrapper around Polars `DataType` that supports additional metadata needed for path traversal
- **Conversion Utilities**: Traits and implementations for converting Rust values to Polars `AnyValue`

This crate is used by:
- `polars_structpath`: The main user-facing library that wraps this crate
- `polars_structpath_derive`: The derive macro implementation that generates code using these types
- `polars_protobuf`: Protocol Buffers integration that leverages these types

## See Also

- [Main README](../../README.md) - Overview of the entire polars_structpath ecosystem
- [polars_structpath](../polars_structpath/README.md) - User-facing API documentation
- [polars_structpath_derive](../polars_structpath_derive/) - Derive macro implementation
- [polars_protobuf](../polars_protobuf/README.md) - Protocol Buffers integration

