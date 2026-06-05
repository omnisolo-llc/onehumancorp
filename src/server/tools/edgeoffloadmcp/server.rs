use ::server_ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
use tracing::Instrument;

pub struct EdgeOffloadMcpServer {
}

impl EdgeOffloadMcpServer {
    pub fn new() -> Self {
        Self { }
    }

    pub fn get_tools(&self) -> Vec<McpToolProto> {
        vec![
            McpToolProto {
                id: "mcp_inference_router".to_string(),
                name: "MCP Inference Router".to_string(),
                description: "Dynamically routes LLM inference requests between the local edge device and the cloud based on task complexity and privacy requirements.".to_string(),
                category: "inference".to_string(),
                status: "active".to_string(),
            },
        ]
    }

    pub async fn invoke_tool(&self, req: &McpInvokeRequest) -> Result<McpInvokeResponse, tonic::Status> {
        let params: serde_json::Value = serde_json::from_str(&req.params)
            .map_err(|e| tonic::Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

        match req.tool_id.as_str() {
            "mcp_inference_router" => {
                let prompt = params["prompt"].as_str().ok_or_else(|| tonic::Status::invalid_argument("prompt is required"))?;
                let is_sensitive = params["is_sensitive"].as_bool().unwrap_or(false);
                let complexity = params["complexity"].as_str().unwrap_or("low");

                async {
                    let spiffe_id_str = &req.spiffe_id;
                    let (tenant_id, _) = ::server_auth::parse_spiffe_id(spiffe_id_str).unwrap_or_else(|_| if spiffe_id_str.starts_with("spiffe://") { ("tenant".to_string(), "".to_string()) } else { ("system".to_string(), "".to_string()) });

                    let route = if is_sensitive || complexity == "low" {
                        "local"
                    } else {
                        if tenant_id == "system" || tenant_id == "" {
                             "local" // fallback to local if auth invalid
                        } else {
                            // simulate check cloud load / auth
                            // for now just "cloud"
                            // However, we should also test fallback, maybe we can inject a parameter?
                            let force_fallback = params["force_fallback"].as_bool().unwrap_or(false);
                            if force_fallback {
                                "local"
                            } else {
                                "cloud"
                            }
                        }
                    };

                    let response_text = if route == "local" {
                        format!("Local Response to: {}", prompt)
                    } else {
                        format!("Cloud Assisted Response to: {}", prompt)
                    };

                    let resp = serde_json::json!({
                        "status": "success",
                        "route": route,
                        "response": response_text
                    });
                    Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                }
                .instrument(tracing::info_span!("mcp_inference_router"))
                .await
            }
            _ => Err(tonic::Status::not_found(format!("tool {} not found", req.tool_id))),
        }
    }
}
