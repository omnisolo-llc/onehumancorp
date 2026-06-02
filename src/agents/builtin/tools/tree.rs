use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use std::fs;
use std::path::Path;

use super::{Tool, ToolExecutor};

struct TreeExecutor;

impl TreeExecutor {
    fn walk(&self, dir: &Path, depth: usize, max_depth: usize, output: &mut String) {
        if depth > max_depth {
            return;
        }

        let name = dir.file_name().unwrap_or_default().to_string_lossy();
        let indent = "  ".repeat(depth);

        if depth == 0 {
            output.push_str(&format!("{}\n", name));
        } else {
            output.push_str(&format!("{}|-- {}\n", indent, name));
        }

        if dir.is_dir() {
            if let Ok(entries) = fs::read_dir(dir) {
                let mut paths: Vec<_> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
                paths.sort();
                for path in paths {
                    self.walk(&path, depth + 1, max_depth, output);
                }
            }
        }
    }
}

#[async_trait::async_trait]
impl ToolExecutor for TreeExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let directory = args["directory"]
            .as_str()
            .unwrap_or(".");
        let max_depth = args["max_depth"]
            .as_u64()
            .unwrap_or(2) as usize;

        let mut output = String::new();
        let path = Path::new(directory);

        if !path.exists() {
            return Ok(format!("Directory '{}' is empty or does not exist.", directory));
        }

        self.walk(path, 0, max_depth, &mut output);

        if output.is_empty() {
            Ok(format!("Directory '{}' is empty or does not exist.", directory))
        } else {
            Ok(output)
        }
    }
}

pub fn tree_tool() -> Tool {
    Tool {
        name: "Tree".to_string(),
        description: "List contents of directories in a tree-like format. Used for Just-in-Time (JIT) Context Retrieval to avoid loading full directory structures at once.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "directory": {
                    "type": "string",
                    "description": "The directory to list."
                },
                "max_depth": {
                    "type": "integer",
                    "description": "Maximum depth to traverse. Default is 2."
                }
            }
        }),
        execute: Arc::new(TreeExecutor),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_tree_executor() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        std::fs::create_dir_all(root.join("a/b/c")).unwrap();
        std::fs::write(root.join("a/file1.txt"), "hello").unwrap();
        std::fs::write(root.join("a/b/file2.txt"), "world").unwrap();

        let executor = TreeExecutor;

        let args = json!({
            "directory": root.join("a").to_str().unwrap(),
            "max_depth": 1
        });

        let res = executor.execute(args).await.unwrap();
        assert!(res.contains("a"));
        assert!(res.contains("file1.txt"));
        assert!(res.contains("b"));
        // file2.txt is at depth 2 (relative to 'a'), so it should not be included
        assert!(!res.contains("file2.txt"));
    }
}
