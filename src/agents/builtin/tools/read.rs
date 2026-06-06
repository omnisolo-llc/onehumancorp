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
        // Just-in-Time (JIT) Retrieval Mechanic with proper token accounting:
        // "Never load full files." We enforce a strict token limit and stream the file to prevent loading it entirely into memory.
        let file = fs::File::open(&actual_path)
            .await
            .map_err(|e| format!("read: {}: {}", path, e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        use tokio::io::{AsyncBufReadExt, BufReader};
        let mut reader = BufReader::new(file);
        let mut line_buffer = String::new();
        let mut result_lines = Vec::new();
        let mut line_count = 0;

        let req_start = args["start_line"].as_u64().map(|n| n.saturating_sub(1) as usize);
        let req_end = args["end_line"].as_u64().map(|n| n as usize);

        if let (Some(s), Some(e)) = (req_start, req_end) {
            if s >= e {
                return Err(ToolError::LlmRecoverable(format!("read: invalid line range {}-{}", s + 1, e)));
            }
        }

        let start = req_start.unwrap_or(0);
        let end = req_end.unwrap_or(usize::MAX); // We rely on token limits now instead of arbitrary line limits

        let max_tokens = 4000;
        let mut current_tokens = 0;
        let mut truncated = false;

        while let Ok(bytes) = reader.read_line(&mut line_buffer).await {
            if bytes == 0 {
                break;
            }
            if line_count >= start && line_count < end {
                let clean_line = line_buffer.trim_end_matches(&['\r', '\n'][..]).to_string();
                let tokens = super::token_estimator::estimate_tokens(&clean_line) + 1; // +1 for newline
                if current_tokens + tokens > max_tokens {
                    truncated = true;
                    break;
                }
                current_tokens += tokens;
                result_lines.push(clean_line);
            }
            line_count += 1;
            line_buffer.clear();

            if line_count >= end {
                break; // Stop reading early if we reached the end of the requested range
            }
        }

        if truncated {
            result_lines.push(format!("... (truncated to {} tokens. Please use start_line and end_line to paginate.)", current_tokens));
        }

        Ok(result_lines.join("
"))
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
    async fn test_read_large_file_streaming() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("jit_streaming_test_large.txt");

        let mut file = std::fs::File::create(&test_file).unwrap();
        // Generate a large file to test memory constraint
        for i in 1..=5000 {
            writeln!(file, "Line {}", i).unwrap();
        }

        let executor = ReadExecutor { working_dir: None };

        let args = json!({
            "path": test_file.to_string_lossy().to_string(),
            "start_line": 4900,
            "end_line": 4903
        });

        let result = executor.execute(args).await.unwrap();
        let expected = "Line 4900\nLine 4901\nLine 4902\nLine 4903";
        assert_eq!(result, expected);

        let _ = std::fs::remove_file(&test_file);
    }

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

        // 1. Try reading the whole file - should be truncated
        let args = json!({ "path": test_file.to_string_lossy().to_string() });
        let result = executor.execute(args).await.unwrap();
        assert!(result.contains("... (truncated to"));

        // 2. Try reading a slice that is too large in tokens - should be truncated
        let args2 = json!({
            "path": test_file.to_string_lossy().to_string(),
            "start_line": 1,
            "end_line": 1500
        });
        let result2 = executor.execute(args2).await.unwrap();
        assert!(result2.contains("... (truncated to"));

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
