use ::server_ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
use tracing::Instrument;
use sqlx::SqlitePool;

pub struct LocalProxyServer {
    db: SqlitePool,
    sandbox_dir: String,
}

impl LocalProxyServer {
    pub fn new(db: SqlitePool, sandbox_dir: String) -> Self {
        Self { db, sandbox_dir }
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

                // Note: creating the session and proxy
                let session = ohc_builtin_agent::sandbox::session::ShellSession::new(context_id, &self.sandbox_dir)
                    .await
                    .map_err(|e| tonic::Status::internal(format!("failed to initialize shell session: {}", e)))?;

                let proxy = ohc_builtin_agent::mcp::proxy::local_proxy::LocalStatefulExecutionProxy::new(self.db.clone(), session)
                    .await
                    .map_err(|e| tonic::Status::internal(format!("failed to initialize local execution proxy: {}", e)))?;

                let cmd_str = command.to_string();
                let context_id_str = context_id.to_string();

                async move {
                    match proxy.execute_command(&cmd_str).await {
                        Ok(output) => {
                            let resp = serde_json::json!({
                                "status": "success",
                                "command": cmd_str,
                                "context_id": context_id_str,
                                "message": "command proxied successfully",
                                "output": output,
                            });
                            Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                        }
                        Err(e) => {
                            let resp = serde_json::json!({
                                "status": "error",
                                "command": cmd_str,
                                "context_id": context_id_str,
                                "message": "command failed",
                                "error": e,
                            });
                            // We still return Ok(Response) for MCP execution errors unless it's a structural failure
                            Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                        }
                    }
                }
                .instrument(tracing::info_span!("local_stateful_proxy"))
                .await
            }
            _ => Err(tonic::Status::unimplemented(format!("tool {} not implemented", req.tool_id))),
        }
    }
}
