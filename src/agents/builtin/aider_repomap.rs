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

    fn walk_dir(&self, dir: &Path, prefix: &str, output: &mut String) -> Result<(), std::io::Error> {
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
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    #[test]
    fn test_repo_map_generation() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create some files and directories
        fs::create_dir(root.join("src")).unwrap();
        File::create(root.join("src/main.rs")).unwrap();
        File::create(root.join("src/lib.rs")).unwrap();

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

        // The expected output should be sorted and look like:
        // ├── Cargo.toml
        // ├── docs
        // │   └── readme.md
        // └── src
        //     ├── lib.rs
        //     └── main.rs

        let expected_lines: Vec<&str> = vec![
            "├── Cargo.toml",
            "├── docs",
            "│   └── readme.md",
            "└── src",
            "    ├── lib.rs",
            "    └── main.rs",
        ];

        let actual_lines: Vec<&str> = output.lines().collect();
        assert_eq!(actual_lines, expected_lines);
    }
}
