use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use std::path::{Path, PathBuf};
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


            "sh" | "bash" => {
                let mut parser = tree_sitter::Parser::new();
                if parser.set_language(&tree_sitter_bash::LANGUAGE.into()).is_ok() {
                    if let Some(tree) = parser.parse(content, None) {
                        let mut stack = vec![tree.root_node()];

                        while let Some(node) = stack.pop() {
                            let kind = node.kind();
                            if kind == "function_definition" {
                                let text = &content[node.start_byte()..node.end_byte()];
                                if let Some(first_line) = text.lines().next() {
                                    let mut clean_line = first_line.trim().to_string();
                                    if clean_line.ends_with('{') {
                                        clean_line.truncate(clean_line.len() - 1);
                                        clean_line = clean_line.trim().to_string();
                                    }
                                    if clean_line.ends_with("()") {
                                        sigs.push(format!("{} {{}}", clean_line));
                                    } else {
                                        sigs.push(clean_line);
                                    }
                                }
                            } else if kind == "variable_assignment" {
                                let text = &content[node.start_byte()..node.end_byte()];
                                if let Some(first_line) = text.lines().next() {
                                    sigs.push(first_line.trim().to_string());
                                }
                            }

                            if kind != "function_definition" && kind != "command" {
                                let mut cursor = node.walk();
                                let mut children = vec![];
                                for child in node.children(&mut cursor) {
                                    children.push(child);
                                }
                                for child in children.into_iter().rev() {
                                    stack.push(child);
                                }
                            }
                        }
                    }
                }
            }



            _ => {}
        }
        sigs
    }


    fn generate_map_recursive(dir: PathBuf, prefix: String, query: Option<&str>) -> Result<String, std::io::Error> {
        let mut map = String::new();
        if !dir.is_dir() {
            return Ok(map);
        }

        let mut entries: Vec<_> = std::fs::read_dir(&dir)?.filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|e| e.path());

        let mut files_to_process = Vec::new();
        let mut dirs_to_process = Vec::new();

        for entry in entries {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            // Skip common hidden or build directories
            if name.starts_with('.') || name == "target" || name == "node_modules" || name == "dist" || name == "build" {
                continue;
            }

            if path.is_dir() {
                dirs_to_process.push((path, name));
            } else {
                files_to_process.push((path, name));
            }
        }

        // Process directories first
        for (path, name) in dirs_to_process {
            let sub_map = Self::generate_map_recursive(path, format!("{}  ", prefix), query)?;
            if !sub_map.trim().is_empty() {
                map.push_str(&format!("{}📁 {}/
", prefix, name));
                map.push_str(&sub_map);
            }
        }

        // Score files
        let mut ranked_files = Vec::new();
        for (path, name) in files_to_process {
            let mut score = 0;
            let mut sigs = Vec::new();

            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    sigs = Self::extract_signatures(&content, ext);

                    if let Some(q) = query {
                        let q_lower = q.to_lowercase();
                        let words = q_lower.split_whitespace();
                        for word in words {
                            if name.to_lowercase().contains(word) {
                                score += 10;
                            }
                            for sig in &sigs {
                                if sig.to_lowercase().contains(word) {
                                    score += 2;
                                }
                            }
                        }
                    } else {
                        score = 1; // Default score if no query
                    }
                }
            } else if query.is_none() {
                score = 1; // Default for non-source files
            } else if let Some(q) = query {
                // Score non-source files by name
                let q_lower = q.to_lowercase();
                for word in q_lower.split_whitespace() {
                    if name.to_lowercase().contains(word) {
                        score += 5;
                    }
                }
            }

            if score > 0 {
                ranked_files.push((name, sigs, score));
            }
        }

        // Sort files by score descending
        ranked_files.sort_by(|a, b| b.2.cmp(&a.2));

        for (name, sigs, _) in ranked_files {
            map.push_str(&format!("{}📄 {}
", prefix, name));
            for sig in sigs.iter().take(10) {
                map.push_str(&format!("{}  │ {}
", prefix, sig));
            }
            if sigs.len() > 10 {
                map.push_str(&format!("{}  │ ... ({} more)
", prefix, sigs.len() - 10));
            }
        }

        Ok(map)
    }

}

#[async_trait::async_trait]
impl ToolExecutor for RepoMapExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let mut target_path = self.workspace_path.clone();
        let query_str = args.get("query").and_then(|v| v.as_str()).map(|s| s.to_string());

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

        let map = tokio::task::spawn_blocking(move || RepoMapExecutor::generate_map_recursive(abs_target.clone(), "".to_string(), query_str.as_deref()))
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
                },
                "query": {
                    "type": "string",
                    "description": "Optional search query to rank files by relevance."
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

        let sh_file = src_dir.join("script.sh");
        fs::write(&sh_file, "function my_bash_func() {\n  echo 'hello'\n}\nMY_VAR='test'\n").unwrap();


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

        assert!(result.contains("📄 script.sh"));
        assert!(result.contains("│ function my_bash_func() {}"));
        assert!(result.contains("│ MY_VAR='test'"));


        assert!(!result.contains(".git"));
        assert!(!result.contains("target"));
    }
}
