use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let proto_dir = "protobuf/example_protobuf";

    // Discover all .proto files
    let proto_files: Vec<String> = fs::read_dir(proto_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "proto") {
                Some(path.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();

    if proto_files.is_empty() {
        println!("cargo:warning=No .proto files found in {}", proto_dir);
        return Ok(());
    }

    let protos: Vec<&str> = proto_files.iter().map(|s| s.as_str()).collect();
    let includes: &[&str] = &[proto_dir];
    let build_config = polars_protobuf::build::BuildConfig::new(out_dir, &protos, includes);
    build_config.with_python(PathBuf::from("example_protobuf"), "_example_protobuf_rust").build()?;

    println!("cargo:rerun-if-changed={}", proto_dir);
    Ok(())
}
