use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use super::{Tool, ToolExecutor};

struct TailExecutor {
    working_dir: Option<std::path::PathBuf>,
}

#[async_trait::async_trait]
impl ToolExecutor for TailExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let path = args["path"].as_str().ok_or_else(|| ToolError::LlmRecoverable("tail: path is required".to_string()))?;

        // Basic path sanitization: disallow relative path traversal
        if path.contains("..") {
            return Err(ToolError::LlmRecoverable("tail: path traversal via '..' is not allowed".to_string()));
        }

        let safe_path = std::path::Path::new(path).strip_prefix("/").unwrap_or(std::path::Path::new(path));
        let actual_path = if let Some(wd) = &self.working_dir { wd.join(safe_path) } else { std::path::PathBuf::from(path) };

        let mut file = File::open(&actual_path)
            .await
            .map_err(|e| format!("tail: {}: {}", path, e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        let lines_to_read = args["lines"].as_u64().unwrap_or(10) as usize;
        if lines_to_read == 0 {
            return Ok(String::new());
        }

        if lines_to_read > 1000 {
            return Err(ToolError::LlmRecoverable("JIT Retrieval Error: Cannot read more than 1000 lines at once. Never load full files.".to_string()));
        }

        let metadata = file.metadata().await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
        let len = metadata.len();

        if len == 0 {
            return Ok(String::new());
        }

        // Just-in-Time (JIT) Retrieval Mechanic: "Never load full files."
        // Chunked backward reading to avoid loading the whole file into memory.
        let chunk_size = 4096;
        let mut num_lines_found = 0;
        let mut current_pos = len as i64;
        let mut buffer = vec![0; chunk_size];

        while current_pos > 0 && num_lines_found <= lines_to_read {
            let read_size = if current_pos >= chunk_size as i64 {
                chunk_size as i64
            } else {
                current_pos
            };

            current_pos -= read_size;
            file.seek(std::io::SeekFrom::Start(current_pos as u64)).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
            let bytes_read = file.read(&mut buffer[0..read_size as usize]).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

            let chunk = &buffer[0..bytes_read];

            // Count newlines from back to front
            for (i, &b) in chunk.iter().enumerate().rev() {
                if b == b'\n' {
                    if current_pos as u64 + i as u64 == len - 1 {
                        // ignore a trailing newline at the very end of file
                        continue;
                    }
                    num_lines_found += 1;
                    if num_lines_found == lines_to_read {
                        let final_start = current_pos as u64 + i as u64 + 1;
                        file.seek(std::io::SeekFrom::Start(final_start)).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
                        let mut final_content = String::new();
                        file.read_to_string(&mut final_content).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
                        return Ok(final_content.trim_end().to_string());
                    }
                }
            }
        }

        // If we reached here, we couldn't find enough newlines, so return the whole file
        file.seek(std::io::SeekFrom::Start(0)).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
        let mut final_content = String::new();
        file.read_to_string(&mut final_content).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
        Ok(final_content.trim_end().to_string())
    }
}

pub fn tail_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "Tail".to_string(),
        description: "Read the last N lines of a file (default 10). Used for Just-in-Time (JIT) Context Retrieval.".to_string(),
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
                    "description": "Number of lines to read from the end (default 10)."
                }
            },
            "required": ["path"]
        }),
        execute: Arc::new(TailExecutor { working_dir }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs;

    #[tokio::test]
    async fn test_tail_basic() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "line1\nline2\nline3\nline4\n").await.unwrap();

        let executor = TailExecutor { working_dir: Some(dir.path().to_path_buf()) };

        let args = json!({ "path": "test.txt", "lines": 2 });
        let result = executor.execute(args).await.unwrap();
        assert_eq!(result, "line3\nline4");
    }

    #[tokio::test]
    async fn test_tail_default_lines() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let content = (1..=15).map(|i| format!("line{}", i)).collect::<Vec<_>>().join("\n");
        fs::write(&file_path, content).await.unwrap();

        let executor = TailExecutor { working_dir: Some(dir.path().to_path_buf()) };

        let args = json!({ "path": "test.txt" });
        let result = executor.execute(args).await.unwrap();
        let result_lines: Vec<&str> = result.split('\n').collect();
        assert_eq!(result_lines.len(), 10);
        assert_eq!(result_lines[0], "line6");
        assert_eq!(result_lines[9], "line15");
    }

    #[tokio::test]
    async fn test_tail_path_traversal() {
        let executor = TailExecutor { working_dir: None };
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
    async fn test_tail_jit_retrieval_limit() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "line1\nline2").await.unwrap();

        let executor = TailExecutor { working_dir: Some(dir.path().to_path_buf()) };

        let args = json!({ "path": "test.txt", "lines": 2000 });
        let result = executor.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("JIT Retrieval Error: Cannot read more than 1000 lines at once"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }

    #[tokio::test]
    async fn test_tail_large_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("large_test.txt");
        let mut content = String::new();
        // create a large file (approx 100k lines)
        for i in 1..=10000 {
             content.push_str(&format!("This is line number {}\n", i));
        }
        fs::write(&file_path, content).await.unwrap();

        let executor = TailExecutor { working_dir: Some(dir.path().to_path_buf()) };
        let args = json!({ "path": "large_test.txt", "lines": 3 });
        let result = executor.execute(args).await.unwrap();
        let expected = "This is line number 9998\nThis is line number 9999\nThis is line number 10000";
        assert_eq!(result, expected);
    }
}
