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

        // Store original content if it exists
        let original_content = fs::read_to_string(&actual_path).await.ok();

        // Create parent directories if needed.
        if let Some(parent) = actual_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("write: create dir {}: {}", parent.display(), e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
        }

        fs::write(&actual_path, content)
            .await
            .map_err(|e| format!("write: {}: {}", actual_path.display(), e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        // Verification Loop: Computational/Guides
        if let Some(ext) = actual_path.extension().and_then(|e| e.to_str()) {
            if ext == "py" {
                let cmd = "python3";
                let args = vec!["-m", "py_compile", actual_path.to_str().unwrap()];

                let mut child = tokio::process::Command::new(cmd);
                child.args(&args);
                if let Some(wd) = &self.working_dir {
                    child.current_dir(wd);
                }

                if let Ok(output) = child.output().await {
                    if !output.status.success() {
                        // Rollback to previous content
                        if let Some(old) = original_content {
                            let _ = fs::write(&actual_path, &old).await;
                        } else {
                            let _ = fs::remove_file(&actual_path).await;
                        }

                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(ToolError::LlmRecoverable(format!(
                            "Syntax verification failed after write. File rolled back to previous state.
Command: {} {}
Error:
{}",
                            cmd, args.join(" "), stderr
                        )));
                    }
                }
            } else if ext == "rs" {
                // For Rust, use standard formatting check or basic parsing check instead of compilation,
                // because rustc --emit=metadata fails on unresolved module dependencies when ran on individual files in a bazel workspace
                let cmd = "rustfmt";
                let args = vec!["--edition", "2021", "--check", actual_path.to_str().unwrap()];

                let mut child = tokio::process::Command::new(cmd);
                child.args(&args);
                if let Some(wd) = &self.working_dir {
                    child.current_dir(wd);
                }

                // We're just running formatting as a syntax parser trick,
                // wait to see if it reports parsing errors.
                if let Ok(output) = child.output().await {
                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        if stderr.contains("error:") {
                            // Only rollback on actual syntax errors, not just formatting diffs
                            if let Some(old) = original_content {
                                let _ = fs::write(&actual_path, &old).await;
                            } else {
                                let _ = fs::remove_file(&actual_path).await;
                            }
                            return Err(ToolError::LlmRecoverable(format!(
                                "Syntax verification failed after write. File rolled back to previous state.
Command: {} {}
Error:
{}",
                                cmd, args.join(" "), stderr
                            )));
                        }
                    }
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
