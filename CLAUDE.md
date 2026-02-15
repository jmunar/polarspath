# Claude Code Guide for PolarsPath

This file provides context for Claude Code sessions working on this codebase.

## Project Overview

PolarsPath is a Rust ecosystem for working with nested data structures using path notation, integrated with Polars DataFrames and Protocol Buffers.

## Crate Architecture

```
polarspath/
├── crates/
│   ├── polars-structpath-derive/   # Derive macros (StructPath, EnumPath)
│   ├── polars-structpath/          # Main library: core types, traits, and derive re-exports
│   └── polars-protobuf/            # Protobuf integration + build utilities
└── examples/
    └── example_protobuf/           # Generated example project (created by script)
```

### Dependency Flow
```
polars-structpath-derive
        ↓
polars-structpath (core types + derive re-exports)
        ↓
polars-protobuf
```

## Key Concepts

### ArrowBuffer Trait
Defines how to build Arrow arrays from Rust types:
- `new_buffer(capacity)` - Create buffer
- `push(element)` / `push_null()` - Add elements
- `to_arrow()` - Convert to Arrow array

### IntoArrow / FromArrow Traits
- `IntoArrow`: Rust type → Arrow array
- `FromArrow`: Arrow array → Vec<Rust type>

### ArrowMessage Trait (polars-protobuf)
Bridge between prost-generated types and polars-structpath types:
- `from_prost(prost_message) -> Self`
- `to_prost(self) -> ProstMessage`
- Inherits `prost::Message` for encode/decode

### Code Generation (polars-protobuf/src/build.rs)
Generates Rust code from .proto files:
1. Runs prost-build to generate standard prost types
2. Generates wrapper types with `#[derive(StructPath)]` or `#[derive(EnumPath)]`
3. Implements `ArrowMessage` trait for each message

## Common Commands

```bash
# Main targets
make all                         # Format, check, test, examples (no build)
make all-python                  # Python format and check only (no build)
make all-rust                    # Rust format, check, test, examples

# Build targets (RELEASE required: 0=debug, 1=release)
make build-python RELEASE=0      # Build Python package in debug mode
make build-python RELEASE=1      # Build Python package in release mode

# Test and check
make test-rust                   # cargo test --workspace
make check-rust                  # cargo clippy -- -D warnings
make examples-rust               # Run all examples

# Format
make format-rust                 # cargo fmt
make format-python               # ruff format

# Clean
make clean                       # Clean all artifacts
```

## Testing

### Test Structure
- Unit tests in each crate's `src/` files
- Integration tests in `tests/` directories
- Doc tests in documentation comments

### Key Test Files
- `polars-protobuf/tests/test_arrow_message.rs` - Roundtrip encode/decode tests
- `polars-structpath/tests/test_to_arrow.rs` - Arrow conversion tests

### Running Specific Tests
```bash
cargo test -p polars-protobuf
cargo test -p polars-structpath --features derive
cargo test -p example_protobuf
```

## Code Style

### Clippy Settings
- Warnings are treated as errors (`-D warnings`)
- Key lints: `redundant_closure`, `unnecessary_map_or`

### Generated Code Patterns
When generating code in build.rs:
- Use `_variable` for unused parameters (e.g., empty message structs)
- Prefer method references over closures: `map(Type::method)` not `map(|x| Type::method(x))`
- Use `is_some_and()` instead of `map_or(false, ...)`

### Derive Macro Paths
Derive macros generate code using absolute paths:
```rust
::polars_structpath::impl_struct_buffer!(...)
```

## Architecture Decisions

### Enum Storage
Enums are stored as `PrimitiveArray<i32>` (not DictionaryArray) to avoid Polars Categorical re-indexing issues. The i32 value is the Rust discriminant.

### Parallelization
Uses Polars' `POOL.install()` for parallel operations:
```rust
use polars_core::POOL;
let result: Vec<T> = POOL.install(|| {
    items.into_par_iter().map(...).collect()
});
```

### Nested Types in Protobuf
Currently, nested message/enum types (defined inside a message) are not fully supported. Use top-level types instead:
```protobuf
// Preferred: top-level
message Address { ... }
message Person { Address address = 1; }

// Avoid: nested (partial support)
message Person { message Address { ... } }
```

## Working with the Script

`crates/polars-protobuf/create_polars_protobuf_project.sh` generates new projects:

```bash
# Create project with sample proto and tests
./create_polars_protobuf_project.sh -n my_project -p

# Options:
# -n, --project-name NAME    Required: project name
# -p, --sample-proto         Create sample person.proto
```

## Common Issues & Solutions

### "PythonScan not covered" (pyo3-polars)
Version incompatibility between pyo3-polars and polars. The script now excludes pyo3-polars to avoid this.

### Enum values corrupted after roundtrip
This was fixed by using PrimitiveArray<i32> instead of DictionaryArray for enum storage.

## Files to Read First

When starting a new session on this codebase:
1. This file (CLAUDE.md)
2. `Cargo.toml` (workspace structure)
3. `crates/polars-protobuf/src/lib.rs` (main exports)
4. `crates/polars-structpath/src/traits.rs` (core traits)

## Useful Patterns

### Adding a new field type to build.rs
1. Add type mapping in `FieldDescriptorWrapper::stmt_definition()`
2. Handle in `stmt_from_prost()` and `stmt_to_prost()` if special conversion needed

### Testing protobuf roundtrip
```rust
// Create prost and arrow versions
let prost_msg = prost::MyMessage { ... };
let arrow_msg = MyMessage::from_prost(prost_msg.clone());

// Verify conversions
assert_eq!(arrow_msg.clone().to_prost(), prost_msg);
assert_eq!(prost_msg.encode_to_vec(), arrow_msg.encode_to_vec());
```

### Using lazy API for encode/decode
```rust
use polars_protobuf::{encode_expr, decode_expr};

let encoded_df = df.lazy()
    .select([encode_expr::<MyMessage>(col("data")).alias("encoded")])
    .collect()?;

let decoded_df = encoded_df.lazy()
    .select([decode_expr::<MyMessage>(col("encoded"), struct_dtype).alias("decoded")])
    .collect()?;
```
