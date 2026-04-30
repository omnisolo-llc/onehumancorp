fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_path = std::path::Path::new(&manifest_dir).join("src");
    unsafe {
        std::env::set_var("SLINT_INCLUDE_PATH", src_path.to_string_lossy().as_ref());
    }

    let app_slint_path = std::path::Path::new(&manifest_dir).join("src/app.slint");
    slint_build::compile(&app_slint_path).unwrap();

    // Fallback logic for protoc without explicitly breaking cross-platform
    if std::env::var("PROTOC").is_err() {
        let default_protoc = std::path::PathBuf::from("../../../protobuf+/protoc");
        if default_protoc.exists() {
            unsafe { std::env::set_var("PROTOC", default_protoc) };
        }
    }

    tonic_build::configure()
        .compile_protos(
            &["../proto/hub.proto"],
            &["../proto"],
        )?;
    Ok(())
}
