use ::server_ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
use tracing::Instrument;

pub struct VectorSyncMcpServer {}

impl VectorSyncMcpServer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_tools(&self) -> Vec<McpToolProto> {
        vec![
            McpToolProto {
                id: "mcp_vector_sync".to_string(),
                name: "Vector Embeddings Sync".to_string(),
                description: "Sync vector embeddings from local standalone to cloud pgvector. Input schema: {\"type\":\"object\",\"properties\":{\"embeddings\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"string\"},\"vector\":{\"type\":\"array\",\"items\":{\"type\":\"number\"}},\"metadata\":{\"type\":\"object\"}}}}}}".to_string(),
                category: "synchronization".to_string(),
                status: "active".to_string(),
            },
        ]
    }

    pub async fn push_to_cloud(&self, embeddings: Vec<serde_json::Value>, cloud_pool: &sqlx::PgPool, tenant_id: &str) -> Result<usize, tonic::Status> {
        let mut synced_count = 0;
        let mut errors = Vec::new();

        for embedding_obj in embeddings {
            let id = embedding_obj["id"].as_str().unwrap_or_default();
            if id.is_empty() { continue; }

            let vector_arr = embedding_obj["vector"].as_array();
            if vector_arr.is_none() { continue; }

            let mut vec_f32 = Vec::new();
            for v in vector_arr.unwrap() {
                if let Some(f) = v.as_f64() {
                    vec_f32.push(f as f32);
                }
            }

            if vec_f32.len() != 1536 {
                // Only accepting 1536 dim vectors for now
                continue;
            }

            // We use sqlx pgvector wrapper syntax if pgvector library is not in dependencies
            let metadata = embedding_obj["metadata"].as_object();

            let content = if let Some(meta) = metadata {
                meta.get("content").and_then(|c| c.as_str()).unwrap_or("").to_string()
            } else {
                "".to_string()
            };

            let source_type = if let Some(meta) = metadata {
                meta.get("source_type").and_then(|c| c.as_str()).unwrap_or("local_sync").to_string()
            } else {
                "local_sync".to_string()
            };

            let vector_str = format!("[{}]", vec_f32.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));

            // We need uuid to insert into vector_embeddings
            let uuid_id = match uuid::Uuid::parse_str(id) {
                Ok(u) => u,
                Err(_) => uuid::Uuid::new_v4(), // generate new if not valid uuid
            };

            let mut tx = cloud_pool.begin().await.map_err(|e| tonic::Status::internal(format!("Failed to begin transaction: {}", e)))?;

            // Set RLS scope
            sqlx::query(&format!("SET LOCAL app.current_tenant = '{}'", tenant_id))
                .execute(&mut *tx)
                .await
                .map_err(|e| tonic::Status::internal(format!("Failed to set RLS scope: {}", e)))?;

            // Push to Cloud API with ID and tenant_id check
            // Only update if tenant_id matches
            let query = "
                INSERT INTO vector_embeddings (id, tenant_id, content, embedding, source_type, last_synced_at, created_at)
                VALUES ($1, $2, $3, $4::vector, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                ON CONFLICT (id) DO UPDATE SET
                embedding = CASE WHEN vector_embeddings.tenant_id = EXCLUDED.tenant_id THEN EXCLUDED.embedding ELSE vector_embeddings.embedding END,
                content = CASE WHEN vector_embeddings.tenant_id = EXCLUDED.tenant_id THEN EXCLUDED.content ELSE vector_embeddings.content END,
                source_type = CASE WHEN vector_embeddings.tenant_id = EXCLUDED.tenant_id THEN EXCLUDED.source_type ELSE vector_embeddings.source_type END,
                last_synced_at = CASE WHEN vector_embeddings.tenant_id = EXCLUDED.tenant_id THEN CURRENT_TIMESTAMP ELSE vector_embeddings.last_synced_at END
            ";

            let res = sqlx::query(query)
                .bind(uuid_id)
                .bind(tenant_id)
                .bind(&content)
                .bind(&vector_str)
                .bind(&source_type)
                .execute(&mut *tx)
                .await;

            match res {
                Ok(_) => {
                    tx.commit().await.map_err(|e| tonic::Status::internal(format!("Failed to commit tx: {}", e)))?;
                    synced_count += 1;
                },
                Err(e) => {
                    tracing::error!("Failed to sync embedding {}: {}", id, e);
                    errors.push(e.to_string());
                    // Rollback implicitly happens when tx is dropped
                }
            }
        }

        if !errors.is_empty() {
            return Err(tonic::Status::internal(format!("Failed to sync some embeddings: {}", errors.join(", "))));
        }

        Ok(synced_count)
    }

    pub async fn invoke_tool(&self, req: &McpInvokeRequest, pool: Option<sqlx::PgPool>) -> Result<McpInvokeResponse, tonic::Status> {
        let params: serde_json::Value = serde_json::from_str(&req.params)
            .map_err(|e| tonic::Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

        match req.tool_id.as_str() {
            "mcp_vector_sync" => {
                let embeddings = params["embeddings"].as_array().ok_or_else(|| tonic::Status::invalid_argument("embeddings array is required"))?;
                let pool = pool.ok_or_else(|| tonic::Status::internal("database pool required for sync operations"))?;

                async {
                    let spiffe_id_str = &req.spiffe_id;
                    let parsed = ::server_auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("system".to_string(), "".to_string()));
                    let mut tenant_id = parsed.0;
                    if tenant_id.is_empty() {
                        tenant_id = "system".to_string();
                    }

                    let embeddings_val = embeddings.iter().cloned().collect::<Vec<_>>();

                    let synced_count = self.push_to_cloud(embeddings_val, &pool, &tenant_id).await?;

                    let resp = serde_json::json!({"status": "success", "synced_count": synced_count});
                    Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                }
                .instrument(tracing::info_span!("mcp_vector_sync"))
                .await
            }
            _ => Err(tonic::Status::unimplemented(format!("tool {} not implemented", req.tool_id))),
        }
    }
}
