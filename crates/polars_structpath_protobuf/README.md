# polars_structpath_protobuf

A Rust library that automatically generates [polars_structpath](https://github.com/jmunar/polarspath)
implementations for Protocol Buffer messages, enabling type-safe field access and Polars integration.

## Quick Start

To create a new project using `polars_structpath_protobuf`, run the project generator script:

```bash
./create_project.sh
```

The script will:
- Create a new cargo project with all necessary dependencies
- Set up the build configuration (`build.rs`)
- Create a Python package with the same name
- Generate a `pyproject.toml` for Python packaging
- Create a `Makefile` for building the project
- Optionally create sample protobuf messages and tests

## Features

- Automatically derives `StructPath` and `EnumPath` traits for protobuf messages and enums
- Adds type hints for nested messages and enums
- Generates Polars extension code for Python integration
- Enables field access using path strings (e.g., `"user.name"`, `"pets[0].birth_year"`)

## Example Usage

After creating your project, you can use polars_structpath to access protobuf fields:

```rust
use my_project::my_package;
use polars_structpath::StructPath;

let person = my_package::Person::default();
let name = person.get_value("name")?;
let street = person.get_value("address.street")?;
```

## See Also

- [polars_structpath](../polars_structpath/README.md) - Core polars_structpath library
- [example_protobuf](../example_protobuf/) - Complete working example
