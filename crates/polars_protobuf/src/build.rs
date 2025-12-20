use prost_build::Config;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::string::{indent_lines, to_snake_case};

/// Configuration for building protobuf files with polars_structpath support.
///
/// This struct configures the build process for protobuf files, including where to find
/// `.proto` files, include paths for protobuf compilation, and optional Python extension
/// generation.
///
/// # Example
///
/// ```rust,no_run
/// use polars_protobuf::build::{BuildConfig, ExtensionConfig};
///
/// let config = BuildConfig {
///     proto_dir: "protobuf/sample".to_string(),
///     include_paths: vec!["protobuf/sample".to_string()],
///     generate_extensions: Some(ExtensionConfig {
///         python_package_dir: "example_protobuf/example_protobuf".to_string(),
///         python_package_name: "example_protobuf".to_string(),
///     }),
/// };
/// ```
#[derive(Default)]
pub struct BuildConfig {
    /// Path to the directory containing .proto files
    pub proto_dir: String,
    /// Include paths for protobuf compilation
    pub include_paths: Vec<String>,
    /// Optional: Generate Polars extension code and Python modules
    pub generate_extensions: Option<ExtensionConfig>,
}

/// Configuration for generating Polars extension code and Python modules.
///
/// When provided in `BuildConfig::generate_extensions`, this configuration enables
/// automatic generation of:
/// - Rust extension code for Polars plugin functions
/// - Python wrapper modules that provide a class-based API for accessing protobuf fields
///
/// The generated Python module will be placed at `{python_package_dir}/structpath.py` and
/// will provide a class-based API like `sample.User.get_value()` for accessing protobuf
/// message fields in Polars DataFrames.
///
/// # Example
///
/// ```no_run
/// use polars_protobuf::build::ExtensionConfig;
///
/// let ext_config = ExtensionConfig {
///     python_package_dir: "example_protobuf/example_protobuf".to_string(),
///     python_package_name: "example_protobuf".to_string(),
/// };
/// ```
#[derive(Clone)]
pub struct ExtensionConfig {
    /// Path to the Python package directory (e.g., "example_protobuf/example_protobuf")
    pub python_package_dir: String,
    /// Name of the Python package (e.g., "example_protobuf")
    pub python_package_name: String,
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
        .map(|part| to_snake_case(part))
        .collect();

    format!("{}::{}", module_path.join("::"), type_name_part)
}

/// Build protobuf files with polars_structpath support.
///
/// This function:
/// 1. Finds all .proto files in the specified directory
/// 2. Applies polars_structpath derives to all message and enum types
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
/// use polars_protobuf::build::BuildConfig;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     polars_protobuf::build::build_protobuf(BuildConfig {
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
    prost_config.enum_attribute(".", "#[derive(polars_structpath::EnumPath)]");
    prost_config.enum_attribute(".", "#[enum_path(camel_case_to_upper_snake_case)]");
    prost_config.message_attribute(".", "#[derive(polars_structpath::StructPath)]");

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

    // Generate extension code if requested (before compile_fds consumes file_descriptor_set)
    if let Some(ext_config) = &config.generate_extensions {
        generate_extensions(&file_descriptor_set, ext_config)?;
    }

    // Generate the Rust code for the types
    prost_config.compile_fds(file_descriptor_set)?;

    Ok(())
}

/// Extract the Rust module path for a message type
fn extract_rust_path(package_name: &str, message_name: &str) -> String {
    let package_snake = to_snake_case(package_name);
    format!("{}::{}", package_snake, message_name)
}

/// Generate Rust extension code for protobuf messages
fn generate_rust_file(
    extension_file: &mut fs::File,
    messages: &[(String, String, String, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(
        extension_file,
        "// Auto-generated extension code
use polars_core::prelude::{{BinaryType, ChunkedArray, Field, PolarsResult, Series}};
use pyo3_polars::derive::{{polars_expr, CallerContext}};
use serde::Deserialize;
use polars_protobuf::{{get_type, get_value}};

#[derive(Deserialize)]
pub struct ExtractKwargs {{
    path: String,
}}

"
    )?;

    for (_package_name, _message_name, rust_path, function_prefix) in messages {
        writeln!(
            extension_file,
            "// Auto-generated function for type inference
fn {}_get_type(input_fields: &[Field], kwargs: ExtractKwargs) -> PolarsResult<Field> {{
    let path = kwargs.path.as_str();
    get_type::<crate::{}>(input_fields, path)
}}

// Auto-generated function for value extraction
#[polars_expr(output_type_func_with_kwargs={}_get_type)]
pub fn {}_get_value(
    inputs: &[Series],
    context: CallerContext,
    kwargs: ExtractKwargs,
) -> PolarsResult<Series> {{
    let ca: &ChunkedArray<BinaryType> = inputs[0].binary()?;
    let path = kwargs.path.as_str();
    get_value::<crate::{}>(ca, path, context.parallel())
}}

",
            function_prefix, rust_path, function_prefix, function_prefix, rust_path
        )?;
    }

    Ok(())
}

/// Tree structure for organizing packages and messages
struct PackageNode {
    messages: Vec<(String, String)>, // (message_name, function_prefix)
    subpackages: HashMap<String, PackageNode>,
}

impl PackageNode {
    fn new() -> Self {
        PackageNode {
            messages: Vec::new(),
            subpackages: HashMap::new(),
        }
    }
}

/// Build a tree structure from flat package names
fn build_package_tree(
    packages: &HashMap<String, Vec<(&String, &String)>>,
) -> HashMap<String, PackageNode> {
    let mut root = HashMap::new();

    for (package_name, messages) in packages {
        // Handle empty package name - put messages in a special root node
        if package_name.is_empty() {
            let node = root
                .entry("_root".to_string())
                .or_insert_with(PackageNode::new);
            for (message_name, function_prefix) in messages {
                node.messages
                    .push((message_name.to_string(), function_prefix.to_string()));
            }
            continue;
        }

        let parts: Vec<&str> = package_name.split('.').filter(|s| !s.is_empty()).collect();
        if parts.is_empty() {
            continue;
        }

        let mut current = &mut root;

        // Navigate/create the tree structure
        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                // Last part - add messages
                let node = current
                    .entry(part.to_string())
                    .or_insert_with(PackageNode::new);
                for (message_name, function_prefix) in messages {
                    node.messages
                        .push((message_name.to_string(), function_prefix.to_string()));
                }
            } else {
                // Intermediate part - create subpackage node
                current = &mut current
                    .entry(part.to_string())
                    .or_insert_with(PackageNode::new)
                    .subpackages;
            }
        }
    }

    root
}

/// Generate nested Python classes recursively
fn generate_python_classes(
    python_file: &mut fs::File,
    node: &PackageNode,
    indent: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let indent_str = "    ".repeat(indent);

    // Generate message classes
    for (message_name, function_prefix) in &node.messages {
        let template = format!(
            "class {message_name}:
    \"\"\"
    Protobuf message type: {message_name}
    \"\"\"

    @staticmethod
    def get_value(expr: IntoExprColumn, path: str) -> \"Expr\":
        \"\"\"
        Extract a field from a binary protobuf column of type {message_name}

        Args:
            expr: Polars expression or column name
            path: Field path to extract (e.g., 'name', 'pets[0].name')

        Returns:
            Polars expression
        \"\"\"
        return register_plugin_function(
            args=[expr],
            kwargs={{\"path\": path}},
            plugin_path=LIB,
            function_name=\"{function_prefix}_get_value\",
            is_elementwise=True,
        )

",
            message_name = message_name,
            function_prefix = function_prefix
        );

        // Add indentation to each line
        let indented = indent_lines(&template, &indent_str);
        write!(python_file, "{}", indented)?;
    }

    // Generate subpackage classes
    for (subpackage_name, subpackage_node) in &node.subpackages {
        write!(python_file, "{}class {}:\n\n", indent_str, subpackage_name)?;
        generate_python_classes(python_file, subpackage_node, indent + 1)?;
    }

    Ok(())
}

/// Generate Python main module with imports, setup, and nested message classes
fn generate_python_file(
    python_file: &mut fs::File,
    packages: &HashMap<String, Vec<(&String, &String)>>,
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(
        python_file,
        "\"\"\"
Auto-generated Python module for protobuf message access.
Provides class-based API like sample.User.get_value()
\"\"\"

from pathlib import Path
from typing import TYPE_CHECKING

from polars._typing import IntoExprColumn
from polars.plugins import register_plugin_function

if TYPE_CHECKING:
    from polars import Expr

# Get the path to the compiled library
# Polars expects either a directory or the full path to the .so file
LIB = Path(__file__).parent

"
    )?;

    // Build package tree
    let package_tree = build_package_tree(packages);

    // Collect top-level package names and message names for __all__
    let mut all_exports: Vec<String> = Vec::new();

    // Collect top-level package names (excluding "_root")
    for package_name in package_tree.keys() {
        if package_name != "_root" {
            all_exports.push(package_name.clone());
        }
    }

    // Collect top-level message names from "_root" if present
    if let Some(root_node) = package_tree.get("_root") {
        for (message_name, _) in &root_node.messages {
            all_exports.push(message_name.clone());
        }
    }

    all_exports.sort();

    // Generate nested classes
    for (package_name, package_node) in &package_tree {
        if package_name == "_root" {
            // For empty package, generate messages directly at top level
            generate_python_classes(python_file, package_node, 0)?;
        } else {
            writeln!(python_file, "class {}:\n", package_name)?;
            generate_python_classes(python_file, package_node, 1)?;
        }
    }

    // Add __all__ attribute with all top-level packages and messages
    if !all_exports.is_empty() {
        writeln!(python_file, "\n__all__ = [")?;
        for export_name in &all_exports {
            writeln!(python_file, "    \"{}\",", export_name)?;
        }
        writeln!(python_file, "]")?;
    }

    Ok(())
}

/// Generate Polars extension code and Python modules for protobuf messages.
///
/// This function generates:
/// 1. Rust extension code (`extension_generated.rs`) that provides Polars plugin functions
///    for each protobuf message type
/// 2. Python wrapper module (`structpath.py`) that provides a class-based API for accessing
///    protobuf fields in Polars DataFrames
///
/// The generated code enables Python users to extract protobuf fields using expressions like:
/// ```python
/// df.with_columns([
///     sample.User.get_value(pl.col("binary_column"), "name").alias("user_name")
/// ])
/// ```
///
/// # Arguments
///
/// * `file_descriptor_set` - The protobuf file descriptor set containing all message definitions
/// * `config` - Configuration specifying where to generate the Python module
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if file generation fails.
///
/// # Errors
///
/// Returns an error if:
/// - The `OUT_DIR` environment variable is not set
/// - File creation or writing fails
///
/// # Note
///
/// This function is typically called automatically by `build_protobuf` when
/// `BuildConfig::generate_extensions` is set. It's exposed as a public function for
/// advanced use cases where you need more control over the generation process.
pub fn generate_extensions(
    file_descriptor_set: &prost_types::FileDescriptorSet,
    config: &ExtensionConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR")?;

    // Collect all top-level messages
    let mut messages = Vec::new();
    for file in &file_descriptor_set.file {
        let empty_package = String::new();
        let package_name = file.package.as_ref().unwrap_or(&empty_package);

        for message in &file.message_type {
            let message_name = message.name.as_ref().unwrap();
            let rust_path = extract_rust_path(package_name, message_name);
            let function_prefix = to_snake_case(message_name);

            messages.push((
                package_name.clone(),
                message_name.clone(),
                rust_path,
                function_prefix,
            ));
        }
    }

    // Group messages by package
    let mut packages: HashMap<String, Vec<(&String, &String)>> = HashMap::new();
    for (package_name, message_name, _, function_prefix) in &messages {
        packages
            .entry(package_name.clone())
            .or_default()
            .push((message_name, function_prefix));
    }

    // Generate Rust extension code
    let extension_path = Path::new(&out_dir).join("extension_generated.rs");
    let mut extension_file = fs::File::create(&extension_path)?;
    generate_rust_file(&mut extension_file, &messages)?;

    // Generate Python wrapper module
    let python_module_path = Path::new(&config.python_package_dir).join("structpath.py");
    if let Some(parent) = python_module_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut python_file = fs::File::create(&python_module_path)?;
    generate_python_file(&mut python_file, &packages)?;

    Ok(())
}
