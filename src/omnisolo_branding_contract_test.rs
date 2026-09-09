//! Source contract for the OmniSolo brand and OmniSolo OneHumanCorp product.
//!
//! This test intentionally checks user-facing source only. Legacy `OHC_*`
//! environment variables, cookie names, API aliases, and deployment keys are
//! compatibility contracts and are documented in `docs/omnisolo-compatibility.md`.

use std::fs;
use std::path::{Path, PathBuf};

fn repository_root() -> PathBuf {
    let mut path = PathBuf::from(option_env!("CARGO_MANIFEST_DIR").unwrap_or("."));
    while !path.join("src/ui/next/src/app").is_dir() {
        let Some(parent) = path.parent() else {
            panic!("unable to locate repository root from {}", path.display());
        };
        path = parent.to_path_buf();
    }
    path
}

fn source_files(root: &Path, relative: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let directory = root.join(relative);
    let mut pending = vec![directory];
    while let Some(path) = pending.pop() {
        let Ok(entries) = fs::read_dir(&path) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.file_name().is_some_and(|name| name == "node_modules") {
                continue;
            }
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| {
                matches!(
                    extension.to_str(),
                    Some("tsx" | "ts" | "md" | "html" | "mjs")
                )
            }) {
                files.push(path);
            }
        }
    }
    files
}

#[test]
fn user_facing_sources_use_omnisolo_branding() {
    let root = repository_root();
    let mut violations = Vec::new();
    for relative in [
        "src/ui/next/src/app",
        "src/ui/next/src/components",
        "src/ui/tauri/src/ui",
        "docs",
    ] {
        let source_is_user_facing = relative.starts_with("src/ui/");
        for path in source_files(&root, relative) {
            // The plan/spec files describe this compatibility contract and are
            // not product copy rendered to users.
            if path.to_string_lossy().contains("docs/superpowers/") {
                continue;
            }
            let Ok(source) = fs::read_to_string(&path) else {
                continue;
            };
            for (line_number, line) in source.lines().enumerate() {
                let normalized_product = line.replace("OmniSolo OneHumanCorp", "OmniSolo");
                let line = normalized_product.as_str();
                // Introductory migration context may name the former product.
                if !source_is_user_facing && line.contains("(formerly One Human Corp / OHC)") {
                    continue;
                }
                let contains_legacy_acronym = source_is_user_facing
                    && line
                        .split(|character: char| !character.is_ascii_alphanumeric())
                        .any(|word| word == "OHC")
                    && !line.contains("OHC_")
                    && !line.contains("window.OHC_");
                if contains_legacy_acronym
                    || line.contains("Powered by OHC")
                    || line.contains("OneHumanCorp")
                    || line.contains("One Human Corp")
                    || (source_is_user_facing
                        && (line.contains("ohc.app") || line.contains("onehumancorp.com")))
                    || (!source_is_user_facing && line.contains("https://ohc.app"))
                    || (!source_is_user_facing && line.contains("https://onehumancorp.com"))
                {
                    violations.push(format!(
                        "{}:{}: {}",
                        path.strip_prefix(&root).unwrap_or(&path).display(),
                        line_number + 1,
                        line.trim()
                    ));
                }
            }
        }
    }
    assert!(
        violations.is_empty(),
        "legacy product branding remains in user-facing source:\n{}",
        violations.join("\n")
    );
}
