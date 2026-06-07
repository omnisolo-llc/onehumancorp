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
}

#[async_trait::async_trait]
impl PydanticToolExecutor<LazyLoadArgs> for LazyLoadToolsExecutor {
    async fn execute_typed(&self, args: LazyLoadArgs) -> Result<String, ToolError> {
        let mut active = self.active_tools.write().await;
        let mut loaded = Vec::new();

        for name in args.tool_names {
            active.insert(name.clone());
            loaded.push(name);
        }

        if loaded.is_empty() {
            Ok("No valid tool names provided to load.".to_string())
        } else {
            Ok(format!(
                "Successfully loaded {} tools into your context window. You may now use them in the next turn.",
                loaded.join(", ")
            ))
        }
    }
}

pub fn lazy_load_tool(active_tools: Arc<RwLock<HashSet<String>>>) -> Tool {
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
        execute: Arc::new(PydanticAdapter::new(LazyLoadToolsExecutor { active_tools })),
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
        let tool = lazy_load_tool(active_tools.clone());

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
        let tool = lazy_load_tool(active_tools.clone());

        let args = serde_json::json!({
            "tool_names": "Not an array"
        });

        let res = tool.execute.execute(args).await;
        assert!(res.is_err());

        if let Err(ToolError::LlmRecoverable(err_msg)) = res {
            assert!(err_msg.contains("Validation Error (Pydantic-first tool schema)"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }
}
