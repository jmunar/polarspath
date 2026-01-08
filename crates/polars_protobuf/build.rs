use std::io::Result;

fn main() -> Result<()> {

    let mut prost_config = prost_build::Config::new();
    prost_config.compile_protos(
        &[
            "tests/sample/empty.proto",
            "tests/sample/enum.proto",
            "tests/sample/string.proto",
        ],
        &["tests/sample/"],
    )?;
    println!("cargo:rerun-if-changed=tests/sample");

    let mut prost_config = prost_build::Config::new();
    prost_config.compile_protos(
        &["examples/benchmark.proto"],
        &["examples/"],
    )?;
    println!("cargo:rerun-if-changed=examples/benchmark.proto");

    Ok(())
}
