use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{Result, Write};
use std::path::PathBuf;

struct EnumDescriptorWrapper<'a>(&'a ::prost_types::EnumDescriptorProto);

impl<'a> EnumDescriptorWrapper<'a> {
    fn build(&self) -> String {
        format!(
            r#"
#[derive(::polars_structpath::EnumPath, Debug, Clone, PartialEq)]
pub enum {enum_name} {{
{variants}
}}
"#,
            enum_name = self.0.name.as_ref().unwrap(),
            variants = self
                .0
                .value
                .iter()
                .map(|variant| {
                    format!(
                    "    #[allow(clippy::upper_case_acronyms, non_camel_case_types)]\n    {} = {},",
                    variant.name.as_ref().unwrap(),
                    variant.number.unwrap_or(0)
                )
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

struct FieldDescriptorWrapper<'a>(&'a ::prost_types::FieldDescriptorProto);

impl<'a> FieldDescriptorWrapper<'a> {
    fn name(&self) -> &str {
        self.0.name.as_ref().unwrap()
    }

    fn is_option(&self) -> bool {
        matches!(self.0.proto3_optional, Some(true))
            || (self.0.r#type().as_str_name() == "TYPE_MESSAGE"
                && self.0.label().as_str_name() != "LABEL_REPEATED")
    }

    fn is_vec(&self) -> bool {
        self.0.label().as_str_name() == "LABEL_REPEATED"
    }

    fn is_enum(&self) -> bool {
        self.0.r#type().as_str_name() == "TYPE_ENUM"
    }

    fn is_message(&self) -> bool {
        self.0.r#type().as_str_name() == "TYPE_MESSAGE"
    }

    fn type_name(&self) -> String {
        let type_name = self.0.type_name.as_ref().unwrap();
        // Extract just the type name from a fully-qualified protobuf type name.
        // Example: ".package_name.TypeName" -> "TypeName"
        // Example: ".package_name.MessageName.TypeName" -> "TypeName"
        type_name
            .trim_start_matches('.')
            .split('.')
            .next_back()
            .unwrap_or(type_name)
            .to_string()
    }

    fn stmt_definition(&self) -> String {
        let field_type: String = match self.0.r#type().as_str_name() {
            "TYPE_DOUBLE" => "f64".to_string(),
            "TYPE_FLOAT" => "f32".to_string(),
            "TYPE_INT64" => "i64".to_string(),
            "TYPE_INT32" => "i32".to_string(),
            "TYPE_UINT64" => "u64".to_string(),
            "TYPE_UINT32" => "u32".to_string(),
            "TYPE_FIXED64" => "u64".to_string(),
            "TYPE_FIXED32" => "u32".to_string(),
            "TYPE_SINT32" => "i32".to_string(),
            "TYPE_SINT64" => "i64".to_string(),
            "TYPE_SFIXED64" => "i64".to_string(),
            "TYPE_SFIXED32" => "i32".to_string(),
            "TYPE_BOOL" => "bool".to_string(),
            "TYPE_STRING" => "String".to_string(),
            "TYPE_BYTES" => "Vec<u8>".to_string(),
            "TYPE_MESSAGE" => self.type_name(),
            "TYPE_ENUM" => self.type_name(),
            _ => panic!("Unknown field type: {}", self.0.r#type().as_str_name()),
        };

        let field_type = if self.is_option() {
            format!("Option<{}>", field_type)
        } else {
            field_type
        };

        let field_type = if self.is_vec() {
            format!("Vec<{}>", field_type)
        } else {
            field_type
        };

        format!("pub {}: {},", self.name(), field_type)
    }

    fn stmt_from_prost(self) -> String {
        let definition = match self.is_vec() {
            false => {
                if self.is_enum() {
                    format!(
                        "{}::from_rust_idx(message.{})",
                        self.type_name(),
                        self.name()
                    )
                } else if self.is_message() {
                    format!(
                        "message.{}.map(|submessage| {}::from_prost(submessage))",
                        self.name(),
                        self.type_name()
                    )
                } else {
                    format!("message.{}", self.name())
                }
            }
            true => {
                if self.is_enum() {
                    format!(
                        "message.{}.into_iter().map(|value| {}::from_rust_idx(value)).collect()",
                        self.name(),
                        self.type_name()
                    )
                } else if self.is_message() {
                    format!(
                        "message.{}.into_iter().map(|value| {}::from_prost(value)).collect()",
                        self.name(),
                        self.type_name()
                    )
                } else {
                    format!("message.{}", self.name())
                }
            }
        };
        format!("{}: {},", self.name(), definition)
    }

    fn stmt_to_prost(self) -> String {
        let definition = match self.is_vec() {
            false => {
                if self.is_enum() {
                    format!("self.{} as i32", self.name())
                } else if self.is_message() {
                    format!("self.{}.map(|value| value.to_prost())", self.name())
                } else {
                    format!("self.{}", self.name())
                }
            }
            true => {
                if self.is_enum() {
                    format!(
                        "self.{}.into_iter().map(|value| value as i32).collect()",
                        self.name()
                    )
                } else if self.is_message() {
                    format!(
                        "self.{}.into_iter().map(|value| value.to_prost()).collect()",
                        self.name()
                    )
                } else {
                    format!("self.{}", self.name())
                }
            }
        };
        format!("{}: {},", self.name(), definition)
    }
}

struct MessageDescriptorWrapper<'a>(&'a ::prost_types::DescriptorProto);

impl<'a> MessageDescriptorWrapper<'a> {
    fn build(&self) -> String {
        format!(
            r#"
#[derive(::polars_structpath::StructPath, Debug, Clone, PartialEq)]
pub struct {message_name} {{
{field_definitions}
}}

impl ::polars_protobuf::ArrowMessage for {message_name} {{
    type ProstMessage = prost::{message_name};

    fn from_prost(message: Self::ProstMessage) -> Self {{
        Self {{
{from_prost_fields}
        }}
    }}

    fn to_prost(self) -> Self::ProstMessage {{
        Self::ProstMessage {{
{to_prost_fields}
        }}
    }}
}}
"#,
            message_name = self.0.name.as_ref().unwrap(),
            field_definitions = self
                .0
                .field
                .iter()
                .map(|field| format!("    {}", FieldDescriptorWrapper(field).stmt_definition()))
                .collect::<Vec<_>>()
                .join("\n"),
            from_prost_fields = self
                .0
                .field
                .iter()
                .map(|field| format!(
                    "            {}",
                    FieldDescriptorWrapper(field).stmt_from_prost()
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            to_prost_fields = self
                .0
                .field
                .iter()
                .map(|field| format!(
                    "            {}",
                    FieldDescriptorWrapper(field).stmt_to_prost()
                ))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

pub fn build(out_dir: PathBuf, protos: &[&str], includes: &[&str]) -> Result<()> {
    println!("cargo:rerun-if-changed={}", protos.join(","));
    println!("cargo:rerun-if-changed={}", includes.join(","));

    // Build prost package
    let out_dir_prost = out_dir.join("prost");
    let mut prost_config = prost_build::Config::new();
    create_dir_all(&out_dir_prost)?;
    prost_config.out_dir(&out_dir_prost);
    prost_config.compile_protos(protos, includes)?;

    // Build file descriptor set
    let file_descriptor_set = prost_config.load_fds(protos, includes)?;

    let mut file_contents = HashMap::new();

    for proto_file in &file_descriptor_set.file {
        let package_name = proto_file.package.as_deref().unwrap_or("");

        let mut file_content = vec![];

        for enum_ in &proto_file.enum_type {
            file_content.push(EnumDescriptorWrapper(enum_).build());
        }

        for message in &proto_file.message_type {
            file_content.push(MessageDescriptorWrapper(message).build());
        }

        file_content.push("".to_string());

        file_contents
            .entry(package_name)
            .or_insert_with(Vec::new)
            .extend(file_content);
    }

    for (package_name, contents) in &file_contents {
        let output_file_name = out_dir.join(format!("{}.rs", package_name));
        let mut output_file = File::create(output_file_name)?;
        writeln!(output_file, "pub mod prost {{")?;
        writeln!(
            output_file,
            "    include!(\"./prost/{}.rs\");",
            package_name
        )?;
        writeln!(output_file, "}}")?;
        writeln!(output_file)?;
        for line in contents {
            writeln!(output_file, "{}", line)?;
        }
        writeln!(output_file)?;
    }

    Ok(())
}
