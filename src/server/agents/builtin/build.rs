use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tell cargo to re-run this script if the proto file changes.
    println!("cargo:rerun-if-changed=../../../proto/agent_service.proto");
    println!("cargo:rerun-if-changed=src/gen/ohc.agent.service.rs");

    // Only regenerate if protoc is available.
    if which_protoc() {
        let gen_dir = "src/gen";
        std::fs::create_dir_all(gen_dir)?;
        tonic_build::configure()
            .build_server(true)
            .build_client(true)
            .out_dir(gen_dir)
            .compile_protos(&["../../../proto/agent_service.proto"], &["../../../proto"])?;
    }
    Ok(())
}

fn which_protoc() -> bool {
    std::process::Command::new("protoc")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
