use polars_protobuf::build::{build_protobuf, BuildConfig, ExtensionConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_protobuf(BuildConfig {
        proto_dir: "protobuf/example_protobuf".to_string(),
        include_paths: vec!["protobuf/example_protobuf".to_string()],
        generate_extensions: Some(ExtensionConfig {
            python_package_dir: "example_protobuf".to_string(),
            python_package_name: "example_protobuf".to_string(),
        }),
    })?;

    println!("cargo:rerun-if-changed=protobuf/example_protobuf");
    Ok(())
}
