use crate::ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use sqlx::Row;

pub struct ConfigSyncServer {
    db: sqlx::PgPool,
}

#[derive(Deserialize)]
struct SyncParams {
    action: String,
    payload: Option<serde_json::Value>,
}

impl ConfigSyncServer {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self { db }
    }

    pub fn get_tools(&self) -> Vec<McpToolProto> {
        vec![McpToolProto {
            id: "mcp_config_sync".to_string(),
            name: "Config Sync".to_string(),
            description: "Synchronize local standalone configuration to cloud profile.".to_string(),
            category: "integration".to_string(),
            status: "active".to_string(),
        }]
    }

    pub async fn invoke_tool(
        &self,
        req: &McpInvokeRequest,
    ) -> Result<McpInvokeResponse, tonic::Status> {
        if req.tool_id != "mcp_config_sync" {
            return Err(tonic::Status::invalid_argument("Invalid tool_id"));
        }

        let parsed = ::server_auth::parse_spiffe_id(&req.spiffe_id)
            .map_err(|_| tonic::Status::unauthenticated("invalid spiffe id"))?;
        let tenant_id = parsed.0;
        let agent_id = req.agent_id.clone();
        if tenant_id.is_empty() {
            return Err(tonic::Status::unauthenticated(
                "empty tenant ID in SPIFFE ID",
            ));
        }
        if agent_id.is_empty() {
            return Err(tonic::Status::unauthenticated("empty agent ID in request"));
        }

        let params: SyncParams = serde_json::from_str(&req.params)
            .map_err(|_| tonic::Status::invalid_argument("Invalid params json"))?;

        match params.action.as_str() {
            "get_hash" => {
                let row = sqlx::query(
                    r#"
                    SELECT metadata->>'hash' as hash
                    FROM mcp_config_sync_log
                    WHERE tenant_id = $1 AND config_key = 'standalone_config'
                    "#,
                )
                .bind(&tenant_id)
                .fetch_optional(&self.db)
                .await
                .map_err(|e| tonic::Status::internal(format!("DB error: {}", e)))?;

                let hash: String = row
                    .and_then(|r| r.try_get::<Option<String>, _>("hash").ok().flatten())
                    .unwrap_or_else(|| "".to_string());

                let resp_payload = serde_json::to_string(&serde_json::json!({
                    "status": "success",
                    "hash": hash,
                }))
                .unwrap_or_else(|_| "".to_string());
                Ok(McpInvokeResponse {
                    payload: resp_payload,
                })
            }
            "push_config" => {
                let mut payload = params
                    .payload
                    .ok_or_else(|| tonic::Status::invalid_argument("Missing payload"))?;

                let max_size: usize = std::env::var("MAX_CONFIG_SIZE")
                    .unwrap_or_else(|_| (1024 * 1024).to_string())
                    .parse()
                    .unwrap_or(1024 * 1024);

                if let serde_json::Value::Object(ref mut map) = payload {
                    if let Some(pwd) = map.get("local_proxy_password").and_then(|v| v.as_str()) {
                        let encrypted = crate::crypto::encrypt_deterministic(pwd);
                        map.insert(
                            "local_proxy_password".to_string(),
                            serde_json::Value::String(encrypted),
                        );
                    }
                }

                let config_str = serde_json::to_string(&payload).unwrap_or_else(|_| "".to_string());
                if crate::is_standalone_runtime() && !::server_config::is_telemetry_enabled() {
                    return Err(tonic::Status::failed_precondition(
                        "Telemetry disabled in standalone mode. Sync is forbidden for local sovereignty.",
                    ));
                }

                if config_str.len() > max_size {
                    return Err(tonic::Status::invalid_argument("Config payload too large"));
                }

                let mut hasher = Sha256::new();
                hasher.update(config_str.as_bytes());
                let hash = format!("{:x}", hasher.finalize());

                let metadata = serde_json::json!({
                    "hash": hash
                });

                let now = chrono::Utc::now().naive_utc();

                sqlx::query(
                    r#"
                    INSERT INTO mcp_config_sync_log (tenant_id, agent_id, config_key, config_value, metadata, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    ON CONFLICT (tenant_id, config_key) DO UPDATE SET
                        agent_id = EXCLUDED.agent_id,
                        config_value = EXCLUDED.config_value,
                        metadata = EXCLUDED.metadata,
                        updated_at = EXCLUDED.updated_at
                    "#
                )
                .bind(&tenant_id)
                .bind(&agent_id)
                .bind("standalone_config")
                .bind(&config_str)
                .bind(metadata)
                .bind(now)
                .execute(&self.db)
                .await
                .map_err(|e| tonic::Status::internal(format!("DB error: {}", e)))?;

                let resp_payload = serde_json::to_string(&serde_json::json!({
                    "status": "success",
                    "merged": true,
                }))
                .unwrap_or_else(|_| "".to_string());
                Ok(McpInvokeResponse {
                    payload: resp_payload,
                })
            }
            _ => Err(tonic::Status::invalid_argument("Invalid action")),
        }
    }
}
