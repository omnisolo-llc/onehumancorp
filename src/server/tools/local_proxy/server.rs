use ::server_ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
use tracing::Instrument;
use std::process::Command;
use sqlx::PgPool;

pub struct LocalProxyServer {
    pub pool: Option<PgPool>,
}

impl LocalProxyServer {
    pub fn new(pool: Option<PgPool>) -> Self {
        Self { pool }
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
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg(command)
                        .output();

                    let (status, stdout, stderr) = match output {
                        Ok(out) => {
                            (
                                out.status.success(),
                                String::from_utf8_lossy(&out.stdout).to_string(),
                                String::from_utf8_lossy(&out.stderr).to_string(),
                            )
                        }
                        Err(e) => (false, "".to_string(), e.to_string()),
                    };

                    if let Some(pool) = &self.pool {
                        let _ = sqlx::query(
                            "INSERT INTO local_execution_logs (context_id, command, stdout, stderr, success, created_at) VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)"
                        )
                        .bind(context_id)
                        .bind(command)
                        .bind(&stdout)
                        .bind(&stderr)
                        .bind(status)
                        .execute(pool)
                        .await;
                    }

                    let resp = serde_json::json!({
                        "status": if status { "success" } else { "error" },
                        "command": command,
                        "context_id": context_id,
                        "stdout": stdout,
                        "stderr": stderr,
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
