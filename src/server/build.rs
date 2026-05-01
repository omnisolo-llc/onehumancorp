use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set PROTOC environment variable to point to the built protoc in runfiles
    let protoc_path = env::var("PROTOC").unwrap_or_else(|_| {
        let default_bazel_path = PathBuf::from("../../../protobuf+/protoc");
        if default_bazel_path.exists() {
            default_bazel_path.to_string_lossy().into_owned()
        } else {
            "/usr/bin/protoc".to_string()
        }
    });
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
