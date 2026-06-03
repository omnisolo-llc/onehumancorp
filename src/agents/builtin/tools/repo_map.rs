use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use std::path::PathBuf;
use regex::Regex;
use once_cell::sync::Lazy;

use super::{Tool, ToolExecutor};

static RS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*(pub(?:\([a-z:]+\))?\s+)?(?:async\s+)?(fn|struct|enum|trait)\s+([a-zA-Z0-9_]+)").unwrap());
static PY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*(?:async\s+)?(def|class)\s+([a-zA-Z0-9_]+)").unwrap());
static TS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*(export\s+)?(?:async\s+)?(function|class|interface|type|const|let|var)\s+([a-zA-Z0-9_]+)").unwrap());

/// SOTA Harness Pattern: Aider's RepoMap for large codebases.
/// Generates a compact summary of the repository's architecture including file structure and basic symbol signatures.
pub struct RepoMapExecutor {
    workspace_path: PathBuf,
}

impl RepoMapExecutor {
    pub fn new(workspace_path: PathBuf) -> Self {
        Self { workspace_path }
    }

    fn extract_signatures(content: &str, ext: &str) -> Vec<String> {
        let mut sigs = Vec::new();
        match ext {
            "rs" => {
                for line in content.lines() {
                    if let Some(_) = RS_REGEX.captures(line) {
                        sigs.push(line.trim().to_string());
                    }
                }
            }
            "py" => {
                for line in content.lines() {
                    if let Some(_) = PY_REGEX.captures(line) {
                        sigs.push(line.trim().to_string());
                    }
                }
            }
            "ts" | "js" | "tsx" | "jsx" => {
                for line in content.lines() {
                    if let Some(_) = TS_REGEX.captures(line) {
                        sigs.push(line.trim().to_string());
                    }
                }
            }
            _ => {}
        }
        sigs
    }

    fn generate_map_recursive(dir: PathBuf, prefix: String) -> Result<String, std::io::Error> {
        let mut map = String::new();
        if !dir.is_dir() {
            return Ok(map);
        }

        let mut entries: Vec<_> = std::fs::read_dir(&dir)?.filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|e| e.path());

        for entry in entries {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            // Skip common hidden or build directories
            if name.starts_with('.') || name == "target" || name == "node_modules" || name == "dist" || name == "build" {
                continue;
            }

            if path.is_dir() {
                map.push_str(&format!("{}📁 {}/\n", prefix, name));
                map.push_str(&Self::generate_map_recursive(path, format!("{}  ", prefix))?);
            } else {
                map.push_str(&format!("{}📄 {}\n", prefix, name));

                // Read file to extract signatures
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        let sigs = Self::extract_signatures(&content, ext);
                        for sig in sigs.iter().take(10) { // Limit to top 10 signatures per file to keep it compact
                            map.push_str(&format!("{}  │ {}\n", prefix, sig));
                        }
                        if sigs.len() > 10 {
                            map.push_str(&format!("{}  │ ... ({} more)\n", prefix, sigs.len() - 10));
                        }
                    }
                }
            }
        }

        Ok(map)
    }
}

#[async_trait::async_trait]
impl ToolExecutor for RepoMapExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let mut target_path = self.workspace_path.clone();

        if let Some(path_val) = args.get("path") {
            if let Some(path_str) = path_val.as_str() {
                target_path = self.workspace_path.join(path_str);
            }
        }

        // Fix path traversal: canonicalize both paths and verify target is within workspace
        let abs_workspace = std::fs::canonicalize(&self.workspace_path)
            .unwrap_or_else(|_| self.workspace_path.clone());
        let abs_target = std::fs::canonicalize(&target_path)
            .map_err(|_| ToolError::LlmRecoverable(format!("Path does not exist: {}", target_path.display())))?;

        if !abs_target.starts_with(&abs_workspace) {
            return Err(ToolError::LlmRecoverable("Path Traversal Denied: target path is outside the workspace directory.".to_string()));
        }

        if !abs_target.exists() {
             return Err(ToolError::LlmRecoverable(format!("Path does not exist: {}", abs_target.display())));
        }

        let map = tokio::task::spawn_blocking(move || RepoMapExecutor::generate_map_recursive(abs_target.clone(), "".to_string()))
            .await
            .map_err(|e| ToolError::Transient(format!("Task Join Error: {}", e)))?
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        let mut final_output = format!("RepoMap for {}:\n", target_path.display());
        if map.is_empty() {
             final_output.push_str("(Empty or access denied)");
        } else {
             final_output.push_str(&map);
        }

        Ok(final_output)
    }
}

pub fn repomap_tool(workspace_path: PathBuf) -> Tool {
    Tool {
        name: "RepoMap".to_string(),
        description: "Generates a compact summary of the repository's architecture including file structure and basic symbol signatures. Highly recommended for understanding large codebases. (Aider's RepoMap Mechanic)".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Optional relative path within the workspace to generate the map for. Defaults to the root workspace."
                }
            }
        }),
        execute: Arc::new(RepoMapExecutor::new(workspace_path)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[tokio::test]
    async fn test_repomap_generation() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        // Create a dummy structure
        let src_dir = root.join("src");
        fs::create_dir(&src_dir).unwrap();

        let rs_file = src_dir.join("main.rs");
        fs::write(&rs_file, "pub fn main() {}\nstruct User {\n  id: u64,\n}\nfn helper() {}\n").unwrap();

        let py_file = src_dir.join("utils.py");
        fs::write(&py_file, "def do_something():\n  pass\n\nclass Data:\n  pass\n").unwrap();

        let ts_file = src_dir.join("app.ts");
        fs::write(&ts_file, "export function init() {}\ninterface Config {}\n").unwrap();

        // Should ignore hidden and target
        let hidden_dir = root.join(".git");
        fs::create_dir(&hidden_dir).unwrap();
        let target_dir = root.join("target");
        fs::create_dir(&target_dir).unwrap();

        let executor = RepoMapExecutor::new(root.to_path_buf());
        let result = executor.execute(json!({})).await.unwrap();

        assert!(result.contains("RepoMap for"));
        assert!(result.contains("📁 src/"));
        assert!(result.contains("📄 main.rs"));
        assert!(result.contains("│ pub fn main() {}"));
        assert!(result.contains("│ struct User {"));
        assert!(result.contains("│ fn helper() {}"));

        assert!(result.contains("📄 utils.py"));
        assert!(result.contains("│ def do_something():"));
        assert!(result.contains("│ class Data:"));

        assert!(result.contains("📄 app.ts"));
        assert!(result.contains("│ export function init() {}"));
        assert!(result.contains("│ interface Config {}"));

        assert!(!result.contains(".git"));
        assert!(!result.contains("target"));
    }

    #[tokio::test]
    async fn test_repomap_path_traversal() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let outside_dir = tempdir().unwrap();
        let outside_root = outside_dir.path();
        fs::write(outside_root.join("secret.txt"), "super secret").unwrap();

        let executor = RepoMapExecutor::new(root.to_path_buf());
        let result = executor.execute(json!({
            "path": outside_root.to_string_lossy().to_string()
        })).await;

        assert!(result.is_err());
        match result {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert!(msg.contains("Path Traversal Denied"));
            }
            _ => panic!("Expected LlmRecoverable error for path traversal"),
        }
    }

    #[tokio::test]
    async fn test_repomap_missing_file() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let executor = RepoMapExecutor::new(root.to_path_buf());
        let result = executor.execute(json!({
            "path": "does_not_exist_folder"
        })).await;

        assert!(result.is_err());
        match result {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert!(msg.contains("Path does not exist"));
            }
            _ => panic!("Expected LlmRecoverable error for missing path"),
        }
    }

    #[tokio::test]
    async fn test_repomap_empty_directory() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let executor = RepoMapExecutor::new(root.to_path_buf());
        let result = executor.execute(json!({})).await.unwrap();

        assert!(result.contains("Empty or access denied"));
    }

    #[tokio::test]
    async fn test_repomap_limit_signatures() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let mut content = String::new();
        for i in 0..15 {
            content.push_str(&format!("pub fn func_{}() {{}}\n", i));
        }

        fs::write(root.join("many_funcs.rs"), content).unwrap();

        let executor = RepoMapExecutor::new(root.to_path_buf());
        let result = executor.execute(json!({})).await.unwrap();

        assert!(result.contains("📄 many_funcs.rs"));
        assert!(result.contains("│ pub fn func_0() {}"));
        assert!(result.contains("│ pub fn func_9() {}"));
        assert!(!result.contains("│ pub fn func_10() {}"));
        assert!(result.contains("│ ... (5 more)"));
    }

    #[test]
    fn test_extract_signatures_rust() {
        let content = "
pub fn my_func() {}
pub(crate) struct MyStruct;
enum MyEnum {}
async fn async_func() {}
pub async trait MyTrait {}
        ";
        let sigs = RepoMapExecutor::extract_signatures(content, "rs");
        assert_eq!(sigs.len(), 5);
        assert_eq!(sigs[0], "pub fn my_func() {}");
        assert_eq!(sigs[1], "pub(crate) struct MyStruct;");
        assert_eq!(sigs[2], "enum MyEnum {}");
        assert_eq!(sigs[3], "async fn async_func() {}");
        assert_eq!(sigs[4], "pub async trait MyTrait {}");
    }

    #[test]
    fn test_extract_signatures_python() {
        let content = "
def my_func():
class MyClass:
async def async_func():
        ";
        let sigs = RepoMapExecutor::extract_signatures(content, "py");
        assert_eq!(sigs.len(), 3);
        assert_eq!(sigs[0], "def my_func():");
        assert_eq!(sigs[1], "class MyClass:");
        assert_eq!(sigs[2], "async def async_func():");
    }

    #[test]
    fn test_extract_signatures_typescript() {
        let content = "
function myFunc() {}
export class MyClass {}
export interface MyInterface {}
type MyType = string;
export const MY_CONST = 1;
async function asyncFunc() {}
        ";
        let sigs = RepoMapExecutor::extract_signatures(content, "ts");
        assert_eq!(sigs.len(), 6);
        assert_eq!(sigs[0], "function myFunc() {}");
        assert_eq!(sigs[1], "export class MyClass {}");
        assert_eq!(sigs[2], "export interface MyInterface {}");
        assert_eq!(sigs[3], "type MyType = string;");
        assert_eq!(sigs[4], "export const MY_CONST = 1;");
        assert_eq!(sigs[5], "async function asyncFunc() {}");
    }
}
