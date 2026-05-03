use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The build script runs from the workspace root (CARGO_MANIFEST_DIR)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let proto_dir = manifest_dir.join("src/proto");

    println!("cargo:rerun-if-changed={}", proto_dir.join("hub.proto").display());
    println!("cargo:rerun-if-changed={}", proto_dir.join("agent_service.proto").display());
    println!("cargo:rerun-if-changed={}", proto_dir.join("organization.proto").display());

    // Find protoc in PATH (works in nix environment)
    let protoc_path = env::var("PROTOC").unwrap_or_else(|_| {
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

    unsafe { env::set_var("PROTOC", &protoc_path) };
    println!("cargo:warning=Using protoc at: {}", protoc_path);

    // Proto files use imports like "src/proto/common.proto" so include root must be workspace root
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(
            &[
                proto_dir.join("hub.proto").to_str().unwrap(),
                proto_dir.join("agent_service.proto").to_str().unwrap(),
                proto_dir.join("organization.proto").to_str().unwrap(),
            ],
            &[manifest_dir.to_str().unwrap()],
        )?;
    Ok(())
}
