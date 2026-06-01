use crate::ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
use sha2::{Sha256, Digest};
use serde::Deserialize;
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

    pub async fn invoke_tool(&self, req: &McpInvokeRequest) -> Result<McpInvokeResponse, tonic::Status> {
        if req.tool_id != "mcp_config_sync" {
            return Err(tonic::Status::invalid_argument("Invalid tool_id"));
        }

        if req.spiffe_id.is_empty() {
            return Err(tonic::Status::unauthenticated("Missing SPIFFE ID"));
        }

        let params: SyncParams = serde_json::from_str(&req.params)
            .map_err(|_| tonic::Status::invalid_argument("Invalid params json"))?;

        match params.action.as_str() {
            "get_hash" => {
                let row = sqlx::query("SELECT hash FROM user_configs WHERE spiffe_id = $1")
                    .bind(&req.spiffe_id)
                    .fetch_optional(&self.db)
                    .await
                    .map_err(|e| tonic::Status::internal(format!("DB error: {}", e)))?;

                let hash: String = row.map(|r| r.get("hash")).unwrap_or_else(|| "".to_string());

                let resp_payload = serde_json::to_string(&serde_json::json!({
                    "status": "success",
                    "hash": hash,
                })).unwrap();
                Ok(McpInvokeResponse { payload: resp_payload })
            }
            "push_config" => {
                let mut payload = params.payload.ok_or_else(|| tonic::Status::invalid_argument("Missing payload"))?;

                let max_size: usize = std::env::var("MAX_CONFIG_SIZE")
                    .unwrap_or_else(|_| (1024 * 1024).to_string())
                    .parse()
                    .unwrap_or(1024 * 1024);

                if let serde_json::Value::Object(ref mut map) = payload {
                    if let Some(pwd) = map.get("local_proxy_password").and_then(|v| v.as_str()) {
                        let encrypted = crate::crypto::encrypt_deterministic(pwd);
                        map.insert("local_proxy_password".to_string(), serde_json::Value::String(encrypted));
                    }
                }

                let config_str = serde_json::to_string(&payload).unwrap();
                if config_str.len() > max_size {
                    return Err(tonic::Status::invalid_argument("Config payload too large"));
                }

                let mut hasher = Sha256::new();
                hasher.update(config_str.as_bytes());
                let hash = format!("{:x}", hasher.finalize());

                let now = chrono::Utc::now().naive_utc();

                sqlx::query(
                    r#"
                    INSERT INTO user_configs (spiffe_id, config_json, updated_at, hash)
                    VALUES ($1, $2, $3, $4)
                    ON CONFLICT (spiffe_id) DO UPDATE SET
                        config_json = EXCLUDED.config_json,
                        updated_at = EXCLUDED.updated_at,
                        hash = EXCLUDED.hash
                    "#
                )
                .bind(&req.spiffe_id)
                .bind(&config_str)
                .bind(now)
                .bind(&hash)
                .execute(&self.db)
                .await
                .map_err(|e| tonic::Status::internal(format!("DB error: {}", e)))?;

                let resp_payload = serde_json::to_string(&serde_json::json!({
                    "status": "success",
                    "merged": true,
                })).unwrap();
                Ok(McpInvokeResponse { payload: resp_payload })
            }
            _ => Err(tonic::Status::invalid_argument("Invalid action")),
        }
    }
}
