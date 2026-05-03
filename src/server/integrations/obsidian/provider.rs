use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct ObsidianProvider {
    metadata: ProviderMetadata,
    base_path: PathBuf,
}

impl ObsidianProvider {
    pub fn new(base_path: &str) -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "obsidian".to_string(),
                name: "Obsidian Local Knowledge Base".to_string(),
                category: "knowledge_base".to_string(),
                base_url: "local://obsidian".to_string(),
            },
            base_path: PathBuf::from(base_path),
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub fn read_file(&self, relative_path: &str) -> Result<String, String> {
        let path = PathBuf::from(relative_path);

        if path.is_absolute() {
             return Err("Access denied: absolute paths are not allowed".to_string());
        }

        for component in path.components() {
            if component.as_os_str() == ".." {
                return Err("Access denied: path is outside the base directory".to_string());
            }
        }

        let full_path = self.base_path.join(relative_path);

        if !full_path.exists() {
            return Err(format!("File not found: {}", relative_path));
        }

        fs::read_to_string(&full_path).map_err(|e| format!("Failed to read file: {}", e))
    }

    pub fn list_markdown_files(&self) -> Result<Vec<String>, String> {
        if !self.base_path.exists() {
            return Err("Base path does not exist".to_string());
        }

        let mut files = Vec::new();
        for entry in WalkDir::new(&self.base_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                if let Ok(rel_path) = path.strip_prefix(&self.base_path) {
                    if let Some(path_str) = rel_path.to_str() {
                        files.push(path_str.to_string());
                    }
                }
            }
        }
        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_obsidian_provider_read_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.md");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "# Hello Obsidian").unwrap();

        let provider = ObsidianProvider::new(dir.path().to_str().unwrap());
        let content = provider.read_file("test.md").unwrap();
        assert_eq!(content, "# Hello Obsidian\n");
    }

    #[test]
    fn test_obsidian_provider_list_files() {
        let dir = tempdir().unwrap();

        let file1_path = dir.path().join("doc1.md");
        File::create(&file1_path).unwrap();

        let subdir = dir.path().join("subdir");
        std::fs::create_dir(&subdir).unwrap();

        let file2_path = subdir.join("doc2.md");
        File::create(&file2_path).unwrap();

        // Non-markdown file
        let file3_path = dir.path().join("image.png");
        File::create(&file3_path).unwrap();

        let provider = ObsidianProvider::new(dir.path().to_str().unwrap());
        let mut files = provider.list_markdown_files().unwrap();

        files.sort();

        let mut expected = vec![
            "doc1.md".to_string(),
            PathBuf::from("subdir").join("doc2.md").to_str().unwrap().to_string(),
        ];
        expected.sort();

        assert_eq!(files, expected);
    }

    #[test]
    fn test_obsidian_provider_directory_traversal() {
        let dir = tempdir().unwrap();
        let provider = ObsidianProvider::new(dir.path().to_str().unwrap());

        let result = provider.read_file("../outside.md");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Access denied: path is outside the base directory");

        let result2 = provider.read_file("/etc/passwd");
        assert!(result2.is_err());
        assert_eq!(result2.unwrap_err(), "Access denied: absolute paths are not allowed");
    }
}
