use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use serde::Deserialize;
use std::sync::Arc;
use tokio::fs;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

#[derive(Deserialize)]
struct FindArgs {
    #[serde(default = "default_path")]
    path: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    file_type: Option<String>, // "f" for file, "d" for dir
}

fn default_path() -> String { ".".to_string() }

struct FindExecutor {
    working_dir: Option<std::path::PathBuf>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<FindArgs> for FindExecutor {
    async fn execute_typed(
        &self,
        args: FindArgs,
    ) -> Result<String, ToolError> {
        let base_dir = args.path;
        let safe_base = base_dir.strip_prefix("/").unwrap_or(&base_dir);

        let search_path = if let Some(wd) = &self.working_dir {
            wd.join(safe_base)
        } else {
            std::path::PathBuf::from(safe_base)
        };

        if !search_path.exists() {
            return Err(ToolError::LlmRecoverable(format!("Directory not found: {}", search_path.display())));
        }

        let mut matches = Vec::new();
        self.search_recursive(&search_path, &args.name, &args.file_type, &mut matches).await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to search directory: {}", e)))?;

        // Just-in-Time (JIT) Retrieval Mechanic:
        if matches.is_empty() {
            return Ok("No files found.".to_string());
        }

        if matches.len() > 100 {
            matches.truncate(100);
            matches.push("... (truncated to 100 results to save context. Please use a more specific pattern or path to narrow the search.)".to_string());
        }

        Ok(matches.join("\n"))
    }
}

impl FindExecutor {
    #[async_recursion::async_recursion]
    async fn search_recursive(
        &self,
        current_dir: &std::path::Path,
        name_filter: &Option<String>,
        type_filter: &Option<String>,
        matches: &mut Vec<String>,
    ) -> Result<(), std::io::Error> {
        let mut entries = fs::read_dir(current_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let metadata = entry.metadata().await?;
            let file_name = entry.file_name().to_string_lossy().to_string();

            // Skip hidden directories and build artifacts
            if metadata.is_dir() && (file_name.starts_with('.') || file_name == "target" || file_name == "node_modules") {
                continue;
            }

            let mut include = true;

            if let Some(f_name) = name_filter {
                if !file_name.contains(f_name) && !glob_match(f_name, &file_name) {
                    include = false;
                }
            }

            if let Some(f_type) = type_filter {
                if f_type == "f" && !metadata.is_file() {
                    include = false;
                } else if f_type == "d" && !metadata.is_dir() {
                    include = false;
                }
            }

            if include {
                let display_path = if let Some(wd) = &self.working_dir {
                    path.strip_prefix(wd).unwrap_or(&path).display().to_string()
                } else {
                    path.display().to_string()
                };
                matches.push(display_path);

                if matches.len() > 100 {
                    return Ok(());
                }
            }

            if metadata.is_dir() {
                self.search_recursive(&path, name_filter, type_filter, matches).await?;
            }
        }
        Ok(())
    }
}

// Simple glob matcher for name filters
fn glob_match(pattern: &str, text: &str) -> bool {
    let re_pattern = pattern.replace(".", "\\.").replace("*", ".*").replace("?", ".");
    if let Ok(re) = regex::Regex::new(&format!("^{}$", re_pattern)) {
        re.is_match(text)
    } else {
        false
    }
}

pub fn find_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "Find".to_string(),
        description: "Search for files in a directory hierarchy. Returns newline-separated paths. Used for Context Management (Preventing Context Rot): Just-in-Time (JIT) Context Retrieval.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Directory to search (default '.')."
                },
                "name": {
                    "type": "string",
                    "description": "Base of file name (the path with the leading directories removed) to search for. Supports wildcards."
                },
                "file_type": {
                    "type": "string",
                    "description": "File type to search for: 'f' for regular files, 'd' for directories.",
                    "enum": ["f", "d"]
                }
            }
        }),
        execute: Arc::new(PydanticAdapter::new(FindExecutor { working_dir })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_find_files_by_name() {
        let temp_dir = std::env::temp_dir();
        let test_dir = temp_dir.join(format!("find_test_{}", uuid::Uuid::new_v4()));
        tokio::fs::create_dir_all(&test_dir).await.unwrap();

        tokio::fs::write(test_dir.join("test1.txt"), "hello").await.unwrap();
        tokio::fs::write(test_dir.join("test2.rs"), "hello").await.unwrap();
        tokio::fs::write(test_dir.join("test3.txt"), "hello").await.unwrap();

        let executor = FindExecutor { working_dir: Some(test_dir.clone()) };

        let args = FindArgs {
            path: ".".to_string(),
            name: Some("*.txt".to_string()),
            file_type: None,
        };

        let result = executor.execute_typed(args).await.unwrap();
        assert!(result.contains("test1.txt"));
        assert!(!result.contains("test2.rs"));
        assert!(result.contains("test3.txt"));

        tokio::fs::remove_dir_all(&test_dir).await.unwrap();
    }

    #[tokio::test]
    async fn test_find_type_filter() {
        let temp_dir = std::env::temp_dir();
        let test_dir = temp_dir.join(format!("find_type_test_{}", uuid::Uuid::new_v4()));
        tokio::fs::create_dir_all(&test_dir).await.unwrap();

        let sub_dir = test_dir.join("subdir");
        tokio::fs::create_dir_all(&sub_dir).await.unwrap();
        tokio::fs::write(test_dir.join("file.txt"), "hello").await.unwrap();

        let executor = FindExecutor { working_dir: Some(test_dir.clone()) };

        let args = FindArgs {
            path: ".".to_string(),
            name: None,
            file_type: Some("d".to_string()),
        };

        let result = executor.execute_typed(args).await.unwrap();
        assert!(result.contains("subdir"));
        assert!(!result.contains("file.txt"));

        let args_file = FindArgs {
            path: ".".to_string(),
            name: None,
            file_type: Some("f".to_string()),
        };

        let result_file = executor.execute_typed(args_file).await.unwrap();
        assert!(!result_file.contains("subdir"));
        assert!(result_file.contains("file.txt"));

        tokio::fs::remove_dir_all(&test_dir).await.unwrap();
    }

    #[tokio::test]
    async fn test_find_tool_pydantic_error() {
        let temp_dir = std::env::temp_dir();
        let tool = find_tool(Some(temp_dir.clone()));

        // Test with invalid type (simulating what LLM might do wrong)
        let bad_args = json!({
            "path": ".",
            "file_type": ["invalid_array_type"]
        });

        let err = tool.execute.execute(bad_args).await;
        assert!(err.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = err {
             assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
        } else {
             panic!("Expected Pydantic Validation Error");
        }
    }
}
