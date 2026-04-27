use serde_json::{json, Value};
use std::sync::Arc;
use tokio::fs;

use super::{Tool, ToolExecutor};

struct WriteExecutor;

#[async_trait::async_trait]
impl ToolExecutor for WriteExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, crate::types::ToolError> {
        let path = args["path"].as_str().ok_or("write: path is required")?;
        let content = args["content"]
            .as_str()
            .ok_or("write: content is required")?;

        // Create parent directories if needed.
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("write: create dir {}: {}", parent.display(), e))?;
        }

        fs::write(path, content)
            .await
            .map_err(|e| format!("write: {}: {}", path, e))?;

        Ok(format!("File written: {}", path))
    }
}

pub fn write_tool() -> Tool {
    Tool {
        name: "Write".to_string(),
        description: "Write content to a file. Creates parent directories as needed. Overwrites any existing content.".to_string(),
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
        execute: Arc::new(WriteExecutor),
    }
}
