use std::error::Error;
use std::fs::create_dir;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = create_dir("src/gen");

    let mut config = prost_build::Config::new();
    let mut includes = vec!["proto/"];
    if Path::new("/usr/include/google/protobuf").exists() {
        includes.push("/usr/include");
    }
    config
        .out_dir("src/gen")
        .compile_protos(&["proto/orders_events.proto"], &includes)?;

    println!("cargo:rerun-if-changed=proto");

    Ok(())
}
