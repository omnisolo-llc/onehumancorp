use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use serde::Deserialize;
use std::sync::Arc;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

#[derive(Deserialize)]
struct GlobArgs {
    pattern: String,
    #[serde(default = "default_path")]
    path: String,
}

fn default_path() -> String { ".".to_string() }

struct GlobExecutor {
    working_dir: Option<std::path::PathBuf>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<GlobArgs> for GlobExecutor {
    async fn execute_typed(
        &self,
        args: GlobArgs,
    ) -> Result<String, ToolError> {
        let pattern = &args.pattern;
        let base_dir = &args.path;

        let safe_base = base_dir.strip_prefix("/").unwrap_or(base_dir);
        let safe_pattern = pattern.strip_prefix("/").unwrap_or(pattern);

        let mut full_pattern = if safe_base == "." || safe_base == "" {
            safe_pattern.to_string()
        } else {
            format!("{}/{}", safe_base.trim_end_matches('/'), safe_pattern)
        };

        if let Some(wd) = &self.working_dir {
            full_pattern = format!("{}/{}", wd.display(), full_pattern);
        }

        let matches: Vec<String> = glob::glob(&full_pattern)
            .map_err(|e| ToolError::LlmRecoverable(format!("glob: invalid pattern: {}", e)))?
            .filter_map(|r| r.ok())
            .map(|p| {
                let mut p_str = p.display().to_string();
                if let Some(wd) = &self.working_dir && let Ok(rel) = p.strip_prefix(wd) {
                    p_str = rel.display().to_string();
                }
                p_str
            })
            .collect();

        // Just-in-Time (JIT) Retrieval Mechanic:
        if matches.is_empty() {
            return Ok("No files found.".to_string());
        }

        let mut output_matches = matches;
        if output_matches.len() > 50 {
            output_matches.truncate(50);
            output_matches.push("... (truncated to 50 results. Please use a more specific glob pattern or use grep/find.)".to_string());
        }

        Ok(output_matches.join("\n"))
    }
}

pub fn glob_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "Glob".to_string(),
        description: "Find files matching a glob pattern. Returns newline-separated paths. Used for Context Management (Preventing Context Rot): Just-in-Time (JIT) Context Retrieval.".to_string(),
        is_read_only: true,
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
        execute: Arc::new(PydanticAdapter::new(GlobExecutor { working_dir })),
    }
}
