use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use serde::Deserialize;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use regex::Regex;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

#[derive(Deserialize)]
struct JitContextArgs {
    path: String,
    symbol: String,
}

struct JitContextExecutor {
    working_dir: Option<std::path::PathBuf>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<JitContextArgs> for JitContextExecutor {
    async fn execute_typed(&self, args: JitContextArgs) -> Result<String, ToolError> {
        let path = args.path;
        let symbol = args.symbol;

        let safe_path = std::path::Path::new(&path).strip_prefix("/").unwrap_or(std::path::Path::new(&path));
        let actual_path = if let Some(wd) = &self.working_dir { wd.join(safe_path) } else { std::path::PathBuf::from(&path) };

        let file = tokio::fs::File::open(&actual_path)
            .await
            .map_err(|e| ToolError::LlmRecoverable(format!("jit_context: failed to open {}: {}", path, e)))?;

        let mut reader = BufReader::new(file);
        let mut line_buffer = String::new();

        let mut in_symbol = false;
        let mut snippet = Vec::new();
        let mut brace_count = 0;
        let mut found = false;

        // Simple heuristic regex to find symbol definition (functions, structs, classes)
        let decl_re = Regex::new(&format!(r"(?i)(?:fn|func|class|struct|interface|type)\s+{}\b", regex::escape(&symbol)))
            .map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        while let Ok(bytes) = reader.read_line(&mut line_buffer).await {
            if bytes == 0 { break; }

            if !in_symbol {
                if decl_re.is_match(&line_buffer) {
                    in_symbol = true;
                    found = true;
                    snippet.push(line_buffer.clone());
                    brace_count += line_buffer.chars().filter(|&c| c == '{').count() as i32;
                    brace_count -= line_buffer.chars().filter(|&c| c == '}').count() as i32;
                    if line_buffer.contains('{') && brace_count == 0 {
                        // single line definition
                        break;
                    }
                }
            } else {
                snippet.push(line_buffer.clone());
                brace_count += line_buffer.chars().filter(|&c| c == '{').count() as i32;
                brace_count -= line_buffer.chars().filter(|&c| c == '}').count() as i32;

                if brace_count <= 0 {
                    break;
                }

                // Safety limit for JIT Context retrieval
                if snippet.len() > 500 {
                    snippet.push("// ... (snippet truncated to 500 lines to prevent context rot)\n".to_string());
                    break;
                }
            }
            line_buffer.clear();
        }

        if !found {
            return Ok(format!("Symbol '{}' not found in {}", symbol, path));
        }

        Ok(snippet.join(""))
    }
}

pub fn jit_context_tool(working_dir: Option<std::path::PathBuf>) -> Tool {
    Tool {
        name: "JitContext".to_string(),
        description: "Extract specific function, class, or symbol definitions from files to prevent context rot. Just-in-Time (JIT) Context Retrieval.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the source file."
                },
                "symbol": {
                    "type": "string",
                    "description": "Name of the function, struct, class, or type to extract."
                }
            },
            "required": ["path", "symbol"]
        }),
        execute: Arc::new(PydanticAdapter::new(JitContextExecutor { working_dir })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_jit_context_extracts_symbol() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("jit_test_source.rs");
        tokio::fs::write(&file_path, "
use std::fmt;

fn ignored_func() {
    println!(\"ignore\");
}

pub struct MyTestSymbol {
    field: i32,
}

impl MyTestSymbol {
    fn new() -> Self { Self { field: 0 } }
}
").await.unwrap();

        let tool = jit_context_tool(Some(temp_dir.clone()));
        let args = json!({
            "path": "jit_test_source.rs",
            "symbol": "MyTestSymbol"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert!(result.contains("pub struct MyTestSymbol"));
        assert!(result.contains("field: i32,"));
        assert!(result.contains("}"));
        assert!(!result.contains("ignored_func"));

        let _ = tokio::fs::remove_file(&file_path).await;
    }

    #[tokio::test]
    async fn test_jit_context_symbol_not_found() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("jit_test_empty.rs");
        tokio::fs::write(&file_path, "fn hello() {}").await.unwrap();

        let tool = jit_context_tool(Some(temp_dir.clone()));
        let args = json!({
            "path": "jit_test_empty.rs",
            "symbol": "MissingSymbol"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert_eq!(result, "Symbol 'MissingSymbol' not found in jit_test_empty.rs");

        let _ = tokio::fs::remove_file(&file_path).await;
    }

    #[tokio::test]
    async fn test_jit_context_truncates_large_symbol() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("jit_test_large.rs");
        let mut content = String::from("fn large_func() {\n");
        for _ in 0..600 {
            content.push_str("    println!(\"test\");\n");
        }
        content.push_str("}\n");
        tokio::fs::write(&file_path, &content).await.unwrap();

        let tool = jit_context_tool(Some(temp_dir.clone()));
        let args = json!({
            "path": "jit_test_large.rs",
            "symbol": "large_func"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert!(result.contains("snippet truncated to 500 lines"));

        let _ = tokio::fs::remove_file(&file_path).await;
    }
}
