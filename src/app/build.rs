fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_path = std::path::Path::new(&manifest_dir).join("src");
    unsafe {
        std::env::set_var("SLINT_INCLUDE_PATH", src_path.to_string_lossy().as_ref());
    }

    let app_slint_path = std::path::Path::new(&manifest_dir).join("src/app.slint");
    slint_build::compile(&app_slint_path).unwrap();

    let protoc_path = std::env::var("PROTOC").unwrap_or_else(|_| {
        let default_bazel_path = std::path::PathBuf::from("../../../protobuf+/protoc");
        if default_bazel_path.exists() {
            default_bazel_path.to_string_lossy().into_owned()
        } else {
            "/usr/bin/protoc".to_string()
        }
    });
    // SAFETY: This is only called at build time and does not concurrently access env
    unsafe { std::env::set_var("PROTOC", protoc_path) };

    tonic_build::configure()
        .compile_protos(
            &["../proto/hub.proto"],
            &["../proto"],
        )?;
    Ok(())
}
