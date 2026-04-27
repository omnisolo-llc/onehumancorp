use async_recursion::async_recursion;
use regex::Regex;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{ToolError, Tool, ToolExecutor};

struct GrepExecutor;

#[async_trait::async_trait]
impl ToolExecutor for GrepExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let pattern = args["pattern"]
            .as_str()
            .ok_or("grep: pattern is required")?;
        let path = args["path"].as_str().unwrap_or(".");
        let case_insensitive = args["case_insensitive"].as_bool().unwrap_or(false);
        let include_pattern = args["include"].as_str().map(str::to_string);

        let re = if case_insensitive {
            Regex::new(&format!("(?i){}", pattern))
        } else {
            Regex::new(pattern)
        }
        .map_err(|e| format!("grep: invalid regex: {}", e))?;

        let mut results = Vec::new();
        search_directory(path, &re, include_pattern.as_deref(), &mut results).await?;

        if results.is_empty() {
            return Ok("No matches found.".to_string());
        }

        // Limit output
        if results.len() > 500 {
            results.truncate(500);
            results.push("... (truncated)".to_string());
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
    let mut entries = tokio::fs::read_dir(dir).await?;
    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        let meta = entry.metadata().await?;
        if meta.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            // Skip hidden and build directories
            if name.starts_with('.') || name == "target" || name == "node_modules" {
                continue;
            }
            search_directory(&path.to_string_lossy(), re, include, results).await?;
        } else if meta.is_file() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if let Some(inc) = include {
                // Simple suffix match
                if !matches_include(name, inc) {
                    continue;
                }
            }
            if let Ok(content) = tokio::fs::read_to_string(&path).await {
                for (i, line) in content.lines().enumerate() {
                    if re.is_match(line) {
                        results.push(format!("{}:{}:{}", path.display(), i + 1, line));
                        if results.len() >= 500 {
                            return Ok(());
                        }
                    }
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

pub fn grep_tool() -> Tool {
    Tool {
        name: "Grep".to_string(),
        description: "Search for a regex pattern in files under a directory. Returns file:line:content matches.".to_string(),
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
        execute: Arc::new(GrepExecutor),
    }
}
