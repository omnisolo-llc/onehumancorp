use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use serde::Deserialize;
use std::sync::Arc;
use tokio::fs;

use super::{Tool, pydantic::PydanticToolExecutor};

#[derive(Deserialize, Debug)]
pub struct ReadArgs {
    path: String,
    start_line: Option<u64>,
    end_line: Option<u64>,
}

pub fn read_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    let working_dir_clone = working_dir.clone();

    let executor_fn = move |args: ReadArgs| {
        let wd = working_dir_clone.clone();

        async move {
            let path = &args.path;
            let safe_path = std::path::Path::new(path).strip_prefix("/").unwrap_or(std::path::Path::new(path));
            let actual_path = if let Some(w) = &wd { w.join(safe_path) } else { std::path::PathBuf::from(path) };

            // Just-in-Time (JIT) Retrieval Mechanic:
            // "Never load full files." We enforce a strict token/line limit and stream the file to prevent loading it entirely into memory.
            let file = fs::File::open(&actual_path)
                .await
                .map_err(|e| format!("read: {}: {}", path, e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

            use tokio::io::{AsyncBufReadExt, BufReader};
            let mut reader = BufReader::new(file);
            let mut line_buffer = String::new();
            let mut result_lines = Vec::new();
            let mut line_count = 0;

            let req_start = args.start_line.map(|n| n.saturating_sub(1) as usize);
            let req_end = args.end_line.map(|n| n as usize);

            if let (Some(s), Some(e)) = (req_start, req_end) {
                if s >= e {
                    return Err(ToolError::LlmRecoverable(format!("read: invalid line range {}-{}", s + 1, e)));
                }
                if e - s > 1000 {
                    return Err(ToolError::LlmRecoverable("JIT Retrieval Error: Cannot read more than 1000 lines at once. Please use start_line and end_line to paginate.".to_string()));
                }
            } else {
                // No range specified, check if file is small enough by just reading it line by line
                // If it exceeds 1000 lines, reject it early without loading it entirely.
                let mut test_reader = BufReader::new(fs::File::open(&actual_path).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?);
                let mut test_buffer = String::new();
                let mut total_lines = 0;
                while let Ok(bytes) = test_reader.read_line(&mut test_buffer).await {
                    if bytes == 0 { break; }
                    total_lines += 1;
                    test_buffer.clear();
                    if total_lines > 1000 {
                        return Err(ToolError::LlmRecoverable(
                            "JIT Retrieval Error: File is too large (> 1000 lines). Never load full files. Please use start_line and end_line to paginate (max 1000 lines per request).".to_string()
                        ));
                    }
                }
            }

            let start = req_start.unwrap_or(0);
            let end = req_end.unwrap_or(1000); // capped at 1000 if not specified (already validated above)

            while let Ok(bytes) = reader.read_line(&mut line_buffer).await {
                if bytes == 0 {
                    break;
                }
                if line_count >= start && line_count < end {
                    result_lines.push(line_buffer.trim_end_matches('\n').trim_end_matches('\r').to_string());
                }
                line_count += 1;
                line_buffer.clear();

                if line_count >= end {
                    break; // Stop reading early if we reached the end of the requested range
                }
            }

            Ok(result_lines.join("\n"))
        }
    };

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
        execute: Arc::new(PydanticToolExecutor::new(executor_fn)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use super::super::ToolExecutor;

    #[tokio::test]
    async fn test_read_large_file_streaming() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("jit_streaming_test_large.txt");

        let mut file = std::fs::File::create(&test_file).unwrap();
        // Generate a large file to test memory constraint
        for i in 1..=5000 {
            writeln!(file, "Line {}", i).unwrap();
        }

        let tool = read_tool(None);

        let args = json!({
            "path": test_file.to_string_lossy().to_string(),
            "start_line": 4900,
            "end_line": 4903
        });

        let result = tool.execute.execute(args).await.unwrap();
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

        let tool = read_tool(None);

        // 1. Try reading the whole file - should fail
        let args = json!({ "path": test_file.to_string_lossy().to_string() });
        let result = tool.execute.execute(args).await;
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
        let result2 = tool.execute.execute(args2).await;
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
        let result3 = tool.execute.execute(args3).await;
        assert!(result3.is_ok());

        let _ = std::fs::remove_file(&test_file);
    }
}
