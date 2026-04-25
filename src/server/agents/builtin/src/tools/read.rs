use serde_json::{json, Value};
use std::sync::Arc;
use tokio::fs;

use super::{Tool, ToolExecutor};

struct ReadExecutor;

#[async_trait::async_trait]
impl ToolExecutor for ReadExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let path = args["path"].as_str().ok_or("read: path is required")?;
        let content = fs::read_to_string(path)
            .await
            .map_err(|e| format!("read: {}: {}", path, e))?;

        // Optional line range
        if let (Some(start), Some(end)) = (
            args["start_line"].as_u64(),
            args["end_line"].as_u64(),
        ) {
            let lines: Vec<&str> = content.lines().collect();
            let start = (start as usize).saturating_sub(1);
            let end = (end as usize).min(lines.len());
            if start >= end {
                return Err(format!("read: invalid line range {}-{}", start + 1, end).into());
            }
            return Ok(lines[start..end].join("\n"));
        }

        Ok(content)
    }
}

pub fn read_tool() -> Tool {
    Tool {
        name: "Read".to_string(),
        is_mutating: false,
        description: "Read the contents of a file. Optionally specify start_line and end_line for partial reads.".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to read."
                },
                "start_line": {
                    "type": "integer",
                    "description": "1-indexed starting line (inclusive)."
                },
                "end_line": {
                    "type": "integer",
                    "description": "1-indexed ending line (inclusive)."
                }
            },
            "required": ["path"]
        }),
        execute: Arc::new(ReadExecutor),
    }
}
