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

        // Computational/Guides Verification Loop
        let ext = actual_path.extension().and_then(|s| s.to_str()).unwrap_or("");

        let mut verification_failed = false;
        let mut error_msg = String::new();

        if ext == "rs" {
            // Find Cargo.toml by searching up
            let mut current_dir = actual_path.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| std::path::PathBuf::from("."));
            let mut cargo_toml_dir = None;
            for _ in 0..5 {
                if current_dir.join("Cargo.toml").exists() {
                    cargo_toml_dir = Some(current_dir.clone());
                    break;
                }
                if let Some(p) = current_dir.parent() {
                    current_dir = p.to_path_buf();
                } else {
                    break;
                }
            }

            if let Some(dir) = cargo_toml_dir {
                let output = tokio::process::Command::new("cargo")
                    .arg("check")
                    .current_dir(&dir)
                    .output()
                    .await;

                if let Ok(out) = output {
                    if !out.status.success() {
                        let stderr = String::from_utf8_lossy(&out.stderr);
                        verification_failed = true;
                        error_msg = format!("File edited, but `cargo check` failed:\n{}", stderr);
                    }
                }
            }
        } else if ext == "py" {
            let output = tokio::process::Command::new("python3")
                .arg("-m")
                .arg("py_compile")
                .arg(&actual_path)
                .output()
                .await;

            if let Ok(out) = output {
                if !out.status.success() {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    verification_failed = true;
                    error_msg = format!("File edited, but python syntax check failed:\n{}", stderr);
                }
            }
        }

        if verification_failed {
            // Revert the file to its previous state
            let _ = fs::write(&actual_path, content).await;
            return Err(ToolError::LlmRecoverable(format!("{} The file has been reverted to its previous state.", error_msg)));
        }

        Ok(format!("File edited: {}", path))
    }
}

pub fn edit_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "Edit".to_string(),
        description: "Replace exactly one occurrence of old_str with new_str in a file. \
            The old_str must appear exactly once in the file. Automatically runs syntax checking (like cargo check or python py_compile) after editing, and reverts changes if syntax check fails."
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
    async fn test_edit_tool_cargo_check_fail_reverts() {
        let dir = tempdir().unwrap();
        let wd = dir.path().to_path_buf();

        fs::write(wd.join("Cargo.toml"), "[package]\nname = \"test\"\nversion = \"0.1.0\"\n[dependencies]\n").await.unwrap();
        fs::create_dir_all(wd.join("src")).await.unwrap();
        let original_content = "fn main() { println!(\"Hello, world!\"); }";
        fs::write(wd.join("src").join("main.rs"), original_content).await.unwrap();

        let tool = edit_tool(Some(wd.clone()));

        let args = json!({
            "path": "src/main.rs",
            "old_str": "println!(\"Hello, world!\");",
            "new_str": "let x = 1; let y = x + \"string\";"
        });

        let result = tool.execute.execute(args).await;

        assert!(result.is_err());
        match result {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert!(msg.contains("cargo check` failed"));
                assert!(msg.contains("reverted"));
            }
            _ => panic!("Expected LlmRecoverable error"),
        }

        // Verify it was reverted
        let current_content = fs::read_to_string(wd.join("src").join("main.rs")).await.unwrap();
        assert_eq!(current_content, original_content);
    }
}
