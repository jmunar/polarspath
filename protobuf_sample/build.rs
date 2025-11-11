use structpath_protobuf::build::{build_protobuf, BuildConfig, ExtensionConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build protobuf files and generate extensions
    build_protobuf(BuildConfig {
        proto_dir: "protobuf/sample".to_string(),
        include_paths: vec!["protobuf/sample".to_string()],
        generate_extensions: Some(ExtensionConfig {
            python_package_dir: "protobuf_sample".to_string(),
            python_package_name: "protobuf_sample".to_string(),
        }),
    })?;

    println!("cargo:rerun-if-changed=protobuf/sample");
    Ok(())
}
