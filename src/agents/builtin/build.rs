fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    // src/agents/builtin -> up two levels -> workspace root -> src/proto
    let workspace_root = manifest_dir.parent().unwrap().parent().unwrap().parent().unwrap();
    let proto_dir = workspace_root.join("src/proto");

    println!("cargo:rerun-if-changed={}", proto_dir.join("agent_service.proto").display());

    let protoc_path = std::env::var("PROTOC").unwrap_or_else(|_| {
        if let Ok(output) = std::process::Command::new("which").arg("protoc").output() {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    return path;
                }
            }
        }
        "protoc".to_string()
    });
    unsafe {
        std::env::set_var("PROTOC", &protoc_path);
    }

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[proto_dir.join("agent_service.proto").to_str().unwrap()],
            &[workspace_root.to_str().unwrap()],
        )?;
    Ok(())
}
