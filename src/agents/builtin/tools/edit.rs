use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use super::{
    Tool,
    pydantic::{PydanticAdapter, PydanticToolExecutor},
    workspace_path,
};

const MAX_EDIT_BYTES: u64 = 4 * 1024 * 1024;

#[derive(Deserialize)]
struct EditArgs {
    path: String,
    old_str: String,
    new_str: String,
}

struct EditExecutor {
    workspace_root: Result<std::path::PathBuf, String>,
    runner: Arc<dyn crate::runner::CommandRunner>,
}

impl EditExecutor {
    async fn verify_rust(
        &self,
        temp_path: &std::path::Path,
        actual_path: &std::path::Path,
        requested_path: &str,
    ) -> Result<(), ToolError> {
        let temp_path_string = temp_path.to_string_lossy();
        let metadata_path = temp_path.with_extension("rmeta");
        let metadata_path_string = metadata_path.to_string_lossy();
        let verification = self
            .runner
            .run(
                "rustc",
                &[
                    "--emit=metadata",
                    "--edition=2021",
                    "--crate-name",
                    "ohc_edit_check",
                    "-o",
                    &metadata_path_string,
                    &temp_path_string,
                ],
                None,
                vec![],
            )
            .await;
        let _ = fs::remove_file(&metadata_path).await;

        match verification {
            Ok(output) if !output.status.success() => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.contains("E0432")
                    && !stderr.contains("E0463")
                    && !stderr.contains("E0433")
                {
                    let actual_path_string = actual_path.to_string_lossy();
                    let clean_stderr = stderr.replace(&*temp_path_string, &actual_path_string);
                    return Err(ToolError::LlmRecoverable(format!(
                        "Verification Loop Failed: `rustc` reported syntax errors before editing {}.\n\nCompiler Output:\n{}\n\nPlease fix the errors and try again.",
                        requested_path, clean_stderr
                    )));
                }
            }
            Err(error) => {
                tracing::debug!("Verification skipped: failed to run rustc: {}", error);
            }
            Ok(_) => {}
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl PydanticToolExecutor<EditArgs> for EditExecutor {
    async fn execute_typed(&self, args: EditArgs) -> Result<String, ToolError> {
        let path = &args.path;
        let old_str = &args.old_str;
        let new_str = &args.new_str;

        let root = self.workspace_root.as_ref().map_err(|error| {
            ToolError::LlmRecoverable(format!("edit: workspace root is unavailable: {error}"))
        })?;
        let actual_path = workspace_path::existing(root, path).await?;
        let metadata = fs::metadata(&actual_path).await.map_err(|error| {
            ToolError::LlmRecoverable(format!("edit: inspect {}: {}", path, error))
        })?;
        if metadata.len() > MAX_EDIT_BYTES {
            return Err(ToolError::LlmRecoverable(
                "edit: file exceeds 4 MiB".to_string(),
            ));
        }
        let existing_permissions = metadata.permissions();
        let content = fs::read_to_string(&actual_path)
            .await
            .map_err(|e| format!("edit: read {}: {}", actual_path.display(), e))
            .map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

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
        if new_content.len() as u64 > MAX_EDIT_BYTES {
            return Err(ToolError::LlmRecoverable(
                "edit: resulting file exceeds 4 MiB".to_string(),
            ));
        }

        let parent = actual_path.parent().ok_or_else(|| {
            ToolError::LlmRecoverable(format!("edit: {} has no parent directory", path))
        })?;
        let file_name = actual_path
            .file_name()
            .ok_or_else(|| ToolError::LlmRecoverable(format!("edit: {} has no file name", path)))?;
        let temp_path = parent.join(format!(
            ".{}.{}.tmp",
            file_name.to_string_lossy(),
            uuid::Uuid::new_v4()
        ));
        let edit_result = async {
            let mut temp_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temp_path)
                .await
                .map_err(|error| {
                    ToolError::LlmRecoverable(format!(
                        "edit: temp file {}: {}",
                        temp_path.display(),
                        error
                    ))
                })?;
            fs::set_permissions(&temp_path, existing_permissions.clone())
                .await
                .map_err(|error| {
                    ToolError::LlmRecoverable(format!("edit: set temp permissions: {error}"))
                })?;
            temp_file
                .write_all(new_content.as_bytes())
                .await
                .map_err(|error| {
                    ToolError::LlmRecoverable(format!("edit: temp content: {error}"))
                })?;
            temp_file.flush().await.map_err(|error| {
                ToolError::LlmRecoverable(format!("edit: flush temp file: {error}"))
            })?;
            drop(temp_file);

            if actual_path
                .extension()
                .and_then(|extension| extension.to_str())
                == Some("rs")
            {
                self.verify_rust(&temp_path, &actual_path, path).await?;
            }
            fs::rename(&temp_path, &actual_path)
                .await
                .map_err(|error| {
                    ToolError::LlmRecoverable(format!(
                        "edit: write {}: {}",
                        actual_path.display(),
                        error
                    ))
                })?;
            Ok::<(), ToolError>(())
        }
        .await;
        if edit_result.is_err() {
            let _ = fs::remove_file(&temp_path).await;
        }
        edit_result?;

        Ok(format!("File edited: {}", path))
    }
}

pub fn edit_tool(
    working_dir: Option<std::path::PathBuf>,
    runner: Arc<dyn crate::runner::CommandRunner>,
) -> Tool {
    let workspace_root =
        workspace_path::configured_root(working_dir).map_err(|error| error.to_string());
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
        execute: Arc::new(PydanticAdapter::new(EditExecutor {
            workspace_root,
            runner,
        })),
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

        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = PydanticAdapter::new(EditExecutor {
            workspace_root: Ok(dir.path().to_path_buf()),
            runner,
        });

        let args = json!({
            "path": "test.txt",
            "old_str": "old",
            "new_str": "new"
        });

        let result = super::super::ToolExecutor::execute(&executor, args)
            .await
            .unwrap();
        assert_eq!(result, "File edited: test.txt");

        let content = fs::read_to_string(file_path).await.unwrap();
        assert_eq!(content, "hello new world");
    }

    #[tokio::test]
    async fn test_edit_tool_missing_args() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = PydanticAdapter::new(EditExecutor {
            workspace_root: Err("unused".to_string()),
            runner,
        });

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

        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = PydanticAdapter::new(EditExecutor {
            workspace_root: Ok(dir.path().to_path_buf()),
            runner,
        });

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
        fs::write(&file_path, "fn main() { println!(\"old\"); }")
            .await
            .unwrap();

        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = PydanticAdapter::new(EditExecutor {
            workspace_root: Ok(dir.path().to_path_buf()),
            runner,
        });

        let args = json!({
            "path": "test.rs",
            "old_str": "old",
            "new_str": "new"
        });

        // Should succeed and pass verification
        let result = super::super::ToolExecutor::execute(&executor, args)
            .await
            .unwrap();
        assert_eq!(result, "File edited: test.rs");
    }

    #[tokio::test]
    async fn test_edit_tool_rust_verification_failure() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        fs::write(&file_path, "fn main() { println!(\"old\"); }")
            .await
            .unwrap();

        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        // Simulate rustc failure
        runner.push_response(Ok(crate::runner::mock::mock_output(
            1,
            "",
            "error: expected expression, found `;`",
        )));

        let executor = PydanticAdapter::new(EditExecutor {
            workspace_root: Ok(dir.path().to_path_buf()),
            runner,
        });

        let args = json!({
            "path": "test.rs",
            "old_str": "println!(\"old\");",
            "new_str": "let x = ;" // Introduce syntax error
        });

        // Should fail verification due to syntax error
        let result = super::super::ToolExecutor::execute(&executor, args).await;
        assert!(
            result.is_err(),
            "Expected error from mock rustc verification"
        );
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Verification Loop Failed: `rustc` reported syntax errors"));
        } else {
            panic!("Expected LlmRecoverable error");
        }

        // Assert the target file was NOT modified since validation failed
        let content = fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(
            content, "fn main() { println!(\"old\"); }",
            "Target file should not be modified if verification fails"
        );
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_edit_rejects_symlink_to_file_outside_workspace() {
        let dir = tempdir().unwrap();
        let outside = tempdir().unwrap();
        let outside_file = outside.path().join("outside.txt");
        fs::write(&outside_file, "old value").await.unwrap();
        std::os::unix::fs::symlink(&outside_file, dir.path().join("link.txt")).unwrap();
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = PydanticAdapter::new(EditExecutor {
            workspace_root: Ok(dir.path().to_path_buf()),
            runner,
        });

        let result = super::super::ToolExecutor::execute(
            &executor,
            json!({"path": "link.txt", "old_str": "old", "new_str": "new"}),
        )
        .await;

        assert!(result.is_err());
        assert_eq!(fs::read_to_string(outside_file).await.unwrap(), "old value");
    }
}
