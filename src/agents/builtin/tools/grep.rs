use ohc_builtin_agent_core::types::ToolError;
use async_recursion::async_recursion;
use regex::Regex;
use serde_json::json;
use serde::Deserialize;
use std::sync::Arc;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

#[derive(Deserialize)]
struct GrepArgs {
    pattern: String,
    #[serde(default = "default_path")]
    path: String,
    #[serde(default)]
    case_insensitive: bool,
    #[serde(default)]
    include: Option<String>,
}

fn default_path() -> String { ".".to_string() }

struct GrepExecutor {
    working_dir: Option<std::path::PathBuf>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<GrepArgs> for GrepExecutor {
    async fn execute_typed(
        &self,
        args: GrepArgs,
    ) -> Result<String, ToolError> {
        let pattern = &args.pattern;
        let path = &args.path;
        let case_insensitive = args.case_insensitive;
        let include_pattern = args.include;

        let re = if case_insensitive {
            Regex::new(&format!("(?i){}", pattern))
        } else {
            Regex::new(pattern)
        }
        .map_err(|e| format!("grep: invalid regex: {}", e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        let mut results = Vec::new();
        let safe_path = std::path::Path::new(path).strip_prefix("/").unwrap_or(std::path::Path::new(path));
        let actual_path = if let Some(wd) = &self.working_dir { wd.join(safe_path).to_string_lossy().to_string() } else { path.to_string() };
        search_directory(&actual_path, &re, include_pattern.as_deref(), &mut results).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        if results.is_empty() {
            return Ok("No matches found.".to_string());
        }

        // Just-in-Time (JIT) Retrieval Mechanic:
        // Limit output
        if results.len() > 100 {
            results.truncate(100);
            results.push("... (truncated to 100 results to save context. Please refine your regex pattern, specify a more restrictive include filter, or use glob/find to narrow the search.)".to_string());
        }

        Ok(results.join("\n"))
    }
}

#[async_recursion]
async fn search_directory(
    dir: &str,
    re: &Regex,
    include: Option<&str>,
    results: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut entries = tokio::fs::read_dir(dir).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        let meta = entry.metadata().await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
        if meta.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            // Skip hidden and build directories
            if name.starts_with('.') || name == "target" || name == "node_modules" {
                continue;
            }
            search_directory(&path.to_string_lossy(), re, include, results).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
        } else if meta.is_file() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if let Some(inc) = include {
                // Simple suffix match
                if !matches_include(name, inc) {
                    continue;
                }
            }
            use tokio::io::{AsyncBufReadExt, BufReader};
            if let Ok(file) = tokio::fs::File::open(&path).await {
                let mut reader = BufReader::new(file);
                let mut line_buffer = String::new();
                let mut line_num = 1;

                while let Ok(bytes) = reader.read_line(&mut line_buffer).await {
                    if bytes == 0 {
                        break; // EOF
                    }
                    if re.is_match(&line_buffer) {
                        results.push(format!("{}:{}:{}", path.display(), line_num, line_buffer.trim_end_matches('\n').trim_end_matches('\r')));
                        if results.len() >= 100 {
                            return Ok(());
                        }
                    }
                    line_num += 1;
                    line_buffer.clear();
                }
            }
        }
    }
    Ok(())
}

fn matches_include(filename: &str, include: &str) -> bool {
    // Support simple glob like "*.go" or "*.{rs,go}"
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
        description: "Search for a regex pattern in files under a directory. Returns file:line:content matches. Used for Just-in-Time (JIT) Context Retrieval.".to_string(),
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
                }
            },
            "required": ["pattern"]
        }),
        execute: Arc::new(PydanticAdapter::new(GrepExecutor { working_dir })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
use crate::ToolExecutor;
    use std::io::Write;

    #[tokio::test]
    async fn test_grep_large_file_streaming() {
        let temp_dir = std::env::temp_dir();
        let test_dir = temp_dir.join(format!("grep_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&test_dir).unwrap();

        let test_file = test_dir.join("large_log.txt");

        let mut file = std::fs::File::create(&test_file).unwrap();
        // Generate a large file to test memory constraint
        for i in 1..=5000 {
            if i == 4500 {
                writeln!(file, "Error: critical failure found here!").unwrap();
            } else {
                writeln!(file, "Line {}", i).unwrap();
            }
        }

        let executor = PydanticAdapter::new(GrepExecutor { working_dir: Some(test_dir.clone()) });

        let args = json!({
            "pattern": "critical failure",
            "path": ".",
        });

        let result = ToolExecutor::execute(&executor, args).await.unwrap();
        let _expected_path = test_file.strip_prefix(&test_dir).unwrap_or(&test_file);
        // The display string might be just the name if we strip it
        assert!(result.contains("critical failure found here!"));
        assert!(result.contains(":4500:"));

        let _ = std::fs::remove_dir_all(&test_dir);
    }
}
