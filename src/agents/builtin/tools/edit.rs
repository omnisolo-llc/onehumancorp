use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::fs;

use super::{Tool, ToolExecutor};

struct EditExecutor {
    working_dir: Option<std::path::PathBuf>,
    #[cfg(test)]
    mock_command: Option<String>,
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
        fs::write(path, &new_content)
            .await
            .map_err(|e| format!("edit: write {}: {}", path, e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

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
                ToolError::LlmRecoverable(format!("edit verification failed: failed to execute cargo check: {}", e))
            })?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                return Err(ToolError::LlmRecoverable(format!(
                    "Verification loop failed: cargo check returned errors after editing {}.\nStdout: {}\nStderr: {}",
                    path, stdout, stderr
                )));
            }
        }

        Ok(format!("File edited: {}", path))
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use ohc_builtin_agent_core::types::ToolError;

    #[tokio::test]
    async fn test_edit_execute_verification_success() {
        let _ = tokio::fs::write("test_edit_verification.rs", "fn main() { println!(\"old\"); }").await;
        let executor = EditExecutor {
            working_dir: None,
            mock_command: Some("echo".to_string()),
        };
        let args = json!({
            "path": "test_edit_verification.rs",
            "old_str": "println!(\"old\");",
            "new_str": "println!(\"new\");"
        });
        let result = executor.execute(args).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "File edited: test_edit_verification.rs");
        let _ = tokio::fs::remove_file("test_edit_verification.rs").await;
    }

    #[tokio::test]
    async fn test_edit_execute_verification_failure() {
        let _ = tokio::fs::write("test_edit_verification_fail.rs", "fn main() { println!(\"old\"); }").await;
        let executor = EditExecutor {
            working_dir: None,
            mock_command: Some("false".to_string()),
        };
        let args = json!({
            "path": "test_edit_verification_fail.rs",
            "old_str": "println!(\"old\");",
            "new_str": "println!(\"new\");"
        });
        let result = executor.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Verification loop failed: cargo check returned errors after editing"));
        } else {
            panic!("Expected LlmRecoverable error for verification failure");
        }
        let _ = tokio::fs::remove_file("test_edit_verification_fail.rs").await;
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
        execute: Arc::new(EditExecutor {
            working_dir,
            #[cfg(test)]
            mock_command: None,
        }),
    }
}
