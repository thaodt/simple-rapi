/// Purpose: Build script for compiling protobuf files.
extern crate tonic_build;

fn main() {
    let mut protos: Vec<String> = Vec::new();
    protos.push("item.proto".to_string());
    tonic_build::configure()
        .build_server(true)
        .out_dir("./src/out")
        .compile(&protos, &["./proto/"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
}
