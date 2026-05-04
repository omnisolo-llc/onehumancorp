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

        // Verification Loop: Computational/Guides (feedforward linters/type-checkers)
        if actual_path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let mut cmd = tokio::process::Command::new("rustfmt");
            // Check only the specific file to avoid whole-workspace errors when other files are broken
            cmd.arg("--edition=2021").arg(&actual_path);

            if let Ok(output) = cmd.output().await {
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(ToolError::LlmRecoverable(format!(
                        "Verification Loop Failed: `rustfmt` reported syntax errors after writing to {}.

Compiler Output:
{}

Please fix the errors and try again.",
                        path, stderr
                    )));
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
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_write_tool_basic() {
        let dir = tempdir().unwrap();
        let executor = WriteExecutor { working_dir: Some(dir.path().to_path_buf()) };

        let args = json!({
            "path": "test.txt",
            "content": "hello world"
        });

        let result = executor.execute(args).await.unwrap();
        assert_eq!(result, "File written: test.txt");

        let content = fs::read_to_string(dir.path().join("test.txt")).await.unwrap();
        assert_eq!(content, "hello world");
    }

    #[tokio::test]
    async fn test_write_tool_missing_args() {
        let executor = WriteExecutor { working_dir: None };

        let args = json!({ "path": "test.txt" });
        let result = executor.execute(args).await;
        assert!(result.is_err());

        let args2 = json!({ "content": "test" });
        let result2 = executor.execute(args2).await;
        assert!(result2.is_err());
    }

    #[tokio::test]
    async fn test_write_tool_rust_verification_success() {
        let dir = tempdir().unwrap();
        let executor = WriteExecutor { working_dir: Some(dir.path().to_path_buf()) };

        let args = json!({
            "path": "test.rs",
            "content": "fn main() { println!(\"Hello\"); }"
        });

        // Should succeed and pass verification
        let result = executor.execute(args).await.unwrap();
        assert_eq!(result, "File written: test.rs");
    }

    #[tokio::test]
    async fn test_write_tool_rust_verification_failure() {
        let dir = tempdir().unwrap();
        let executor = WriteExecutor { working_dir: Some(dir.path().to_path_buf()) };

        let args = json!({
            "path": "test.rs",
            "content": "fn main() { let x = ; }"
        });

        // Should fail verification due to syntax error
        let result = executor.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Verification Loop Failed: `rustfmt` reported syntax errors"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }
}
