use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{Result, Write};
use std::path::PathBuf;

struct EnumDescriptorWrapper<'a>(&'a ::prost_types::EnumDescriptorProto);

impl<'a> EnumDescriptorWrapper<'a> {
    #[rustfmt::skip]
    fn build(&self) -> Result<String> {
        let enum_name = self.0.name.as_ref().unwrap();
        let variants = self.0.value.iter()
            .map(|variant| {
                let name = variant.name.as_ref().unwrap();
                let number = variant.number.unwrap_or(0);
                format!("#[allow(clippy::upper_case_acronyms, non_camel_case_types)]\n    {name} = {number},")
            })
            .collect::<Vec<_>>().join("\n    ");

        let mut buf = Vec::new();
        writeln!(buf)?;
        writeln!(buf, "#[derive(::polars_protobuf::polars_structpath::EnumPath, Debug, Clone, PartialEq)]")?;
        writeln!(buf, "pub enum {enum_name} {{")?;
        writeln!(buf, "    {variants}")?;
        writeln!(buf, "}}")?;
        Ok(String::from_utf8(buf).expect("generated code is valid UTF-8"))
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

    #[rustfmt::skip]
    fn dtype_expr(&self) -> String {
        let base_dtype = match self.0.r#type().as_str_name() {
            "TYPE_DOUBLE" => "::polars_protobuf::polars_core::prelude::DataType::Float64".to_string(),
            "TYPE_FLOAT" => "::polars_protobuf::polars_core::prelude::DataType::Float32".to_string(),
            "TYPE_INT64" => "::polars_protobuf::polars_core::prelude::DataType::Int64".to_string(),
            "TYPE_INT32" => "::polars_protobuf::polars_core::prelude::DataType::Int32".to_string(),
            "TYPE_UINT64" => "::polars_protobuf::polars_core::prelude::DataType::UInt64".to_string(),
            "TYPE_UINT32" => "::polars_protobuf::polars_core::prelude::DataType::UInt32".to_string(),
            "TYPE_FIXED64" => "::polars_protobuf::polars_core::prelude::DataType::UInt64".to_string(),
            "TYPE_FIXED32" => "::polars_protobuf::polars_core::prelude::DataType::UInt32".to_string(),
            "TYPE_SINT32" => "::polars_protobuf::polars_core::prelude::DataType::Int32".to_string(),
            "TYPE_SINT64" => "::polars_protobuf::polars_core::prelude::DataType::Int64".to_string(),
            "TYPE_SFIXED64" => "::polars_protobuf::polars_core::prelude::DataType::Int64".to_string(),
            "TYPE_SFIXED32" => "::polars_protobuf::polars_core::prelude::DataType::Int32".to_string(),
            "TYPE_BOOL" => "::polars_protobuf::polars_core::prelude::DataType::Boolean".to_string(),
            "TYPE_STRING" => "::polars_protobuf::polars_core::prelude::DataType::String".to_string(),
            "TYPE_BYTES" => {
                "::polars_protobuf::polars_core::prelude::DataType::List(Box::new(::polars_protobuf::polars_core::prelude::DataType::UInt8))"
                    .to_string()
            }
            "TYPE_MESSAGE" => format!("{}::dtype()", self.type_name()),
            "TYPE_ENUM" => "::polars_protobuf::polars_core::prelude::DataType::Int32".to_string(),
            _ => panic!("Unknown field type: {}", self.0.r#type().as_str_name()),
        };

        if self.is_vec() {
            format!("::polars_protobuf::polars_core::prelude::DataType::List(Box::new({}))", base_dtype)
        } else {
            base_dtype
        }
    }

    #[rustfmt::skip]
    fn stmt_from_prost(self) -> String {
        let name = self.name();
        let type_name = if self.is_enum() || self.is_message() { self.type_name() } else { String::new() };
        let definition = match self.is_vec() {
            false => {
                if self.is_enum() {
                    format!("{type_name}::from_rust_idx(message.{name})")
                } else if self.is_message() {
                    format!("message.{name}.map({type_name}::from_prost)")
                } else {
                    format!("message.{name}")
                }
            }
            true => {
                if self.is_enum() {
                    format!("message.{name}.into_iter().map({type_name}::from_rust_idx).collect()")
                } else if self.is_message() {
                    format!("message.{name}.into_iter().map({type_name}::from_prost).collect()")
                } else {
                    format!("message.{name}")
                }
            }
        };
        format!("{name}: {definition},")
    }

    #[rustfmt::skip]
    fn stmt_to_prost(self) -> String {
        let name = self.name();
        let definition = match self.is_vec() {
            false => {
                if self.is_enum() {
                    format!("self.{name} as i32")
                } else if self.is_message() {
                    format!("self.{name}.map(|value| value.to_prost())")
                } else {
                    format!("self.{name}")
                }
            }
            true => {
                if self.is_enum() {
                    format!("self.{name}.into_iter().map(|value| value as i32).collect()")
                } else if self.is_message() {
                    format!("self.{name}.into_iter().map(|value| value.to_prost()).collect()")
                } else {
                    format!("self.{name}")
                }
            }
        };
        format!("{name}: {definition},")
    }
}

struct MessageDescriptorWrapper<'a>(&'a ::prost_types::DescriptorProto);

impl<'a> MessageDescriptorWrapper<'a> {
    fn name(&self) -> &str {
        self.0.name.as_ref().unwrap()
    }

    #[rustfmt::skip]
    fn build(&self) -> Result<String> {
        let message_name = self.name();
        // Use _message if there are no fields to avoid unused variable warning
        let message_param = if self.0.field.is_empty() { "_message" } else { "message" };
        let fields = &self.0.field;
        let field_definitions = fields.iter()
            .map(|f| FieldDescriptorWrapper(f).stmt_definition())
            .collect::<Vec<_>>().join("\n    ");
        let from_prost_fields = fields.iter()
            .map(|f| FieldDescriptorWrapper(f).stmt_from_prost())
            .collect::<Vec<_>>().join("\n            ");
        let to_prost_fields = fields.iter()
            .map(|f| FieldDescriptorWrapper(f).stmt_to_prost())
            .collect::<Vec<_>>().join("\n            ");

        let mut buf = Vec::new();
        writeln!(buf)?;
        writeln!(buf, "#[derive(::polars_protobuf::polars_structpath::StructPath, Debug, Clone, PartialEq)]")?;
        writeln!(buf, "pub struct {message_name} {{")?;
        writeln!(buf, "    {field_definitions}")?;
        writeln!(buf, "}}")?;
        writeln!(buf)?;
        writeln!(buf, "impl ::polars_protobuf::ArrowMessage for {message_name} {{")?;
        writeln!(buf, "    type ProstMessage = prost::{message_name};")?;
        writeln!(buf)?;
        writeln!(buf, "    fn from_prost({message_param}: Self::ProstMessage) -> Self {{")?;
        writeln!(buf, "        Self {{")?;
        writeln!(buf, "            {from_prost_fields}")?;
        writeln!(buf, "        }}")?;
        writeln!(buf, "    }}")?;
        writeln!(buf)?;
        writeln!(buf, "    fn to_prost(self) -> Self::ProstMessage {{")?;
        writeln!(buf, "        Self::ProstMessage {{")?;
        writeln!(buf, "            {to_prost_fields}")?;
        writeln!(buf, "        }}")?;
        writeln!(buf, "    }}")?;
        writeln!(buf, "}}")?;
        Ok(String::from_utf8(buf).expect("generated code is valid UTF-8"))
    }

    #[rustfmt::skip]
    fn build_dtype(&self) -> Result<String> {
        let message_name = self.name();
        let fields = self.0.field.iter()
            .map(|field| {
                let wrapper = FieldDescriptorWrapper(field);
                let name = wrapper.name();
                let dtype = wrapper.dtype_expr();
                format!("::polars_protobuf::polars_core::prelude::Field::new(\"{name}\".into(), {dtype})")
            })
            .collect::<Vec<_>>().join(",\n            ");

        let mut buf = Vec::new();
        writeln!(buf)?;
        writeln!(buf, "impl {message_name} {{")?;
        writeln!(buf, "    /// Returns the Polars DataType for this message type.")?;
        writeln!(buf, "    pub fn dtype() -> ::polars_protobuf::polars_core::prelude::DataType {{")?;
        writeln!(buf, "        ::polars_protobuf::polars_core::prelude::DataType::Struct(vec![")?;
        writeln!(buf, "            {fields}")?;
        writeln!(buf, "        ])")?;
        writeln!(buf, "    }}")?;
        writeln!(buf, "}}")?;
        Ok(String::from_utf8(buf).expect("generated code is valid UTF-8"))
    }

    #[rustfmt::skip]
    fn build_polars_expr(&self, package_name: &str) -> Result<String> {
        let message_name = self.name();
        let fn_prefix = format!("{}_{}", package_name.replace('.', "_"), message_name);

        let mut buf = Vec::new();
        writeln!(buf)?;
        // encode output type function
        writeln!(buf, "#[cfg(feature = \"extension-module\")]")?;
        writeln!(buf, "fn {fn_prefix}_encode_output(_input_fields: &[::polars_protobuf::polars_core::prelude::Field]) -> ::polars_protobuf::polars_core::prelude::PolarsResult<::polars_protobuf::polars_core::prelude::Field> {{")?;
        writeln!(buf, "    Ok(::polars_protobuf::polars_core::prelude::Field::new(")?;
        writeln!(buf, "        _input_fields[0].name().clone(),")?;
        writeln!(buf, "        ::polars_protobuf::polars_core::prelude::DataType::List(Box::new(::polars_protobuf::polars_core::prelude::DataType::UInt8)),")?;
        writeln!(buf, "    ))")?;
        writeln!(buf, "}}")?;
        writeln!(buf)?;
        // encode function
        writeln!(buf, "#[cfg(feature = \"extension-module\")]")?;
        writeln!(buf, "#[::pyo3_polars::derive::polars_expr(output_type_func={fn_prefix}_encode_output)]")?;
        writeln!(buf, "fn {fn_prefix}_encode(inputs: &[::polars_protobuf::polars_core::prelude::Series]) -> ::polars_protobuf::polars_core::prelude::PolarsResult<::polars_protobuf::polars_core::prelude::Series> {{")?;
        writeln!(buf, "    use ::polars_protobuf::ArrowMessage;")?;
        writeln!(buf, "    use ::polars_protobuf::polars_structpath::{{ArrowBuffer, FromArrow, IntoArrow}};")?;
        writeln!(buf, "    use ::polars_protobuf::rayon::prelude::*;")?;
        writeln!(buf, "    use ::pyo3_polars::export::polars_core::POOL;")?;
        writeln!(buf)?;
        writeln!(buf, "    let series = inputs[0].clone();")?;
        writeln!(buf, "    let name = series.name().clone();")?;
        writeln!(buf)?;
        writeln!(buf, "    let chunks = series.into_chunks();")?;
        writeln!(buf, "    let encoded_chunks: Vec<Box<dyn ::polars_protobuf::polars_arrow::array::Array>> = chunks")?;
        writeln!(buf, "        .into_iter()")?;
        writeln!(buf, "        .map(|chunk| {{")?;
        writeln!(buf, "            let messages = {message_name}::from_arrow_opt(chunk);")?;
        writeln!(buf)?;
        writeln!(buf, "            let encoded: Vec<Option<Vec<u8>>> = POOL.install(|| {{")?;
        writeln!(buf, "                messages")?;
        writeln!(buf, "                    .into_par_iter()")?;
        writeln!(buf, "                    .map(|opt_msg| opt_msg.map(|msg| msg.encode_to_vec()))")?;
        writeln!(buf, "                    .collect()")?;
        writeln!(buf, "            }});")?;
        writeln!(buf)?;
        writeln!(buf, "            let mut buffer = <Vec<u8>>::new_buffer(encoded.len());")?;
        writeln!(buf, "            for bytes in encoded {{")?;
        writeln!(buf, "                match bytes {{")?;
        writeln!(buf, "                    Some(b) => buffer.push(b),")?;
        writeln!(buf, "                    None => buffer.push_null(),")?;
        writeln!(buf, "                }}")?;
        writeln!(buf, "            }}")?;
        writeln!(buf)?;
        writeln!(buf, "            Ok(Box::new(buffer.to_arrow()?) as Box<dyn ::polars_protobuf::polars_arrow::array::Array>)")?;
        writeln!(buf, "        }})")?;
        writeln!(buf, "        .collect::<::polars_protobuf::polars_core::prelude::PolarsResult<Vec<_>>>()?;")?;
        writeln!(buf)?;
        writeln!(buf, "    ::polars_protobuf::polars_core::prelude::Series::from_arrow_chunks(name, encoded_chunks)")?;
        writeln!(buf, "}}")?;
        writeln!(buf)?;
        // decode output type function
        writeln!(buf, "#[cfg(feature = \"extension-module\")]")?;
        writeln!(buf, "fn {fn_prefix}_decode_output(_input_fields: &[::polars_protobuf::polars_core::prelude::Field]) -> ::polars_protobuf::polars_core::prelude::PolarsResult<::polars_protobuf::polars_core::prelude::Field> {{")?;
        writeln!(buf, "    Ok(::polars_protobuf::polars_core::prelude::Field::new(")?;
        writeln!(buf, "        _input_fields[0].name().clone(),")?;
        writeln!(buf, "        {message_name}::dtype(),")?;
        writeln!(buf, "    ))")?;
        writeln!(buf, "}}")?;
        writeln!(buf)?;
        // decode function
        writeln!(buf, "#[cfg(feature = \"extension-module\")]")?;
        writeln!(buf, "#[::pyo3_polars::derive::polars_expr(output_type_func={fn_prefix}_decode_output)]")?;
        writeln!(buf, "fn {fn_prefix}_decode(inputs: &[::polars_protobuf::polars_core::prelude::Series]) -> ::polars_protobuf::polars_core::prelude::PolarsResult<::polars_protobuf::polars_core::prelude::Series> {{")?;
        writeln!(buf, "    use ::polars_protobuf::polars_arrow::array::Array as ArrowArray;")?;
        writeln!(buf, "    use ::polars_protobuf::ArrowMessage;")?;
        writeln!(buf, "    use ::polars_protobuf::polars_structpath::{{ArrowBuffer, IntoArrow}};")?;
        writeln!(buf, "    use ::polars_protobuf::rayon::prelude::*;")?;
        writeln!(buf, "    use ::pyo3_polars::export::polars_core::POOL;")?;
        writeln!(buf)?;
        writeln!(buf, "    let series = inputs[0].clone();")?;
        writeln!(buf, "    let name = series.name().clone();")?;
        writeln!(buf)?;
        writeln!(buf, "    let chunks = series.into_chunks();")?;
        writeln!(buf, "    let decoded_chunks: Vec<Box<dyn ArrowArray>> = chunks")?;
        writeln!(buf, "        .into_iter()")?;
        writeln!(buf, "        .map(|chunk| {{")?;
        writeln!(buf, "            // Extract bytes from list array")?;
        writeln!(buf, "            let list_array = chunk")?;
        writeln!(buf, "                .as_any()")?;
        writeln!(buf, "                .downcast_ref::<::polars_protobuf::polars_arrow::array::ListArray<i64>>()")?;
        writeln!(buf, "                .or_else(|| {{")?;
        writeln!(buf, "                    chunk")?;
        writeln!(buf, "                        .as_any()")?;
        writeln!(buf, "                        .downcast_ref::<::polars_protobuf::polars_arrow::array::ListArray<i32>>()")?;
        writeln!(buf, "                        .map(|_| panic!(\"i32 offset not supported, use i64\"))")?;
        writeln!(buf, "                }})")?;
        writeln!(buf, "                .ok_or_else(|| {{")?;
        writeln!(buf, "                    ::polars_protobuf::polars_core::prelude::PolarsError::ComputeError(")?;
        writeln!(buf, "                        format!(\"Expected ListArray, got {{:?}}\", chunk.dtype()).into(),")?;
        writeln!(buf, "                    )")?;
        writeln!(buf, "                }})?;")?;
        writeln!(buf)?;
        writeln!(buf, "            let byte_slices: Vec<Option<Vec<u8>>> = list_array")?;
        writeln!(buf, "                .iter()")?;
        writeln!(buf, "                .map(|opt_array| {{")?;
        writeln!(buf, "                    opt_array.map(|byte_array| {{")?;
        writeln!(buf, "                        let primitive_array = byte_array")?;
        writeln!(buf, "                            .as_any()")?;
        writeln!(buf, "                            .downcast_ref::<::polars_protobuf::polars_arrow::array::PrimitiveArray<u8>>()")?;
        writeln!(buf, "                            .expect(\"Expected PrimitiveArray<u8>\");")?;
        writeln!(buf, "                        primitive_array.values().as_slice().to_vec()")?;
        writeln!(buf, "                    }})")?;
        writeln!(buf, "                }})")?;
        writeln!(buf, "                .collect();")?;
        writeln!(buf)?;
        writeln!(buf, "            let decoded: Vec<Option<{message_name}>> = POOL.install(|| {{")?;
        writeln!(buf, "                byte_slices")?;
        writeln!(buf, "                    .into_par_iter()")?;
        writeln!(buf, "                    .map(|opt_bytes| {{")?;
        writeln!(buf, "                        opt_bytes.map(|bytes| {{")?;
        writeln!(buf, "                            {message_name}::decode(bytes.as_slice())")?;
        writeln!(buf, "                                .expect(\"Failed to decode protobuf message\")")?;
        writeln!(buf, "                        }})")?;
        writeln!(buf, "                    }})")?;
        writeln!(buf, "                    .collect()")?;
        writeln!(buf, "            }});")?;
        writeln!(buf)?;
        writeln!(buf, "            let mut buffer = {message_name}::new_buffer(decoded.len());")?;
        writeln!(buf, "            for message in decoded {{")?;
        writeln!(buf, "                match message {{")?;
        writeln!(buf, "                    Some(msg) => buffer.push(msg),")?;
        writeln!(buf, "                    None => buffer.push_null(),")?;
        writeln!(buf, "                }}")?;
        writeln!(buf, "            }}")?;
        writeln!(buf)?;
        writeln!(buf, "            Ok(Box::new(buffer.to_arrow()?) as Box<dyn ArrowArray>)")?;
        writeln!(buf, "        }})")?;
        writeln!(buf, "        .collect::<::polars_protobuf::polars_core::prelude::PolarsResult<Vec<_>>>()?;")?;
        writeln!(buf)?;
        writeln!(buf, "    ::polars_protobuf::polars_core::prelude::Series::from_arrow_chunks(name, decoded_chunks)")?;
        writeln!(buf, "}}")?;
        Ok(String::from_utf8(buf).expect("generated code is valid UTF-8"))
    }
}

// Python code generation

/// Information about a protobuf message for Python generation.
#[derive(Debug, Clone)]
pub struct MessageInfo {
    pub name: String,
    pub package: String,
}

impl MessageInfo {
    fn rust_fn_prefix(&self) -> String {
        format!("{}_{}", self.package.replace('.', "_"), self.name)
    }
}

/// Generate Python modules for all protobuf packages.
pub fn generate_python_package(
    python_package_path: &std::path::Path,
    packages: &HashMap<String, Vec<MessageInfo>>,
) -> Result<()> {
    std::fs::create_dir_all(python_package_path)?;

    // Generate package modules for each protobuf package
    for (package_name, messages) in packages {
        generate_package_module(python_package_path, package_name, messages)?;
    }

    // Generate root __init__.py that imports all packages
    generate_root_init(python_package_path, packages)?;

    Ok(())
}

fn generate_package_module(
    python_package_path: &std::path::Path,
    package_name: &str,
    messages: &[MessageInfo],
) -> Result<()> {
    // Create package directory (e.g., example_protobuf/example_protobuf/)
    let package_parts: Vec<&str> = package_name.split('.').collect();
    let depth = package_parts.len();
    let mut current_dir = python_package_path.to_path_buf();

    for (i, part) in package_parts.iter().enumerate() {
        current_dir = current_dir.join(part);
        std::fs::create_dir_all(&current_dir)?;

        // Create intermediate __init__.py files
        let init_file = current_dir.join("__init__.py");
        if i < package_parts.len() - 1 && !init_file.exists() {
            let mut f = File::create(&init_file)?;
            writeln!(f, "# Auto-generated package")?;
        }
    }

    // Generate _messages.py with message classes
    let messages_file = current_dir.join("_messages.py");
    let mut f = File::create(&messages_file)?;

    write_messages_module(&mut f, depth, messages)?;

    // Generate __init__.py that exports message classes
    let init_file = current_dir.join("__init__.py");
    let mut f = File::create(&init_file)?;
    write_package_init(&mut f, messages)?;

    Ok(())
}

#[rustfmt::skip]
fn write_messages_module(
    f: &mut File,
    depth: usize,
    messages: &[MessageInfo],
) -> Result<()> {
    writeln!(f, "# Auto-generated Python bindings for protobuf messages")?;
    writeln!(f, "# DO NOT EDIT - generated by polars-protobuf build")?;
    writeln!(f)?;
    writeln!(f, "from __future__ import annotations")?;
    writeln!(f)?;
    writeln!(f, "from pathlib import Path")?;
    writeln!(f)?;
    writeln!(f, "import polars as pl")?;
    writeln!(f, "from polars.plugins import register_plugin_function")?;
    writeln!(f)?;

    // Generate library path finder using __file__ relative path
    // depth = number of package levels between _messages.py and the .so directory
    writeln!(f, "_LIB = Path(__file__).resolve().parents[{}]", depth)?;
    writeln!(f)?;

    // Generate message classes
    for msg in messages {
        write_message_class(f, msg)?;
    }

    Ok(())
}

#[rustfmt::skip]
fn write_message_class(f: &mut File, msg: &MessageInfo) -> Result<()> {
    let fn_prefix = msg.rust_fn_prefix();

    writeln!(f)?;
    writeln!(f, "class {}:", msg.name)?;
    writeln!(f, "    \"\"\"")?;
    writeln!(f, "    Polars expression plugins for {} protobuf message.", msg.name)?;
    writeln!(f)?;
    writeln!(f, "    Example:")?;
    writeln!(f, "        >>> import polars as pl")?;
    writeln!(f, "        >>> # Decode binary protobuf to struct")?;
    writeln!(f, "        >>> df.lazy().select(")?;
    writeln!(f, "        ...     {}.decode(pl.col(\"data\")).alias(\"message\")", msg.name)?;
    writeln!(f, "        ... ).collect()")?;
    writeln!(f, "        >>>")?;
    writeln!(f, "        >>> # Encode struct to binary protobuf")?;
    writeln!(f, "        >>> df.lazy().select(")?;
    writeln!(f, "        ...     {}.encode(pl.col(\"message\")).alias(\"data\")", msg.name)?;
    writeln!(f, "        ... ).collect()")?;
    writeln!(f, "    \"\"\"")?;
    writeln!(f)?;
    writeln!(f, "    @classmethod")?;
    writeln!(f, "    def encode(cls, expr: pl.Expr) -> pl.Expr:")?;
    writeln!(f, "        \"\"\"")?;
    writeln!(f, "        Encode a struct column to binary protobuf.")?;
    writeln!(f)?;
    writeln!(f, "        Args:")?;
    writeln!(f, "            expr: Expression selecting a struct column matching {} schema.", msg.name)?;
    writeln!(f)?;
    writeln!(f, "        Returns:")?;
    writeln!(f, "            Expression producing List[UInt8] with encoded protobuf bytes.")?;
    writeln!(f, "        \"\"\"")?;
    writeln!(f, "        return register_plugin_function(")?;
    writeln!(f, "            plugin_path=_LIB,")?;
    writeln!(f, "            function_name=\"{}_encode\",", fn_prefix)?;
    writeln!(f, "            args=[expr],")?;
    writeln!(f, "            is_elementwise=True,")?;
    writeln!(f, "        )")?;
    writeln!(f)?;
    writeln!(f, "    @classmethod")?;
    writeln!(f, "    def decode(cls, expr: pl.Expr) -> pl.Expr:")?;
    writeln!(f, "        \"\"\"")?;
    writeln!(f, "        Decode binary protobuf to a struct column.")?;
    writeln!(f)?;
    writeln!(f, "        Args:")?;
    writeln!(f, "            expr: Expression selecting a List[UInt8] column with protobuf bytes.")?;
    writeln!(f)?;
    writeln!(f, "        Returns:")?;
    writeln!(f, "            Expression producing struct column matching {} schema.", msg.name)?;
    writeln!(f, "        \"\"\"")?;
    writeln!(f, "        return register_plugin_function(")?;
    writeln!(f, "            plugin_path=_LIB,")?;
    writeln!(f, "            function_name=\"{}_decode\",", fn_prefix)?;
    writeln!(f, "            args=[expr],")?;
    writeln!(f, "            is_elementwise=True,")?;
    writeln!(f, "        )")?;
    writeln!(f)?;

    Ok(())
}

fn write_package_init(f: &mut File, messages: &[MessageInfo]) -> Result<()> {
    writeln!(f, "# Auto-generated package exports")?;
    writeln!(f, "from ._messages import (")?;
    for msg in messages {
        writeln!(f, "    {},", msg.name)?;
    }
    writeln!(f, ")")?;
    writeln!(f)?;
    writeln!(f, "__all__ = [")?;
    for msg in messages {
        writeln!(f, "    \"{}\",", msg.name)?;
    }
    writeln!(f, "]")?;

    Ok(())
}

#[rustfmt::skip]
fn generate_root_init(
    python_package_path: &std::path::Path,
    packages: &HashMap<String, Vec<MessageInfo>>,
) -> Result<()> {
    let init_file = python_package_path.join("__init__.py");
    let mut f = File::create(&init_file)?;

    writeln!(f, "# Auto-generated Python bindings for protobuf messages")?;
    writeln!(f, "#")?;
    writeln!(f, "# Usage:")?;
    writeln!(f, "#     from my_package import my_proto")?;
    writeln!(f, "#     df.lazy().select(")?;
    writeln!(f, "#         my_proto.Person.decode(pl.col(\"data\")).alias(\"person\")")?;
    writeln!(f, "#     ).collect()")?;
    writeln!(f)?;

    // Import all top-level packages
    let mut top_levels: Vec<&str> = packages
        .keys()
        .filter_map(|p| p.split('.').next())
        .collect();
    top_levels.sort();
    top_levels.dedup();

    for top_level in &top_levels {
        writeln!(f, "from . import {}", top_level)?;
    }

    writeln!(f)?;
    writeln!(f, "__all__ = [")?;
    for top_level in &top_levels {
        writeln!(f, "    \"{}\",", top_level)?;
    }
    writeln!(f, "]")?;

    Ok(())
}

// Build configuration

/// Configuration for the protobuf build process.
#[derive(Debug, Clone)]
pub struct BuildConfig {
    /// Output directory for generated Rust code.
    pub out_dir: PathBuf,
    /// Proto files to compile.
    pub protos: Vec<String>,
    /// Include directories for proto imports.
    pub includes: Vec<String>,
    /// Path to the Python package directory (enables Python generation when set).
    pub python_package_path: Option<PathBuf>,
    /// Name of the Rust library for Python imports.
    pub rust_lib_name: Option<String>,
}

#[allow(dead_code)]
impl BuildConfig {
    /// Create a new build configuration.
    pub fn new(out_dir: PathBuf, protos: &[&str], includes: &[&str]) -> Self {
        Self {
            out_dir,
            protos: protos.iter().map(|s| s.to_string()).collect(),
            includes: includes.iter().map(|s| s.to_string()).collect(),
            python_package_path: None,
            rust_lib_name: None,
        }
    }

    /// Enable Python package generation.
    ///
    /// # Arguments
    /// * `package_path` - Path where Python modules will be generated
    /// * `lib_name` - Name of the Rust library (used for imports in Python)
    pub fn with_python(mut self, package_path: PathBuf, lib_name: &str) -> Self {
        self.python_package_path = Some(package_path);
        self.rust_lib_name = Some(lib_name.to_string());
        self
    }

    /// Run the build process.
    #[rustfmt::skip]
    pub fn build(self) -> Result<()> {
        let protos: Vec<&str> = self.protos.iter().map(|s| s.as_str()).collect();
        let includes: Vec<&str> = self.includes.iter().map(|s| s.as_str()).collect();

        println!("cargo:rerun-if-changed={}", self.protos.join(","));
        println!("cargo:rerun-if-changed={}", self.includes.join(","));

        // Build prost package
        let out_dir_prost = self.out_dir.join("prost");
        let mut prost_config = prost_build::Config::new();
        create_dir_all(&out_dir_prost)?;
        prost_config.out_dir(&out_dir_prost);
        prost_config.compile_protos(&protos, &includes)?;

        // Build file descriptor set
        let file_descriptor_set = prost_config.load_fds(&protos, &includes)?;

        let mut file_contents: HashMap<&str, Vec<String>> = HashMap::new();
        let mut python_messages: HashMap<String, Vec<MessageInfo>> = HashMap::new();
        let generate_python = self.python_package_path.is_some();

        for proto_file in &file_descriptor_set.file {
            let package_name = proto_file.package.as_deref().unwrap_or("");

            let mut file_content = vec![];

            for enum_ in &proto_file.enum_type {
                file_content.push(EnumDescriptorWrapper(enum_).build()?);
            }

            for message in &proto_file.message_type {
                let wrapper = MessageDescriptorWrapper(message);
                file_content.push(wrapper.build()?);

                if generate_python {
                    file_content.push(wrapper.build_dtype()?);
                    file_content.push(wrapper.build_polars_expr(package_name)?);

                    // Collect message info for Python generation
                    python_messages
                        .entry(package_name.to_string())
                        .or_default()
                        .push(MessageInfo {
                            name: wrapper.name().to_string(),
                            package: package_name.to_string(),
                        });
                }
            }

            file_content.push(String::new());

            file_contents
                .entry(package_name)
                .or_default()
                .extend(file_content);
        }

        for (package_name, contents) in &file_contents {
            let output_file_name = self.out_dir.join(format!("{}.rs", package_name));
            let mut output_file = File::create(output_file_name)?;
            writeln!(output_file, "pub mod prost {{")?;
            writeln!(output_file, "    include!(\"./prost/{}.rs\");", package_name)?;
            writeln!(output_file, "}}")?;
            writeln!(output_file)?;
            for line in contents {
                writeln!(output_file, "{}", line)?;
            }
            writeln!(output_file)?;
        }

        // Generate Python package if configured
        if let (Some(python_path), Some(_)) =
            (&self.python_package_path, &self.rust_lib_name)
        {
            generate_python_package(python_path, &python_messages)?;
        }

        Ok(())
    }
}

/// Build protobuf definitions (legacy API for backwards compatibility).
///
/// For new code, prefer using `BuildConfig` directly.
pub fn build(out_dir: PathBuf, protos: &[&str], includes: &[&str]) -> Result<()> {
    BuildConfig::new(out_dir, protos, includes).build()
}
