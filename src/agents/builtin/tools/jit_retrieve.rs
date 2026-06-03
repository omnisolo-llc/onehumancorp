use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, BufReader};

use super::{Tool, ToolExecutor};

struct JitRetrieveExecutor {
    working_dir: Option<std::path::PathBuf>,
}

impl JitRetrieveExecutor {
    async fn read_head(&self, path: &std::path::Path, lines: usize) -> Result<String, String> {
        let file = File::open(path).await.map_err(|e| e.to_string())?;
        let mut reader = BufReader::new(file);
        let mut output = String::new();
        let mut line_count = 0;
        let mut line = String::new();

        while line_count < lines {
            line.clear();
            let bytes_read = reader.read_line(&mut line).await.map_err(|e| e.to_string())?;
            if bytes_read == 0 {
                break;
            }
            output.push_str(&line);
            line_count += 1;
        }
        Ok(output.trim_end().to_string())
    }

    async fn read_tail(&self, path: &std::path::Path, lines_to_read: usize) -> Result<String, String> {
        if lines_to_read == 0 {
            return Ok(String::new());
        }

        let mut file = File::open(path).await.map_err(|e| e.to_string())?;
        let metadata = file.metadata().await.map_err(|e| e.to_string())?;
        let len = metadata.len();

        if len == 0 {
            return Ok(String::new());
        }

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
            file.seek(std::io::SeekFrom::Start(current_pos as u64)).await.map_err(|e| e.to_string())?;
            let bytes_read = file.read(&mut buffer[0..read_size as usize]).await.map_err(|e| e.to_string())?;

            let chunk = &buffer[0..bytes_read];

            for (i, &b) in chunk.iter().enumerate().rev() {
                if b == b'\n' {
                    if current_pos as u64 + i as u64 == len - 1 {
                        continue;
                    }
                    num_lines_found += 1;
                    if num_lines_found == lines_to_read {
                        let final_start = current_pos as u64 + i as u64 + 1;
                        file.seek(std::io::SeekFrom::Start(final_start)).await.map_err(|e| e.to_string())?;
                        let mut final_content = String::new();
                        file.read_to_string(&mut final_content).await.map_err(|e| e.to_string())?;
                        return Ok(final_content.trim_end().to_string());
                    }
                }
            }
        }

        file.seek(std::io::SeekFrom::Start(0)).await.map_err(|e| e.to_string())?;
        let mut final_content = String::new();
        file.read_to_string(&mut final_content).await.map_err(|e| e.to_string())?;
        Ok(final_content.trim_end().to_string())
    }
}

#[async_trait::async_trait]
impl ToolExecutor for JitRetrieveExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let glob_pattern = args["glob_pattern"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("jit_retrieve: glob_pattern is required".to_string()))?;

        if glob_pattern.contains("..") {
            return Err(ToolError::LlmRecoverable("jit_retrieve: path traversal via '..' is not allowed".to_string()));
        }

        let regex_pattern = args["regex_pattern"].as_str();
        let head_lines = args["head_lines"].as_u64().unwrap_or(0) as usize;
        let tail_lines = args["tail_lines"].as_u64().unwrap_or(0) as usize;

        let regex = if let Some(p) = regex_pattern {
            Some(regex::Regex::new(p).map_err(|e| ToolError::LlmRecoverable(format!("jit_retrieve: invalid regex: {}", e)))?)
        } else {
            None
        };

        let wd = self.working_dir.clone().unwrap_or_else(|| std::path::PathBuf::from("."));
        let full_pattern = wd.join(glob_pattern.trim_start_matches('/'))
            .to_string_lossy()
            .into_owned();

        // Spawn blocking to avoid stalling the async reactor
        let matches_res = tokio::task::spawn_blocking(move || {
            let mut matches = Vec::new();
            if let Ok(paths) = glob::glob(&full_pattern) {
                for p in paths.filter_map(|res| res.ok()) {
                    if p.is_file() {
                        matches.push(p.to_string_lossy().into_owned());
                        if matches.len() >= 50 {
                            break;
                        }
                    }
                }
            }
            matches
        }).await.map_err(|e| ToolError::LlmRecoverable(format!("jit_retrieve: glob task failed: {}", e)))?;

        let matches = matches_res;

        if matches.is_empty() {
            return Ok(format!("No files matched glob pattern: {}", glob_pattern));
        }

        let mut output = String::new();
        for path_str in matches {
            let path = std::path::Path::new(&path_str);
            let display_path = path.strip_prefix(&wd).unwrap_or(path).to_string_lossy();
            output.push_str(&format!("\n--- File: {} ---\n", display_path));

            if let Some(ref re) = regex {
                let file = match File::open(path).await {
                    Ok(f) => f,
                    Err(e) => {
                        output.push_str(&format!("Error reading file: {}\n", e));
                        continue;
                    }
                };
                let mut reader = BufReader::new(file);
                let mut line = String::new();
                let mut line_num = 1;
                let mut matched_lines = 0;

                loop {
                    line.clear();
                    match reader.read_line(&mut line).await {
                        Ok(0) => break, // EOF
                        Ok(_) => {
                            if re.is_match(&line) {
                                output.push_str(&format!("{}: {}", line_num, line));
                                matched_lines += 1;
                                if matched_lines >= 100 {
                                    output.push_str("... (truncated to 100 matches per file)\n");
                                    break;
                                }
                            }
                            line_num += 1;
                        },
                        Err(e) => {
                            output.push_str(&format!("Error reading file: {}\n", e));
                            break;
                        }
                    }
                }

                if matched_lines == 0 {
                    output.push_str("No regex matches found.\n");
                }
            } else {
                if head_lines > 0 {
                    output.push_str("[HEAD]\n");
                    match self.read_head(path, head_lines).await {
                        Ok(head_content) => output.push_str(&format!("{}\n", head_content)),
                        Err(e) => output.push_str(&format!("Error reading head: {}\n", e)),
                    }
                }

                if head_lines > 0 && tail_lines > 0 {
                    output.push_str("\n...\n\n");
                }

                if tail_lines > 0 {
                    output.push_str("[TAIL]\n");
                    match self.read_tail(path, tail_lines).await {
                        Ok(tail_content) => output.push_str(&format!("{}\n", tail_content)),
                        Err(e) => output.push_str(&format!("Error reading tail: {}\n", e)),
                    }
                }

                if head_lines == 0 && tail_lines == 0 {
                    output.push_str("No content retrieved because neither regex_pattern, head_lines, nor tail_lines were specified.\n");
                }
            }
        }

        let max_output_len = 100_000;
        if output.len() > max_output_len {
            let mut limit = max_output_len;
            while !output.is_char_boundary(limit) {
                limit -= 1;
            }
            output.truncate(limit);
            output.push_str("\n\n... (Output truncated to 100,000 bytes. JIT retrieval limit reached.)");
        }

        Ok(output.trim().to_string())
    }
}

pub fn jit_retrieve_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "JitRetrieve".to_string(),
        description: "A helper for Context Management: Just-in-Time (JIT) Retrieval. Uses targeted pattern matching (glob) and returns either regex matches or truncated head/tail content instead of loading full files. \
                      Usage: Provide `glob_pattern` to find files. Optionally provide `regex_pattern` to search within files. If `regex_pattern` is omitted, provide `head_lines` and `tail_lines` to preview the top and bottom of matched files. Never use this to load entire files into memory.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "glob_pattern": {
                    "type": "string",
                    "description": "Glob pattern to match files (e.g. '**/*.go', 'src/*.rs')."
                },
                "regex_pattern": {
                    "type": "string",
                    "description": "Optional regex pattern to search within the matched files."
                },
                "head_lines": {
                    "type": "integer",
                    "description": "Optional number of lines to read from the start of each matched file."
                },
                "tail_lines": {
                    "type": "integer",
                    "description": "Optional number of lines to read from the end of each matched file."
                }
            },
            "required": ["glob_pattern"]
        }),
        execute: Arc::new(JitRetrieveExecutor { working_dir }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs;

    #[tokio::test]
    async fn test_jit_retrieve_basic_head_tail() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_jit.txt");
        let content = (1..=20).map(|i| format!("line{}", i)).collect::<Vec<_>>().join("\n");
        fs::write(&file_path, content).await.unwrap();

        let executor = JitRetrieveExecutor { working_dir: Some(dir.path().to_path_buf()) };

        let args = json!({
            "glob_pattern": "test_jit.txt",
            "head_lines": 2,
            "tail_lines": 2
        });

        let result = executor.execute(args).await.unwrap();
        assert!(result.contains("--- File: test_jit.txt ---"));
        assert!(result.contains("[HEAD]"));
        assert!(result.contains("line1\nline2"));
        assert!(result.contains("[TAIL]"));
        assert!(result.contains("line19\nline20"));
    }

    #[tokio::test]
    async fn test_jit_retrieve_regex() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_regex.txt");
        fs::write(&file_path, "apple\nbanana\ncherry\n").await.unwrap();

        let executor = JitRetrieveExecutor { working_dir: Some(dir.path().to_path_buf()) };

        let args = json!({
            "glob_pattern": "test_regex.txt",
            "regex_pattern": "an"
        });

        let result = executor.execute(args).await.unwrap();
        assert!(result.contains("--- File: test_regex.txt ---"));
        assert!(result.contains("2: banana"));
        assert!(!result.contains("apple"));
    }

    #[tokio::test]
    async fn test_jit_retrieve_path_traversal() {
        let executor = JitRetrieveExecutor { working_dir: None };
        let args = json!({ "glob_pattern": "../../../etc/passwd" });
        let result = executor.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("path traversal"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }
}
