use ::server_ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
use tracing::Instrument;

pub struct CrdtSyncMcpServer {}

impl CrdtSyncMcpServer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_tools(&self) -> Vec<McpToolProto> {
        vec![
            McpToolProto {
                id: "crdt_pull".to_string(),
                name: "CRDT Pull".to_string(),
                description: "Pull CRDT state. Input schema: {\"type\":\"object\",\"properties\":{\"entity_id\":{\"type\":\"string\"}}}".to_string(),
                category: "sync".to_string(),
                status: "active".to_string(),
            },
            McpToolProto {
                id: "crdt_push".to_string(),
                name: "CRDT Push".to_string(),
                description: "Push CRDT state deltas. Input schema: {\"type\":\"object\",\"properties\":{\"deltas\":{\"type\":\"array\",\"items\":{\"type\":\"object\"}}}}".to_string(),
                category: "sync".to_string(),
                status: "active".to_string(),
            },
        ]
    }

    pub async fn invoke_tool(&self, req: &McpInvokeRequest, pool: Option<sqlx::PgPool>, _sqlite_pool: Option<sqlx::SqlitePool>) -> Result<McpInvokeResponse, tonic::Status> {
        let params: serde_json::Value = serde_json::from_str(&req.params)
            .map_err(|e| tonic::Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

        match req.tool_id.as_str() {
            "crdt_push" => {
                let deltas = params.get("deltas").and_then(|v| v.as_array()).ok_or_else(|| tonic::Status::invalid_argument("deltas array is required"))?;

                let is_standalone = std::env::var("OHC_STANDALONE").unwrap_or_default() == "true";
                if is_standalone {
                    if let Ok(sqlite) = crate::db::get_pool_from_env().await.into_sqlite() {
                        for delta in deltas {
                            let id = delta.get("id").and_then(|v| v.as_str()).unwrap_or("");
                            let entity_id = delta.get("entity_id").and_then(|v| v.as_str()).unwrap_or("");
                            let data = delta.get("data").and_then(|v| v.as_str()).unwrap_or("");
                            let updated_at = delta.get("updated_at").and_then(|v| v.as_str()).unwrap_or("");

                            async {
                                sqlx::query("INSERT INTO crdt_deltas (id, entity_id, data, updated_at) VALUES ($1, $2, $3, $4) ON CONFLICT(id) DO UPDATE SET data = excluded.data, updated_at = excluded.updated_at")
                                    .bind(id)
                                    .bind(entity_id)
                                    .bind(data)
                                    .bind(updated_at)
                                    .execute(&sqlite)
                                    .await
                                    .map_err(|e| tonic::Status::internal(e.to_string()))
                            }.instrument(tracing::info_span!("sqlite_crdt_push")).await?;
                        }
                    } else {
                        return Err(tonic::Status::internal("sqlite pool required for standalone push"));
                    }
                } else {
                    if let Some(pg) = pool {
                        for delta in deltas {
                            let id = delta.get("id").and_then(|v| v.as_str()).unwrap_or("");
                            let entity_id = delta.get("entity_id").and_then(|v| v.as_str()).unwrap_or("");
                            let data = delta.get("data").and_then(|v| v.as_str()).unwrap_or("");
                            let updated_at = delta.get("updated_at").and_then(|v| v.as_str()).unwrap_or("");

                            async {
                                sqlx::query("INSERT INTO crdt_deltas (id, entity_id, data, updated_at) VALUES ($1, $2, $3, $4) ON CONFLICT(id) DO UPDATE SET data = EXCLUDED.data, updated_at = EXCLUDED.updated_at WHERE crdt_deltas.updated_at < EXCLUDED.updated_at")
                                    .bind(id)
                                    .bind(entity_id)
                                    .bind(data)
                                    .bind(updated_at)
                                    .execute(&pg)
                                    .await
                                    .map_err(|e| tonic::Status::internal(e.to_string()))
                            }.instrument(tracing::info_span!("pg_crdt_push")).await?;
                        }
                    } else {
                        return Err(tonic::Status::internal("pg pool required for cloud push"));
                    }
                }

                let resp = serde_json::json!({"status": "success"});
                Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
            }
            "crdt_pull" => {
                let resp = serde_json::json!({"status": "success", "data": "[]"});
                Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
            }
            _ => Err(tonic::Status::unimplemented("Not implemented")),
        }
    }
}
