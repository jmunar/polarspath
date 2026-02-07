# polars-structpath-types

This crate provides the core types and traits for converting Rust types to and from Apache Arrow arrays, forming the foundation of the `polars-structpath` ecosystem. It enables seamless bidirectional conversion between Rust data structures and Polars-compatible Arrow arrays.

## Purpose

`polars-structpath-types` is a foundational library that provides:

- **Arrow Buffer Traits**: Core traits (`ArrowBuffer`, `IntoArrow`, `FromArrow`) for converting Rust types to Arrow arrays
- **Buffer Implementations**: Ready-to-use buffer implementations for:
  - Primitive types: `i32`, `i64`, `u8`, `u32`, `u64`, `f32`, `f64`, `bool`
  - Strings: `String`
  - Collections: `Vec<T>` and `Option<T>` with full nesting support
  - Custom types: Structs and enums via macros
- **Conversion Macros**: Procedural macros for generating Arrow buffer code:
  - `impl_struct_buffer!`: Generates buffer implementations for structs
  - `impl_enum_buffer!`: Generates buffer implementations for enums
- **Type Safety**: Compile-time type checking ensures correct Arrow array generation

## Core Traits

### `ArrowBuffer`

The main trait for types that can accumulate values and convert them to Arrow arrays:

```rust,ignore
pub trait ArrowBuffer {
    type Element;
    type Arrow: Array;
    
    fn new(nrows: usize) -> Self;
    fn push(&mut self, value: impl Into<Self::Element>);
    fn push_null(&mut self);
    fn to_arrow(self) -> PolarsResult<Self::Arrow>;
}
```

### `IntoArrow`

Marks types that can be converted to Arrow arrays. Provides a convenience method `new_buffer()`:

```rust,ignore
pub trait IntoArrow: Sized {
    type Buffer: ArrowBuffer;
    
    fn new_buffer(nrows: usize) -> Self::Buffer;
}
```

### `FromArrow`

Enables conversion from Arrow arrays back to Rust types:

```rust,ignore
pub trait FromArrow where Self: Sized {
    fn from_arrow(array: Box<dyn Array>) -> Vec<Self>;
    fn from_arrow_opt(array: Box<dyn Array>) -> Vec<Option<Self>>;
}
```

## Usage Examples

### Using Built-in Types

All primitive types, strings, and collections implement `IntoArrow` and `FromArrow`:

```rust
use polars_structpath_types::{ArrowBuffer, IntoArrow, FromArrow};

// Create a buffer for i32 values
let mut buffer = i32::new_buffer(3);
buffer.push(1);
buffer.push(2);
buffer.push_null();

// Convert to Arrow array
let array = buffer.to_arrow().unwrap();

// Convert back from Arrow array
let values: Vec<i32> = i32::from_arrow(Box::new(array));
```

### Custom Structs

Use the `impl_struct_buffer!` macro to generate buffer code for your structs:

```rust
use polars_structpath_types::{ArrowBuffer, IntoArrow, impl_struct_buffer};

pub struct Person {
    name: String,
    age: i32,
}

impl_struct_buffer!(
    Person,
    [(name, String), (age, i32)]
);

// Now Person implements IntoArrow and FromArrow
let mut buffer = Person::new_buffer(2);
buffer.push(Person {
    name: "Alice".to_string(),
    age: 30,
});
let array = buffer.to_arrow().unwrap();
```

### Custom Enums

Use the `impl_enum_buffer!` macro for enums:

```rust
use polars_structpath_types::{ArrowBuffer, IntoArrow, impl_enum_buffer};

pub enum Status {
    Active = 1,
    Inactive = 2,
}

impl_enum_buffer!(Status, [(Active, 1), (Inactive, 2)]);

// Status now implements IntoArrow and FromArrow
let mut buffer = Status::new_buffer(2);
buffer.push(Status::Active);
buffer.push(Status::Inactive);
let array = buffer.to_arrow().unwrap();
```

### Nested Types

The crate fully supports nested `Option` and `Vec` types:

```rust
use polars_structpath_types::{ArrowBuffer, IntoArrow};

// Option<String>
let mut buffer = Option::<String>::new_buffer(2);
buffer.push(Some("hello".to_string()));
buffer.push(None);

// Vec<i32>
let mut buffer = Vec::<i32>::new_buffer(2);
buffer.push(vec![1, 2, 3]);
buffer.push(vec![4, 5]);

// Option<Vec<String>>
let mut buffer = Option::<Vec<String>>::new_buffer(2);
buffer.push(Some(vec!["a".to_string(), "b".to_string()]));
buffer.push(None);
```

## Integration with Polars

The Arrow arrays produced by this crate are fully compatible with Polars DataFrames:

```rust
use polars_core::prelude::*;
use polars_structpath_types::{ArrowBuffer, IntoArrow, FromArrow};

let mut buffer = String::new_buffer(3);
buffer.push("Alice");
buffer.push("Bob");
buffer.push("Charlie");

let array = buffer.to_arrow().unwrap();
let series = Series::from_arrow("names".into(), Box::new(array)).unwrap();
let df = DataFrame::new(vec![series.into()]).unwrap();
```

## This crate is used by:

- `polars-structpath`: The main user-facing library that re-exports these types
- `polars-structpath-derive`: The derive macro implementation that generates code using these types
- `polars-protobuf`: Protocol Buffers integration that leverages these types

## See Also

- [Main README](../../README.md) - Overview of the entire `polarspath` ecosystem
- [polars-structpath](../polars-structpath/README.md) - User-facing API documentation
- [polars-structpath-derive](../polars-structpath-derive/) - Derive macro implementation
- [polars-protobuf](../polars-protobuf/README.md) - Protocol Buffers integration

