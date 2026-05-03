use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::fs;

use super::{Tool, ToolExecutor};

struct WriteExecutor {
    working_dir: Option<std::path::PathBuf>,
    #[cfg(test)]
    mock_command: Option<String>,
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

        // Verification Loop Mechanic: Computational/Guides
        if actual_path.extension().and_then(|e| e.to_str()) == Some("rs") {
            #[cfg(not(test))]
            let base_cmd = "cargo";

            #[cfg(test)]
            let base_cmd = self.mock_command.as_deref().unwrap_or("cargo");

            let mut cmd = tokio::process::Command::new(base_cmd);
            cmd.arg("check").arg("--color=never");

            #[cfg(test)]
            if base_cmd == "echo" {
                // For testing success
                cmd.arg("success");
            } else if base_cmd == "false" {
                // For testing failure exit code
            }

            if let Some(wd) = &self.working_dir {
                cmd.current_dir(wd);
            }

            let output = cmd.output().await.map_err(|e| {
                ToolError::LlmRecoverable(format!("write verification failed: failed to execute cargo check: {}", e))
            })?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                return Err(ToolError::LlmRecoverable(format!(
                    "Verification loop failed: cargo check returned errors after writing {}.\nStdout: {}\nStderr: {}",
                    path, stdout, stderr
                )));
            }
        }

        Ok(format!("File written: {}", path))
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use ohc_builtin_agent_core::types::ToolError;

    #[tokio::test]
    async fn test_write_execute_verification_success() {
        let executor = WriteExecutor {
            working_dir: None,
            mock_command: Some("echo".to_string()),
        };
        let args = json!({ "path": "test_verification.rs", "content": "fn main() {}" });
        let result = executor.execute(args).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "File written: test_verification.rs");
        let _ = tokio::fs::remove_file("test_verification.rs").await;
    }

    #[tokio::test]
    async fn test_write_execute_verification_failure() {
        let executor = WriteExecutor {
            working_dir: None,
            mock_command: Some("false".to_string()),
        };
        let args = json!({ "path": "test_verification_fail.rs", "content": "fn main() {" });
        let result = executor.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Verification loop failed: cargo check returned errors"));
        } else {
            panic!("Expected LlmRecoverable error for verification failure");
        }
        let _ = tokio::fs::remove_file("test_verification_fail.rs").await;
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
        execute: Arc::new(WriteExecutor {
            working_dir,
            #[cfg(test)]
            mock_command: None,
        }),
    }
}
