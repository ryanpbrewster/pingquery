use std::{env, path::PathBuf};

fn main() {
    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("api_descriptor.bin");
    tonic_build::configure()
        .out_dir("src/proto")
        .file_descriptor_set_path(descriptor_path)
        .compile(&["../proto/api.proto"], &["../proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
