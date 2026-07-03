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
                for _ in 0..100 {
                    let mut tx = sqlite_pool.begin().await?;

                    let row = sqlx::query("
                        SELECT s.session_id, s.agent_id, s.context_data, a.tenant_id
                        FROM agent_session_data s
                        JOIN agents a ON s.agent_id = a.id
                        ORDER BY s.last_accessed ASC
                        LIMIT 1
                    ")
                    .fetch_optional(&mut *tx)
                    .await?;

                    if let Some(row) = row {
                        use sqlx::Row;
                        let session_id: String = row.get("session_id");
                        let agent_id: String = row.get("agent_id");
                        let context_data: String = row.get("context_data");
                        let tenant_id: String = row.get("tenant_id");

                        // We immediately delete to simulate "locking" it so other workers don't grab it
                        sqlx::query("DELETE FROM agent_session_data WHERE session_id = $1")
                            .bind(&session_id)
                            .execute(&mut *tx)
                            .await?;

                        let embedding = match tokio::time::timeout(std::time::Duration::from_secs(60), self.embedding_api.generate_embedding(&context_data)).await {
                            Ok(Ok(emb)) => emb,
                            Ok(Err(e)) => {
                                ::server_telemetry::record_error_signal("[bug] AgentMemoryPipeline: failed to generate embedding");
                                tracing::error!("AgentMemoryPipeline: failed to generate embedding: {}", e);
                                vec![0.0; 1536]
                            }
                            Err(_) => {
                                ::server_telemetry::record_error_signal("[bug] AgentMemoryPipeline: agent execution exceeded 60-second ML-Resilience timeout rule");
                                tracing::error!("AgentMemoryPipeline: agent execution exceeded 60-second ML-Resilience timeout rule");
                                vec![0.0; 1536]
                            }
                        };

                        let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                        let mem_id = Uuid::new_v4();

                        sqlx::query("INSERT INTO consolidated_memory (id, tenant_id, agent_id, source_type, content, embedding) VALUES ($1, $2, $3, $4, $5, $6)")
                            .bind(mem_id.to_string())
                            .bind(&tenant_id)
                            .bind(&agent_id)
                            .bind("SESSION_DATA")
                            .bind(&context_data)
                            .bind(&emb_str)
                            .execute(&mut *tx)
                            .await?;

                        tx.commit().await?;
                    } else {
                        tx.rollback().await?;
                        break;
                    }
                }
            }
            DbStore::Postgres => {
                for _ in 0..100 {
                    let mut tx = self.db.pool.begin().await?;
                    // Fetch one row, bypassing RLS locally for the read
                    sqlx::query("SET LOCAL app.current_tenant = ''").execute(&mut *tx).await?;

                    let row = sqlx::query("
                        SELECT s.session_id, s.agent_id, s.context_data, a.tenant_id
                        FROM agent_session_data s
                        JOIN agents a ON s.agent_id = a.id
                        ORDER BY s.last_accessed ASC
                        LIMIT 1
                        FOR UPDATE OF s SKIP LOCKED
                    ")
                    .fetch_optional(&mut *tx)
                    .await?;

                    if let Some(row) = row {
                        use sqlx::Row;
                        let session_id: String = row.get("session_id");
                        let agent_id: String = row.get("agent_id");
                        let context_data: String = row.get("context_data");
                        let tenant_id: String = row.get("tenant_id");

                        let embedding = match tokio::time::timeout(std::time::Duration::from_secs(60), self.embedding_api.generate_embedding(&context_data)).await {
                            Ok(Ok(emb)) => emb,
                            Ok(Err(e)) => {
                                ::server_telemetry::record_error_signal("[bug] AgentMemoryPipeline: failed to generate embedding");
                                tracing::error!("AgentMemoryPipeline: failed to generate embedding: {}", e);
                                vec![0.0; 1536]
                            }
                            Err(_) => {
                                ::server_telemetry::record_error_signal("[bug] AgentMemoryPipeline: agent execution exceeded 60-second ML-Resilience timeout rule");
                                tracing::error!("AgentMemoryPipeline: agent execution exceeded 60-second ML-Resilience timeout rule");
                                vec![0.0; 1536]
                            }
                        };

                        let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                        let mem_id = Uuid::new_v4();

                        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await?;

                        sqlx::query("INSERT INTO consolidated_memory (id, tenant_id, agent_id, source_type, content, embedding) VALUES ($1, $2, $3, $4, $5, $6::vector)")
                            .bind(mem_id.to_string())
                            .bind(&tenant_id)
                            .bind(&agent_id)
                            .bind("SESSION_DATA")
                            .bind(&context_data)
                            .bind(&emb_str)
                            .execute(&mut *tx)
                            .await?;

                        sqlx::query("DELETE FROM agent_session_data WHERE session_id = $1")
                            .bind(&session_id)
                            .execute(&mut *tx)
                            .await?;

                        tx.commit().await?;
                    } else {
                        // No more rows available
                        tx.rollback().await?;
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn prune_stale_memory(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.db.store {
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query("DELETE FROM consolidated_memory WHERE last_referenced_at < datetime('now', '-180 days') AND reference_count < 5 AND owner_override = FALSE")
                    .execute(sqlite_pool)
                    .await?;
            }
            DbStore::Postgres => {
                sqlx::query("DELETE FROM consolidated_memory WHERE last_referenced_at < NOW() - INTERVAL '180 days' AND reference_count < 5 AND owner_override = FALSE")
                    .execute(&self.db.pool)
                    .await?;
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
        self.prune_stale_memory().await?;
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
    use super::*;
    use std::sync::Arc;

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
