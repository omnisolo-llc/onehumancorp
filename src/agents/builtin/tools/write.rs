use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::fs;

use super::{Tool, TypedToolExecutor, TypedToolExecutorImpl};

#[derive(serde::Deserialize, Debug)]
pub struct WriteArgs {
    pub path: String,
    pub content: String,
}

struct WriteExecutor {
    working_dir: Option<std::path::PathBuf>,
    runner: Arc<dyn crate::runner::CommandRunner>,
}

#[async_trait::async_trait]
impl TypedToolExecutorImpl<WriteArgs> for WriteExecutor {
    async fn execute_typed(
        &self,
        args: WriteArgs,
    ) -> Result<String, ToolError> {
        let path = args.path;
        let content = args.content;

        let safe_path = std::path::Path::new(&path).strip_prefix("/").unwrap_or(std::path::Path::new(&path));
        let actual_path = if let Some(wd) = &self.working_dir { wd.join(safe_path) } else { std::path::PathBuf::from(&path) };

        // Create parent directories if needed.
        if let Some(parent) = actual_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("write: create dir {}: {}", parent.display(), e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
        }

        fs::write(&actual_path, &content)
            .await
            .map_err(|e| format!("write: {}: {}", actual_path.display(), e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        // Verification Loop: Computational/Guides (feedforward linters/type-checkers)
        if actual_path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let actual_path_str = actual_path.to_string_lossy();
            // Note: In Bazel/Cargo projects, `rustc` alone may miss external crate dependencies
            // and fail with `E0432`. However, it catches pure syntax errors and basic type errors
            // within the file itself without requiring a full `cargo check` of a potentially broken tree.
            // We ignore errors related to missing external crates (E0432, E0463) as they are false positives
            // when running `rustc` outside the build system.
            match self.runner.run("rustc", &["--emit=metadata", "--edition=2021", &actual_path_str], None, vec![]).await {
                Ok(output) => {
                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        if !stderr.contains("E0432") && !stderr.contains("E0463") && !stderr.contains("E0433") {
                            return Err(ToolError::LlmRecoverable(format!(
                                "Verification Loop Failed: `rustc` reported syntax errors after writing to {}.

Compiler Output:
{}

Please fix the errors and try again.",
                                path, stderr
                            )));
                        }
                    }
                }
                Err(e) => {
                    tracing::debug!("Verification skipped: failed to run rustc: {}", e);
                }
            }
        }



        Ok(format!("File written: {}", path))
    }
}

pub fn write_tool(working_dir: Option<std::path::PathBuf>, runner: Arc<dyn crate::runner::CommandRunner>) -> Tool {
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
        execute: Arc::new(TypedToolExecutor::new(Arc::new(WriteExecutor { working_dir, runner }))),
    }
}

#[cfg(test)]
mod tests {
use crate::ToolExecutor;
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_write_tool_basic() {
        let dir = tempdir().unwrap();
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = TypedToolExecutor::new(Arc::new(WriteExecutor { working_dir: Some(dir.path().to_path_buf()), runner }));

        let args = json!({
            "path": "test.txt",
            "content": "hello world"
        });

        let result = ToolExecutor::execute(&executor, args).await.unwrap();
        assert_eq!(result, "File written: test.txt");

        let content = fs::read_to_string(dir.path().join("test.txt")).await.unwrap();
        assert_eq!(content, "hello world");
    }

    #[tokio::test]
    async fn test_write_tool_missing_args() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = TypedToolExecutor::new(Arc::new(WriteExecutor { working_dir: None, runner }));

        let args = json!({ "path": "test.txt" });
        let result = ToolExecutor::execute(&executor, args).await;
        assert!(result.is_err());

        let args2 = json!({ "content": "test" });
        let result2 = ToolExecutor::execute(&executor, args2).await;
        assert!(result2.is_err());
    }

    #[tokio::test]
    async fn test_write_tool_rust_verification_success() {
        let dir = tempdir().unwrap();
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = TypedToolExecutor::new(Arc::new(WriteExecutor { working_dir: Some(dir.path().to_path_buf()), runner }));

        let args = json!({
            "path": "test.rs",
            "content": "fn main() { println!(\"Hello\"); }"
        });

        // Should succeed and pass verification
        let result = ToolExecutor::execute(&executor, args).await.unwrap();
        assert_eq!(result, "File written: test.rs");
    }

    #[tokio::test]
    async fn test_write_tool_rust_verification_failure() {
        let dir = tempdir().unwrap();
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        // Simulate rustc failure
        runner.push_response(Ok(crate::runner::mock::mock_output(1, "", "error: expected expression, found `;`")));

        let executor = TypedToolExecutor::new(Arc::new(WriteExecutor { working_dir: Some(dir.path().to_path_buf()), runner }));

        let args = json!({
            "path": "test.rs",
            "content": "fn main() { let x = ; }"
        });

        // Should fail verification due to syntax error
        let result = ToolExecutor::execute(&executor, args).await;
        assert!(result.is_err(), "Expected error from mock rustc verification");
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Verification Loop Failed: `rustc` reported syntax errors"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }
}
