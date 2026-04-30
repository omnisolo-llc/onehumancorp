use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set PROTOC environment variable to point to the built protoc in runfiles if it exists
    if std::env::var("PROTOC").is_err() {
        let protoc_path = PathBuf::from("../../../protobuf+/protoc");
        if protoc_path.exists() {
            // SAFETY: This is only called at build time and does not concurrently access env
            unsafe { env::set_var("PROTOC", protoc_path) };
        }
    }

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(
            &["src/proto/hub.proto", "src/proto/agent_service.proto", "src/proto/organization.proto"],
            &["src/proto", "."],
        )?;
    Ok(())
}
