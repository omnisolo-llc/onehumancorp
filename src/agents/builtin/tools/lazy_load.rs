use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{pydantic::{PydanticAdapter, PydanticToolExecutor}, Tool};

#[derive(Deserialize)]
struct LazyLoadArgs {
    tool_names: Vec<String>,
}

struct LazyLoadToolsExecutor {
    active_tools: Arc<RwLock<HashSet<String>>>,
    available_tools: Arc<Vec<String>>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<LazyLoadArgs> for LazyLoadToolsExecutor {
    async fn execute_typed(&self, args: LazyLoadArgs) -> Result<String, ToolError> {
        let mut invalid_tools = Vec::new();
        for name in &args.tool_names {
            if !self.available_tools.contains(name) {
                invalid_tools.push(name.clone());
            }
        }

        if !invalid_tools.is_empty() {
            return Err(ToolError::LlmRecoverable(format!(
                "Validation Error (Pydantic-first tool schema): The following tools are not available in the global registry: {}. Please check your tool name spelling or use ToolSearch to find the correct tool name.",
                invalid_tools.join(", ")
            )));
        }

        if args.tool_names.is_empty() {
            Ok("No valid tool names provided to load.".to_string())
        } else {
            let mut active = self.active_tools.write().await;
            let mut loaded = Vec::new();

            for name in args.tool_names {
                // Re-check just to be absolutely certain we don't load something that wasn't allowed,
                // though we already validated above.
                active.insert(name.clone());
                loaded.push(name);
            }

            Ok(format!(
                "Successfully loaded {} tools into your context window. You may now use them in the next turn.",
                loaded.join(", ")
            ))
        }
    }
}

pub fn lazy_load_tool(active_tools: Arc<RwLock<HashSet<String>>>, available_tools: Arc<Vec<String>>) -> Tool {
    Tool {
        name: "LazyLoadTools".to_string(),
        description: "Loads additional tools into your context window. Use this when you discover tools via ToolSearch that you need to use.".to_string(),
        is_read_only: false, // State mutation for the agent context
        parameters: json!({
            "type": "object",
            "properties": {
                "tool_names": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    },
                    "description": "A list of exact tool names to load into your context."
                }
            },
            "required": ["tool_names"]
        }),
        execute: Arc::new(PydanticAdapter::new(LazyLoadToolsExecutor { active_tools, available_tools })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_lazy_load_tool() {
        let active_tools = Arc::new(RwLock::new(HashSet::new()));
        let available_tools = Arc::new(vec!["Bash".to_string(), "Write".to_string()]);
        let tool = lazy_load_tool(active_tools.clone(), available_tools);

        let args = serde_json::json!({
            "tool_names": ["Bash", "Write"]
        });

        let res = tool.execute.execute(args).await;
        assert!(res.is_ok());

        let msg = res.unwrap();
        assert!(msg.contains("Successfully loaded"));
        assert!(msg.contains("Bash"));
        assert!(msg.contains("Write"));

        let lock = active_tools.read().await;
        assert!(lock.contains("Bash"));
        assert!(lock.contains("Write"));
        assert_eq!(lock.len(), 2);
    }

    #[tokio::test]
    async fn test_lazy_load_tool_invalid_args() {
        let active_tools = Arc::new(RwLock::new(HashSet::new()));
        let available_tools = Arc::new(vec!["Bash".to_string(), "ValidTool".to_string()]);
        let tool = lazy_load_tool(active_tools.clone(), available_tools);

        let args = serde_json::json!({
            "tool_names": ["Bash", "InvalidTool"]
        });

        let res = tool.execute.execute(args).await;
        assert!(res.is_err());

        if let Err(ToolError::LlmRecoverable(err_msg)) = res {
            assert!(err_msg.contains("Validation Error (Pydantic-first tool schema)"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }

    #[tokio::test]
    async fn test_lazy_load_tool_empty_args() {
        let active_tools = Arc::new(RwLock::new(HashSet::new()));
        let available_tools = Arc::new(vec!["Bash".to_string()]);
        let tool = lazy_load_tool(active_tools.clone(), available_tools);

        let args = serde_json::json!({
            "tool_names": []
        });

        let res = tool.execute.execute(args).await;
        assert!(res.is_ok());
        let msg = res.unwrap();
        assert!(msg.contains("No valid tool names provided to load"));
    }
}
