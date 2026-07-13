#![allow(clippy::unnecessary_cast)]
use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::fs;

use super::{
    Tool,
    pydantic::{PydanticAdapter, PydanticToolExecutor},
    workspace_path,
};

const MAX_SELECTED_LINES: usize = 1_000;
const MAX_SELECTED_BYTES: usize = 1_048_576;

// Pydantic-first tool schema validation: ReadArgs
#[derive(Deserialize)]
struct ReadArgs {
    path: String,
    #[serde(default)]
    start_line: Option<usize>,
    #[serde(default)]
    end_line: Option<usize>,
}

struct ReadExecutor {
    workspace_root: Result<std::path::PathBuf, String>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<ReadArgs> for ReadExecutor {
    async fn execute_typed(&self, args: ReadArgs) -> Result<String, ToolError> {
        let path = args.path;
        let root = self.workspace_root.as_ref().map_err(|error| {
            ToolError::LlmRecoverable(format!("read: workspace root is unavailable: {error}"))
        })?;
        let actual_path = workspace_path::existing(root, &path).await?;

        // Context Management (Preventing Context Rot): JetBrains JIT Retrieval (grep, glob) Mechanic:
        // "Never load full files." We enforce a strict token/line limit and stream the file to prevent loading it entirely into memory.
        let file = fs::File::open(&actual_path)
            .await
            .map_err(|error| ToolError::LlmRecoverable(format!("read: {}: {}", path, error)))?;

        use tokio::io::{AsyncBufReadExt, BufReader};
        let mut reader = BufReader::new(file);
        let mut line_buffer = String::new();
        let mut result = String::new();
        let mut line_index = 0;
        let mut selected_lines = 0;
        let mut selected_bytes = 0;

        if args.start_line == Some(0) || args.end_line == Some(0) {
            return Err(ToolError::LlmRecoverable(
                "read: line numbers are 1-indexed".to_string(),
            ));
        }
        let start = args.start_line.unwrap_or(1) - 1;
        let end = args.end_line;
        let unbounded_read = args.start_line.is_none() && args.end_line.is_none();

        if let Some(end) = end {
            if start >= end {
                return Err(ToolError::LlmRecoverable(format!(
                    "read: invalid line range {}-{}",
                    start + 1,
                    end
                )));
            }
            if end - start > MAX_SELECTED_LINES {
                return Err(ToolError::LlmRecoverable("JIT Retrieval Error: Cannot read more than 1000 lines at once. Please use start_line and end_line to paginate.".to_string()));
            }
        }

        loop {
            if end.is_some_and(|end| line_index >= end) {
                break;
            }
            let bytes = reader
                .read_line(&mut line_buffer)
                .await
                .map_err(|error| ToolError::LlmRecoverable(format!("read: {}: {}", path, error)))?;
            if bytes == 0 {
                break;
            }
            if line_index >= start {
                if selected_lines == MAX_SELECTED_LINES {
                    if unbounded_read {
                        return Err(ToolError::LlmRecoverable(
                            "JIT Retrieval Error: File is too large (> 1000 lines). Never load full files. Please use start_line and end_line to paginate (max 1000 lines per request).".to_string()
                        ));
                    }
                    break;
                }
                if bytes > MAX_SELECTED_BYTES.saturating_sub(selected_bytes) {
                    return Err(ToolError::LlmRecoverable(
                        "JIT Retrieval Error: Selected content exceeds 1 MiB. Please request a smaller line range.".to_string(),
                    ));
                }
                if !result.is_empty() {
                    result.push('\n');
                }
                result.push_str(line_buffer.trim_end_matches('\n').trim_end_matches('\r'));
                selected_lines += 1;
                selected_bytes += bytes;
            }
            line_index += 1;
            line_buffer.clear();
        }

        Ok(result)
    }
}

pub fn read_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    let workspace_root =
        workspace_path::configured_root(working_dir).map_err(|error| error.to_string());
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
        execute: Arc::new(PydanticAdapter::new(ReadExecutor { workspace_root })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[tokio::test]
    async fn test_read_large_file_streaming() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("jit_streaming_test_large.txt");

        let mut file = std::fs::File::create(&test_file).unwrap();
        // Generate a large file to test memory constraint
        for i in 1..=5000 {
            writeln!(file, "Line {}", i).unwrap();
        }

        let executor = PydanticAdapter::new(ReadExecutor {
            workspace_root: Ok(temp_dir.path().to_path_buf()),
        });

        let args = json!({
            "path": "jit_streaming_test_large.txt",
            "start_line": 4900,
            "end_line": 4903
        });

        let result: String = crate::ToolExecutor::execute(&executor, args).await.unwrap();
        let expected = "Line 4900\nLine 4901\nLine 4902\nLine 4903";
        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn test_read_jit_retrieval_limit() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("jit_test_large.txt");

        // Create a file with 1500 lines
        let mut file = std::fs::File::create(&test_file).unwrap();
        for i in 1..=1500 {
            writeln!(file, "Line {}", i).unwrap();
        }

        let executor = PydanticAdapter::new(ReadExecutor {
            workspace_root: Ok(temp_dir.path().to_path_buf()),
        });

        // 1. Try reading the whole file - should fail
        let args = json!({ "path": "jit_test_large.txt" });
        let result: Result<String, ohc_builtin_agent_core::types::ToolError> =
            crate::ToolExecutor::execute(&executor, args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("JIT Retrieval Error: File is too large"));
        } else {
            panic!("Expected JIT Retrieval Error");
        }

        // 2. Try reading a slice larger than 1000 lines - should fail
        let args2 = json!({
            "path": "jit_test_large.txt",
            "start_line": 1,
            "end_line": 1200
        });
        let result2: Result<String, ohc_builtin_agent_core::types::ToolError> =
            crate::ToolExecutor::execute(&executor, args2).await;
        assert!(result2.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result2 {
            assert!(msg.contains("Cannot read more than 1000 lines at once"));
        } else {
            panic!("Expected JIT Retrieval Error");
        }

        // 3. Try reading a valid slice - should succeed
        let args3 = json!({
            "path": "jit_test_large.txt",
            "start_line": 500,
            "end_line": 600
        });
        let result3: Result<String, ohc_builtin_agent_core::types::ToolError> =
            crate::ToolExecutor::execute(&executor, args3).await;
        assert!(result3.is_ok());
    }

    #[tokio::test]
    async fn test_read_rejects_selected_content_over_one_mib() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("large.txt"), vec![b'a'; 1_048_577])
            .await
            .unwrap();
        let executor = PydanticAdapter::new(ReadExecutor {
            workspace_root: Ok(dir.path().to_path_buf()),
        });

        let result = crate::ToolExecutor::execute(&executor, json!({"path": "large.txt"})).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("1 MiB"));
    }

    #[tokio::test]
    async fn test_read_rejects_absolute_paths_outside_workspace() {
        let root = tempfile::tempdir().unwrap();
        let outside = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(outside.path(), "secret").unwrap();
        let executor = PydanticAdapter::new(ReadExecutor {
            workspace_root: Ok(root.path().to_path_buf()),
        });

        let result = crate::ToolExecutor::execute(
            &executor,
            json!({"path": outside.path().to_string_lossy()}),
        )
        .await;

        assert!(result.is_err());
        assert!(!result.unwrap_err().to_string().contains("secret"));
    }
}
