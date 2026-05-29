fn main() {
    // Use a bundled protoc compiler so the project works more easily on Windows.
    let protoc_path = protoc_bin_vendored::protoc_bin_path()
        .expect("Failed to find bundled protoc");

    std::env::set_var("PROTOC", protoc_path);

    // Compile the shared protobuf schema into Rust code.
    prost_build::compile_protos(&["../proto/signal.proto"], &["../proto"])
        .expect("Failed to compile protobuf files");
}