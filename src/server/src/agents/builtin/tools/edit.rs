use serde_json::{json, Value};
use std::sync::Arc;
use tokio::fs;

use super::{Tool, ToolExecutor};

struct EditExecutor;

#[async_trait::async_trait]
impl ToolExecutor for EditExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, super::ToolError> {
        let path = args["path"].as_str().ok_or_else(|| super::ToolError::LlmRecoverable("edit: path is required".to_string()))?;
        let old_str = args["old_str"]
            .as_str()
            .ok_or_else(|| super::ToolError::LlmRecoverable("edit: old_str is required".to_string()))?;
        let new_str = args["new_str"]
            .as_str()
            .ok_or_else(|| super::ToolError::LlmRecoverable("edit: new_str is required".to_string()))?;

        let content = fs::read_to_string(path)
            .await
            .map_err(|e| super::ToolError::LlmRecoverable(format!("edit: read {}: {}", path, e)))?;

        // Ensure exactly one occurrence.
        let count = content.matches(old_str).count();
        if count == 0 {
            return Err(super::ToolError::LlmRecoverable(format!("edit: old_str not found in {} (must match exactly once)", path)));
        }
        if count > 1 {
            return Err(super::ToolError::LlmRecoverable(format!("edit: old_str found {} times in {} (must match exactly once)", count, path)));
        }

        let new_content = content.replacen(old_str, new_str, 1);
        fs::write(path, &new_content)
            .await
            .map_err(|e| super::ToolError::LlmRecoverable(format!("edit: write {}: {}", path, e)))?;

        Ok(format!("File edited: {}", path))
    }
}

pub fn edit_tool() -> Tool {
    Tool {
        name: "Edit".to_string(),
        description: "Replace exactly one occurrence of old_str with new_str in a file. \
            The old_str must appear exactly once in the file."
            .to_string(),
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
        execute: Arc::new(EditExecutor),
    }
}
