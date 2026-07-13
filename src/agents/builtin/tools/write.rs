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

const MAX_CONTENT_BYTES: usize = 4 * 1024 * 1024;

#[derive(Deserialize)]
struct WriteArgs {
    path: String,
    content: String,
}

struct WriteExecutor {
    workspace_root: Result<std::path::PathBuf, String>,
    runner: Arc<dyn crate::runner::CommandRunner>,
}

impl WriteExecutor {
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
                    "ohc_write_check",
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
                        "Verification Loop Failed: `rustc` reported syntax errors before writing to {}.\n\nCompiler Output:\n{}\n\nPlease fix the errors and try again.",
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
impl PydanticToolExecutor<WriteArgs> for WriteExecutor {
    async fn execute_typed(&self, args: WriteArgs) -> Result<String, ToolError> {
        let path = &args.path;
        let content = &args.content;
        if content.len() > MAX_CONTENT_BYTES {
            return Err(ToolError::LlmRecoverable(
                "write: content exceeds 4 MiB".to_string(),
            ));
        }
        let root = self.workspace_root.as_ref().map_err(|error| {
            ToolError::LlmRecoverable(format!("write: workspace root is unavailable: {error}"))
        })?;
        let initial_path = workspace_path::for_write(root, path).await?;
        let initial_parent = initial_path.parent().ok_or_else(|| {
            ToolError::LlmRecoverable(format!("write: {} has no parent directory", path))
        })?;
        fs::create_dir_all(initial_parent).await.map_err(|error| {
            ToolError::LlmRecoverable(format!(
                "write: create dir {}: {}",
                initial_parent.display(),
                error
            ))
        })?;

        let actual_path = workspace_path::for_write(root, path).await?;
        let existing_permissions = match fs::symlink_metadata(&actual_path).await {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(ToolError::LlmRecoverable(format!(
                    "write: refusing to replace symlink {}",
                    path
                )));
            }
            Ok(metadata) => Some(metadata.permissions()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => {
                return Err(ToolError::LlmRecoverable(format!(
                    "write: inspect {}: {}",
                    path, error
                )));
            }
        };
        let parent = actual_path.parent().ok_or_else(|| {
            ToolError::LlmRecoverable(format!("write: {} has no parent directory", path))
        })?;
        let file_name = actual_path.file_name().ok_or_else(|| {
            ToolError::LlmRecoverable(format!("write: {} has no file name", path))
        })?;
        let temp_path = parent.join(format!(
            ".{}.{}.tmp",
            file_name.to_string_lossy(),
            uuid::Uuid::new_v4()
        ));

        let write_result = async {
            let mut temp_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temp_path)
                .await
                .map_err(|error| {
                    ToolError::LlmRecoverable(format!(
                        "write: temp file {}: {}",
                        temp_path.display(),
                        error
                    ))
                })?;
            if let Some(permissions) = existing_permissions.clone() {
                fs::set_permissions(&temp_path, permissions)
                    .await
                    .map_err(|error| {
                        ToolError::LlmRecoverable(format!("write: set temp permissions: {error}"))
                    })?;
            }
            temp_file
                .write_all(content.as_bytes())
                .await
                .map_err(|error| {
                    ToolError::LlmRecoverable(format!("write: temp content: {error}"))
                })?;
            temp_file.flush().await.map_err(|error| {
                ToolError::LlmRecoverable(format!("write: flush temp file: {error}"))
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
                        "write: {}: {}",
                        actual_path.display(),
                        error
                    ))
                })?;
            Ok::<(), ToolError>(())
        }
        .await;
        if write_result.is_err() {
            let _ = fs::remove_file(&temp_path).await;
        }
        write_result?;

        Ok(format!("File written: {}", path))
    }
}

pub fn write_tool(
    working_dir: Option<std::path::PathBuf>,
    runner: Arc<dyn crate::runner::CommandRunner>,
) -> Tool {
    let workspace_root =
        workspace_path::configured_root(working_dir).map_err(|error| error.to_string());
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
        execute: Arc::new(PydanticAdapter::new(WriteExecutor {
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
    async fn test_write_tool_basic() {
        let dir = tempdir().unwrap();
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = PydanticAdapter::new(WriteExecutor {
            workspace_root: Ok(dir.path().to_path_buf()),
            runner,
        });

        let args = json!({
            "path": "test.txt",
            "content": "hello world"
        });

        let result = crate::ToolExecutor::execute(&executor, args).await.unwrap();
        assert_eq!(result, "File written: test.txt");

        let content = fs::read_to_string(dir.path().join("test.txt"))
            .await
            .unwrap();
        assert_eq!(content, "hello world");
    }

    #[tokio::test]
    async fn test_write_tool_missing_args() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = PydanticAdapter::new(WriteExecutor {
            workspace_root: Err("unused".to_string()),
            runner,
        });

        let args = json!({ "path": "test.txt" });
        let result = crate::ToolExecutor::execute(&executor, args).await;
        assert!(result.is_err());

        let args2 = json!({ "content": "test" });
        let result2 = crate::ToolExecutor::execute(&executor, args2).await;
        assert!(result2.is_err());
    }

    #[tokio::test]
    async fn test_write_tool_rust_verification_success() {
        let dir = tempdir().unwrap();
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = PydanticAdapter::new(WriteExecutor {
            workspace_root: Ok(dir.path().to_path_buf()),
            runner,
        });

        let args = json!({
            "path": "test.rs",
            "content": "fn main() { println!(\"Hello\"); }"
        });

        // Should succeed and pass verification
        let result = crate::ToolExecutor::execute(&executor, args).await.unwrap();
        assert_eq!(result, "File written: test.rs");
    }

    #[tokio::test]
    async fn test_write_tool_rust_verification_failure() {
        let dir = tempdir().unwrap();
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        // Simulate rustc failure
        runner.push_response(Ok(crate::runner::mock::mock_output(
            1,
            "",
            "error: expected expression, found `;`",
        )));

        let executor = PydanticAdapter::new(WriteExecutor {
            workspace_root: Ok(dir.path().to_path_buf()),
            runner,
        });

        let args = json!({
            "path": "test.rs",
            "content": "fn main() { let x = ; }"
        });

        // Should fail verification due to syntax error
        let result = crate::ToolExecutor::execute(&executor, args).await;
        assert!(
            result.is_err(),
            "Expected error from mock rustc verification"
        );
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Verification Loop Failed: `rustc` reported syntax errors"));
        } else {
            panic!("Expected LlmRecoverable error");
        }

        // Assert the target file was NOT created/written since validation failed
        assert!(
            !dir.path().join("test.rs").exists(),
            "Target file should not be created if verification fails"
        );
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
    }

    #[tokio::test]
    async fn test_write_rejects_content_over_four_mib() {
        let dir = tempdir().unwrap();
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = PydanticAdapter::new(WriteExecutor {
            workspace_root: Ok(dir.path().to_path_buf()),
            runner,
        });

        let result = crate::ToolExecutor::execute(
            &executor,
            json!({"path": "large.txt", "content": "x".repeat(4 * 1024 * 1024 + 1)}),
        )
        .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("4 MiB"));
        assert!(!dir.path().join("large.txt").exists());
    }

    #[tokio::test]
    async fn test_write_rejects_parent_traversal() {
        let parent = tempdir().unwrap();
        let root = parent.path().join("root");
        fs::create_dir(&root).await.unwrap();
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = PydanticAdapter::new(WriteExecutor {
            workspace_root: Ok(root),
            runner,
        });

        let result = crate::ToolExecutor::execute(
            &executor,
            json!({"path": "../outside.txt", "content": "escaped"}),
        )
        .await;

        assert!(result.is_err());
        assert!(!parent.path().join("outside.txt").exists());
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_write_rejects_existing_destination_symlink() {
        let dir = tempdir().unwrap();
        let outside = tempdir().unwrap();
        let outside_file = outside.path().join("outside.txt");
        fs::write(&outside_file, "unchanged").await.unwrap();
        std::os::unix::fs::symlink(&outside_file, dir.path().join("link.txt")).unwrap();
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = PydanticAdapter::new(WriteExecutor {
            workspace_root: Ok(dir.path().to_path_buf()),
            runner,
        });

        let result = crate::ToolExecutor::execute(
            &executor,
            json!({"path": "link.txt", "content": "changed"}),
        )
        .await;

        assert!(result.is_err());
        assert_eq!(fs::read_to_string(outside_file).await.unwrap(), "unchanged");
    }
}
