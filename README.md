# PolarsPath

A Rust ecosystem for dynamically accessing nested structures using path notation, with seamless
integration to Polars DataFrames and Protocol Buffers support.

## Overview

PolarsPath provides a powerful way to access nested data structures in Rust using dot notation and array
indexing, similar to JSONPath or XPath. It's designed to work seamlessly with [Polars](https://pola.rs/)
DataFrames and includes support for Protocol Buffers, making it ideal for data processing and analytics
workflows.

## Project Structure

This repository contains several interconnected crates:

### Core Libraries

- **`polars-structpath`** ([README](./crates/polars-structpath/README.md)): Main library providing the
  `StructPath` trait and all related functionality. It's a wrapper for the 2 cargos below
- **`polars-structpath-types`**: Helper library for `polars-structpath`, defining and implementing all
  types and traits
- **`polars-structpath-derive`**: Helper library for `polars-structpath`, implementing the derive macros
  `StructPath` and `EnumPath`

### Protobuf cargo

- **`polars-protobuf`** ([README](./crates/polars-protobuf/README.md)): Library for automatically
  generating polars-structpath implementations for Protocol Buffer messages

## Development

Contributions are welcome! Please see the individual crate READMEs for specific contribution guidelines.

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

### Code quality

The project uses a Makefile for the main developer steps. You can run all of them using

```bash
make all
```

### Publishing the crates

Prerequisites:

* Create a crates.io account** (if you don't have one), signing up with your GitHub account
* Get your API token from your personal space at crates.io
* Login to cargo:
   ```bash
   cargo login <your-api-token>
   ```

Then, simply do:

```shell
cargo publish --workspace
```

If the publishing fails and you end up with a partial publishing, remove the
already published packages using the option `--exclude [package_name]`

## Roadmap

- [ ] Full support of protobuf messages: `one_of`
- [ ] JSONPath-like parser for Polars DataFrame
- [ ] Convert JSON payloads to columnar format, with schema evolution

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Related Projects

- [Polars](https://pola.rs/) - Fast DataFrame library
- [Protocol Buffers](https://protobuf.dev/) - Language-neutral data serialization
- [PyO3](https://pyo3.rs/) - Rust-Python bindings
