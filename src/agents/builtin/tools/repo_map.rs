use ohc_builtin_agent_core::types::ToolError;

use serde_json::json;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use walkdir::{DirEntry, WalkDir};

use super::Tool;
use super::pydantic::{PydanticToolExecutor, PydanticAdapter};

/// Aider RepoMap Mechanic: A heuristic-based extractor to compress codebase contexts.
/// Provides a JIT (Just-In-Time) retrieval of file structure and important symbols.
#[derive(serde::Deserialize)]
pub struct RepoMapArgs {
    pub path: Option<String>,
}

pub struct RepoMapExecutor {
    pub working_dir: Option<PathBuf>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<RepoMapArgs> for RepoMapExecutor {
    async fn execute_typed(&self, args: RepoMapArgs) -> Result<String, ToolError> {
        let base_path = match &self.working_dir {
            Some(dir) => dir.clone(),
            None => std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        };

        let target_dir = match &args.path {
            Some(p) => {
                // Prevent absolute paths from replacing the base path
                let mut path = std::path::Path::new(p);
                if path.is_absolute() {
                    path = path.strip_prefix("/").unwrap_or(path);
                }
                base_path.join(path)
            },
            None => base_path.clone(),
        };

        // Ensure canonicalized path is a descendant of the base path to prevent path traversal
        let canonical_base = match base_path.canonicalize() {
            Ok(b) => b,
            Err(_) => return Err(ToolError::LlmRecoverable("Failed to canonicalize base path.".to_string())),
        };

        let canonical_target = match target_dir.canonicalize() {
            Ok(t) => t,
            Err(_) => return Err(ToolError::LlmRecoverable(format!("Directory not found: {}", target_dir.display()))),
        };

        if !canonical_target.starts_with(&canonical_base) {
            return Err(ToolError::LlmRecoverable("Path traversal detected. Access denied.".to_string()));
        }

        if !canonical_target.is_dir() {
            return Err(ToolError::LlmRecoverable(format!(
                "Path is not a directory: {}",
                canonical_target.display()
            )));
        }

        let target_dir = canonical_target.clone();
        let mut output = format!("RepoMap for {}\n", canonical_target.display());

        let mut file_count = 0;

        let walker = WalkDir::new(&target_dir).into_iter();
        for entry in walker.filter_entry(|e| !is_hidden_or_ignored(e)) {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            if entry.file_type().is_file() {
                let path = entry.path();
                let relative_path = path.strip_prefix(&target_dir).unwrap_or(path);

                if is_source_file(path) {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        let symbols = extract_symbols(&content, path);
                        if !symbols.is_empty() {
                            output.push_str(&format!("\n📄 {}\n", relative_path.display()));
                            for sym in symbols {
                                output.push_str(&format!("  │ {}\n", sym));
                            }
                            file_count += 1;
                        } else {
                             // File has no important symbols or was too short
                             output.push_str(&format!("\n📄 {}\n  │ (no major symbols)\n", relative_path.display()));
                             file_count += 1;
                        }
                    }
                }
            }
        }

        output.push_str(&format!("\nTotal files indexed: {}\n", file_count));
        output.push_str("Use 'read_file' or 'grep' to inspect specific files further.");

        Ok(output)
    }
}

fn is_hidden_or_ignored(entry: &DirEntry) -> bool {
    let file_name = entry.file_name().to_str().unwrap_or("");
    if file_name.starts_with(".") && entry.depth() > 0 {
        return true;
    }
    file_name == "node_modules" || file_name == "target" || file_name.starts_with("bazel-") || file_name == "dist" || file_name == "build"
}

fn is_source_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        matches!(
            ext.to_str().unwrap_or(""),
            "rs" | "py" | "js" | "ts" | "go" | "c" | "cpp" | "h" | "hpp" | "java" | "cs" | "rb"
        )
    } else {
        false
    }
}

fn extract_symbols(content: &str, path: &Path) -> Vec<String> {
    let mut symbols = Vec::new();
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    for line in content.lines() {
        let trimmed = line.trim();
        let extracted = match ext {
            "rs" => {
                if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") ||
                   trimmed.starts_with("pub struct ") || trimmed.starts_with("struct ") ||
                   trimmed.starts_with("pub enum ") || trimmed.starts_with("enum ") ||
                   trimmed.starts_with("pub trait ") || trimmed.starts_with("trait ") ||
                   trimmed.starts_with("impl ") {
                    Some(trimmed)
                } else {
                    None
                }
            }
            "py" => {
                if trimmed.starts_with("def ") || trimmed.starts_with("class ") {
                    Some(trimmed)
                } else {
                    None
                }
            }
            "js" | "ts" => {
                if trimmed.starts_with("function ") || trimmed.starts_with("export function ") ||
                   trimmed.starts_with("class ") || trimmed.starts_with("export class ") ||
                   trimmed.starts_with("interface ") || trimmed.starts_with("export interface ") ||
                   trimmed.starts_with("type ") || trimmed.starts_with("export type ") ||
                   (trimmed.starts_with("const ") && trimmed.contains("=>")) {
                    Some(trimmed)
                } else {
                    None
                }
            }
            "go" => {
                if trimmed.starts_with("func ") || (trimmed.starts_with("type ") && (trimmed.contains("struct") || trimmed.contains("interface"))) {
                    Some(trimmed)
                } else {
                    None
                }
            }
            _ => {
                // Fallback for other languages: just look for functions/classes
                if trimmed.starts_with("function ") || trimmed.starts_with("class ") || trimmed.starts_with("def ") || trimmed.starts_with("fn ") {
                    Some(trimmed)
                } else {
                    None
                }
            }
        };

        if let Some(s) = extracted {
            let limit = 80;
            if s.len() > limit {
                symbols.push(format!("{}...", &s[..limit]));
            } else {
                symbols.push(s.to_string());
            }
        }
    }
    symbols
}

pub fn repo_map_tool(working_dir: Option<PathBuf>) -> Tool {
    Tool {
        name: "repo_map".to_string(),
        description: "Aider RepoMap Mechanic: Get a structural overview of the codebase in a directory. Extracts file paths, functions, classes, and important symbols without reading entire files. Use this first to understand large codebases (JIT retrieval).".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The relative path to the directory to map. Defaults to the current directory."
                }
            }
        }),
        execute: Arc::new(PydanticAdapter::new(RepoMapExecutor { working_dir })),
    }
}
