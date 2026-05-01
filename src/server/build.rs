use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set PROTOC environment variable to point to the built protoc in runfiles
    // Fallback to system protoc if available
    let protoc_path = std::env::var("PROTOC").unwrap_or_else(|_| "/usr/bin/protoc".to_string());
    let protoc_path = PathBuf::from(protoc_path);
    // SAFETY: This is only called at build time and does not concurrently access env
    unsafe { env::set_var("PROTOC", protoc_path) };
    
    println!("cargo:warning=Set PROTOC to {:?}", env::var("PROTOC"));

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(
            &["src/proto/hub.proto", "src/proto/agent_service.proto", "src/proto/organization.proto"],
            &["src/proto", "."],
        )?;
    Ok(())
}
