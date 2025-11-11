use std::error::Error;
use std::fs::create_dir;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = create_dir("src/gen");

    tonic_prost_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir("src/gen")
        .compile_protos(&["proto/geo.proto"], &["proto/"])?;

    println!("cargo:rerun-if-changed=proto");

    Ok(())
}
