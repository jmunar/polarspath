use structpath_protobuf::build::{build_protobuf, BuildConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_protobuf(BuildConfig {
        proto_dir: "protobuf/sample".to_string(),
        include_paths: vec!["protobuf/sample".to_string()],
    })?;
    Ok(())
}
