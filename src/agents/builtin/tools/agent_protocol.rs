use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use url::Url;
use std::time::Duration;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

// Pydantic-first tool schema validation: AgentProtocolArgs
#[derive(Deserialize)]
struct AgentProtocolArgs {
    endpoint: String,
    method: String,
    params: serde_json::Value,
}

#[derive(Deserialize)]
struct JsonRpcResponse {
    result: Option<serde_json::Value>,
    error: Option<serde_json::Value>,
}

struct AgentProtocolExecutor {
    client: reqwest::Client,
}

fn is_safe_url(url_str: &str) -> bool {
    let url: Url = match Url::parse(url_str) {
        Ok(u) => u,
        Err(_) => return false,
    };

    if url.scheme() != "http" && url.scheme() != "https" {
        return false;
    }

    if let Some(host) = url.host_str() {
        if host == "localhost" || host.starts_with("127.") || host.starts_with("10.") ||
           host.starts_with("192.168.") || host.starts_with("169.254.") ||
           host.starts_with("::1") || host.starts_with("fc00:") || host.starts_with("fe80:") ||
           host == "0.0.0.0" {
               if std::env::var("MCPANY_DANGEROUS_ALLOW_LOCAL_IPS").unwrap_or_default() != "true" {
                   return false;
               }
           }
    }
    true
}

#[async_trait::async_trait]
impl PydanticToolExecutor<AgentProtocolArgs> for AgentProtocolExecutor {
    async fn execute_typed(&self, args: AgentProtocolArgs) -> Result<String, ToolError> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": "1",
            "method": args.method,
            "params": args.params
        });

        if !is_safe_url(&args.endpoint) {
            return Err(ToolError::LlmRecoverable(format!("Agent Protocol endpoint {} is invalid or points to a blocked local/private IP address (SSRF protection).", args.endpoint)));
        }

        let resp = self.client.post(&args.endpoint)
            .json(&payload)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to make request to {}: {}", args.endpoint, e)))?;

        let json_resp: JsonRpcResponse = resp.json().await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to parse JSON response: {}", e)))?;

        if let Some(err) = json_resp.error {
            return Err(ToolError::LlmRecoverable(format!("Agent Protocol returned error: {}", err)));
        }

        let result = json_resp.result.unwrap_or(json!({}));
        Ok(format!("Agent Protocol {} executed successfully. Result: {}", args.method, result))
    }
}

pub fn agent_protocol_tool() -> Tool {
    Tool {
        name: "agent_protocol".to_string(),
        description: "Interact with the standardized Agent Protocol (AutoGPT Unique Harness Innovations).".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "endpoint": {
                    "type": "string",
                    "description": "The Agent Protocol API endpoint."
                },
                "method": {
                    "type": "string",
                    "description": "The Agent Protocol method to execute."
                },
                "params": {
                    "type": "object",
                    "description": "The parameters for the Agent Protocol method."
                }
            },
            "required": ["endpoint", "method", "params"]
        }),
        execute: Arc::new(PydanticAdapter::new(AgentProtocolExecutor { client: reqwest::Client::new() })),
    }
}
