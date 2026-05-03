use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::fs;

use super::{Tool, ToolExecutor};

struct ReadExecutor {
    working_dir: Option<std::path::PathBuf>,
}

#[async_trait::async_trait]
impl ToolExecutor for ReadExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let path = args["path"].as_str().ok_or_else(|| ToolError::LlmRecoverable("read: path is required".to_string()))?;
        let safe_path = std::path::Path::new(path).strip_prefix("/").unwrap_or(std::path::Path::new(path));
        let actual_path = if let Some(wd) = &self.working_dir { wd.join(safe_path) } else { std::path::PathBuf::from(path) };
        let content = fs::read_to_string(&actual_path)
            .await
            .map_err(|e| format!("read: {}: {}", path, e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        // Optional line range
        if let (Some(start), Some(end)) = (
            args["start_line"].as_u64(),
            args["end_line"].as_u64(),
        ) {
            let lines: Vec<&str> = content.lines().collect();
            let start = (start as usize).saturating_sub(1);
            let end = (end as usize).min(lines.len());
            if start >= end {
                return Err(ToolError::LlmRecoverable(format!("read: invalid line range {}-{}", start + 1, end)));
            }
            return Ok(lines[start..end].join("\n"));
        }

        Ok(content)
    }
}

pub fn read_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "Read".to_string(),
        description: "Read the contents of a file. Optionally specify start_line and end_line for partial reads.".to_string(),
        is_read_only: true,
        is_subagent: false,
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
        execute: Arc::new(ReadExecutor { working_dir }),
    }
}
