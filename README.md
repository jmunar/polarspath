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

- **`structpath`**: Main library providing the `StructPath` trait and path-based access functionality
- **`structpath_types`**: Helper library with common types and data structures shared across the ecosystem
- **`structpath_derive`**: Helper library for procedural macros for automatic `StructPath` trait implementation

### Sample Applications

- **`protobuf_sample`**: Sample Protocol Buffer messages and basic extraction functionality
- **`protobuf_sample_polars`**: Advanced protobuf integration with Polars, including Python bindings

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
use protobuf_sample_polars::get_value;

// Extract values from protobuf messages
let value = get_value(&message, "f_submessage.f_string")?;
```

### Python Integration

```python
import polars as pl
from protobuf_sample_polars import get_value

# Extract values from protobuf data in Python
df = pl.DataFrame({"data": [protobuf_bytes]})
result = df.with_columns([
    pl.col("data").map_elements(lambda x: get_value(x, "field.path"))
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
structpath = { version = "0.1.0", features = ["derive"] }
```

### Python

```bash
# Install the Python package
pip install protobuf_sample_polars
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
- `protobuf_sample_polars/examples/`: Protocol Buffers integration

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
