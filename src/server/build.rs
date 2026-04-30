use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set PROTOC environment variable to point to the built protoc in runfiles
    // Let's use the one in PATH or PROTOC instead
    // let protoc_path = PathBuf::from("../../../protobuf+/protoc");
    // SAFETY: This is only called at build time and does not concurrently access env
    // unsafe { env::set_var("PROTOC", protoc_path) };
    
    println!("cargo:warning=Set PROTOC to {:?}", env::var("PROTOC"));

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(
            &["src/proto/hub.proto", "src/proto/agent_service.proto", "src/proto/organization.proto"],
            &["src/proto", "."],
        )?;
    Ok(())
}
