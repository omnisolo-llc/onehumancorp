fn main() {
    println!("cargo::rustc-check-cfg=cfg(ohc_bazel_tauri_context)");
    tauri_build::build()
}
