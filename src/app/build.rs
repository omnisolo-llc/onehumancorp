fn main() -> Result<(), Box<dyn std::error::Error>> {
    slint_build::compile("src/app.slint").unwrap();
    
    // Set PROTOC environment variable to point to the built protoc in runfiles
    let protoc_path = std::path::PathBuf::from("../../../protobuf+/protoc");
    std::env::set_var("PROTOC", protoc_path);
    
    tonic_build::configure()
        .compile(
            &["../proto/hub.proto"],
            &["../proto"],
        )?;
    Ok(())
}
