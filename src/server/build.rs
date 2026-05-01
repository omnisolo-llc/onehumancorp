use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if PROTOC is already set (e.g. by our environment for local testing)
    if env::var("PROTOC").is_err() {
        // Set PROTOC environment variable to point to the built protoc in runfiles
        let protoc_path = PathBuf::from("../../../protobuf+/protoc");
        // SAFETY: This is only called at build time and does not concurrently access env
        unsafe { env::set_var("PROTOC", protoc_path) };
    }
    
    println!("cargo:warning=Set PROTOC to {:?}", env::var("PROTOC"));

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(
            &["../proto/hub.proto", "../proto/agent_service.proto", "../proto/organization.proto"],
            &["../proto", "../.."],
        )?;
    Ok(())
}
