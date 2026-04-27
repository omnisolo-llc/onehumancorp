use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

struct GlobExecutor;

#[async_trait::async_trait]
impl ToolExecutor for GlobExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, crate::ToolError> {
        let pattern = args["pattern"]
            .as_str()
            .ok_or("glob: pattern is required")?;
        let base_dir = args["path"].as_str().unwrap_or(".");

        let full_pattern = if base_dir == "." {
            pattern.to_string()
        } else {
            format!("{}/{}", base_dir.trim_end_matches('/'), pattern)
        };

        let matches: Vec<String> = glob::glob(&full_pattern)
            .map_err(|e| format!("glob: invalid pattern: {}", e))?
            .filter_map(|r| r.ok())
            .map(|p| p.display().to_string())
            .collect();

        if matches.is_empty() {
            return Ok("No files found.".to_string());
        }

        Ok(matches.join("\n"))
    }
}

pub fn glob_tool() -> Tool {
    Tool {
        name: "Glob".to_string(),
        description: "Find files matching a glob pattern. Returns newline-separated paths.".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "Glob pattern to match (e.g. '**/*.go', 'src/*.rs')."
                },
                "path": {
                    "type": "string",
                    "description": "Base directory to search from (default '.')."
                }
            },
            "required": ["pattern"]
        }),
        execute: Arc::new(GlobExecutor),
    }
}
