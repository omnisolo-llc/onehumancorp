use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{Tool, ToolExecutor};

struct LazyLoadToolsExecutor {
    active_tools: Arc<RwLock<HashSet<String>>>,
}

#[async_trait::async_trait]
impl ToolExecutor for LazyLoadToolsExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let tool_names = args["tool_names"]
            .as_array()
            .ok_or_else(|| ToolError::LlmRecoverable("lazy_load_tools: 'tool_names' must be an array of strings".to_string()))?;

        let mut active = self.active_tools.write().await;
        let mut loaded = Vec::new();

        for name_val in tool_names {
            if let Some(name) = name_val.as_str() {
                active.insert(name.to_string());
                loaded.push(name.to_string());
            }
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
        execute: Arc::new(LazyLoadToolsExecutor { active_tools }),
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
            assert!(err_msg.contains("must be an array"));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }
}
