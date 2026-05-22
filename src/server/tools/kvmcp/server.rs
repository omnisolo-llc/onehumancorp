use std::sync::Arc;
use crate::ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
use crate::db::{DB, DbStore};
use redis::AsyncCommands;
use tracing::Instrument;

pub struct KvMcpServer {
    db: Arc<DB>,
    redis_client: Option<redis::Client>,
    redis_conn: Option<tokio::sync::OnceCell<redis::aio::MultiplexedConnection>>,
}

impl KvMcpServer {
    pub fn new(db: Arc<DB>, redis_client: Option<redis::Client>) -> Self {
        let redis_conn = redis_client.is_some().then(|| tokio::sync::OnceCell::new());
        Self { db, redis_client, redis_conn }
    }

    async fn get_redis_conn(&self) -> Result<redis::aio::MultiplexedConnection, tonic::Status> {
        if let Some(client) = &self.redis_client {
            if let Some(cell) = &self.redis_conn {
                cell.get_or_try_init(|| async {
                    client.get_multiplexed_async_connection().await
                })
                .await
                .map(|c| c.clone())
                .map_err(|e| tonic::Status::internal(format!("failed to connect to redis: {}", e)))
            } else {
                Err(tonic::Status::internal("redis client configured but no connection cell"))
            }
        } else {
            Err(tonic::Status::internal("redis client not configured for cloud mode"))
        }
    }

    pub fn get_tools(&self) -> Vec<McpToolProto> {
        vec![
            McpToolProto {
                id: "kv_get".to_string(),
                name: "KV Get".to_string(),
                description: "Get a value from the KV store. Input schema: {\"type\":\"object\",\"properties\":{\"key\":{\"type\":\"string\"}}}".to_string(),
                category: "kv".to_string(),
                status: "active".to_string(),
            },
            McpToolProto {
                id: "kv_set".to_string(),
                name: "KV Set".to_string(),
                description: "Set a value in the KV store. Input schema: {\"type\":\"object\",\"properties\":{\"key\":{\"type\":\"string\"},\"value\":{\"type\":\"string\"}}}".to_string(),
                category: "kv".to_string(),
                status: "active".to_string(),
            },
            McpToolProto {
                id: "kv_delete".to_string(),
                name: "KV Delete".to_string(),
                description: "Delete a value from the KV store. Input schema: {\"type\":\"object\",\"properties\":{\"key\":{\"type\":\"string\"}}}".to_string(),
                category: "kv".to_string(),
                status: "active".to_string(),
            },
            McpToolProto {
                id: "kv_list".to_string(),
                name: "KV List".to_string(),
                description: "List keys in the KV store with an optional prefix. Input schema: {\"type\":\"object\",\"properties\":{\"prefix\":{\"type\":\"string\"}}}".to_string(),
                category: "kv".to_string(),
                status: "active".to_string(),
            },
        ]
    }

    pub fn get_tenant_id(&self, spiffe_id_str: &str) -> Result<String, tonic::Status> {
        let parsed = crate::auth::parse_spiffe_id(spiffe_id_str)
            .map_err(|_| tonic::Status::unauthenticated("invalid SPIFFE ID"))?;
        let tenant_id = parsed.0;
        if tenant_id.is_empty() {
            return Err(tonic::Status::unauthenticated("empty tenant ID in SPIFFE ID"));
        }
        Ok(tenant_id)
    }

    fn is_standalone(&self) -> bool {
        std::env::var("OHC_STANDALONE").unwrap_or_else(|_| "false".to_string()) == "true" || self.redis_client.is_none()
    }

    pub async fn invoke_tool(&self, req: &McpInvokeRequest) -> Result<McpInvokeResponse, tonic::Status> {
        let params: serde_json::Value = serde_json::from_str(&req.params)
            .map_err(|e| tonic::Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

        let tenant_id = self.get_tenant_id(&req.spiffe_id)?;

        match req.tool_id.as_str() {
            "kv_get" => {
                let key = params["key"].as_str().ok_or_else(|| tonic::Status::invalid_argument("key is required"))?;
                async {
                    if self.is_standalone() {
                        let row: Result<(String,), sqlx::Error> = match &self.db.store {
                            DbStore::Sqlite(pool) => {
                                sqlx::query_as("SELECT kv_value FROM agent_kv_store WHERE tenant_id = ? AND kv_key = ?")
                                    .bind(&tenant_id)
                                    .bind(key)
                                    .fetch_one(pool)
                                    .await
                            }
                            DbStore::Postgres => {
                                sqlx::query_as("SELECT kv_value FROM agent_kv_store WHERE tenant_id = $1 AND kv_key = $2")
                                    .bind(&tenant_id)
                                    .bind(key)
                                    .fetch_one(&self.db.pool)
                                    .await
                            }
                        };

                        match row {
                            Ok((val,)) => {
                                let resp = serde_json::json!({"value": val});
                                Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                            }
                            Err(sqlx::Error::RowNotFound) => {
                                let resp = serde_json::json!({"value": null});
                                Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                            }
                            Err(e) => Err(tonic::Status::internal(format!("db error: {}", e))),
                        }
                    } else {
                        let mut conn = self.get_redis_conn().await?;
                        let redis_key = format!("tenant:{}:kv:{}", tenant_id, key);
                        let val: Option<String> = conn.get(&redis_key).await.map_err(|e| tonic::Status::internal(format!("redis error: {}", e)))?;
                        let resp = serde_json::json!({"value": val});
                        Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                    }
                }.instrument(tracing::info_span!("kv_get")).await
            }
            "kv_set" => {
                let key = params["key"].as_str().ok_or_else(|| tonic::Status::invalid_argument("key is required"))?;
                let value = params["value"].as_str().ok_or_else(|| tonic::Status::invalid_argument("value is required"))?;
                async {
                    if self.is_standalone() {
                        match &self.db.store {
                            DbStore::Sqlite(pool) => {
                                sqlx::query("INSERT INTO agent_kv_store (tenant_id, kv_key, kv_value, updated_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP) ON CONFLICT(tenant_id, kv_key) DO UPDATE SET kv_value = excluded.kv_value, updated_at = CURRENT_TIMESTAMP")
                                    .bind(&tenant_id)
                                    .bind(key)
                                    .bind(value)
                                    .execute(pool)
                                    .await
                                    .map_err(|e| tonic::Status::internal(format!("db error: {}", e)))?;
                            }
                            DbStore::Postgres => {
                                sqlx::query("INSERT INTO agent_kv_store (tenant_id, kv_key, kv_value, updated_at) VALUES ($1, $2, $3, CURRENT_TIMESTAMP) ON CONFLICT (tenant_id, kv_key) DO UPDATE SET kv_value = EXCLUDED.kv_value, updated_at = CURRENT_TIMESTAMP")
                                    .bind(&tenant_id)
                                    .bind(key)
                                    .bind(value)
                                    .execute(&self.db.pool)
                                    .await
                                    .map_err(|e| tonic::Status::internal(format!("db error: {}", e)))?;
                            }
                        }
                        let resp = serde_json::json!({"status": "success"});
                        Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                    } else {
                        let mut conn = self.get_redis_conn().await?;
                        let redis_key = format!("tenant:{}:kv:{}", tenant_id, key);
                        let _: () = conn.set(&redis_key, value).await.map_err(|e| tonic::Status::internal(format!("redis error: {}", e)))?;
                        let resp = serde_json::json!({"status": "success"});
                        Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                    }
                }.instrument(tracing::info_span!("kv_set")).await
            }
            "kv_delete" => {
                let key = params["key"].as_str().ok_or_else(|| tonic::Status::invalid_argument("key is required"))?;
                async {
                    if self.is_standalone() {
                        match &self.db.store {
                            DbStore::Sqlite(pool) => {
                                sqlx::query("DELETE FROM agent_kv_store WHERE tenant_id = ? AND kv_key = ?")
                                    .bind(&tenant_id)
                                    .bind(key)
                                    .execute(pool)
                                    .await
                                    .map_err(|e| tonic::Status::internal(format!("db error: {}", e)))?;
                            }
                            DbStore::Postgres => {
                                sqlx::query("DELETE FROM agent_kv_store WHERE tenant_id = $1 AND kv_key = $2")
                                    .bind(&tenant_id)
                                    .bind(key)
                                    .execute(&self.db.pool)
                                    .await
                                    .map_err(|e| tonic::Status::internal(format!("db error: {}", e)))?;
                            }
                        }
                        let resp = serde_json::json!({"status": "success"});
                        Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                    } else {
                        let mut conn = self.get_redis_conn().await?;
                        let redis_key = format!("tenant:{}:kv:{}", tenant_id, key);
                        let _: () = conn.del(&redis_key).await.map_err(|e| tonic::Status::internal(format!("redis error: {}", e)))?;
                        let resp = serde_json::json!({"status": "success"});
                        Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                    }
                }.instrument(tracing::info_span!("kv_delete")).await
            }
            "kv_list" => {
                let prefix = params["prefix"].as_str().unwrap_or("");
                async {
                    if self.is_standalone() {
                        let rows: Result<Vec<(String,)>, sqlx::Error> = match &self.db.store {
                            DbStore::Sqlite(pool) => {
                                let like_pattern = format!("{}%", prefix);
                                sqlx::query_as("SELECT kv_key FROM agent_kv_store WHERE tenant_id = ? AND kv_key LIKE ?")
                                    .bind(&tenant_id)
                                    .bind(&like_pattern)
                                    .fetch_all(pool)
                                    .await
                            }
                            DbStore::Postgres => {
                                let like_pattern = format!("{}%", prefix);
                                sqlx::query_as("SELECT kv_key FROM agent_kv_store WHERE tenant_id = $1 AND kv_key LIKE $2")
                                    .bind(&tenant_id)
                                    .bind(&like_pattern)
                                    .fetch_all(&self.db.pool)
                                    .await
                            }
                        };

                        match rows {
                            Ok(results) => {
                                let keys: Vec<String> = results.into_iter().map(|r| r.0).collect();
                                let resp = serde_json::json!({"keys": keys});
                                Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                            }
                            Err(e) => Err(tonic::Status::internal(format!("db error: {}", e))),
                        }
                    } else {
                        let mut conn = self.get_redis_conn().await?;
                        let redis_pattern = format!("tenant:{}:kv:{}*", tenant_id, prefix);
                        let keys: Vec<String> = conn.keys(&redis_pattern).await.map_err(|e| tonic::Status::internal(format!("redis error: {}", e)))?;

                        // strip prefix
                        let prefix_len = format!("tenant:{}:kv:", tenant_id).len();
                        let clean_keys: Vec<String> = keys.into_iter().map(|k| k[prefix_len..].to_string()).collect();

                        let resp = serde_json::json!({"keys": clean_keys});
                        Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                    }
                }.instrument(tracing::info_span!("kv_list")).await
            }
            _ => Ok(McpInvokeResponse { payload: serde_json::to_string(&serde_json::json!({"status": "error", "message": format!("tool {} not implemented", req.tool_id)})).unwrap() }),
        }
    }
}
