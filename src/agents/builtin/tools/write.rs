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

        // Computational/Guides Verification Loop
        let ext = actual_path.extension().and_then(|s| s.to_str()).unwrap_or("");

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
                        // Clean up the file to not leave it broken? The prompt says: "if it fails, returning the raw error as an LlmRecoverable error directly back to the model so it can self-correct before the action is considered complete"
                        // Usually we just return the error, leaving the file as is so the agent can fix it.
                        return Err(ToolError::LlmRecoverable(format!(
                            "File written, but `cargo check` failed:\n{}", stderr
                        )));
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
                    return Err(ToolError::LlmRecoverable(format!(
                        "File written, but python syntax check failed:\n{}", stderr
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
        description: "Write content to a file. Creates parent directories as needed. Overwrites any existing content. Automatically runs syntax checking (like cargo check or python py_compile) after writing.".to_string(),
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
    async fn test_write_tool_cargo_check_fail() {
        let dir = tempdir().unwrap();
        let wd = dir.path().to_path_buf();

        // Setup a fake Cargo project
        fs::write(wd.join("Cargo.toml"), "[package]\nname = \"test\"\nversion = \"0.1.0\"\n[dependencies]\n").await.unwrap();
        fs::create_dir_all(wd.join("src")).await.unwrap();
        fs::write(wd.join("src").join("main.rs"), "fn main() { println!(\"Hello, world!\"); }").await.unwrap();

        let tool = write_tool(Some(wd.clone()));

        // Write a bad rust file
        let args = json!({
            "path": "src/main.rs",
            "content": "fn main() { let x = 1; let y = x + \"string\"; }"
        });

        let result = tool.execute.execute(args).await;

        assert!(result.is_err());
        match result {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert!(msg.contains("cargo check` failed"));
            }
            _ => panic!("Expected LlmRecoverable error"),
        }
    }
}
