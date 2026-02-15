use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let proto_dir = "protobuf/example_protobuf";

    polars_protobuf::build::BuildConfig::from_proto_dir(out_dir, proto_dir)?
        .with_python(PathBuf::from("example_protobuf"), "_example_protobuf_rust")
        .build()?;

    println!("cargo:rerun-if-changed={}", proto_dir);
    Ok(())
}
