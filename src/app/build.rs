fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the directory containing this build script (src/app/)
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // Set include path so slint can find imported files
    // The slint files are in src/ subdirectory relative to manifest_dir
    let include_path = std::path::Path::new(&manifest_dir).join("src");
    // SAFETY: This is only called at build time and does not concurrently access env
    unsafe {
        std::env::set_var("SLINT_INCLUDE_PATH", include_path.to_string_lossy().as_ref());
    }

    let app_slint_path = std::path::Path::new(&manifest_dir).join("src/app.slint");
    slint_build::compile(&app_slint_path).unwrap();

    // Set PROTOC environment variable to point to the built protoc in runfiles
    let protoc_path = std::path::PathBuf::from("../../../protobuf+/protoc");
    // SAFETY: This is only called at build time and does not concurrently access env
    unsafe { std::env::set_var("PROTOC", protoc_path) };

    tonic_build::configure()
        .compile_protos(
            &["../proto/hub.proto"],
            &["../proto"],
        )?;
    Ok(())
}
