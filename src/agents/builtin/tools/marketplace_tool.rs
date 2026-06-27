use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};
use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use serde::Deserialize;
use std::sync::Arc;
use super::marketplace::MarketplaceClient;

// Pydantic-first tool schema validation: MarketplaceArgs
#[derive(Deserialize)]
struct MarketplaceArgs {
    action: String,
    query: Option<String>,
    agent_id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    role: Option<String>,
    system_prompt: Option<String>,
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
            let name = args.name.as_deref().ok_or_else(|| {
                ToolError::LlmRecoverable("Missing 'name' for publish action".to_string())
            })?;
            let description = args.description.as_deref().ok_or_else(|| {
                ToolError::LlmRecoverable("Missing 'description' for publish action".to_string())
            })?;
            let role = args.role.as_deref().unwrap_or("General").to_string();
            let system_prompt = args.system_prompt.as_deref().unwrap_or("").to_string();

            let agent = super::marketplace::MarketplaceAgent {
                id: format!("agent-{}", uuid::Uuid::new_v4()),
                name: name.to_string(),
                description: description.to_string(),
                role,
                system_prompt,
                author: "AutoGPT".to_string(),
                version: "1.0.0".to_string(),
                endpoint: "https://marketplace.example.com/agents/new".to_string(),
            };

            match self.client.publish_agent(agent).await {
                Ok(_) => Ok("Successfully published agent to the marketplace.".to_string()),
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
                "name": {
                    "type": "string",
                    "description": "The name of the agent to publish (used if action is 'publish')."
                },
                "description": {
                    "type": "string",
                    "description": "The description of the agent to publish (used if action is 'publish')."
                },
                "role": {
                    "type": "string",
                    "description": "The role of the agent to publish (used if action is 'publish')."
                },
                "system_prompt": {
                    "type": "string",
                    "description": "The system prompt of the agent to publish (used if action is 'publish')."
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
    async fn test_marketplace_tool_publish() {
        let client = Arc::new(MarketplaceClient::new(Box::new(MockMarketplaceProvider)));
        let tool = marketplace_tool(client);

        let args = json!({
            "action": "publish",
            "name": "New Agent",
            "description": "A new test agent",
            "role": "Tester",
            "system_prompt": "You are a tester."
        });

        let result = tool.execute.execute(args).await.unwrap();
        assert!(result.contains("Successfully published"));
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
