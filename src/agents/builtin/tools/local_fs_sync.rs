use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::fs;
use std::path::Path;
use super::{Tool, ToolExecutor};

struct LocalFSSyncExecutor {
    working_dir: Option<std::path::PathBuf>,
}

#[async_trait::async_trait]
impl ToolExecutor for LocalFSSyncExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let action = args["Action"].as_str().ok_or_else(|| ToolError::LlmRecoverable("local_fs_sync: Action is required".to_string()))?;
        let path = args["Path"].as_str().ok_or_else(|| ToolError::LlmRecoverable("local_fs_sync: Path is required".to_string()))?;

        let mut clean_path = Path::new(path).to_path_buf();
        if !clean_path.starts_with(".agent-task/") || path.contains("..") {
            return Err(ToolError::LlmRecoverable("sandbox violation: path must start with .agent-task/".to_string()));
        }
        if let Some(wd) = &self.working_dir {
            clean_path = wd.join(clean_path);
        }

        match action {
            "read" => {
                let content = fs::read_to_string(&clean_path).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
                Ok(content)
            }
            "write" => {
                let content = args["Content"].as_str().ok_or_else(|| ToolError::LlmRecoverable("local_fs_sync: Content is required for write".to_string()))?;
                if let Some(parent) = clean_path.parent() {
                    fs::create_dir_all(parent).await.ok();
                }
                fs::write(&clean_path, content).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
                Ok(json!({"status":"written"}).to_string())
            }
            "sync" => {
                let exists = clean_path.exists();
                if !exists {
                    return Err(ToolError::LlmRecoverable("file not found".to_string()));
                }
                Ok(json!({"status":"synced"}).to_string())
            }
            _ => Err(ToolError::LlmRecoverable("invalid action".to_string())),
        }
    }
}

pub fn local_fs_sync_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "local_fs_sync".to_string(),
        description: "Performs local file system operations restricted to .agent-task/ directory.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "Action": {
                    "type": "string",
                    "description": "Action to perform: read, write, sync"
                },
                "Path": {
                    "type": "string",
                    "description": "Path to the file, must start with .agent-task/"
                },
                "Content": {
                    "type": "string",
                    "description": "Content to write (required for write action)"
                }
            },
            "required": ["Action", "Path"]
        }),
        execute: Arc::new(LocalFSSyncExecutor { working_dir }),
    }
}
