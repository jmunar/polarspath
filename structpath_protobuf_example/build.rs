use structpath_protobuf::build::{build_protobuf, BuildConfig, ExtensionConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_protobuf(BuildConfig {
        proto_dir: "protobuf/structpath_protobuf_example".to_string(),
        include_paths: vec!["protobuf/structpath_protobuf_example".to_string()],
        generate_extensions: Some(ExtensionConfig {
            python_package_dir: "structpath_protobuf_example".to_string(),
            python_package_name: "structpath_protobuf_example".to_string(),
        }),
    })?;

    println!("cargo:rerun-if-changed=protobuf/structpath_protobuf_example");
    Ok(())
}
