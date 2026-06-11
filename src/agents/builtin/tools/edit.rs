use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::fs;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

#[derive(Deserialize)]
struct EditArgs {
    path: String,
    old_str: String,
    new_str: String,
}

struct EditExecutor {
    working_dir: Option<std::path::PathBuf>,
    runner: Arc<dyn crate::runner::CommandRunner>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<EditArgs> for EditExecutor {
    async fn execute_typed(&self, args: EditArgs) -> Result<String, ToolError> {
        let path = &args.path;
        let old_str = &args.old_str;
        let new_str = &args.new_str;

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

        // Verification Loop: Computational/Guides (feedforward linters/type-checkers)
        if actual_path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let tmp_path = actual_path.with_extension("tmp.rs");
            fs::write(&tmp_path, &new_content)
                .await
                .map_err(|e| format!("edit: temp file {}: {}", tmp_path.display(), e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

            let tmp_path_str = tmp_path.to_string_lossy();
            let res = self.runner.run("rustc", &["--emit=metadata", "--edition=2021", &tmp_path_str], None, vec![]).await;

            // Clean up temp file
            let _ = fs::remove_file(&tmp_path).await;

            match res {
                Ok(output) => {
                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        if !stderr.contains("E0432") && !stderr.contains("E0463") && !stderr.contains("E0433") {
                            let actual_path_str = actual_path.to_string_lossy();
                            let clean_stderr = stderr.replace(&*tmp_path_str, &actual_path_str);
                            return Err(ToolError::LlmRecoverable(format!(
                                "Verification Loop Failed: `rustc` reported syntax errors before editing {}.

Compiler Output:
{}

Please fix the errors and try again.",
                                path, clean_stderr
                            )));
                        }
                    }
                }
                Err(e) => {
                    tracing::debug!("Verification skipped: failed to run rustc: {}", e);
                }
            }
        }

        fs::write(&actual_path, &new_content)
            .await
            .map_err(|e| format!("edit: write {}: {}", actual_path.display(), e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;



        Ok(format!("File edited: {}", path))
    }
}

pub fn edit_tool(working_dir: Option<std::path::PathBuf>, runner: Arc<dyn crate::runner::CommandRunner>) -> Tool {
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
        execute: Arc::new(PydanticAdapter::new(EditExecutor { working_dir, runner })),
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

        let runner = crate::runner::mock::MockCommandRunner::new_arc();
        let executor = PydanticAdapter::new(EditExecutor { working_dir: Some(dir.path().to_path_buf()), runner });

        let args = json!({
            "path": "test.txt",
            "old_str": "old",
            "new_str": "new"
        });

        let result = super::super::ToolExecutor::execute(&executor, args).await.unwrap();
        assert_eq!(result, "File edited: test.txt");

        let content = fs::read_to_string(file_path).await.unwrap();
        assert_eq!(content, "hello new world");
    }

    #[tokio::test]
    async fn test_edit_tool_missing_args() {
        let runner = crate::runner::mock::MockCommandRunner::new_arc();
        let executor = PydanticAdapter::new(EditExecutor { working_dir: None, runner });

        let args = json!({ "path": "test.txt", "old_str": "old" });
        let result = super::super::ToolExecutor::execute(&executor, args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }

    #[tokio::test]
    async fn test_edit_tool_multiple_matches() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "hello old old world").await.unwrap();

        let runner = crate::runner::mock::MockCommandRunner::new_arc();
        let executor = PydanticAdapter::new(EditExecutor { working_dir: Some(dir.path().to_path_buf()), runner });

        let args = json!({
            "path": "test.txt",
            "old_str": "old",
            "new_str": "new"
        });

        let result = super::super::ToolExecutor::execute(&executor, args).await;
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

        let runner = crate::runner::mock::MockCommandRunner::new_arc();
        let executor = PydanticAdapter::new(EditExecutor { working_dir: Some(dir.path().to_path_buf()), runner });

        let args = json!({
            "path": "test.rs",
            "old_str": "old",
            "new_str": "new"
        });

        // Should succeed and pass verification
        let result = super::super::ToolExecutor::execute(&executor, args).await.unwrap();
        assert_eq!(result, "File edited: test.rs");
    }

    #[tokio::test]
    async fn test_edit_tool_rust_verification_failure() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        fs::write(&file_path, "fn main() { println!(\"old\"); }").await.unwrap();

        let runner = crate::runner::mock::MockCommandRunner::new_arc();
        // Simulate rustc failure
        runner.push_response(Ok(crate::runner::mock::mock_output(1, "", "error: expected expression, found `;`")));

        let executor = PydanticAdapter::new(EditExecutor { working_dir: Some(dir.path().to_path_buf()), runner });

        let args = json!({
            "path": "test.rs",
            "old_str": "println!(\"old\");",
            "new_str": "let x = ;" // Introduce syntax error
        });

        // Should fail verification due to syntax error
        let result = super::super::ToolExecutor::execute(&executor, args).await;
        assert!(result.is_err(), "Expected error from mock rustc verification");
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Verification Loop Failed: `rustc` reported syntax errors"));
        } else {
            panic!("Expected LlmRecoverable error");
        }

        // Assert the target file was NOT modified since validation failed
        let content = fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, "fn main() { println!(\"old\"); }", "Target file should not be modified if verification fails");
    }
}
