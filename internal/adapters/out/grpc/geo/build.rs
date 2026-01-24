use std::error::Error;
use std::fs::create_dir;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = create_dir("src/gen");

    let mut includes = vec!["proto/"];
    if Path::new("/usr/include/google/protobuf").exists() {
        includes.push("/usr/include");
    }

    tonic_prost_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir("src/gen")
        .compile_protos(&["proto/geo.proto"], &includes)?;

    println!("cargo:rerun-if-changed=proto");

    Ok(())
}
