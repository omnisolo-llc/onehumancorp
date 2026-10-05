use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Source(PathBuf);
impl Source {
    fn new(bytes: &[u8]) -> Self {
        let path = std::env::temp_dir().join(format!(
            "ohc-source-{}-{}.rs",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::write(&path, bytes).unwrap();
        Self(path)
    }
    fn command(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_ohc-rust-source-extract"));
        command.arg(&self.0);
        command
    }
}
impl Drop for Source {
    fn drop(&mut self) {
        fs::remove_file(&self.0).unwrap();
    }
}

#[test]
fn cli_emits_exact_fragment_with_source_and_context_evidence() {
    let source =
        b"#[cfg(test)] mod inner { impl Item { #[inline] fn selected() { let brace = '}'; } } }";
    let input = Source::new(source);
    let output = input
        .command()
        .args([
            "function",
            "selected",
            "--module",
            "inner",
            "--impl-type",
            "Item",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let proof: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        proof["source_sha256"],
        format!("{:x}", Sha256::digest(source))
    );
    assert_eq!(
        proof["text"],
        "#[inline] fn selected() { let brace = '}'; }"
    );
    let start = proof["start"].as_u64().unwrap() as usize;
    let end = proof["end"].as_u64().unwrap() as usize;
    assert_eq!(
        &source[start..end],
        proof["text"].as_str().unwrap().as_bytes()
    );
    assert_eq!(proof["enclosing"].as_array().unwrap().len(), 2);
}

#[test]
fn cli_rejects_stale_expected_source_without_emitting_fragment() {
    let input = Source::new(b"fn selected() {}");
    let output = input
        .command()
        .args([
            "function",
            "selected",
            "--expect-source-sha256",
            &"0".repeat(64),
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("source fingerprint"));
}

#[test]
fn cli_rejects_unknown_or_duplicate_flags() {
    let input = Source::new(b"fn selected() {}");
    for args in [
        vec!["--unknown", "value"],
        vec!["--module", "one", "--module", "two"],
    ] {
        let output = input
            .command()
            .args(["function", "selected"])
            .args(args)
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
    }
}
