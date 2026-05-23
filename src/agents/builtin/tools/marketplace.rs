use ohc_builtin_agent_core::types::ToolError;
use super::{Tool, ToolExecutor};
use std::sync::Arc;
use serde_json::Value;

/// AutoGPT Unique Harness Innovations: Agent Marketplace
/// Pre-built agent distribution

#[derive(Debug, Clone)]
pub struct MarketplaceExecutor {
    registry_url: String,
}

impl MarketplaceExecutor {
    pub fn new(registry_url: String) -> Self {
        Self { registry_url }
    }
}

#[async_trait::async_trait]
impl ToolExecutor for MarketplaceExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let action = args.get("action").and_then(|v| v.as_str()).unwrap_or("");

        match action {
            "search" => {
                let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");

                let client = reqwest::Client::new();
                let url = format!("{}/api/agents/search?q={}", self.registry_url, urlencoding::encode(query));

                let resp = match client.get(&url).send().await {
                    Ok(r) => r,
                    Err(e) => return Err(ToolError::Transient(format!("Marketplace API request failed: {}", e))),
                };

                if !resp.status().is_success() {
                    return Err(ToolError::LlmRecoverable(format!("Marketplace search failed with status: {}", resp.status())));
                }

                let results: Value = match resp.json().await {
                    Ok(r) => r,
                    Err(e) => return Err(ToolError::LlmRecoverable(format!("Failed to parse marketplace response: {}", e))),
                };

                Ok(format!("Agent Marketplace Search Results for '{}':\n{}", query, serde_json::to_string_pretty(&results).unwrap_or_default()))
            }
            "install" => {
                let agent_id = args.get("agent_id").and_then(|v| v.as_str()).unwrap_or("");
                if agent_id.is_empty() {
                    return Err(ToolError::LlmRecoverable("agent_id is required for install action".to_string()));
                }

                // Sanitize agent_id to prevent path traversal by extracting the file name.
                let safe_agent_id = std::path::Path::new(agent_id)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("");
                if safe_agent_id.is_empty() || safe_agent_id != agent_id {
                    return Err(ToolError::LlmRecoverable(format!("Invalid agent_id format. Provide a simple filename without path separators.")));
                }

                let client = reqwest::Client::new();
                let url = format!("{}/api/agents/download/{}", self.registry_url, urlencoding::encode(agent_id));

                let resp = match client.get(&url).send().await {
                    Ok(r) => r,
                    Err(e) => return Err(ToolError::Transient(format!("Marketplace API request failed: {}", e))),
                };

                if !resp.status().is_success() {
                    return Err(ToolError::LlmRecoverable(format!("Marketplace install failed with status: {}", resp.status())));
                }

                let agent_code = match resp.text().await {
                    Ok(t) => t,
                    Err(e) => return Err(ToolError::LlmRecoverable(format!("Failed to read agent code: {}", e))),
                };

                // Write the downloaded agent code to the local skills directory
                let skills_dir = std::path::PathBuf::from(".ohc/skills");
                if !skills_dir.exists() {
                    let _ = std::fs::create_dir_all(&skills_dir);
                }

                let agent_path = skills_dir.join(format!("{}.json", agent_id));
                if let Err(e) = std::fs::write(&agent_path, agent_code) {
                    return Err(ToolError::LlmRecoverable(format!("Failed to write agent to disk: {}", e)));
                }

                Ok(format!("Successfully installed agent '{}' from the Marketplace.", agent_id))
            }
            _ => Err(ToolError::LlmRecoverable(format!("Unknown action: {}", action))),
        }
    }
}

pub fn marketplace_tool(registry_url: String) -> Tool {
    Tool {
        name: "agent_marketplace".to_string(),
        description: "Interact with the Agent Marketplace to search for and install pre-built agents.".to_string(),
        is_read_only: false,
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "description": "The action to perform: 'search' or 'install'"
                },
                "query": {
                    "type": "string",
                    "description": "The search query (required if action is 'search')"
                },
                "agent_id": {
                    "type": "string",
                    "description": "The ID of the agent to install (required if action is 'install')"
                }
            },
            "required": ["action"]
        }),
        execute: Arc::new(MarketplaceExecutor::new(registry_url)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_marketplace_tool_install_path_traversal() {
        let executor = MarketplaceExecutor::new("http://example.com".to_string());

        let args_parent = serde_json::json!({
            "action": "install",
            "agent_id": "../banned"
        });

        let err_parent = executor.execute(args_parent).await.unwrap_err();
        assert!(matches!(err_parent, ToolError::LlmRecoverable(_)));
        if let ToolError::LlmRecoverable(msg) = err_parent {
            assert!(msg.contains("Invalid agent_id format"));
        }

        let args_slash = serde_json::json!({
            "action": "install",
            "agent_id": "/absolute/path"
        });

        let err_slash = executor.execute(args_slash).await.unwrap_err();
        assert!(matches!(err_slash, ToolError::LlmRecoverable(_)));
        if let ToolError::LlmRecoverable(msg) = err_slash {
            assert!(msg.contains("Invalid agent_id format"));
        }
    }
}
