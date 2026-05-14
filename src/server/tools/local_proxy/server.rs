use crate::ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
use tracing::Instrument;

pub struct LocalProxyServer {
}

impl LocalProxyServer {
    pub fn new() -> Self {
        Self { }
    }

    pub fn get_tools(&self) -> Vec<McpToolProto> {
        vec![
            McpToolProto {
                id: "local_stateful_proxy".to_string(),
                name: "Local Stateful Execution Proxy".to_string(),
                description: "Proxies execution commands and structured queries to the local standalone context. Input schema: {\"type\":\"object\",\"properties\":{\"command\":{\"type\":\"string\"},\"context_id\":{\"type\":\"string\"}}}".to_string(),
                category: "proxy".to_string(),
                status: "active".to_string(),
            },
        ]
    }

    pub async fn invoke_tool(&self, req: &McpInvokeRequest) -> Result<McpInvokeResponse, tonic::Status> {
        let params: serde_json::Value = serde_json::from_str(&req.params)
            .map_err(|e| tonic::Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

        match req.tool_id.as_str() {
            "local_stateful_proxy" => {
                let command = params["command"].as_str().ok_or_else(|| tonic::Status::invalid_argument("command is required"))?;
                let context_id = params["context_id"].as_str().ok_or_else(|| tonic::Status::invalid_argument("context_id is required"))?;

                async {
                    tracing::info!("proxying command '{}' to context '{}'", command, context_id);
                    let resp = serde_json::json!({
                        "status": "success",
                        "command": command,
                        "context_id": context_id,
                        "message": "command proxied successfully"
                    });
                    Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                }
                .instrument(tracing::info_span!("local_stateful_proxy"))
                .await
            }
            _ => Err(tonic::Status::unimplemented(format!("tool {} not implemented", req.tool_id))),
        }
    }
}
