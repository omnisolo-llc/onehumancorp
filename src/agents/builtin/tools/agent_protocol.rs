use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use std::sync::Once;
use std::time::Duration;
use url::Url;

use super::{
    Tool, network_policy,
    pydantic::{PydanticAdapter, PydanticToolExecutor},
};

static LEGACY_PRIVATE_NETWORK_WARNING: Once = Once::new();

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

struct AgentProtocolExecutor;

fn private_network_allowed() -> bool {
    let legacy_allowed = std::env::var("MCPANY_DANGEROUS_ALLOW_LOCAL_IPS")
        .is_ok_and(|value| value.eq_ignore_ascii_case("true"));
    if legacy_allowed {
        LEGACY_PRIVATE_NETWORK_WARNING.call_once(|| {
            tracing::warn!(
                "MCPANY_DANGEROUS_ALLOW_LOCAL_IPS is deprecated; use OHC_AGENT_ALLOW_PRIVATE_NETWORK=true"
            );
        });
    }
    network_policy::private_network_allowed() || legacy_allowed
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

        let endpoint = Url::parse(&args.endpoint).map_err(|error| {
            ToolError::LlmRecoverable(format!(
                "Agent Protocol endpoint {} is invalid: {}",
                args.endpoint, error
            ))
        })?;
        let addresses =
            network_policy::validate_and_resolve(&endpoint, private_network_allowed()).await?;
        let client = network_policy::pin_resolved_addresses(
            reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .no_proxy()
                .timeout(Duration::from_secs(10)),
            &endpoint,
            &addresses,
        )
        .build()
        .map_err(|error| {
            ToolError::Unexpected(format!("Failed to build Agent Protocol client: {error}"))
        })?;

        let resp = client
            .post(endpoint)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                ToolError::LlmRecoverable(format!(
                    "Failed to make request to {}: {}",
                    args.endpoint, e
                ))
            })?;

        let json_resp: JsonRpcResponse = resp.json().await.map_err(|e| {
            ToolError::LlmRecoverable(format!("Failed to parse JSON response: {}", e))
        })?;

        if let Some(err) = json_resp.error {
            return Err(ToolError::LlmRecoverable(format!(
                "Agent Protocol returned error: {}",
                err
            )));
        }

        let result = json_resp.result.unwrap_or(json!({}));
        Ok(format!(
            "Agent Protocol {} executed successfully. Result: {}",
            args.method, result
        ))
    }
}

pub fn agent_protocol_tool() -> Tool {
    Tool {
        name: "agent_protocol".to_string(),
        description:
            "Interact with the standardized Agent Protocol (AutoGPT Unique Harness Innovations)."
                .to_string(),
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
        execute: Arc::new(PydanticAdapter::new(AgentProtocolExecutor)),
    }
}
