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
                        let _ = fs::write(&actual_path, &content).await;

                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(ToolError::LlmRecoverable(format!(
                            "Syntax verification failed after edit. File rolled back to previous state.
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
                            let _ = fs::write(&actual_path, &content).await;
                            return Err(ToolError::LlmRecoverable(format!(
                                "Syntax verification failed after edit. File rolled back to previous state.
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
