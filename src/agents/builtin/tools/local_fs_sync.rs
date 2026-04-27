use serde_json::{json, Value};
use std::sync::Arc;
use tokio::fs;
use std::path::Path;
use super::{Tool, ToolExecutor};

struct LocalFSSyncExecutor;

#[async_trait::async_trait]
impl ToolExecutor for LocalFSSyncExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, crate::types::ToolError> {
        let action = args["Action"].as_str().ok_or_else(|| crate::types::ToolError::LlmRecoverable("local_fs_sync: Action is required".to_string()))?;
        let path = args["Path"].as_str().ok_or_else(|| crate::types::ToolError::LlmRecoverable("local_fs_sync: Path is required".to_string()))?;

        let clean_path = Path::new(path);
        if !clean_path.starts_with(".agent-task/") || path.contains("..") {
            return Err("sandbox violation: path must start with .agent-task/".into());
        }

        match action {
            "read" => {
                let content = fs::read_to_string(clean_path).await?;
                Ok(content)
            }
            "write" => {
                let content = args["Content"].as_str().ok_or_else(|| crate::types::ToolError::LlmRecoverable("local_fs_sync: Content is required for write".to_string()))?;
                fs::write(clean_path, content).await?;
                Ok(json!({"status":"written"}).to_string())
            }
            "sync" => {
                let exists = clean_path.exists();
                if !exists {
                    return Err("file not found".into());
                }
                Ok(json!({"status":"synced"}).to_string())
            }
            _ => Err("invalid action".into()),
        }
    }
}

pub fn local_fs_sync_tool() -> Tool {
    Tool {
        name: "local_fs_sync".to_string(),
        description: "Performs local file system operations restricted to .agent-task/ directory.".to_string(),
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
        execute: Arc::new(LocalFSSyncExecutor),
    }
}
