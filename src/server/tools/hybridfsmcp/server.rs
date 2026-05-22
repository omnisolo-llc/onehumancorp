use std::sync::Arc;
use ::server_ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
use super::provider::FileSystemProvider;
use tracing::Instrument;

pub struct HybridFSMcpServer {
    provider: Arc<dyn FileSystemProvider>,
}

impl HybridFSMcpServer {
    pub fn new(provider: Arc<dyn FileSystemProvider>) -> Self {
        Self { provider }
    }

    pub fn get_tools(&self) -> Vec<McpToolProto> {
        vec![
            McpToolProto {
                id: "fs_hybrid_read".to_string(),
                name: "Read File".to_string(),
                description: "Read a file from the file system. Input schema: {\"type\":\"object\",\"properties\":{\"path\":{\"type\":\"string\"}}}".to_string(),
                category: "filesystem".to_string(),
                status: "active".to_string(),
            },
            McpToolProto {
                id: "fs_hybrid_write".to_string(),
                name: "Write File".to_string(),
                description: "Write content to a file. Input schema: {\"type\":\"object\",\"properties\":{\"path\":{\"type\":\"string\"},\"content\":{\"type\":\"string\"}}}".to_string(),
                category: "filesystem".to_string(),
                status: "active".to_string(),
            },
            McpToolProto {
                id: "fs_list_dir".to_string(),
                name: "List Directory".to_string(),
                description: "List contents of a directory. Input schema: {\"type\":\"object\",\"properties\":{\"path\":{\"type\":\"string\"}}}".to_string(),
                category: "filesystem".to_string(),
                status: "active".to_string(),
            },
            McpToolProto {
                id: "fs_search_files".to_string(),
                name: "Search Files".to_string(),
                description: "Search for files in a directory. Input schema: {\"type\":\"object\",\"properties\":{\"path\":{\"type\":\"string\"},\"query\":{\"type\":\"string\"}}}".to_string(),
                category: "filesystem".to_string(),
                status: "active".to_string(),
            },
            McpToolProto {
                id: "fs_hybrid_sync".to_string(),
                name: "Hybrid Sync File".to_string(),
                description: "Sync a file from local to cloud. Input schema: {\"type\":\"object\",\"properties\":{\"local_path\":{\"type\":\"string\"},\"cloud_path\":{\"type\":\"string\"}}}".to_string(),
                category: "filesystem".to_string(),
                status: "active".to_string(),
            },
        ]
    }

    pub async fn invoke_tool(&self, req: &McpInvokeRequest, pool: Option<sqlx::PgPool>) -> Result<McpInvokeResponse, tonic::Status> {
        let params: serde_json::Value = serde_json::from_str(&req.params)
            .map_err(|e| tonic::Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

        match req.tool_id.as_str() {
            "fs_hybrid_read" => {
                let path = params["path"].as_str().ok_or_else(|| tonic::Status::invalid_argument("path is required"))?;
                async {
                    match self.provider.read_file(path).await {
                        Ok(content) => {
                            let content_str = String::from_utf8_lossy(&content).to_string();
                            let resp = serde_json::json!({"content": content_str});
                            Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                        }
                        Err(e) => Err(tonic::Status::internal(format!("failed to read file: {}", e))),
                    }
                }
                .instrument(tracing::info_span!("fs_hybrid_read"))
                .await
            }
            "fs_hybrid_write" => {
                let path = params["path"].as_str().ok_or_else(|| tonic::Status::invalid_argument("path is required"))?;
                let content = params["content"].as_str().ok_or_else(|| tonic::Status::invalid_argument("content is required"))?;

                async {
                    match self.provider.write_file(path, content.as_bytes()).await {
                        Ok(_) => {
                            let resp = serde_json::json!({"status": "success"});
                            Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                        }
                        Err(e) => Err(tonic::Status::internal(format!("failed to write file: {}", e))),
                    }
                }
                .instrument(tracing::info_span!("fs_hybrid_write"))
                .await
            }
            "fs_list_dir" => {
                let path = params["path"].as_str().ok_or_else(|| tonic::Status::invalid_argument("path is required"))?;

                match self.provider.list_dir(path).await {
                    Ok(entries) => {
                        let resp = serde_json::json!({"entries": entries});
                        Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                    }
                    Err(e) => Err(tonic::Status::internal(format!("failed to list dir: {}", e))),
                }
            }
            "fs_search_files" => {
                let path = params["path"].as_str().ok_or_else(|| tonic::Status::invalid_argument("path is required"))?;
                let query = params["query"].as_str().ok_or_else(|| tonic::Status::invalid_argument("query is required"))?;

                match self.provider.search_files(path, query).await {
                    Ok(entries) => {
                        let resp = serde_json::json!({"entries": entries});
                        Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                    }
                    Err(e) => Err(tonic::Status::internal(format!("failed to search files: {}", e))),
                }
            }
            "fs_hybrid_sync" => {
                let local_path = params["local_path"].as_str().ok_or_else(|| tonic::Status::invalid_argument("local_path is required"))?;
                let cloud_path = params["cloud_path"].as_str().ok_or_else(|| tonic::Status::invalid_argument("cloud_path is required"))?;

                let pool = pool.ok_or_else(|| tonic::Status::internal("database pool required for sync operations"))?;
                async {
                    let id = uuid::Uuid::new_v4().to_string();
                    let spiffe_id_str = &req.spiffe_id;
                    let parsed = ::server_auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("system".to_string(), "".to_string()));
                    let mut tenant_id = parsed.0;
                    if tenant_id.is_empty() {
                        tenant_id = "system".to_string();
                    }

                    // Add to hybrid_fs_sync_queue
                    let query = "
                        INSERT INTO hybrid_fs_sync_queue (id, organization_id, local_path, cloud_path, status, created_at, updated_at)
                        VALUES ($1, $2, $3, $4, 'FILE_SYNC_PENDING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                    ";

                    sqlx::query(query)
                        .bind(&id)
                        .bind(&tenant_id)
                        .bind(local_path)
                        .bind(cloud_path)
                        .execute(&pool)
                        .await
                        .map_err(|e| tonic::Status::internal(format!("failed to enqueue file sync: {}", e)))?;

                    let resp = serde_json::json!({"status": "success", "sync_id": id, "message": "file queued for sync"});
                    Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                }
                .instrument(tracing::info_span!("fs_hybrid_sync"))
                .await
            }
            _ => Ok(McpInvokeResponse { payload: serde_json::to_string(&serde_json::json!({"status": "error", "message": format!("tool {} not implemented", req.tool_id)})).unwrap() }),
        }
    }
}
pub fn pad_chaos_1() { let _p = 1; }
