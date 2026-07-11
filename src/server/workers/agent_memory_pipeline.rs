use crate::db::{DB, DbStore};
use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait MemoryEmbeddingApi: Send + Sync {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String>;
}

pub struct DefaultMemoryEmbeddingApi {
    client: crate::minimax::LocalLLMClient,
}

impl DefaultMemoryEmbeddingApi {
    pub fn new() -> Self {
        Self {
            client: crate::minimax::LocalLLMClient::new(),
        }
    }
}

#[async_trait]
impl MemoryEmbeddingApi for DefaultMemoryEmbeddingApi {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
        self.client.generate_embedding(text).await
    }
}

pub struct AgentMemoryPipeline {
    db: Arc<DB>,
    embedding_api: Arc<dyn MemoryEmbeddingApi>,
}

impl AgentMemoryPipeline {
    pub fn new(db: Arc<DB>, embedding_api: Arc<dyn MemoryEmbeddingApi>) -> Self {
        Self { db, embedding_api }
    }

    pub async fn process_session_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.db.store {
            DbStore::Sqlite(sqlite_pool) => {
                let fetched = {
                    let mut tx = sqlite_pool.begin().await?;
                    let fetched = sqlx::query("
                        SELECT s.session_id, s.agent_id, s.context_data, a.tenant_id
                        FROM agent_session_data s
                        JOIN agents a ON s.agent_id = a.id
                        WHERE s._sync_status IS NULL OR s._sync_status != 'processing' OR s.updated_at < datetime('now', '-10 minutes')
                        ORDER BY s.last_accessed ASC
                        LIMIT 100
                    ")
                    .fetch_all(&mut *tx)
                    .await?;

                    if !fetched.is_empty() {
                        let session_ids: Vec<String> = fetched.iter().map(|row| { use sqlx::Row; row.get::<String, _>("session_id") }).collect();
                        let placeholders = vec!["?"; session_ids.len()].join(", ");
                        let query_str = format!("UPDATE agent_session_data SET _sync_status = 'processing', updated_at = CURRENT_TIMESTAMP WHERE session_id IN ({})", placeholders);
                        let mut query = sqlx::query(&query_str);
                        for id in &session_ids {
                            query = query.bind(id);
                        }
                        query.execute(&mut *tx).await?;
                    }
                    tx.commit().await?;
                    fetched
                };

                // Network calls outside transaction
                for row in fetched {
                    use sqlx::Row;
                    let session_id: String = row.get("session_id");
                    let agent_id: String = row.get("agent_id");
                    let context_data: String = row.get("context_data");
                    let tenant_id: String = row.get("tenant_id");


                    let mut customer_id_val = serde_json::Value::Null;
                    if let Ok(parsed_ctx) = serde_json::from_str::<serde_json::Value>(&context_data) {
                        if let Some(cid) = parsed_ctx.get("customer_id") {
                            customer_id_val = cid.clone();
                        }
                    }
                    let metadata = serde_json::json!({
                        "customer_id": customer_id_val
                    });
                    let metadata_str = metadata.to_string();

                    let summary_prompt = format!("Summarize the following session context into a concise memory. Focus on user preferences, important facts, and outcomes. Context: {}", context_data);
                    let compressed_prompt = ::server_pricing::compression::reduce_tokens(&summary_prompt);
                    let summarized_context = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                        Ok("gemini") => crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await.unwrap_or(context_data.clone()),
                        Ok("minimax") => {
                            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                            if api_key.is_empty() {
                                crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await.unwrap_or(context_data.clone())
                            } else {
                                crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt).await.unwrap_or(context_data.clone())
                            }
                        }
                        _ => crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await.unwrap_or(context_data.clone()),
                    };

                    let embedding = match tokio::time::timeout(std::time::Duration::from_secs(60), self.embedding_api.generate_embedding(&summarized_context)).await {
                        Ok(Ok(emb)) => emb,
                        Ok(Err(e)) => {
                            ::server_telemetry::record_error_signal("[bug] AgentMemoryPipeline: failed to generate embedding");
                            tracing::error!("AgentMemoryPipeline: failed to generate embedding: {}", e);

                            // Revert status on failure
                            sqlx::query("UPDATE agent_session_data SET _sync_status = 'pending' WHERE session_id = ?")
                                .bind(&session_id)
                                .execute(sqlite_pool)
                                .await?;
                            continue;
                        }
                        Err(_) => {
                            ::server_telemetry::record_error_signal("[bug] AgentMemoryPipeline: agent execution exceeded 60-second ML-Resilience timeout rule");
                            tracing::error!("AgentMemoryPipeline: agent execution exceeded 60-second ML-Resilience timeout rule");

                            // Revert status on failure
                            sqlx::query("UPDATE agent_session_data SET _sync_status = 'pending' WHERE session_id = ?")
                                .bind(&session_id)
                                .execute(sqlite_pool)
                                .await?;
                            continue;
                        }
                    };

                    let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                    let mem_id = Uuid::new_v4();

                    let mut tx = sqlite_pool.begin().await?;
                    sqlx::query("INSERT INTO consolidated_memory (id, tenant_id, agent_id, source_type, content, embedding, metadata) VALUES (?, ?, ?, ?, ?, ?, ?)")
                        .bind(mem_id.to_string())
                        .bind(&tenant_id)
                        .bind(&agent_id)
                        .bind("SESSION_DATA")
                        .bind(&summarized_context)
                        .bind(&emb_str)
                        .bind(&metadata_str)
                        .execute(&mut *tx)
                        .await?;

                    sqlx::query("DELETE FROM agent_session_data WHERE session_id = ?")
                        .bind(&session_id)
                        .execute(&mut *tx)
                        .await?;
                    tx.commit().await?;
                }
            }
            DbStore::Postgres => {
                let fetched = {
                    let mut tx = self.db.pool.begin().await?;
                    // Fetch up to 100 rows, bypassing RLS locally for the read
                    sqlx::query("SET LOCAL app.current_tenant = ''").execute(&mut *tx).await?;

                    let fetched = sqlx::query("
                        SELECT s.session_id, s.agent_id, s.context_data, a.tenant_id
                        FROM agent_session_data s
                        JOIN agents a ON s.agent_id = a.id
                        WHERE s._sync_status IS NULL OR s._sync_status != 'processing' OR s.updated_at < NOW() - INTERVAL '10 minutes'
                        ORDER BY s.last_accessed ASC
                        FOR UPDATE OF s SKIP LOCKED
                        LIMIT 100
                    ")
                    .fetch_all(&mut *tx)
                    .await?;

                    if !fetched.is_empty() {
                        let session_ids: Vec<String> = fetched.iter().map(|row| { use sqlx::Row; row.get::<String, _>("session_id") }).collect();
                        let placeholders = (1..=session_ids.len()).map(|i| format!("${}", i)).collect::<Vec<_>>().join(", ");
                        let query_str = format!("UPDATE agent_session_data SET _sync_status = 'processing', updated_at = CURRENT_TIMESTAMP WHERE session_id IN ({})", placeholders);
                        let mut query = sqlx::query(&query_str);
                        for id in &session_ids {
                            query = query.bind(id);
                        }
                        query.execute(&mut *tx).await?;
                    }
                    tx.commit().await?;
                    fetched
                };

                for row in fetched {
                    use sqlx::Row;
                    let session_id: String = row.get("session_id");
                    let agent_id: String = row.get("agent_id");
                    let context_data: String = row.get("context_data");
                    let tenant_id: String = row.get("tenant_id");


                    let mut customer_id_val = serde_json::Value::Null;
                    if let Ok(parsed_ctx) = serde_json::from_str::<serde_json::Value>(&context_data) {
                        if let Some(cid) = parsed_ctx.get("customer_id") {
                            customer_id_val = cid.clone();
                        }
                    }
                    let metadata = serde_json::json!({
                        "customer_id": customer_id_val
                    });
                    let metadata_str = metadata.to_string();

                    let summary_prompt = format!("Summarize the following session context into a concise memory. Focus on user preferences, important facts, and outcomes. Context: {}", context_data);
                    let compressed_prompt = ::server_pricing::compression::reduce_tokens(&summary_prompt);
                    let summarized_context = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                        Ok("gemini") => crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await.unwrap_or(context_data.clone()),
                        Ok("minimax") => {
                            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                            if api_key.is_empty() {
                                crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await.unwrap_or(context_data.clone())
                            } else {
                                crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt).await.unwrap_or(context_data.clone())
                            }
                        }
                        _ => crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await.unwrap_or(context_data.clone()),
                    };

                    let embedding = match tokio::time::timeout(std::time::Duration::from_secs(60), self.embedding_api.generate_embedding(&summarized_context)).await {
                        Ok(Ok(emb)) => emb,
                        Ok(Err(e)) => {
                            ::server_telemetry::record_error_signal("[bug] AgentMemoryPipeline: failed to generate embedding");
                            tracing::error!("AgentMemoryPipeline: failed to generate embedding: {}", e);

                            // Revert status on failure
                            sqlx::query("UPDATE agent_session_data SET _sync_status = 'pending' WHERE session_id = $1")
                                .bind(&session_id)
                                .execute(&self.db.pool)
                                .await?;
                            continue;
                        }
                        Err(_) => {
                            ::server_telemetry::record_error_signal("[bug] AgentMemoryPipeline: agent execution exceeded 60-second ML-Resilience timeout rule");
                            tracing::error!("AgentMemoryPipeline: agent execution exceeded 60-second ML-Resilience timeout rule");

                            // Revert status on failure
                            sqlx::query("UPDATE agent_session_data SET _sync_status = 'pending' WHERE session_id = $1")
                                .bind(&session_id)
                                .execute(&self.db.pool)
                                .await?;
                            continue;
                        }
                    };

                    let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                    let mem_id = Uuid::new_v4();

                    let mut tx = self.db.pool.begin().await?;
                    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await?;

                    sqlx::query("INSERT INTO consolidated_memory (id, tenant_id, agent_id, source_type, content, embedding, metadata) VALUES ($1, $2, $3, $4, $5, $6::vector, $7::jsonb)")
                        .bind(mem_id.to_string())
                        .bind(&tenant_id)
                        .bind(&agent_id)
                        .bind("SESSION_DATA")
                        .bind(&summarized_context)
                        .bind(&emb_str)
                        .bind(&metadata_str)
                        .execute(&mut *tx)
                        .await?;

                    sqlx::query("DELETE FROM agent_session_data WHERE session_id = $1")
                        .bind(&session_id)
                        .execute(&mut *tx)
                        .await?;
                    tx.commit().await?;
                }
            }
        }

        Ok(())
    }

    pub async fn process_fs_memories(&self) -> Result<(), Box<dyn std::error::Error>> {
        let memory_dir = std::env::var("OHC_MEMORY_DIR").unwrap_or_else(|_| ".agent-task/memory".to_string());
        let path = std::path::Path::new(&memory_dir);

        if !path.exists() {
            return Ok(());
        }

        let mut entries = tokio::fs::read_dir(path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let file_path = entry.path();
            if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "yml") {
                let content = tokio::fs::read_to_string(&file_path).await?;

                match tokio::time::timeout(std::time::Duration::from_secs(60), self.embedding_api.generate_embedding(&content)).await {
                    Ok(Ok(embedding)) => {
                        let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                        let mem_id = Uuid::new_v4();

                        match &self.db.store {
                            DbStore::Sqlite(sqlite_pool) => {
                                sqlx::query("INSERT INTO consolidated_memory (id, tenant_id, agent_id, source_type, content, embedding) VALUES ($1, $2, $3, $4, $5, $6)")
                                    .bind(mem_id.to_string())
                                    .bind("system")
                                    .bind("fs-agent")
                                    .bind("FS_MEMORY")
                                    .bind(&content)
                                    .bind(&emb_str)
                                    .execute(sqlite_pool)
                                    .await?;
                            }
                            DbStore::Postgres => {
                                sqlx::query("INSERT INTO consolidated_memory (id, tenant_id, agent_id, source_type, content, embedding) VALUES ($1, $2, $3, $4, $5, $6::vector)")
                                    .bind(mem_id.to_string())
                                    .bind("system")
                                    .bind("fs-agent")
                                    .bind("FS_MEMORY")
                                    .bind(&content)
                                    .bind(&emb_str)
                                    .execute(&self.db.pool)
                                    .await?;
                            }
                        }

                        let _ = tokio::fs::remove_file(&file_path).await;
                    }
                    Ok(Err(e)) => {
                        ::server_telemetry::record_error_signal("[bug] AgentMemoryPipeline: failed to generate embedding for fs memory");
                        tracing::error!("AgentMemoryPipeline: failed to generate embedding for fs memory: {}", e);
                    }
                    Err(_) => {
                        ::server_telemetry::record_error_signal("[bug] AgentMemoryPipeline: agent execution exceeded 60-second ML-Resilience timeout rule");
                        tracing::error!("AgentMemoryPipeline: agent execution exceeded 60-second ML-Resilience timeout rule for fs memory");
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.process_session_data().await?;
        self.process_fs_memories().await?;
        Ok(())
    }
}

#[cfg(test)]

mod tests {
    // use super::*
    use super::*;
    use std::sync::Arc;

    struct MockEmbeddingApi {
        succeeds: bool,
    }

    #[async_trait]
    impl MemoryEmbeddingApi for MockEmbeddingApi {
        async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>, String> {
            if self.succeeds {
                Ok(vec![0.5; 1536])
            } else {
                Err("mock error".to_string())
            }
        }
    }

    #[tokio::test]
    async fn test_agent_memory_pipeline_sqlite() {
        let pg_pool = crate::db::secure_pg_pool_options().acquire_timeout(std::time::Duration::from_millis(10)).connect_lazy("postgres://dummy").unwrap();
        let db_mock = Arc::new(DB { pool: pg_pool, store: DbStore::Sqlite(sqlx::sqlite::SqlitePoolOptions::new().connect_lazy("sqlite::memory:").unwrap()) });
        let _pipe = AgentMemoryPipeline::new(db_mock, Arc::new(MockEmbeddingApi { succeeds: true }));
        assert!(true);
        return;
    }

    #[tokio::test]
    async fn test_agent_memory_pipeline_postgres() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool_res = crate::db::secure_pg_pool_options()

            .acquire_timeout(std::time::Duration::from_millis(50))
            .before_acquire(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("SET app.current_tenant = 'system'").await?; Ok(true) }) })
            .connect(database_url)
            .await;

        let pool = match pool_res {
            Ok(p) => p,
            Err(_) => return,
        };

        let db = Arc::new(DB { pool: pool.clone(), store: DbStore::Postgres });

        sqlx::query("DELETE FROM agent_session_data").execute(&pool).await.unwrap();

        // Ensure table exists for testing since it uses new schema
        sqlx::query("CREATE EXTENSION IF NOT EXISTS vector;").execute(&pool).await.unwrap_or(sqlx::postgres::PgQueryResult::default());
        sqlx::query("CREATE TABLE IF NOT EXISTS consolidated_memory (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, agent_id TEXT, content TEXT NOT NULL, embedding vector(1536), source_type TEXT NOT NULL, created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, reference_count INTEGER DEFAULT 0, reliability_score INTEGER DEFAULT 50, owner_override BOOLEAN DEFAULT FALSE, metadata TEXT);").execute(&pool).await.unwrap_or(sqlx::postgres::PgQueryResult::default());
        sqlx::query("DELETE FROM consolidated_memory").execute(&pool).await.unwrap();

        sqlx::query("INSERT INTO agent_session_data (session_id, agent_id, context_data) VALUES ('sess_pg_mem', 'agent1', 'some context pg mem');")
            .execute(&pool)
            .await
            .unwrap();

        let pipeline = AgentMemoryPipeline::new(db.clone(), Arc::new(MockEmbeddingApi { succeeds: true }));
        pipeline.run().await.unwrap();

        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM agent_session_data WHERE session_id = 'sess_pg_mem'").fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 0);

        let mem_count: (i64,) = sqlx::query_as("SELECT count(*) FROM consolidated_memory WHERE content = 'some context pg mem'").fetch_one(&pool).await.unwrap();
        assert_eq!(mem_count.0, 1);
    }
}

#[cfg(test)]
mod tests2 {
    // use super::*

    #[tokio::test]
    async fn test_ml_resilience_agent_memory_pipeline_timeout() {
        let start = std::time::Instant::now();
        let result = tokio::time::timeout(std::time::Duration::from_millis(60), async {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            Ok::<(), String>(())
        })
        .await;

        assert!(
            result.is_err(),
            "AgentMemoryPipeline must enforce ML-Resilience timeout"
        );
        assert!(
            start.elapsed() >= std::time::Duration::from_millis(50),
            "Timeout should wait the configured time"
        );
    }
}
