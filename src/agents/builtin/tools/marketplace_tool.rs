use super::{Tool, ToolExecutor};
use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use std::sync::Arc;
use super::marketplace::MarketplaceClient;

/// Exposes the marketplace to the agent so it can dynamically fetch new tools/agents.
pub struct MarketplaceToolExecutor {
    pub client: Arc<MarketplaceClient>,
}

#[async_trait::async_trait]
impl ToolExecutor for MarketplaceToolExecutor {
    async fn execute(&self, args: serde_json::Value) -> Result<String, ToolError> {
        let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
        let action = args.get("action").and_then(|v| v.as_str()).unwrap_or("search");

        if action == "search" {
            match self.client.search(query).await {
                Ok(agents) => Ok(serde_json::to_string_pretty(&agents).unwrap_or_default()),
                Err(e) => Err(ToolError::Transient(e)),
            }
        } else if action == "fetch" {
            let agent_id = args.get("agent_id").and_then(|v| v.as_str()).ok_or_else(|| {
                ToolError::LlmRecoverable("Missing agent_id for fetch action".to_string())
            })?;

            match self.client.fetch_agent(agent_id).await {
                Ok(agent) => Ok(format!("Successfully fetched agent definition:\n{}", serde_json::to_string_pretty(&agent).unwrap_or_default())),
                Err(e) => Err(ToolError::Transient(e)),
            }
        } else {
            Err(ToolError::LlmRecoverable(format!("Unknown action: {}", action)))
        }
    }
}

pub fn marketplace_tool(client: Arc<MarketplaceClient>) -> Tool {
    Tool {
        name: "agent_marketplace".to_string(),
        description: "Search for and fetch pre-built agents from the AutoGPT Agent Marketplace.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["search", "fetch"],
                    "description": "Action to perform: 'search' to find agents, 'fetch' to get a specific agent's details."
                },
                "query": {
                    "type": "string",
                    "description": "Search query (used if action is 'search')."
                },
                "agent_id": {
                    "type": "string",
                    "description": "The ID of the agent to fetch (used if action is 'fetch')."
                }
            },
            "required": ["action"]
        }),
        execute: Arc::new(MarketplaceToolExecutor { client }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::marketplace::test_utils::MockMarketplaceProvider;

    #[tokio::test]
    async fn test_marketplace_tool_search() {
        let client = Arc::new(MarketplaceClient::new(Box::new(MockMarketplaceProvider)));
        let tool = marketplace_tool(client);

        let args = json!({
            "action": "search",
            "query": "data"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert!(result.contains("Data Analyst"));
        assert!(result.contains("agent-1"));
    }

    #[tokio::test]
    async fn test_marketplace_tool_fetch() {
        let client = Arc::new(MarketplaceClient::new(Box::new(MockMarketplaceProvider)));
        let tool = marketplace_tool(client);

        let args = json!({
            "action": "fetch",
            "agent_id": "agent-1"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert!(result.contains("Successfully fetched"));
        assert!(result.contains("Data Analyst"));
    }
}
