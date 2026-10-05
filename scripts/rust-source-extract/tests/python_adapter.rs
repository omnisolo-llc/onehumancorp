#[test]
fn python_adapter_preserves_source_and_fails_closed() {
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("test_rust_source.py");
    let output = std::process::Command::new("python3")
        .arg("-B")
        .arg(script)
        .env(
            "OHC_RUST_EXTRACT_TEST_BINARY",
            env!("CARGO_BIN_EXE_ohc-rust-source-extract"),
        )
        .output()
        .expect("native build requires Python 3");
    assert!(
        output.status.success(),
        "Python exact-source adapter failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}
