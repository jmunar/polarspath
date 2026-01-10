use std::io::{Error, Result};
use std::path::Path;

mod build {
    include!("src/build.rs");
}

fn main() -> Result<()> {
    let base_dir_str = std::env::var("OUT_DIR").map_err(Error::other)?;
    let base_dir = Path::new(&base_dir_str);

    build::build(
        base_dir.join("tests"),
        &[
            "tests/sample/empty.proto",
            "tests/sample/enum.proto",
            "tests/sample/string.proto",
        ],
        &["tests/sample/"],
    )?;
    build::build(
        base_dir.join("examples"),
        &["examples/benchmark.proto"],
        &["examples/"],
    )?;

    Ok(())
}
