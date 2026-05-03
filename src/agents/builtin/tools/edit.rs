use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::fs;

use super::{Tool, ToolExecutor};

struct EditExecutor {
    working_dir: Option<std::path::PathBuf>,
}

#[async_trait::async_trait]
impl ToolExecutor for EditExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let path = args["path"].as_str().ok_or_else(|| ToolError::LlmRecoverable("edit: path is required".to_string()))?;
        let old_str = args["old_str"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("edit: old_str is required".to_string()))?;
        let new_str = args["new_str"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("edit: new_str is required".to_string()))?;

        let safe_path = std::path::Path::new(path).strip_prefix("/").unwrap_or(std::path::Path::new(path));
        let actual_path = if let Some(wd) = &self.working_dir { wd.join(safe_path) } else { std::path::PathBuf::from(path) };
        let content = fs::read_to_string(&actual_path)
            .await
            .map_err(|e| format!("edit: read {}: {}", actual_path.display(), e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        // Ensure exactly one occurrence.
        let count = content.matches(old_str).count();
        if count == 0 {
            return Err(ToolError::LlmRecoverable(format!(
                "edit: old_str not found in {} (must match exactly once)",
                path
            )));
        }
        if count > 1 {
            return Err(ToolError::LlmRecoverable(format!(
                "edit: old_str found {} times in {} (must match exactly once)",
                count, path
            )));
        }

        let new_content = content.replacen(old_str, new_str, 1);
        fs::write(&actual_path, &new_content)
            .await
            .map_err(|e| format!("edit: write {}: {}", path, e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        // Verification Loops (Quality x3): Computational/Guides (feedforward: linters, type-checkers, unit tests)
        if path.ends_with(".rs") {
            let wd = if let Some(w) = &self.working_dir {
                w.clone()
            } else {
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
            };

            if wd.join("Cargo.toml").exists() {
                let output = tokio::process::Command::new("cargo")
                    .current_dir(&wd)
                    .arg("check")
                    .output()
                    .await;

                if let Ok(out) = output {
                    if !out.status.success() {
                        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                        return Err(ToolError::LlmRecoverable(format!(
                            "File edited, but syntax check failed. Please fix the following errors:\n{}",
                            stderr
                        )));
                    }
                }
            }
        }

        Ok(format!("File edited: {}", path))
    }
}

pub fn edit_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "Edit".to_string(),
        description: "Replace exactly one occurrence of old_str with new_str in a file. \
            The old_str must appear exactly once in the file."
            .to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to edit."
                },
                "old_str": {
                    "type": "string",
                    "description": "The exact string to replace (must appear exactly once)."
                },
                "new_str": {
                    "type": "string",
                    "description": "The replacement string."
                }
            },
            "required": ["path", "old_str", "new_str"]
        }),
        execute: Arc::new(EditExecutor { working_dir }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_computational_guides_mechanic_in_edit() {
        let dir = tempfile::tempdir().unwrap();
        let wd = dir.path().to_path_buf();
        let executor = EditExecutor { working_dir: Some(wd.clone()) };

        // Dummy Cargo.toml
        tokio::fs::write(wd.join("Cargo.toml"), "[package]\nname = \"test_guide\"\nversion = \"0.1.0\"\n[lib]\npath = \"test_guide.rs\"").await.unwrap();
        tokio::fs::write(wd.join("test_guide.rs"), "fn main() {}").await.unwrap();

        // Edit with bad code
        let args = serde_json::json!({
            "path": "test_guide.rs",
            "old_str": "fn main() {}",
            "new_str": "fn main() { broken code }"
        });

        let result = executor.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("syntax check failed"));
        } else {
            panic!("Expected LlmRecoverable");
        }
    }
}
