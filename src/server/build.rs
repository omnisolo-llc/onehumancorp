use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set PROTOC environment variable to point to the built protoc in runfiles
    let protoc_path = PathBuf::from("../../../protobuf+/protoc");
    env::set_var("PROTOC", protoc_path);
    
    println!("cargo:warning=Set PROTOC to {:?}", env::var("PROTOC"));

    tonic_build::configure()
        .compile_protos(
            &["../proto/hub.proto", "../proto/agent_service.proto"],
            &["../proto"],
        )?;
    Ok(())
}
