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
    allow_sensitive_upload: Option<bool>,
    client_updated_at: Option<i64>,
    force_overwrite: Option<bool>,
}

impl ConfigSyncServer {
    pub fn new(db: sqlx::PgPool) -> Self {
        Self { db }
    }

    fn validate_and_scrub(val: &mut serde_json::Value, allow_sensitive: bool, depth: usize) -> Result<(), tonic::Status> {
        if depth > 10 {
            return Err(tonic::Status::invalid_argument("JSON payload exceeds maximum depth of 10"));
        }
        match val {
            serde_json::Value::Object(map) => {
                let mut keys_to_remove = Vec::new();
                for (k, v) in map.iter_mut() {
                    let k_lower = k.to_lowercase();
                    if k_lower.contains("password") || k_lower.contains("secret") || k_lower.contains("local_proxy") {
                        if !allow_sensitive {
                            keys_to_remove.push(k.clone());
                        } else if let Some(pwd) = v.as_str() {
                            *v = serde_json::Value::String(crate::crypto::encrypt_deterministic(pwd));
                        }
                    } else {
                        Self::validate_and_scrub(v, allow_sensitive, depth + 1)?;
                    }
                }
                for k in keys_to_remove {
                    map.remove(&k);
                }
            }
            serde_json::Value::Array(arr) => {
                for v in arr.iter_mut() {
                    Self::validate_and_scrub(v, allow_sensitive, depth + 1)?;
                }
            }
            serde_json::Value::String(s) => {
                if s.len() > 1024 * 100 { // Max string length 100KB
                    return Err(tonic::Status::invalid_argument("String value exceeds maximum length"));
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn decrypt_secrets(val: &mut serde_json::Value, depth: usize) {
        if depth > 10 {
            return;
        }
        match val {
            serde_json::Value::Object(map) => {
                for (k, v) in map.iter_mut() {
                    let k_lower = k.to_lowercase();
                    if k_lower.contains("password") || k_lower.contains("secret") || k_lower.contains("local_proxy") {
                        if let Some(pwd) = v.as_str() {
                            *v = serde_json::Value::String(crate::crypto::decrypt_deterministic(pwd));
                        }
                    } else {
                        Self::decrypt_secrets(v, depth + 1);
                    }
                }
            }
            serde_json::Value::Array(arr) => {
                for v in arr.iter_mut() {
                    Self::decrypt_secrets(v, depth + 1);
                }
            }
            _ => {}
        }
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
            "get_config" => {
                let row = sqlx::query("SELECT config_json, updated_at FROM user_configs WHERE spiffe_id = $1")
                    .bind(&req.spiffe_id)
                    .fetch_optional(&self.db)
                    .await
                    .map_err(|e| tonic::Status::internal(format!("DB error: {}", e)))?;

                if let Some(r) = row {
                    let config_str: String = r.get("config_json");
                    let updated_at: chrono::NaiveDateTime = r.get("updated_at");
                    let mut payload: serde_json::Value = serde_json::from_str(&config_str)
                        .map_err(|_| tonic::Status::internal("Invalid JSON in database"))?;

                    Self::decrypt_secrets(&mut payload, 0);

                    let resp_payload = serde_json::to_string(&serde_json::json!({
                        "status": "success",
                        "config": payload,
                        "updated_at": updated_at.and_utc().timestamp(),
                    })).unwrap();
                    Ok(McpInvokeResponse { payload: resp_payload })
                } else {
                    let resp_payload = serde_json::to_string(&serde_json::json!({
                        "status": "not_found",
                    })).unwrap();
                    Ok(McpInvokeResponse { payload: resp_payload })
                }
            }
            "push_config" => {
                let mut payload = params.payload.ok_or_else(|| tonic::Status::invalid_argument("Missing payload"))?;

                let max_size: usize = std::env::var("MAX_CONFIG_SIZE")
                    .unwrap_or_else(|_| (1024 * 1024).to_string())
                    .parse()
                    .unwrap_or(1024 * 1024);

                let force_overwrite = params.force_overwrite.unwrap_or(false);

                // State resolution logic
                if !force_overwrite {
                    if let Some(client_time) = params.client_updated_at {
                        let existing_row = sqlx::query("SELECT updated_at FROM user_configs WHERE spiffe_id = $1")
                            .bind(&req.spiffe_id)
                            .fetch_optional(&self.db)
                            .await
                            .map_err(|e| tonic::Status::internal(format!("DB error: {}", e)))?;

                        if let Some(r) = existing_row {
                            let server_updated_at: chrono::NaiveDateTime = r.get("updated_at");
                            if server_updated_at.and_utc().timestamp() > client_time {
                                let resp_payload = serde_json::to_string(&serde_json::json!({
                                    "status": "conflict",
                                    "message": "Server configuration is newer than client configuration",
                                })).unwrap();
                                return Ok(McpInvokeResponse { payload: resp_payload });
                            }
                        }
                    }
                }

                let allow_sensitive = params.allow_sensitive_upload.unwrap_or(false);

                Self::validate_and_scrub(&mut payload, allow_sensitive, 0)?;

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
