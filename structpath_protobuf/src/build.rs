use prost_build::Config;
use std::fs;
use std::path::Path;

/// Configuration for building protobuf files with structpath support.
#[derive(Default)]
pub struct BuildConfig {
    /// Path to the directory containing .proto files
    pub proto_dir: String,
    /// Include paths for protobuf compilation
    pub include_paths: Vec<String>,
}

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

/// Build protobuf files with structpath support.
///
/// This function:
/// 1. Finds all .proto files in the specified directory
/// 2. Applies structpath derives to all message and enum types
/// 3. Adds type hints for message and enum fields
/// 4. Compiles the protobuf files to Rust code
///
/// # Arguments
///
/// * `config` - Configuration specifying the proto directory and include paths
///
/// # Example
///
/// ```no_run
/// use structpath_protobuf::build::BuildConfig;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     structpath_protobuf::build::build_protobuf(BuildConfig {
///         proto_dir: "protobuf/sample".to_string(),
///         include_paths: vec!["protobuf/sample".to_string()],
///     })?;
///     Ok(())
/// }
/// ```
pub fn build_protobuf(config: BuildConfig) -> Result<(), Box<dyn std::error::Error>> {
    let schemas_dir = Path::new(&config.proto_dir);

    // Walk through the protobuf directory recursively to find all proto files
    let mut proto_files = Vec::new();
    for entry in fs::read_dir(schemas_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "proto") {
            proto_files.push(path.to_string_lossy().into_owned());
        }
    }

    let mut prost_config = Config::new();

    // Load the file descriptor set from the proto files to analyze the types
    let include_paths: Vec<&str> = config.include_paths.iter().map(|s| s.as_str()).collect();
    let file_descriptor_set = prost_config.load_fds(&proto_files, &include_paths)?;

    // Apply derive attribute to all message types at once
    prost_config.enum_attribute(".", "#[derive(structpath::EnumPath)]");
    prost_config.enum_attribute(".", "#[enum_path(camel_case_to_upper_snake_case)]");
    prost_config.message_attribute(".", "#[derive(structpath::StructPath)]");

    for file in &file_descriptor_set.file {
        let empty_package = String::new();
        let package_name = file.package.as_ref().unwrap_or(&empty_package);

        for message in &file.message_type {
            let message_name = message.name.as_ref().unwrap();
            for field in &message.field {
                let field_name = field.name.as_ref().unwrap();
                let field_path = format!(".{}.{}.{}", package_name, message_name, field_name);
                match field.r#type().as_str_name() {
                    "TYPE_DOUBLE" | "TYPE_FLOAT" | "TYPE_INT64" | "TYPE_UINT64" | "TYPE_INT32"
                    | "TYPE_FIXED64" | "TYPE_FIXED32" | "TYPE_BOOL" | "TYPE_STRING"
                    | "TYPE_GROUP" | "TYPE_BYTES" | "TYPE_UINT32" | "TYPE_SFIXED32"
                    | "TYPE_SFIXED64" => continue,
                    "TYPE_MESSAGE" => {
                        prost_config.field_attribute(&field_path, "#[type_hint(\"struct\")]")
                    }
                    "TYPE_ENUM" => {
                        let enum_type_name = extract_type_name(package_name, field.type_name());
                        prost_config.field_attribute(
                            &field_path,
                            format!("#[type_hint(\"enum\", {:?})]", enum_type_name),
                        )
                    }
                    _ => panic!("Unknown field type: {}", field.r#type().as_str_name()),
                };
            }
        }
    }

    // Generate the Rust code for the types
    prost_config.compile_fds(file_descriptor_set)?;

    Ok(())
}
