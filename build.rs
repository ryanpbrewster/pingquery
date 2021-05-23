fn main() {
    tonic_build::configure()
        .out_dir("src/proto")
        .compile(&["proto/api.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
