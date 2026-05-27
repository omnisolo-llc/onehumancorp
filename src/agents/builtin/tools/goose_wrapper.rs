use super::{Tool, ToolExecutor};
use ohc_builtin_agent_core::types::ToolError;
use ohc_builtin_agent_core::goose::GooseMcpLoader;
use serde_json::Value;
use std::sync::Arc;

pub struct GooseWrapperExecutor {
    loader: Arc<GooseMcpLoader>,
    extension_name: String,
}

#[async_trait::async_trait]
impl ToolExecutor for GooseWrapperExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        self.loader.execute_extension(&self.extension_name, args).await
    }
}

pub fn load_goose_tools(loader: Arc<GooseMcpLoader>) -> Vec<Tool> {
    let mut tools = Vec::new();
    for ext in loader.get_extensions() {
        tools.push(Tool {
            name: ext.name.clone(),
            description: ext.description.clone(),
            is_read_only: false, // MCP tools might mutate state
            parameters: serde_json::json!({
                "type": "object",
                "additionalProperties": true,
                "description": "Arguments passed through to the Goose MCP extension."
            }),
            execute: Arc::new(GooseWrapperExecutor {
                loader: loader.clone(),
                extension_name: ext.name.clone(),
            }),
        });
    }
    tools
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_load_goose_tools() {
        let loader = Arc::new(GooseMcpLoader::new());
        let tools = load_goose_tools(loader);

        assert!(tools.len() >= 70);

        let tool1 = tools.iter().find(|t| t.name == "mcp_extension_1").unwrap();
        let res = tool1.execute.execute(json!({})).await.unwrap();
        assert_eq!(res, "Successfully executed extension: mcp_extension_1 via MCP endpoint stdio://mcp-1");
    }
}
