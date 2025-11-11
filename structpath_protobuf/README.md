# structpath_protobuf

A Rust library that automatically generates [structpath](https://github.com/your-org/structpath) implementations for Protocol Buffer messages, enabling type-safe field access and Polars integration.

## Overview

`structpath_protobuf` provides build-time code generation that:
- Automatically derives `StructPath` and `EnumPath` traits for protobuf messages and enums
- Adds type hints for nested messages and enums
- Generates Polars extension code for Python integration (optional)
- Enables field access using path strings (e.g., `"user.name"`, `"pets[0].birth_year"`)

## Quick Start: Creating a New Project

This guide walks you through creating a completely new cargo project that uses `structpath_protobuf` with your own protobuf messages.

The files defined below should lead you to the following project structure:

```
my_protobuf_project/
├── Cargo.toml
├── build.rs
├── src/
│   └── lib.rs
├── protobuf/
│   └── my_package/
│       └── person.proto
└── tests/
    └── test_person.rs
```

### Step 1: Create a New Cargo Project

```bash
cargo new --lib my_protobuf_project
cd my_protobuf_project
```

### Step 2: Add Dependencies to `Cargo.toml`

Add the following dependencies to your `Cargo.toml`:

```toml
[package]
name = "my_protobuf_project"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
prost = "*"
structpath = { version = "*", features = ["derive"] }
structpath_protobuf = { version = "*" }

[build-dependencies]
structpath_protobuf = { path = "*", features = ["build"] }
prost-build = "*"
prost-types = "*"

[features]
default = []
extension-module = ["pyo3", "pyo3/extension-module", "pyo3-polars", "serde"]

[dependencies]
pyo3 = { version = "*", optional = true }
pyo3-polars = { version = "*", optional = true }
serde = { version = "*", optional = true }
```

### Step 3: Create Your Protobuf Files

Create a directory for your protobuf definitions:

```bash
mkdir -p protobuf/my_package
```

Create a simple protobuf file `protobuf/my_package/person.proto`:

```protobuf
syntax = "proto3";

package my_package;

message Person {
  string name = 1;
  int64 age = 2;
  optional string email = 3;
  bool is_active = 4;
  
  message Address {
    string street = 1;
    string city = 2;
    int32 zip_code = 3;
  }
  
  enum Status {
    UNKNOWN = 0;
    ACTIVE = 1;
    INACTIVE = 2;
  }
  
  Address address = 5;
  repeated string tags = 6;
  Status status = 7;
  repeated Address previous_addresses = 8;
}
```

### Step 4: Create `build.rs`

Create a `build.rs` file in the root of your project:

```rust
use structpath_protobuf::build::{build_protobuf, BuildConfig, ExtensionConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_protobuf(BuildConfig {
        proto_dir: "protobuf/my_package".to_string(),
        include_paths: vec!["protobuf/my_package".to_string()],
        generate_extensions: Some(ExtensionConfig {
            python_package_dir: "my_protobuf_project".to_string(),
            python_package_name: "my_protobuf_project".to_string(),
        }),
    })?;

    println!("cargo:rerun-if-changed=protobuf/my_package");
    Ok(())
}
```

### Step 5: Create `src/lib.rs`

Create or update `src/lib.rs` to include the generated protobuf code:

```rust
// Include the generated protobuf code
// The module name should match your package name (converted to snake_case)
pub mod my_package {
    include!(concat!(env!("OUT_DIR"), "/my_package.rs"));
}

// Extension code being called from python
#[cfg(feature = "extension-module")]
include!(concat!(env!("OUT_DIR"), "/extension_generated.rs"));
```

### Step 6: Write Rust Tests

Create `tests/test_person.rs` to verify everything works:

```rust
use my_protobuf_project::my_package;
use structpath::polars_core::prelude::{AnyValue, DataType, Field};
use structpath::{data_type_wrapper, StructPath};

#[test]
fn test_get_type_person() -> Result<(), Box<dyn std::error::Error>> {
    // Test that we can get the type of the Person struct
    let person_type = my_package::Person::data_type();
    assert!(matches!(person_type, DataType::Struct(_)));
    Ok(())
}

#[test]
fn test_get_type_fields() -> Result<(), Box<dyn std::error::Error>> {
    // Test getting types of individual fields
    let name_type = my_package::Person::get_type("name")?;
    assert_eq!(name_type, data_type_wrapper!(String));
    
    let age_type = my_package::Person::get_type("age")?;
    assert_eq!(age_type, data_type_wrapper!(Int64));
    
    let email_type = my_package::Person::get_type("email")?;
    assert_eq!(email_type, data_type_wrapper!(Option(String)));
    
    // Test nested field access
    let street_type = my_package::Person::get_type("address.street")?;
    assert_eq!(street_type, data_type_wrapper!(Option(String)));
    
    // Test array access
    let tag_type = my_package::Person::get_type("tags")?;
    assert_eq!(tag_type, data_type_wrapper!(List(String)));
    
    let tag0_type = my_package::Person::get_type("tags[0]")?;
    assert_eq!(tag0_type, data_type_wrapper!(String));
    
    Ok(())
}

#[test]
fn test_get_value_person() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Person instance
    let mut person = my_package::Person::default();
    person.name = "Alice".to_string();
    person.age = 30;
    person.email = Some("alice@example.com".to_string());
    person.is_active = true;
    
    // Set nested address
    person.address = Some(my_package::person::Address {
        street: "123 Main St".to_string(),
        city: "Springfield".to_string(),
        zip_code: 12345,
    });
    
    // Add tags
    person.tags.push("premium".to_string());
    person.tags.push("verified".to_string());
    
    // Set enum
    person.status = 1; // ACTIVE
    
    // Test getting values
    let name = person.get_value("name")?;
    assert_eq!(name, AnyValue::String("Alice"));
    
    let age = person.get_value("age")?;
    assert_eq!(age, AnyValue::Int64(30));
    
    let email = person.get_value("email")?;
    assert_eq!(email, AnyValue::String("alice@example.com"));
    
    // Test nested field access
    let street = person.get_value("address.street")?;
    assert_eq!(street, AnyValue::String("123 Main St"));
    
    // Test array access
    let tag0 = person.get_value("tags[0]")?;
    assert_eq!(tag0, AnyValue::String("premium"));
    
    // Test enum
    let status = person.get_value("status")?;
    // Status will be an Enum AnyValue
    
    Ok(())
}

#[test]
fn test_get_value_nested_array() -> Result<(), Box<dyn std::error::Error>> {
    let mut person = my_package::Person::default();
    
    // Add previous addresses
    person.previous_addresses.push(my_package::person::Address {
        street: "456 Old St".to_string(),
        city: "Oldtown".to_string(),
        zip_code: 54321,
    });
    
    person.previous_addresses.push(my_package::person::Address {
        street: "789 New St".to_string(),
        city: "Newtown".to_string(),
        zip_code: 98765,
    });
    
    // Test nested array access
    let first_old_street = person.get_value("previous_addresses[0].street")?;
    assert_eq!(first_old_street, AnyValue::String("456 Old St"));
    
    let second_old_city = person.get_value("previous_addresses[1].city")?;
    assert_eq!(second_old_city, AnyValue::String("Newtown"));
    
    Ok(())
}
```

### Step 7: Build and Test in Rust

Build your project:

```bash
cargo build
```

Run the tests:

```bash
cargo test
```

If everything compiles and tests pass, your setup is working correctly!

### Step 8: Create Python extension

If you want to use your protobuf messages from Python with Polars:

1. Enable the `extension-module` feature in `Cargo.toml`
2. Set `generate_extensions` in `build.rs` (see Step 4)
3. Create `pyproject.toml` for Python packaging
4. Build with `maturin` or `pyo3-pack`

The generated Python code will provide a class-based API:

```python
import polars as pl
from my_protobuf_project import Person

# Extract fields from binary protobuf data
df = pl.DataFrame({
    "data": binary_protobuf_column
})

result = df.with_columns([
    Person.get_value(pl.col("data"), "name").alias("name"),
    Person.get_value(pl.col("data"), "age").alias("age"),
    Person.get_value(pl.col("data"), "address.street").alias("street"),
])
```

## Troubleshooting

### Build Errors

- **"Package name mismatch"**: Ensure the package name in your `.proto` file matches what you're using in `build.rs`
- **"Cannot find module"**: Check that the module path in `lib.rs` matches your package name (converted to snake_case)

### Test Failures

- **Type mismatches**: Verify your protobuf field types match what you expect
- **Missing fields**: Ensure all required fields are set when creating test instances

## Validating the Process

To verify that the process described in this README works correctly, you can:

1. **Check the existing example**: The `protobuf_sample` project in this repository follows these exact steps and can serve as a reference implementation.

2. **Run the example tests**: From the repository root, run:
   ```bash
   cd protobuf_sample
   cargo test
   ```

3. **Follow the steps yourself**: Create a new project following this README and verify:
   - `cargo build` succeeds
   - `cargo test` passes all tests
   - You can access fields using `get_type()` and `get_value()`

The `protobuf_sample` project demonstrates:
- Basic message types (`User`, `Group`)
- Nested messages (`Pet` inside `User`)
- Enums (`Loyalty`)
- Repeated fields (`tags`, `pets`)
- Optional fields (`email`, `favourite_pet`)
- Complex nested paths (`members[0].pets[0].birth_year`)

## See Also

- [structpath](../structpath/README.md) - Core structpath library
- [protobuf_sample](../protobuf_sample/) - Complete working example that validates this process

