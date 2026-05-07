use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

struct GlobExecutor {
    working_dir: Option<std::path::PathBuf>,
}

#[async_trait::async_trait]
impl ToolExecutor for GlobExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let pattern = args["pattern"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("glob: pattern is required".to_string()))?;
        let base_dir = args["path"].as_str().unwrap_or(".");

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

        let mut matches: Vec<String> = glob::glob(&full_pattern)
            .map_err(|e| ToolError::LlmRecoverable(format!("glob: invalid pattern: {}", e)))?
            .filter_map(|r| r.ok())
            .map(|p| {
                let mut p_str = p.display().to_string();
                if let Some(wd) = &self.working_dir {
                    if let Ok(rel) = p.strip_prefix(wd) {
                        p_str = rel.display().to_string();
                    }
                }
                p_str
            })
            .collect();
        if matches.is_empty() {
            return Ok("No files found.".to_string());
        }

        if matches.len() > 500 {
            matches.truncate(500);
            matches.push("... (truncated)".to_string());
        }

        Ok(matches.join("\n"))

    }
}

pub fn glob_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "Glob".to_string(),
        description: "Find files matching a glob pattern. Returns newline-separated paths.".to_string(),
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
        execute: Arc::new(GlobExecutor { working_dir }),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs;

    #[tokio::test]
    async fn test_glob_truncation() {
        let dir = tempdir().unwrap();
        let executor = GlobExecutor { working_dir: Some(dir.path().to_path_buf()) };

        for i in 1..=600 {
            let file_path = dir.path().join(format!("test_file_{}.txt", i));
            fs::write(&file_path, "test").await.unwrap();
        }

        let args = json!({ "pattern": "*.txt" });
        let result = executor.execute(args).await.unwrap();

        let lines: Vec<&str> = result.split('\n').collect();
        assert_eq!(lines.len(), 501);
        assert_eq!(lines.last().unwrap(), &"... (truncated)");
    }
}
