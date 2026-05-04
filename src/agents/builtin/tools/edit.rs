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

        // Verification Loop: Computational/Guides (feedforward linters/type-checkers)
        if actual_path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let mut cmd = tokio::process::Command::new("rustc");
            cmd.arg("--emit=metadata").arg("--edition=2021").arg(&actual_path);

            if let Ok(output) = cmd.output().await {
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if !stderr.contains("E0432") && !stderr.contains("E0463") && !stderr.contains("E0433") {
                        return Err(ToolError::LlmRecoverable(format!(
                            "Verification Loop Failed: `rustc` reported syntax errors after editing {}.

Compiler Output:
{}

Please fix the errors and try again.",
                            path, stderr
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
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_edit_tool_basic() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "hello old world").await.unwrap();

        let executor = EditExecutor { working_dir: Some(dir.path().to_path_buf()) };

        let args = json!({
            "path": "test.txt",
            "old_str": "old",
            "new_str": "new"
        });

        let result = executor.execute(args).await.unwrap();
        assert_eq!(result, "File edited: test.txt");

        let content = fs::read_to_string(file_path).await.unwrap();
        assert_eq!(content, "hello new world");
    }

    #[tokio::test]
    async fn test_edit_tool_missing_args() {
        let executor = EditExecutor { working_dir: None };

        let args = json!({ "path": "test.txt", "old_str": "old" });
        let result = executor.execute(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_tool_multiple_matches() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "hello old old world").await.unwrap();

        let executor = EditExecutor { working_dir: Some(dir.path().to_path_buf()) };

        let args = json!({
            "path": "test.txt",
            "old_str": "old",
            "new_str": "new"
        });

        let result = executor.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("must match exactly once"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }

    #[tokio::test]
    async fn test_edit_tool_rust_verification_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        fs::write(&file_path, "fn main() { println!(\"old\"); }").await.unwrap();

        let executor = EditExecutor { working_dir: Some(dir.path().to_path_buf()) };

        let args = json!({
            "path": "test.rs",
            "old_str": "old",
            "new_str": "new"
        });

        // Should succeed and pass verification
        let result = executor.execute(args).await.unwrap();
        assert_eq!(result, "File edited: test.rs");
    }

    #[tokio::test]
    async fn test_edit_tool_rust_verification_failure() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        fs::write(&file_path, "fn main() { println!(\"old\"); }").await.unwrap();

        let executor = EditExecutor { working_dir: Some(dir.path().to_path_buf()) };

        let args = json!({
            "path": "test.rs",
            "old_str": "println!(\"old\");",
            "new_str": "let x = ;" // Introduce syntax error
        });

        // Should fail verification due to syntax error
        let result = executor.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Verification Loop Failed: `rustc` reported syntax errors"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }
}
