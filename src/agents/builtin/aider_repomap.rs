use std::fs;
use std::path::{Path, PathBuf};

/// Master Catalog: Aider: RepoMap for large codebases
///
/// The `RepoMap` provides a concise structural view of a codebase by walking
/// a target directory and generating a formatted tree-like structure.
/// This prevents blowing up the LLM token window while still providing
/// essential context about file locations and structural layout.
pub struct RepoMap {
    root_path: PathBuf,
}

impl RepoMap {
    /// Creates a new RepoMap for the given root path.
    pub fn new<P: AsRef<Path>>(root_path: P) -> Self {
        Self {
            root_path: root_path.as_ref().to_path_buf(),
        }
    }

    /// Generates the concise map of the codebase.
    pub fn generate_map(&self) -> Result<String, std::io::Error> {
        let mut result = String::new();
        self.walk_dir(&self.root_path, "", &mut result)?;
        Ok(result)
    }

    fn extract_signatures(path: &Path) -> Vec<String> {
        let mut signatures = Vec::new();
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return signatures,
        };

        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        for line in content.lines() {
            let trimmed = line.trim();
            match ext {
                "rs" => {
                    if trimmed.starts_with("fn ")
                        || trimmed.starts_with("pub fn ")
                        || trimmed.starts_with("pub(crate) fn ")
                        || trimmed.starts_with("struct ")
                        || trimmed.starts_with("pub struct ")
                        || trimmed.starts_with("impl ")
                        || trimmed.starts_with("trait ")
                        || trimmed.starts_with("pub trait ")
                        || trimmed.starts_with("enum ")
                        || trimmed.starts_with("pub enum ")
                    {
                        signatures.push(trimmed.to_string());
                    }
                }
                "go" => {
                    if trimmed.starts_with("func ") || trimmed.starts_with("type ") {
                        signatures.push(trimmed.to_string());
                    }
                }
                "ts" | "js" => {
                    if trimmed.starts_with("class ")
                        || trimmed.starts_with("export class ")
                        || trimmed.starts_with("function ")
                        || trimmed.starts_with("export function ")
                        || trimmed.starts_with("interface ")
                        || trimmed.starts_with("export interface ")
                        || trimmed.starts_with("type ")
                        || trimmed.starts_with("export type ")
                    {
                        signatures.push(trimmed.to_string());
                    }
                }
                _ => {}
            }
        }

        // Limit the number of signatures to avoid blowing up the map
        if signatures.len() > 50 {
            signatures.truncate(50);
            signatures.push("...".to_string());
        }

        signatures
    }

    fn walk_dir(
        &self,
        dir: &Path,
        prefix: &str,
        output: &mut String,
    ) -> Result<(), std::io::Error> {
        if !dir.is_dir() {
            return Ok(());
        }

        let mut entries: Vec<_> = fs::read_dir(dir)?.filter_map(|e| e.ok()).collect();
        // Sort entries alphabetically for deterministic output
        entries.sort_by_key(|e| e.file_name());

        // Filter out hidden files/directories like .git or .superpowers
        entries.retain(|e| {
            if let Some(name) = e.file_name().to_str() {
                !name.starts_with('.') && name != "target" && name != "build" && name != "dist"
            } else {
                false
            }
        });

        for (i, entry) in entries.iter().enumerate() {
            let is_last = i == entries.len() - 1;
            let file_name = entry.file_name().into_string().unwrap_or_default();

            let pointer = if is_last { "└── " } else { "├── " };

            output.push_str(&format!("{}{}{}\n", prefix, pointer, file_name));

            let path = entry.path();
            if path.is_dir() {
                let new_prefix = if is_last {
                    format!("{}    ", prefix)
                } else {
                    format!("{}│   ", prefix)
                };
                self.walk_dir(&path, &new_prefix, output)?;
            } else {
                let signatures = Self::extract_signatures(&path);
                if !signatures.is_empty() {
                    let new_prefix = if is_last {
                        format!("{}    ", prefix)
                    } else {
                        format!("{}│   ", prefix)
                    };
                    for (j, sig) in signatures.iter().enumerate() {
                        let sig_pointer = if j == signatures.len() - 1 {
                            "└── "
                        } else {
                            "├── "
                        };
                        output.push_str(&format!("{}{}{} \n", new_prefix, sig_pointer, sig));
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_repo_map_generation() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create some files and directories
        fs::create_dir(root.join("src")).unwrap();

        let mut main_rs = File::create(root.join("src/main.rs")).unwrap();
        main_rs
            .write_all(b"fn main() {\n    println!(\"Hello\");\n}\n")
            .unwrap();

        let mut lib_rs = File::create(root.join("src/lib.rs")).unwrap();
        lib_rs
            .write_all(b"pub struct MyStruct;\n\nimpl MyStruct {}\n")
            .unwrap();

        fs::create_dir(root.join("docs")).unwrap();
        File::create(root.join("docs/readme.md")).unwrap();

        File::create(root.join("Cargo.toml")).unwrap();

        // Hidden directory that should be ignored
        fs::create_dir(root.join(".git")).unwrap();
        File::create(root.join(".git/config")).unwrap();

        // Build directory that should be ignored
        fs::create_dir(root.join("target")).unwrap();
        File::create(root.join("target/debug")).unwrap();

        let repo_map = RepoMap::new(root);
        let output = repo_map.generate_map().unwrap();

        let expected_lines: Vec<&str> = vec![
            "├── Cargo.toml",
            "├── docs",
            "│   └── readme.md",
            "└── src",
            "    ├── lib.rs",
            "    │   ├── pub struct MyStruct;",
            "    │   └── impl MyStruct {}",
            "    └── main.rs",
            "        └── fn main() {",
        ];

        let actual_lines: Vec<&str> = output.lines().map(|l| l.trim_end()).collect();
        assert_eq!(actual_lines, expected_lines);
    }

    #[test]
    fn test_repo_map_extract_signatures_go() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        let mut main_go = File::create(root.join("main.go")).unwrap();
        main_go
            .write_all(b"package main\n\ntype MyType struct {}\n\nfunc main() {\n}\n")
            .unwrap();

        let repo_map = RepoMap::new(root);
        let output = repo_map.generate_map().unwrap();

        let expected_lines: Vec<&str> = vec![
            "└── main.go",
            "    ├── type MyType struct {}",
            "    └── func main() {",
        ];

        let actual_lines: Vec<&str> = output.lines().map(|l| l.trim_end()).collect();
        assert_eq!(actual_lines, expected_lines);
    }

    #[test]
    fn test_repo_map_extract_signatures_ts() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        let mut main_ts = File::create(root.join("main.ts")).unwrap();
        main_ts
            .write_all(b"export interface Config {}\n\nclass App {}\n\nfunction start() {}\n")
            .unwrap();

        let repo_map = RepoMap::new(root);
        let output = repo_map.generate_map().unwrap();

        let expected_lines: Vec<&str> = vec![
            "└── main.ts",
            "    ├── export interface Config {}",
            "    ├── class App {}",
            "    └── function start() {}",
        ];

        let actual_lines: Vec<&str> = output.lines().map(|l| l.trim_end()).collect();
        assert_eq!(actual_lines, expected_lines);
    }
}
