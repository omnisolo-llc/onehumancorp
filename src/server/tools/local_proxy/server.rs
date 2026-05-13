use ::server_ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
use tracing::Instrument;
use crate::harness::sandbox::manager::{SandboxManager, SandboxAdapter};
use std::sync::Arc;
use tokio::process::Command;

pub struct LocalProxyServer {
    sandbox: Arc<SandboxManager>,
}

impl LocalProxyServer {
    pub fn new() -> Self {
        Self { sandbox: Arc::new(SandboxManager::new(None)) }
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

                let sandbox = self.sandbox.clone();
                async move {
                    let wrapped_cmd = sandbox.wrap_command(command).await.map_err(|e| tonic::Status::permission_denied(e))?;

                    let output = Command::new("bash")
                        .arg("-c")
                        .arg(&wrapped_cmd)
                        .output()
                        .await
                        .map_err(|e| tonic::Status::internal(format!("failed to execute command: {}", e)))?;

                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                    let status_str = if output.status.success() { "success" } else { "error" };

                    let resp = serde_json::json!({
                        "status": status_str,
                        "command": command,
                        "context_id": context_id,
                        "stdout": stdout,
                        "stderr": stderr,
                        "message": if output.status.success() { "command proxied successfully" } else { "command failed" }
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
