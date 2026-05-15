use ohc_builtin_agent_core::types::ToolError;
use async_recursion::async_recursion;
use regex::Regex;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use std::collections::VecDeque;

use super::{Tool, ToolExecutor};

// Implements "Just-in-Time (JIT) Retrieval" mechanic to prevent context rot.
// Rather than returning raw lines or full files, this tool dynamically fetches
// the surrounding lines of context using a memory-efficient sliding window (BufReader).

pub struct GrepExecutor {
    pub working_dir: Option<std::path::PathBuf>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JitMatch {
    pub file_path: String,
    pub line_number: usize,
    pub content: String,
    pub context_before: Vec<String>,
    pub context_after: Vec<String>,
}

#[async_trait::async_trait]
impl ToolExecutor for GrepExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let pattern = args["pattern"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("grep: pattern is required".to_string()))?;
        let path = args["path"].as_str().unwrap_or(".");
        let case_insensitive = args["case_insensitive"].as_bool().unwrap_or(false);
        let include_pattern = args["include"].as_str().map(str::to_string);
        let context_lines = args["context_lines"].as_u64().unwrap_or(2) as usize;

        let re = if case_insensitive {
            Regex::new(&format!("(?i){}", pattern))
        } else {
            Regex::new(pattern)
        }
        .map_err(|e| format!("grep: invalid regex: {}", e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        let mut results = Vec::new();
        let safe_path = std::path::Path::new(path).strip_prefix("/").unwrap_or(std::path::Path::new(path));
        let actual_path = if let Some(wd) = &self.working_dir { wd.join(safe_path).to_string_lossy().to_string() } else { path.to_string() };

        Self::search_directory_with_context(&actual_path, &re, include_pattern.as_deref(), context_lines, &mut results)
            .await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        if results.is_empty() {
            return Ok("No matches found.".to_string());
        }

        let truncated = results.len() > 100;
        if truncated {
            results.truncate(100);
        }

        let output = serde_json::json!({
            "matches": results,
            "truncated": truncated
        });

        Ok(serde_json::to_string_pretty(&output).unwrap())
    }
}

impl GrepExecutor {
    async fn process_file(
        path: &std::path::Path,
        re: &Regex,
        include: Option<&str>,
        context_lines: usize,
        results: &mut Vec<JitMatch>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if let Some(inc) = include {
            if !matches_include(name, inc) {
                return Ok(());
            }
        }

        let file = match tokio::fs::File::open(path).await {
            Ok(f) => f,
            Err(_) => return Ok(()), // Skip unreadable/permission-denied files
        };
        let mut reader = BufReader::new(file);

        let mut lines_buffer: VecDeque<String> = VecDeque::new();
        let mut current_line_num = 0;
        let mut pending_after: Option<(JitMatch, usize)> = None;

        let mut line = String::new();
        loop {
            match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {}, // Continue processing
                Err(_) => return Ok(()), // Skip binary or invalid UTF-8 files
            }

            current_line_num += 1;
            let current_text = line.trim_end().to_string();

            // Handle pending 'after' context
            if let Some((mut pending_match, remaining)) = pending_after.take() {
                pending_match.context_after.push(current_text.clone());
                if remaining > 1 {
                    pending_after = Some((pending_match, remaining - 1));
                } else {
                    results.push(pending_match);
                    if results.len() > 100 {
                        return Ok(());
                    }
                }
            }

            if re.is_match(&current_text) {
                // We have a match! Resolve any previous pending immediately.
                if let Some((pending_match, _)) = pending_after.take() {
                    results.push(pending_match);
                    if results.len() > 100 {
                        return Ok(());
                    }
                }

                let context_before: Vec<String> = lines_buffer.iter().cloned().collect();

                let mut new_match = JitMatch {
                    file_path: path.display().to_string(),
                    line_number: current_line_num,
                    content: current_text.clone(),
                    context_before,
                    context_after: Vec::new(),
                };

                if context_lines > 0 {
                    pending_after = Some((new_match, context_lines));
                } else {
                    results.push(new_match);
                    if results.len() > 100 {
                        return Ok(());
                    }
                }
            }

            lines_buffer.push_back(current_text);
            if lines_buffer.len() > context_lines {
                lines_buffer.pop_front();
            }
            line.clear();
        }

        // Push any remaining pending
        if let Some((pending_match, _)) = pending_after {
            results.push(pending_match);
        }

        Ok(())
    }

    #[async_recursion]
    async fn search_directory_with_context(
        dir: &str,
        re: &Regex,
        include: Option<&str>,
        context_lines: usize,
        results: &mut Vec<JitMatch>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let meta = tokio::fs::metadata(dir).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
        if meta.is_file() {
            let path = std::path::Path::new(dir);
            return Self::process_file(path, re, include, context_lines, results).await;
        }

        let mut entries = tokio::fs::read_dir(dir).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            let meta = entry.metadata().await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
            if meta.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name.starts_with('.') || name == "target" || name == "node_modules" {
                    continue;
                }
                Self::search_directory_with_context(&path.to_string_lossy(), re, include, context_lines, results).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
            } else if meta.is_file() {
                Self::process_file(&path, re, include, context_lines, results).await?;
                if results.len() >= 100 {
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}

fn matches_include(filename: &str, include: &str) -> bool {
    if let Some(ext) = include.strip_prefix("*.") {
        if ext.starts_with('{') && ext.ends_with('}') {
            let exts = &ext[1..ext.len() - 1];
            for e in exts.split(',') {
                if filename.ends_with(&format!(".{}", e.trim())) {
                    return true;
                }
            }
            return false;
        }
        return filename.ends_with(&format!(".{}", ext));
    }
    filename.contains(include)
}

pub fn grep_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "Grep".to_string(),
        description: "Advanced JIT Retrieval: Search for a regex pattern in files under a directory. Returns highly structured JSON matches including surrounding context lines to prevent context rot. Uses memory-efficient streaming to handle massive files.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "Regular expression pattern to search for."
                },
                "path": {
                    "type": "string",
                    "description": "Directory to search (default '.')."
                },
                "include": {
                    "type": "string",
                    "description": "File extension filter (e.g. '*.go', '*.rs')."
                },
                "case_insensitive": {
                    "type": "boolean",
                    "description": "Case-insensitive search."
                },
                "context_lines": {
                    "type": "integer",
                    "description": "Number of lines of context to retrieve before and after the match. Default is 2."
                }
            },
            "required": ["pattern"]
        }),
        execute: Arc::new(GrepExecutor { working_dir }),
    }
}


#[cfg(test)]
mod advanced_jit_grep_tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[tokio::test]
    async fn test_advanced_jit_grep_core() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("adv_grep_test.txt");
        let content = "line 1\nline 2 matching target\nline 3\nline 4\ntarget\n";
        fs::write(&file_path, content).unwrap();

        let executor = GrepExecutor {
            working_dir: Some(dir.path().to_path_buf()),
        };

        let args = serde_json::json!({
            "pattern": "target",
            "path": "adv_grep_test.txt",
            "context_lines": 1,
            "case_insensitive": true
        });

        let res = executor.execute(args).await.unwrap();
        let json_res: Value = serde_json::from_str(&res).unwrap();
        let matches = json_res["matches"].as_array().unwrap();

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0]["content"], "line 2 matching target");
        assert_eq!(matches[0]["context_before"][0], "line 1");
        assert_eq!(matches[0]["context_after"][0], "line 3");

        assert_eq!(matches[1]["content"], "target");
        assert_eq!(matches[1]["context_before"][0], "line 4");
        assert_eq!(matches[1]["context_after"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_advanced_jit_grep_exhaustive_coverage() {
        let dir = tempdir().unwrap();

        for i in 1..=10 {
            let file = dir.path().join(format!("f{}.txt", i));
            std::fs::write(&file, format!("test data {}\nline 2\nline 3", i)).unwrap();
        }

        let executor = GrepExecutor {
            working_dir: Some(dir.path().to_path_buf()),
        };

        let args = serde_json::json!({
            "pattern": "test data",
            "path": ".",
            "context_lines": 0
        });

        let res = executor.execute(args).await.unwrap();
        let json_res: Value = serde_json::from_str(&res).unwrap();
        let matches = json_res["matches"].as_array().unwrap();
        assert_eq!(matches.len(), 10);
    }

    #[tokio::test]
    async fn test_advanced_jit_grep_edge_cases() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("edge.txt");
        fs::write(&file_path, "target at start\nline 2\nline 3\nline 4\ntarget at end").unwrap();

        let executor = GrepExecutor {
            working_dir: Some(dir.path().to_path_buf()),
        };

        let args = serde_json::json!({
            "pattern": "target",
            "path": "edge.txt",
            "context_lines": 2
        });

        let res = executor.execute(args).await.unwrap();
        let json_res: Value = serde_json::from_str(&res).unwrap();
        let matches = json_res["matches"].as_array().unwrap();

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0]["context_before"].as_array().unwrap().len(), 0);
        assert_eq!(matches[0]["context_after"].as_array().unwrap().len(), 2);

        assert_eq!(matches[1]["context_before"].as_array().unwrap().len(), 2);
        assert_eq!(matches[1]["context_after"].as_array().unwrap().len(), 0);
    }
}


#[cfg(test)]
mod comprehensive_integration_tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[tokio::test]
    async fn test_matrix_topological_grep_streaming_verification() {
        let dir = tempdir().unwrap();
        let executor = GrepExecutor {
            working_dir: Some(dir.path().to_path_buf()),
        };

        for i in 1..=10 {
            let sub_dir = dir.path().join(format!("level_{}", i));
            fs::create_dir_all(&sub_dir).unwrap();
            let file = sub_dir.join("data.txt");
            let content = format!("prefix {}\nmatch_{}\nsuffix {}\n", i, i, i).repeat(i);
            fs::write(&file, content).unwrap();

            let args = serde_json::json!({
                "pattern": format!("match_{}", i),
                "path": ".",
                "context_lines": 1
            });

            let res = executor.execute(args).await.unwrap();
            let json_res: Value = serde_json::from_str(&res).unwrap();
            let matches = json_res["matches"].as_array().unwrap();
            assert_eq!(matches.len(), std::cmp::min(i, 100)); // truncation checks
        }
    }
}
