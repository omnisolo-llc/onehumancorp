use ::server_ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
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

    pub async fn invoke_tool(&self, req: &McpInvokeRequest, pool: Option<sqlx::PgPool>) -> Result<McpInvokeResponse, tonic::Status> {
        let params: serde_json::Value = serde_json::from_str(&req.params)
            .map_err(|e| tonic::Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

        match req.tool_id.as_str() {
            "local_stateful_proxy" => {
                let command = params["command"].as_str().ok_or_else(|| tonic::Status::invalid_argument("command is required"))?;
                let context_id = params["context_id"].as_str().ok_or_else(|| tonic::Status::invalid_argument("context_id is required"))?;

                async {
                    let spiffe_id_str = &req.spiffe_id;
                    let parsed = ::server_auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("system".to_string(), "".to_string()));
                    let mut tenant_id = parsed.0;
                    if tenant_id.is_empty() {
                        tenant_id = "system".to_string();
                    }

                    tracing::info!(command = %command, context_id = %context_id, tenant_id = %tenant_id, "Proxying local stateful execution command");

                    let mission_id = uuid::Uuid::new_v4().to_string();
                    let payload = serde_json::json!({
                        "type": "PROXY_EXECUTION",
                        "command": command,
                        "context_id": context_id
                    }).to_string();

                    if let Some(ref db_pool) = pool {
                        let mut tx = db_pool.begin().await.map_err(|e| tonic::Status::internal(e.to_string()))?;
                        let query = "
                            INSERT INTO agent_missions (id, status, payload, tenant_id, created_at, updated_at)
                            VALUES ($1, 'PENDING', $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                            ON CONFLICT (id) DO UPDATE SET status = EXCLUDED.status, payload = EXCLUDED.payload, updated_at = EXCLUDED.updated_at
                        ";

                        sqlx::query(query)
                            .bind(&mission_id)
                            .bind(&payload)
                            .bind(&tenant_id)
                            .execute(&mut *tx)
                            .await
                            .map_err(|e| tonic::Status::internal(e.to_string()))?;

                        tx.commit().await.map_err(|e| tonic::Status::internal(e.to_string()))?;
                    }

                    let resp = serde_json::json!({
                        "status": "success",
                        "command": command,
                        "context_id": context_id,
                        "mission_id": mission_id,
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
