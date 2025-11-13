# PolarsPath

A Rust ecosystem for dynamically accessing nested structures using path notation, with seamless integration to Polars DataFrames and Protocol Buffers support.

## Overview

PolarsPath provides a powerful way to access nested data structures in Rust using dot notation and array indexing, similar to JSONPath or XPath. It's designed to work seamlessly with [Polars](https://pola.rs/) DataFrames and includes support for Protocol Buffers, making it ideal for data processing and analytics workflows.

## Features

- **Dynamic Path Access**: Access nested structures using intuitive path notation (`parent[0].name`)
- **Polars Integration**: Native support for Polars `AnyValue` and `DataType` types
- **Protocol Buffers Support**: Built-in support for protobuf message extraction
- **Python Bindings**: Python extension module for easy integration with data science workflows
- **Type Safety**: Compile-time type checking with derive macros
- **Performance**: Optimized for high-performance data processing

## Project Structure

This repository contains several interconnected crates:

### Core Libraries

- **`structpath`**: Main library providing the `StructPath` trait and path-based access functionality.
  It's a wrapper for the 2 cargos below
- **`structpath_types`**: Helper library for `structpath`, defining and implementing all types and traits
- **`structpath_derive`**: Helper library for `structpath`, implementing the derive macros `StructPath` and `EnumPath`

### Protobuf cargo

- **`structpath_protobuf`**: Library for automatically generating structpath implementations for Protocol Buffer messages
- **`structpath_protobuf_example`**: Example project demonstrating protobuf integration with Polars, including Python bindings

## Quick Start

### Basic Usage (`structpath` crate)

```rust
use polars_core::prelude::{AnyValue, DataType};
use structpath::StructPath;

#[derive(StructPath, Debug, Clone)]
struct Parent {
    name: String,
    age: i64,
}

#[derive(StructPath, Debug, Clone)]
struct User {
    name: String,
    age: i64,
    #[type_hint = "struct"]
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
    let name_type = User::get_type("name").unwrap().to_data_type();
    assert_eq!(name_type, DataType::String);
}
```

### Protocol Buffers Integration

```rust
use structpath::StructPath;

// After deriving StructPath for your protobuf message, you can access fields directly
let person = my_package::Person::default();
let name = person.get_value("name")?;
let street = person.get_value("address.street")?;
```

### Python Integration

```python
import polars as pl
from structpath_protobuf_example import structpath_protobuf_example

# Extract values from protobuf data in Python using Polars expressions
df = pl.DataFrame({"data": [protobuf_bytes]})
result = df.with_columns([
    structpath_protobuf_example.Person.get_value(pl.col("data"), "name").alias("name"),
    structpath_protobuf_example.Person.get_value(pl.col("data"), "address.street").alias("street"),
])
```

## Supported Types

The library currently supports:

- **Scalar Types**: `String`, `i32`, `i64`, `f64`, `bool`
- **Nested Structures**: Any struct implementing the `StructPath` trait
- **Optional Types**: `Option<T>` for all types above
- **Vectors**: `Vec<T>` for all supported types

## Installation

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
structpath = { version = "*", features = ["derive"] }
```

### Python

The Python package is built from source using `maturin`. To install:

```bash
# Build and install from source
cd structpath_protobuf_example
uv run maturin develop --release
```

Or install using pip after building:

```bash
pip install structpath_protobuf_example
```

## Building from Source

The project uses a Makefile for the main developer steps. You can run all of them using

```bash
make all
```

Otherwise, you can run a subset:

```bash
# Format code
make format

# Run linters
make check

# Run tests
make test

# Build everything (Rust + Python)
make build
```

## Development

### Prerequisites

- Rust (latest stable)
- Python 3.8+
- `uv` package manager (for Python dependencies)

Additionally, you will need the Protocol Buffers compiler (`protoc`) if you plan to
work with protobuf messages from python.

### Setup

1. Clone the repository
2. Install the latest stable version of Rust with your package manager (e.g. Homebrew)
3. Install `uv` for managing python environments: `make install-uv`
4. Build the project: `make build`

## Examples

See the `examples/` directories in each crate for comprehensive usage examples:

- `structpath/examples/`: Basic structpath usage
- `structpath_protobuf/examples/`: Protocol Buffers integration and benchmarks

## Performance

The library is designed for high-performance data processing. Benchmark results and performance characteristics are available in the `work-notebooks/` directory.

## Contributing

Contributions are welcome! Please see the individual crate READMEs for specific contribution guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Full support of protobuf messages: `enum` and `one_of`
- [ ] Performance optimisation using multiple threads
- [ ] JSONPath-like parser for Polars DataFrame
- [ ] Convert JSON payloads to columnar format, with schema evolution

## Related Projects

- [Polars](https://pola.rs/) - Fast DataFrame library
- [Protocol Buffers](https://protobuf.dev/) - Language-neutral data serialization
- [PyO3](https://pyo3.rs/) - Rust-Python bindings
