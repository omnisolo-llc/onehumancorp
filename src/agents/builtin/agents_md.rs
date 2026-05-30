use std::path::{Path, PathBuf};
use tokio::fs;

const MAX_CASCADING_BYTES: usize = 32768; // 32 KiB

pub struct AgentsMdLoader;

impl AgentsMdLoader {
    /// Loads cascading AGENTS.md files starting from the given path up to the root,
    /// or stops at the workspace boundary (e.g. where .git is found).
    /// Concatenates them (most deeply nested taking precedence, i.e. first in the prompt)
    /// and caps the result at 32 KiB.
    pub async fn load_cascading(start_path: &Path) -> String {
        let mut paths_to_read = Vec::new();

        let mut current_dir = if fs::metadata(start_path).await.map(|m| m.is_file()).unwrap_or(false) {
            start_path.parent().map(|p| p.to_path_buf())
        } else {
            Some(start_path.to_path_buf())
        };

        while let Some(dir) = current_dir {
            let agents_md_path = dir.join("AGENTS.md");
            if fs::metadata(&agents_md_path).await.map(|m| m.is_file()).unwrap_or(false) {
                paths_to_read.push(agents_md_path);
            }

            // Stop traversing up if we hit a workspace boundary (e.g., .git folder or workspace root)
            if fs::metadata(dir.join(".git")).await.is_ok()
                || fs::metadata(dir.join("WORKSPACE")).await.is_ok()
                || fs::metadata(dir.join("MODULE.bazel")).await.is_ok() {
                break;
            }

            current_dir = dir.parent().map(|p| p.to_path_buf());
        }

        let mut concatenated = String::new();

        // The most deeply nested AGENTS.md should take precedence. We'll append them in order of discovery
        // (which is from innermost to outermost).
        for path in paths_to_read {
            if let Ok(contents) = fs::read_to_string(&path).await {
                if !concatenated.is_empty() {
                    concatenated.push_str("\n\n");
                }
                concatenated.push_str(&format!("[Contents of {}]\n", path.display()));
                concatenated.push_str(&contents);
            }

            if concatenated.len() >= MAX_CASCADING_BYTES {
                break;
            }
        }

        // Cap at 32 KiB, taking care not to split a character boundary.
        let mut end_idx = MAX_CASCADING_BYTES;
        if concatenated.len() > MAX_CASCADING_BYTES {
            while end_idx > 0 && !concatenated.is_char_boundary(end_idx) {
                end_idx -= 1;
            }
            let mut truncated = concatenated[..end_idx].to_string();
            truncated.push_str("\n\n[System: AGENTS.md content truncated to 32KiB limit.]");
            truncated
        } else {
            concatenated
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a temp dir
    fn setup_temp_dir() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
    }

    #[tokio::test]
    async fn test_load_cascading_single_file() {
        let temp = setup_temp_dir();
        let md_path = temp.path().join("AGENTS.md");
        std::fs::write(&md_path, "Inner AGENTS").unwrap();

        let loaded = AgentsMdLoader::load_cascading(temp.path()).await;
        assert!(loaded.contains("Inner AGENTS"));
        assert!(loaded.contains(&md_path.display().to_string()));
    }

    #[tokio::test]
    async fn test_load_cascading_multiple_files_order() {
        let temp = setup_temp_dir();
        let root_md = temp.path().join("AGENTS.md");
        std::fs::write(&root_md, "Root AGENTS").unwrap();

        let sub_dir = temp.path().join("sub");
        std::fs::create_dir(&sub_dir).unwrap();
        let sub_md = sub_dir.join("AGENTS.md");
        std::fs::write(&sub_md, "Sub AGENTS").unwrap();

        let loaded = AgentsMdLoader::load_cascading(&sub_dir).await;

        // Deeply nested takes precedence (concatenated first)
        let root_idx = loaded.find("Root AGENTS").unwrap();
        let sub_idx = loaded.find("Sub AGENTS").unwrap();
        assert!(sub_idx < root_idx, "Sub directory should be processed before root directory");
    }

    #[tokio::test]
    async fn test_stops_at_workspace_boundary() {
        let temp = setup_temp_dir();
        let root_md = temp.path().join("AGENTS.md");
        std::fs::write(&root_md, "Root AGENTS").unwrap();

        let workspace_file = temp.path().join("WORKSPACE");
        std::fs::write(&workspace_file, "").unwrap();

        let sub_dir = temp.path().join("sub");
        std::fs::create_dir(&sub_dir).unwrap();
        let sub_md = sub_dir.join("AGENTS.md");
        std::fs::write(&sub_md, "Sub AGENTS").unwrap();

        let outer_dir = setup_temp_dir();
        let outer_md = outer_dir.path().join("AGENTS.md");
        std::fs::write(&outer_md, "Outer AGENTS").unwrap();

        let ws_dir = outer_dir.path().join("ws");
        std::fs::create_dir(&ws_dir).unwrap();
        let ws_file = ws_dir.join("WORKSPACE");
        std::fs::write(&ws_file, "").unwrap();
        let ws_md = ws_dir.join("AGENTS.md");
        std::fs::write(&ws_md, "WS AGENTS").unwrap();

        let inner_dir = ws_dir.join("inner");
        std::fs::create_dir(&inner_dir).unwrap();
        let inner_md = inner_dir.join("AGENTS.md");
        std::fs::write(&inner_md, "Inner AGENTS").unwrap();

        let loaded = AgentsMdLoader::load_cascading(&inner_dir).await;

        assert!(loaded.contains("Inner AGENTS"));
        assert!(loaded.contains("WS AGENTS"));
        assert!(!loaded.contains("Outer AGENTS"));
    }

    #[tokio::test]
    async fn test_caps_at_32_kib() {
        let temp = setup_temp_dir();
        let md_path = temp.path().join("AGENTS.md");

        // Create an AGENTS.md that is > 32 KiB
        let content = "A".repeat(40000);
        std::fs::write(&md_path, content).unwrap();

        let loaded = AgentsMdLoader::load_cascading(temp.path()).await;
        // Total length will be truncated to 32KiB and then the system notice appended
        assert!(loaded.len() <= MAX_CASCADING_BYTES + 100);
        assert!(loaded.ends_with("[System: AGENTS.md content truncated to 32KiB limit.]"));
    }
}
