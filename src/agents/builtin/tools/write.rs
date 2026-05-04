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

        let file_existed = actual_path.exists();
        let old_content = if file_existed {
            fs::read_to_string(&actual_path).await.unwrap_or_default()
        } else {
            String::new()
        };

        fs::write(&actual_path, content)
            .await
            .map_err(|e| format!("write: {}: {}", actual_path.display(), e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

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
                        if file_existed {
                            let _ = fs::write(&actual_path, &old_content).await;
                        } else {
                            let _ = fs::remove_file(&actual_path).await;
                        }
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
                        if file_existed {
                            let _ = fs::write(&actual_path, &old_content).await;
                        } else {
                            let _ = fs::remove_file(&actual_path).await;
                        }
                        let stderr = String::from_utf8_lossy(&out.stderr);
                        return Err(ToolError::LlmRecoverable(format!(
                            "Syntax validation failed (py_compile). Changes reverted. Fix the syntax and try again.\nSTDERR:\n{}",
                            stderr
                        )));
                    }
                }
            }
        }

        Ok(format!("File written: {}", path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_write_tool_success() {
        let dir = tempdir().unwrap();
        let tool = write_tool(Some(dir.path().to_path_buf()));
        let args = json!({
            "path": "test.txt",
            "content": "hello world"
        });

        let result = tool.execute.execute(args).await;
        assert!(result.is_ok());
        let file_path = dir.path().join("test.txt");
        let content = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, "hello world");
    }

    #[tokio::test]
    async fn test_write_tool_validation_failure_rs() {
        let dir = tempdir().unwrap();
        let tool = write_tool(Some(dir.path().to_path_buf()));
        let args = json!({
            "path": "test.rs",
            "content": "fn main() { let x = 1;" // Missing closing brace
        });

        let result = tool.execute.execute(args).await;
        assert!(result.is_err());
        match result {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert!(msg.contains("Syntax validation failed (rustfmt)"));
            }
            _ => panic!("Expected LlmRecoverable error"),
        }

        let file_path = dir.path().join("test.rs");
        assert!(!file_path.exists()); // Should be deleted since it didn't exist before
    }

    #[tokio::test]
    async fn test_write_tool_validation_failure_py_revert() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.py");
        tokio::fs::write(&file_path, "print('valid')").await.unwrap();

        let tool = write_tool(Some(dir.path().to_path_buf()));
        let args = json!({
            "path": "test.py",
            "content": "print('hello'" // Missing closing parenthesis
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
        assert_eq!(content, "print('valid')"); // Should be reverted to old content
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
