use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};
use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use serde::Deserialize;
use std::sync::Arc;
use super::marketplace::MarketplaceClient;

use super::marketplace::MarketplaceAgent;

// Pydantic-first tool schema validation: MarketplaceArgs
#[derive(Deserialize)]
struct MarketplaceArgs {
    action: String,
    query: Option<String>,
    agent_id: Option<String>,
    agent_name: Option<String>,
    agent_description: Option<String>,
    agent_author: Option<String>,
    agent_version: Option<String>,
    agent_endpoint: Option<String>,
}

/// Exposes the marketplace to the agent so it can dynamically fetch new tools/agents.
pub struct MarketplaceToolExecutor {
    pub client: Arc<MarketplaceClient>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<MarketplaceArgs> for MarketplaceToolExecutor {
    async fn execute_typed(&self, args: MarketplaceArgs) -> Result<String, ToolError> {
        let action = args.action.as_str();

        if action == "search" {
            let query = args.query.as_deref().unwrap_or("");
            match self.client.search(query).await {
                Ok(agents) => Ok(serde_json::to_string_pretty(&agents).unwrap_or_default()),
                Err(e) => Err(ToolError::Transient(e)),
            }
        } else if action == "fetch" {
            let agent_id = args.agent_id.as_deref().ok_or_else(|| {
                ToolError::LlmRecoverable("Missing agent_id for fetch action".to_string())
            })?;

            match self.client.fetch_agent(agent_id).await {
                Ok(agent) => Ok(format!("Successfully fetched agent definition:\n{}", serde_json::to_string_pretty(&agent).unwrap_or_default())),
                Err(e) => Err(ToolError::Transient(e)),
            }
        } else if action == "publish" {
            let agent = MarketplaceAgent {
                id: "".to_string(), // Let server assign
                name: args.agent_name.unwrap_or_default(),
                description: args.agent_description.unwrap_or_default(),
                author: args.agent_author.unwrap_or_default(),
                version: args.agent_version.unwrap_or_else(|| "1.0.0".to_string()),
                endpoint: args.agent_endpoint.unwrap_or_default(),
            };

            match self.client.publish_agent(agent).await {
                Ok(published) => Ok(format!("Successfully published agent to marketplace:\n{}", serde_json::to_string_pretty(&published).unwrap_or_default())),
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
        description: "Search for, fetch, and publish pre-built agents to the AutoGPT Agent Marketplace API distribution.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["search", "fetch", "publish"],
                    "description": "Action to perform: 'search' to find agents, 'fetch' to get a specific agent's details, 'publish' to publish a new agent."
                },
                "query": {
                    "type": "string",
                    "description": "Search query (used if action is 'search')."
                },
                "agent_id": {
                    "type": "string",
                    "description": "The ID of the agent to fetch (used if action is 'fetch')."
                },
                "agent_name": {
                    "type": "string",
                    "description": "The name of the agent to publish (used if action is 'publish')."
                },
                "agent_description": {
                    "type": "string",
                    "description": "The description of the agent to publish (used if action is 'publish')."
                },
                "agent_author": {
                    "type": "string",
                    "description": "The author of the agent to publish (used if action is 'publish')."
                },
                "agent_version": {
                    "type": "string",
                    "description": "The version of the agent to publish (used if action is 'publish')."
                },
                "agent_endpoint": {
                    "type": "string",
                    "description": "The endpoint where the agent definition can be fetched (used if action is 'publish')."
                }
            },
            "required": ["action"]
        }),
        execute: Arc::new(PydanticAdapter::new(MarketplaceToolExecutor { client })),
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

    #[tokio::test]
    async fn test_marketplace_tool_publish() {
        let client = Arc::new(MarketplaceClient::new(Box::new(MockMarketplaceProvider)));
        let tool = marketplace_tool(client);

        let args = json!({
            "action": "publish",
            "agent_name": "Writer",
            "agent_description": "Writes essays",
            "agent_author": "Tester"
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert!(result.contains("Successfully published"));
        assert!(result.contains("mock-id-123"));
        assert!(result.contains("Writer"));
    }

    #[tokio::test]
    async fn test_marketplace_tool_pydantic_validation() {
        let client = Arc::new(MarketplaceClient::new(Box::new(MockMarketplaceProvider)));
        let tool = marketplace_tool(client);

        let args = json!({
            "wrong_key": "fetch"
        });

        let result = tool.execute.execute(args).await;
        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
        } else {
            panic!("Expected Pydantic-first validation error");
        }
    }
}
