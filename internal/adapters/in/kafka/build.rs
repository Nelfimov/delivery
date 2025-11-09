use std::error::Error;
use std::fs::create_dir;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = create_dir("src/gen");

    let mut config = prost_build::Config::new();
    config
        .out_dir("src/gen")
        .compile_protos(&["proto/baskets_events.proto"], &["proto/"])?;

    println!("cargo:rerun-if-changed=proto");

    Ok(())
}
