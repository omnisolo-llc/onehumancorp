fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rustc-check-cfg=cfg(ohc_bazel)");
    println!("cargo::rustc-check-cfg=cfg(ohc_bazel_package)");
    Ok(())
}
