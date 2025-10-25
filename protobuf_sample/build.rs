use prost_types::FileDescriptorSet;
use std::fs;
use std::path::Path;

fn extract_enum_values(
    file_descriptor_set: &FileDescriptorSet,
    enum_type_name: &str,
) -> Vec<(String, i32)> {
    for file in &file_descriptor_set.file {
        // Check top-level enums
        for enum_type in &file.enum_type {
            let full_enum_name = format!(
                ".{}.{}",
                file.package.as_ref().unwrap_or(&String::new()),
                enum_type.name.as_ref().unwrap()
            );
            if full_enum_name == enum_type_name {
                return enum_type
                    .value
                    .iter()
                    .map(|value| (value.name.as_ref().unwrap().clone(), value.number.unwrap()))
                    .collect();
            }
        }

        // Check enums inside messages
        for message in &file.message_type {
            for enum_type in &message.enum_type {
                let full_enum_name = format!(
                    ".{}.{}.{}",
                    file.package.as_ref().unwrap_or(&String::new()),
                    message.name.as_ref().unwrap(),
                    enum_type.name.as_ref().unwrap()
                );
                if full_enum_name == enum_type_name {
                    return enum_type
                        .value
                        .iter()
                        .map(|value| (value.name.as_ref().unwrap().clone(), value.number.unwrap()))
                        .collect();
                }
            }
        }
    }
    vec![] // Return empty vec if enum not found
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

    let mut config = prost_build::Config::new();

    // Load the file descriptor set from the proto files to analyze the types
    let file_descriptor_set = config.load_fds(&proto_files, &["protobuf/sample"])?;

    // Apply derive attribute to all message types at once
    config.enum_attribute(".", "#[derive(structpath::EnumPath)]");
    config.message_attribute(".", "#[derive(structpath::StructPath)]");

    for file in &file_descriptor_set.file {
        for message in &file.message_type {
            let message_name = message.name.as_ref().unwrap();
            for field in &message.field {
                let field_name = field.name.as_ref().unwrap();
                let field_path = format!(
                    ".{}.{}.{}",
                    file.package.as_ref().unwrap_or(&String::new()),
                    message_name,
                    field_name
                );
                match field.r#type().as_str_name() {
                    "TYPE_DOUBLE" => continue,
                    "TYPE_FLOAT" => panic!("TYPE_FLOAT not supported"),
                    "TYPE_INT64" => continue,
                    "TYPE_UINT64" => panic!("TYPE_UINT64 not supported"),
                    "TYPE_INT32" => continue,
                    "TYPE_FIXED64" => panic!("TYPE_FIXED64 not supported"),
                    "TYPE_FIXED32" => panic!("TYPE_FIXED32 not supported"),
                    "TYPE_BOOL" => continue,
                    "TYPE_STRING" => continue,
                    "TYPE_GROUP" => panic!("TYPE_GROUP not supported"), // Proto2 syntax only, and deprecated.
                    "TYPE_MESSAGE" => {
                        config.field_attribute(&field_path, "#[type_hint(\"struct\")]")
                    }
                    "TYPE_BYTES" => panic!("TYPE_BYTES not supported"),
                    "TYPE_UINT32" => panic!("TYPE_UINT32 not supported"),
                    "TYPE_ENUM" => {
                        // Find the enum type for this field
                        let enum_type_name = field.type_name.as_ref().unwrap();
                        let enum_values = extract_enum_values(&file_descriptor_set, enum_type_name);
                        let enum_attr = format!("#[type_hint(\"enum\", {:?})]", enum_values);
                        config.field_attribute(&field_path, &enum_attr)
                    }
                    "TYPE_SFIXED32" => panic!("TYPE_SFIXED32 not supported"),
                    "TYPE_SFIXED64" => panic!("TYPE_SFIXED64 not supported"),
                    _ => panic!("Unknown field type: {}", field.r#type().as_str_name()),
                };
            }
        }
    }

    // Generate the Rust code for the types
    config.compile_fds(file_descriptor_set)?;

    Ok(())
}
