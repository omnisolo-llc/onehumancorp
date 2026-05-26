use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs::File;

use super::{Tool, ToolExecutor};

struct HeadExecutor {
    working_dir: Option<std::path::PathBuf>,
}

#[async_trait::async_trait]
impl ToolExecutor for HeadExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let path = args["path"].as_str().ok_or_else(|| ToolError::LlmRecoverable("head: path is required".to_string()))?;

        // Basic path sanitization: disallow relative path traversal
        if path.contains("..") {
            return Err(ToolError::LlmRecoverable("head: path traversal via '..' is not allowed".to_string()));
        }

        let safe_path = std::path::Path::new(path).strip_prefix("/").unwrap_or(std::path::Path::new(path));
        let actual_path = if let Some(wd) = &self.working_dir { wd.join(safe_path) } else { std::path::PathBuf::from(path) };

        let file = File::open(&actual_path)
            .await
            .map_err(|e| format!("head: {}: {}", path, e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        // Just-in-Time (JIT) Retrieval Mechanic: "Never load full files."
        let lines_to_read = args["lines"].as_u64().unwrap_or(10) as usize;

        if lines_to_read > 1000 {
            return Err(ToolError::LlmRecoverable("JIT Retrieval Error: Cannot read more than 1000 lines at once. Never load full files.".to_string()));
        }

        let mut reader = BufReader::new(file);
        let mut lines = Vec::new();
        let mut buffer = String::new();

        for _ in 0..lines_to_read {
            buffer.clear();
            let bytes_read = reader.read_line(&mut buffer).await
                .map_err(|e| ToolError::LlmRecoverable(format!("head: read error: {}", e)))?;
            if bytes_read == 0 {
                break;
            }
            lines.push(buffer.trim_end_matches(&['\r', '\n'][..]).to_string());
        }

        Ok(lines.join("\n"))
    }
}

pub fn head_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "Head".to_string(),
        description: "Read the first N lines of a file (default 10). Used for Just-in-Time (JIT) Context Retrieval.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to read."
                },
                "lines": {
                    "type": "integer",
                    "description": "Number of lines to read from the beginning (default 10)."
                }
            },
            "required": ["path"]
        }),
        execute: Arc::new(HeadExecutor { working_dir }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs;

    #[tokio::test]
    async fn test_head_basic() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "line1\nline2\nline3\nline4\n").await.unwrap();

        let executor = HeadExecutor { working_dir: Some(dir.path().to_path_buf()) };

        let args = json!({ "path": "test.txt", "lines": 2 });
        let result = executor.execute(args).await.unwrap();
        assert_eq!(result, "line1\nline2");
    }

    #[tokio::test]
    async fn test_head_default_lines() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let content = (1..=15).map(|i| format!("line{}", i)).collect::<Vec<_>>().join("\n");
        fs::write(&file_path, content).await.unwrap();

        let executor = HeadExecutor { working_dir: Some(dir.path().to_path_buf()) };

        let args = json!({ "path": "test.txt" });
        let result = executor.execute(args).await.unwrap();
        let result_lines: Vec<&str> = result.split('\n').collect();
        assert_eq!(result_lines.len(), 10);
        assert_eq!(result_lines[0], "line1");
        assert_eq!(result_lines[9], "line10");
    }

    #[tokio::test]
    async fn test_head_path_traversal() {
        let executor = HeadExecutor { working_dir: None };
        let args = json!({ "path": "../../../etc/passwd" });
        let result = executor.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("path traversal"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }

    #[tokio::test]
    async fn test_head_jit_retrieval_limit() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "line1\nline2").await.unwrap();

        let executor = HeadExecutor { working_dir: Some(dir.path().to_path_buf()) };

        let args = json!({ "path": "test.txt", "lines": 2000 });
        let result = executor.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("JIT Retrieval Error: Cannot read more than 1000 lines at once"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }
}
