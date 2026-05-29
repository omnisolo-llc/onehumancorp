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


        // Just-in-Time (JIT) Retrieval Mechanic:
        let max_bytes = 16 * 1024; // 16KB JIT retrieval limit
        let mut final_result = String::new();
        let matches: Vec<String> = glob::glob(&full_pattern)
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

        // Just-in-Time (JIT) Retrieval Mechanic:
        if matches.is_empty() {
            return Ok("No files found.".to_string());
        }
        for res in matches {
            if final_result.len() + res.len() > max_bytes {
                final_result.push_str("\n... (truncated due to JIT byte limit)");
                break;
            }
            if !final_result.is_empty() {
                final_result.push('\n');
            }
            final_result.push_str(&res);
        }
        Ok(final_result)
    }
}

pub fn glob_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "Glob".to_string(),
        description: "Find files matching a glob pattern. Returns newline-separated paths. Used for Just-in-Time (JIT) Context Retrieval.".to_string(),
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

    #[tokio::test]
    async fn test_glob_jit_retrieval_limit() {
        let temp_dir = std::env::temp_dir();
        let test_dir = temp_dir.join(format!("glob_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&test_dir).unwrap();

        for i in 1..=500 {
            let file_name = format!("file_{}_{}.txt", i, "a".repeat(100));
            std::fs::File::create(test_dir.join(file_name)).unwrap();
        }

        let executor = GlobExecutor { working_dir: Some(test_dir.clone()) };

        let args = serde_json::json!({
            "pattern": "*.txt",
            "path": test_dir.to_string_lossy().to_string(),
        });

        let result = executor.execute(args).await.unwrap();
        assert!(result.contains("... (truncated due to JIT byte limit)"));
        assert!(result.len() < 20000);

        let _ = std::fs::remove_dir_all(&test_dir);
    }
}
