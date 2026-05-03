use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::fs;

use super::{Tool, ToolExecutor};

struct WriteExecutor {
    working_dir: Option<std::path::PathBuf>,
}

#[async_trait::async_trait]
impl ToolExecutor for WriteExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let path = args["path"].as_str().ok_or_else(|| ToolError::LlmRecoverable("write: path is required".to_string()))?;
        let content = args["content"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("write: content is required".to_string()))?;

        let safe_path = std::path::Path::new(path).strip_prefix("/").unwrap_or(std::path::Path::new(path));
        let actual_path = if let Some(wd) = &self.working_dir { wd.join(safe_path) } else { std::path::PathBuf::from(path) };

        // Create parent directories if needed.
        if let Some(parent) = actual_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("write: create dir {}: {}", parent.display(), e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
        }

        fs::write(&actual_path, content)
            .await
            .map_err(|e| format!("write: {}: {}", actual_path.display(), e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        // Verification Loops (Quality x3): Computational/Guides (feedforward: linters, type-checkers, unit tests)
        if path.ends_with(".rs") {
            let wd = if let Some(w) = &self.working_dir {
                w.clone()
            } else {
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
            };

            // Only run if cargo is available and Cargo.toml exists, or just run and handle missing gracefully
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
                            "File written, but syntax check failed. Please fix the following errors:\n{}",
                            stderr
                        )));
                    }
                }
            }
        }

        Ok(format!("File written: {}", path))
    }
}

pub fn write_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "Write".to_string(),
        description: "Write content to a file. Creates parent directories as needed. Overwrites any existing content.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to write."
                },
                "content": {
                    "type": "string",
                    "description": "Content to write to the file."
                }
            },
            "required": ["path", "content"]
        }),
        execute: Arc::new(WriteExecutor { working_dir }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_computational_guides_mechanic_in_write() {
        let dir = tempfile::tempdir().unwrap();
        let wd = dir.path().to_path_buf();
        let executor = WriteExecutor { working_dir: Some(wd.clone()) };

        // Dummy Cargo.toml
        tokio::fs::write(wd.join("Cargo.toml"), "[package]\nname = \"test_guide\"\nversion = \"0.1.0\"\n[lib]\npath = \"test_guide.rs\"").await.unwrap();

        // Write bad code
        let args = serde_json::json!({
            "path": "test_guide.rs",
            "content": "fn main() { broken code }"
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
