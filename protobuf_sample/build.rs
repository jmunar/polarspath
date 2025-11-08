use prost_build::Config;
use std::fs;
use std::path::Path;

/// Extract the qualified type name from a protobuf fully-qualified path.
/// For nested types, returns the module path::Type format including package name.
/// Examples:
/// - ".sample.User.Loyalty" -> "user::Loyalty" (enum inside User message)
/// - ".sample.Loyalty" -> "Loyalty" (top-level enum)
/// - ".sample.User" -> "User" (message)
fn extract_type_name(package_name: &str, type_name: &str) -> String {
    // Protobuf identifiers always start with '.' for fully-qualified paths
    let parts: Vec<&str> = type_name.trim_start_matches('.').split('.').collect();

    // Make sure that there are at least two parts, and the first part is the package name
    if parts.len() < 2 {
        panic!("Invalid type name: {}", type_name);
    }
    if parts[0] != package_name {
        panic!(
            "Package name mismatch: expected {} but got {}",
            package_name, parts[0]
        );
    }

    // Remove the first part (package name)
    let parts = parts[1..].to_vec();

    // For nested types, convert all parts except the last to snake_case
    let type_name_part = parts.last().unwrap();

    let module_path: Vec<String> = parts[..parts.len() - 1]
        .iter()
        .map(|part| to_snake(part))
        .collect();

    format!("{}::{}", module_path.join("::"), type_name_part)
}

/// Convert a CamelCase or PascalCase string to snake_case.
/// Example: "UserLoyalty" -> "user_loyalty"
fn to_snake(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            // Add underscore before uppercase if not at start and previous char was lowercase
            if i > 0 && chars[i - 1].is_lowercase() {
                result.push('_');
            }
            result.push_str(&c.to_lowercase().to_string());
        } else {
            result.push(*c);
        }
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schemas_dir = Path::new("protobuf/sample");

    // Walk through the protobuf directory recursively to find all proto files
    let mut proto_files = Vec::new();
    for entry in fs::read_dir(schemas_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "proto") {
            proto_files.push(path.to_string_lossy().into_owned());
        }
    }

    let mut config = Config::new();

    // Load the file descriptor set from the proto files to analyze the types
    let file_descriptor_set = config.load_fds(&proto_files, &["protobuf/sample"])?;

    // Apply derive attribute to all message types at once
    config.enum_attribute(".", "#[derive(structpath::EnumPath)]");
    config.enum_attribute(".", "#[enum_path(camel_case_to_upper_snake_case)]");
    config.message_attribute(".", "#[derive(structpath::StructPath)]");

    for file in &file_descriptor_set.file {
        let empty_package = String::new();
        let package_name = file.package.as_ref().unwrap_or(&empty_package);

        for message in &file.message_type {
            let message_name = message.name.as_ref().unwrap();
            for field in &message.field {
                let field_name = field.name.as_ref().unwrap();
                let field_path = format!(".{}.{}.{}", package_name, message_name, field_name);
                match field.r#type().as_str_name() {
                    "TYPE_DOUBLE" => continue,
                    "TYPE_FLOAT" => continue,
                    "TYPE_INT64" => continue,
                    "TYPE_UINT64" => continue,
                    "TYPE_INT32" => continue,
                    "TYPE_FIXED64" => continue,
                    "TYPE_FIXED32" => continue,
                    "TYPE_BOOL" => continue,
                    "TYPE_STRING" => continue,
                    "TYPE_GROUP" => panic!("TYPE_GROUP not supported"), // Proto2 syntax only, and deprecated.
                    "TYPE_MESSAGE" => {
                        config.field_attribute(&field_path, "#[type_hint(\"struct\")]")
                    }
                    "TYPE_BYTES" => panic!("TYPE_BYTES not supported"),
                    "TYPE_UINT32" => continue,
                    "TYPE_ENUM" => {
                        let enum_type_name = extract_type_name(package_name, field.type_name());
                        config.field_attribute(
                            &field_path,
                            format!("#[type_hint(\"enum\", {:?})]", enum_type_name),
                        )
                    }
                    "TYPE_SFIXED32" => continue,
                    "TYPE_SFIXED64" => continue,
                    _ => panic!("Unknown field type: {}", field.r#type().as_str_name()),
                };
            }
        }
    }

    // Generate the Rust code for the types
    config.compile_fds(file_descriptor_set)?;

    Ok(())
}
