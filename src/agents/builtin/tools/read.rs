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

        // Just-in-Time (JIT) Retrieval Mechanic:
        // "Never load full files." We enforce a strict token/line limit.
        // If the user requests the whole file and it's large, we force them to paginate using start_line/end_line.
        let lines: Vec<&str> = content.lines().collect();

        // Optional line range
        if let (Some(start), Some(end)) = (
            args["start_line"].as_u64(),
            args["end_line"].as_u64(),
        ) {
            let start = (start as usize).saturating_sub(1);
            let end = (end as usize).min(lines.len());
            if start >= end {
                return Err(ToolError::LlmRecoverable(format!("read: invalid line range {}-{}", start + 1, end)));
            }

            // Enforce maximum window size
            if end - start > 1000 {
                 return Err(ToolError::LlmRecoverable("JIT Retrieval Error: Cannot read more than 1000 lines at once. Please use start_line and end_line to paginate.".to_string()));
            }

            return Ok(lines[start..end].join("\n"));
        }

        // If no range specified and file is large, reject it.
        if lines.len() > 1000 {
             return Err(ToolError::LlmRecoverable(format!(
                 "JIT Retrieval Error: File is too large ({} lines). Never load full files. Please use start_line and end_line to paginate (max 1000 lines per request).",
                 lines.len()
             )));
        }

        Ok(lines.join("\n"))
    }
}

pub fn read_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "Read".to_string(),
        description: "Read the contents of a file. Optionally specify start_line and end_line for partial reads.".to_string(),
        is_read_only: true,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[tokio::test]
    async fn test_read_jit_retrieval_limit() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("jit_test_large.txt");

        // Create a file with 1500 lines
        let mut file = std::fs::File::create(&test_file).unwrap();
        for i in 1..=1500 {
            writeln!(file, "Line {}", i).unwrap();
        }

        let executor = ReadExecutor { working_dir: None };

        // 1. Try reading the whole file - should fail
        let args = json!({ "path": test_file.to_string_lossy().to_string() });
        let result = executor.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("JIT Retrieval Error: File is too large"));
        } else {
            panic!("Expected JIT Retrieval Error");
        }

        // 2. Try reading a slice larger than 1000 lines - should fail
        let args2 = json!({
            "path": test_file.to_string_lossy().to_string(),
            "start_line": 1,
            "end_line": 1200
        });
        let result2 = executor.execute(args2).await;
        assert!(result2.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result2 {
            assert!(msg.contains("Cannot read more than 1000 lines at once"));
        } else {
            panic!("Expected JIT Retrieval Error");
        }

        // 3. Try reading a valid slice - should succeed
        let args3 = json!({
            "path": test_file.to_string_lossy().to_string(),
            "start_line": 500,
            "end_line": 600
        });
        let result3 = executor.execute(args3).await;
        assert!(result3.is_ok());

        let _ = std::fs::remove_file(&test_file);
    }
}
