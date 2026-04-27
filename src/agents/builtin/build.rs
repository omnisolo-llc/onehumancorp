use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../../proto/agent_service.proto");

    let protoc_path = std::path::PathBuf::from("../../../../protobuf+/protoc");
    unsafe {
        std::env::set_var("PROTOC", protoc_path);
    }

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &["../../proto/agent_service.proto"],
            &["../../proto"],
        )?;
    Ok(())
}


