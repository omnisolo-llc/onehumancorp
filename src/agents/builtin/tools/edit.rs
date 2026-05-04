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
            .map_err(|e| format!("edit: write {}: {}", actual_path.display(), e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        // Computational Guides: Verification loop
        if let Some(ext) = actual_path.extension().and_then(|e| e.to_str()) {
            if ext == "rs" {
                let output = tokio::process::Command::new("rustfmt")
                    .arg(&actual_path)
                    .output()
                    .await;
                if let Ok(out) = output {
                    if !out.status.success() {
                        // Revert changes
                        let _ = fs::write(&actual_path, &content).await;
                        let stderr = String::from_utf8_lossy(&out.stderr);
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        return Err(ToolError::LlmRecoverable(format!(
                            "Syntax validation failed (rustfmt). Changes reverted. Fix the syntax and try again.\nSTDOUT:\n{}\nSTDERR:\n{}",
                            stdout, stderr
                        )));
                    }
                }
            } else if ext == "py" {
                let output = tokio::process::Command::new("python3")
                    .arg("-m")
                    .arg("py_compile")
                    .arg(&actual_path)
                    .output()
                    .await;
                if let Ok(out) = output {
                    if !out.status.success() {
                        // Revert changes
                        let _ = fs::write(&actual_path, &content).await;
                        let stderr = String::from_utf8_lossy(&out.stderr);
                        return Err(ToolError::LlmRecoverable(format!(
                            "Syntax validation failed (py_compile). Changes reverted. Fix the syntax and try again.\nSTDERR:\n{}",
                            stderr
                        )));
                    }
                }
            }
        }

        Ok(format!("File edited: {}", path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_edit_tool_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        tokio::fs::write(&file_path, "hello world").await.unwrap();

        let tool = edit_tool(Some(dir.path().to_path_buf()));
        let args = json!({
            "path": "test.txt",
            "old_str": "world",
            "new_str": "rust"
        });

        let result = tool.execute.execute(args).await;
        assert!(result.is_ok());
        let content = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, "hello rust");
    }

    #[tokio::test]
    async fn test_edit_tool_validation_failure_rs() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        tokio::fs::write(&file_path, "fn main() { let x = 1; }").await.unwrap();

        let tool = edit_tool(Some(dir.path().to_path_buf()));
        let args = json!({
            "path": "test.rs",
            "old_str": "1; }",
            "new_str": "1;" // Missing closing brace
        });

        let result = tool.execute.execute(args).await;
        assert!(result.is_err());
        match result {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert!(msg.contains("Syntax validation failed (rustfmt)"));
            }
            _ => panic!("Expected LlmRecoverable error"),
        }

        let content = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, "fn main() { let x = 1; }"); // Reverted
    }

    #[tokio::test]
    async fn test_edit_tool_validation_failure_py() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.py");
        tokio::fs::write(&file_path, "print('hello')").await.unwrap();

        let tool = edit_tool(Some(dir.path().to_path_buf()));
        let args = json!({
            "path": "test.py",
            "old_str": "('hello')",
            "new_str": "('hello'" // Missing closing parenthesis
        });

        let result = tool.execute.execute(args).await;
        assert!(result.is_err());
        match result {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert!(msg.contains("Syntax validation failed (py_compile)"));
            }
            _ => panic!("Expected LlmRecoverable error"),
        }

        let content = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, "print('hello')"); // Reverted
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
