fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_path = std::path::Path::new(&manifest_dir);
    unsafe {
        std::env::set_var("SLINT_INCLUDE_PATH", src_path.to_string_lossy().as_ref());
    }

    let app_slint_path = std::path::Path::new(&manifest_dir).join("app.slint");
    slint_build::compile(&app_slint_path).unwrap();

    Ok(())
}
