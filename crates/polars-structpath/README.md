# polars-structpath

The main user-facing library for converting Rust structures to and from Apache Arrow arrays, with seamless integration to Polars DataFrames. This crate provides a unified API that re-exports all necessary types, traits, and derive macros from the underlying implementation crates.

## Purpose

`polars-structpath` is the primary entry point for the `polars-structpath` ecosystem. It provides:

- **Unified API**: A single crate that re-exports all necessary types, traits, and derive macros
- **Arrow Buffer Conversion**: Bidirectional conversion between Rust types and Arrow arrays via `IntoArrow` and `FromArrow` traits
- **Derive Macros**: Procedural macros (`StructPath`, `EnumPath`) that automatically generate Arrow buffer implementations
- **Polars Integration**: Native support for Polars DataFrames using Arrow arrays

This crate contains the core types and traits directly, and optionally re-exports derive macros:
- Core types and traits: `ArrowBuffer`, `IntoArrow`, `FromArrow`
- `polars-structpath-derive`: Derive macro implementations (optional, enabled via `derive` feature)

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
polars-structpath = { version = "*", features = ["derive"] }
```

### Converting to Arrow Arrays

The derive macros automatically generate `IntoArrow` implementations, enabling conversion to Arrow arrays:

```rust
use polars_structpath::{StructPath, IntoArrow, ArrowBuffer};
use polars_core::prelude::*;

#[derive(StructPath, Debug, Clone, PartialEq)]
pub struct Person {
    name: String,
    age: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a buffer and push values
    let mut buffer = Person::new_buffer(2);
    buffer.push(Person {
        name: "Alice".to_string(),
        age: 30,
    });
    buffer.push(Person {
        name: "Bob".to_string(),
        age: 25,
    });

    // Convert to Arrow array
    let array = buffer.to_arrow()?;

    // Use with Polars
    let series = Series::from_arrow("person".into(), Box::new(array))?;
    let df = DataFrame::new(vec![series.into()])?;
    
    Ok(())
}
```

### Converting from Arrow Arrays

The derive macros also generate `FromArrow` implementations for bidirectional conversion:

```rust
use polars_structpath::{StructPath, IntoArrow, FromArrow, ArrowBuffer};

#[derive(StructPath, Debug, Clone, PartialEq)]
pub struct User {
    name: String,
    age: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and populate a buffer
    let mut buffer = User::new_buffer(2);
    buffer.push(User {
        name: "Alice".to_string(),
        age: 30,
    });
    buffer.push(User {
        name: "Bob".to_string(),
        age: 25,
    });

    // Convert to Arrow array
    let array = buffer.to_arrow()?;

    // Convert back from Arrow array
    let users: Vec<User> = User::from_arrow(Box::new(array));
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name, "Alice");
    assert_eq!(users[1].name, "Bob");

    Ok(())
}
```

### Handling Nullable Arrays

Use `from_arrow_opt()` when arrays may contain null values:

```rust
use polars_structpath::{StructPath, IntoArrow, FromArrow, ArrowBuffer};

#[derive(StructPath, Debug, Clone, PartialEq)]
pub struct Person {
    name: String,
    age: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Person::new_buffer(3);
    buffer.push(Person {
        name: "Alice".to_string(),
        age: 30,
    });
    buffer.push_null(); // Push a null Person
    buffer.push(Person {
        name: "Bob".to_string(),
        age: 25,
    });

    let array = buffer.to_arrow()?;
    
    // Convert back, preserving null information
    let people: Vec<Option<Person>> = Person::from_arrow_opt(Box::new(array));
    assert_eq!(people.len(), 3);
    assert_eq!(people[0], Some(Person { name: "Alice".to_string(), age: 30 }));
    assert_eq!(people[1], None);
    assert_eq!(people[2], Some(Person { name: "Bob".to_string(), age: 25 }));

    Ok(())
}
```

### Enums

Enums with explicit discriminants can also derive `EnumPath`:

```rust
use polars_structpath::{EnumPath, IntoArrow, FromArrow, ArrowBuffer};

#[derive(EnumPath, Debug, PartialEq)]
pub enum Status {
    Active = 1,
    Inactive = 2,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Status::new_buffer(3);
    buffer.push(Status::Active);
    buffer.push(Status::Inactive);
    buffer.push(Status::Active);

    let array = buffer.to_arrow()?;
    
    // Convert back from Arrow array
    let statuses: Vec<Status> = Status::from_arrow(Box::new(array));
    assert_eq!(statuses[0], Status::Active);
    assert_eq!(statuses[1], Status::Inactive);
    assert_eq!(statuses[2], Status::Active);

    Ok(())
}
```

## Core Traits

### `IntoArrow`

Types implementing `IntoArrow` can create Arrow buffers for accumulating values:

```rust
use polars_structpath::{IntoArrow, ArrowBuffer};

let mut buffer = String::new_buffer(2);
buffer.push("hello");
buffer.push("world");
let array = buffer.to_arrow().unwrap();
```

### `FromArrow`

Types implementing `FromArrow` can be converted back from Arrow arrays:

```rust
use polars_structpath::{IntoArrow, FromArrow, ArrowBuffer};

let mut buffer = i32::new_buffer(3);
buffer.push(1);
buffer.push(2);
buffer.push(3);

let array = buffer.to_arrow().unwrap();
let values: Vec<i32> = i32::from_arrow(Box::new(array));
```

### `ArrowBuffer`

The `ArrowBuffer` trait is used internally to accumulate values and convert them to Arrow arrays. Types implementing `IntoArrow` have an associated `Buffer` type that implements `ArrowBuffer`.

## Features

- **`derive`** (default): Enables the `StructPath` and `EnumPath` derive macros
- **`std`** (default): Standard library support

## Supported Types

The derive macros support all types that implement `IntoArrow` and `FromArrow`:

- **Primitive types**: `i32`, `i64`, `u8`, `u32`, `u64`, `f32`, `f64`, `bool`
- **Strings**: `String`
- **Collections**: `Vec<T>`, `Option<T>`
- **Nested combinations**: `Option<Vec<T>>`, `Vec<Option<T>>`, etc.
- **Custom types**: Any struct or enum that also derives `StructPath` or `EnumPath`

## What the Derive Macros Generate

When you apply `#[derive(StructPath)]` or `#[derive(EnumPath)]`, the macros automatically generate:

- A buffer struct (e.g., `UserBuffer`) implementing `ArrowBuffer`
- `IntoArrow` implementation for converting Rust types to Arrow arrays
- `FromArrow` implementation for converting Arrow arrays back to Rust types

This enables **bidirectional conversion** between Rust types and Arrow arrays, making it easy to:
- Serialize Rust data structures to Arrow format for Polars DataFrames
- Deserialize Arrow arrays back to Rust types after processing

## Integration with Polars

The Arrow arrays produced by this crate are fully compatible with Polars DataFrames:

```rust
use polars_core::prelude::*;
use polars_structpath::{StructPath, IntoArrow, ArrowBuffer};

#[derive(StructPath)]
pub struct Person {
    name: String,
    age: i64,
}

let mut buffer = Person::new_buffer(3);
buffer.push(Person { name: "Alice".to_string(), age: 30 });
buffer.push(Person { name: "Bob".to_string(), age: 25 });
buffer.push(Person { name: "Charlie".to_string(), age: 35 });

let array = buffer.to_arrow().unwrap();
let series = Series::from_arrow("person".into(), Box::new(array)).unwrap();
let df = DataFrame::new(vec![series.into()]).unwrap();
```

## See Also

- [Main README](../../README.md) - Overview of the entire polars-structpath ecosystem
- [polars-structpath-derive](../polars-structpath-derive/README.md) - Derive macro implementation details
- [polars-protobuf](../polars-protobuf/README.md) - Protocol Buffers integration

