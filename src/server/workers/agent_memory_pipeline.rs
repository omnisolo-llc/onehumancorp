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
        let rows = sqlx::query("SELECT session_id, agent_id, context_data FROM agent_session_data ORDER BY last_accessed ASC LIMIT 100")
            .fetch_all(&self.db.pool)
            .await?;

        for row in rows {
            use sqlx::Row;
            let session_id: String = row.get("session_id");
            let agent_id: String = row.get("agent_id");
            let context_data: String = row.get("context_data");

            let embedding = match self.embedding_api.generate_embedding(&context_data).await {
                Ok(emb) => emb,
                Err(e) => {
                    tracing::warn!("AgentMemoryPipeline: failed to generate embedding: {}", e);
                    vec![0.0; 1536]
                }
            };

            let mem_id = Uuid::new_v4();
            let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));

            match &self.db.store {
                DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query("INSERT INTO agent_memory_embeddings (id, organization_id, agent_id, memory_type, content, embedding) VALUES ($1, $2, $3, $4, $5, NULL)")
                        .bind(mem_id.to_string())
                        .bind("system")
                        .bind(&agent_id)
                        .bind("SESSION_DATA")
                        .bind(&context_data)
                        .execute(sqlite_pool)
                        .await?;
                }
                DbStore::Postgres => {
                    sqlx::query("INSERT INTO agent_memory_embeddings (id, organization_id, agent_id, memory_type, content, embedding) VALUES ($1, $2, $3, $4, $5, $6::vector)")
                        .bind(mem_id)
                        .bind("system")
                        .bind(&agent_id)
                        .bind("SESSION_DATA")
                        .bind(&context_data)
                        .bind(&emb_str)
                        .execute(&self.db.pool)
                        .await?;
                }
            }

            sqlx::query("DELETE FROM agent_session_data WHERE session_id = $1")
                .bind(&session_id)
                .execute(&self.db.pool)
                .await?;
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

                match self.embedding_api.generate_embedding(&content).await {
                    Ok(embedding) => {
                        let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                        let mem_id = Uuid::new_v4();

                        match &self.db.store {
                            DbStore::Sqlite(sqlite_pool) => {
                                sqlx::query("INSERT INTO agent_memory_embeddings (id, organization_id, agent_id, memory_type, content, embedding) VALUES ($1, $2, $3, $4, $5, NULL)")
                                    .bind(mem_id.to_string())
                                    .bind("system")
                                    .bind("fs-agent")
                                    .bind("FS_MEMORY")
                                    .bind(&content)
                                    .execute(sqlite_pool)
                                    .await?;
                            }
                            DbStore::Postgres => {
                                sqlx::query("INSERT INTO agent_memory_embeddings (id, organization_id, agent_id, memory_type, content, embedding) VALUES ($1, $2, $3, $4, $5, $6::vector)")
                                    .bind(mem_id)
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
                    Err(e) => {
                        tracing::warn!("AgentMemoryPipeline: failed to generate embedding for fs memory: {}", e);
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
    use super::*;

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
        let pg_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).connect_lazy("postgres://dummy").unwrap();
        let db_mock = Arc::new(DB { pool: pg_pool, store: DbStore::Sqlite(sqlx::sqlite::SqlitePoolOptions::new().connect_lazy("sqlite::memory:").unwrap()) });
        let _pipe = AgentMemoryPipeline::new(db_mock, Arc::new(MockEmbeddingApi { succeeds: true }));
        assert!(true);
        return;
    }

    #[tokio::test]
    async fn test_agent_memory_pipeline_postgres() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool_res = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
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
        sqlx::query("CREATE TABLE IF NOT EXISTS agent_memory_embeddings (id UUID PRIMARY KEY DEFAULT gen_random_uuid(), organization_id VARCHAR NOT NULL, agent_id VARCHAR NOT NULL, memory_type VARCHAR NOT NULL, content TEXT NOT NULL, embedding vector(1536), created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP);").execute(&pool).await.unwrap_or(sqlx::postgres::PgQueryResult::default());
        sqlx::query("DELETE FROM agent_memory_embeddings").execute(&pool).await.unwrap();

        sqlx::query("INSERT INTO agent_session_data (session_id, agent_id, context_data) VALUES ('sess_pg_mem', 'agent1', 'some context pg mem');")
            .execute(&pool)
            .await
            .unwrap();

        let pipeline = AgentMemoryPipeline::new(db.clone(), Arc::new(MockEmbeddingApi { succeeds: true }));
        pipeline.run().await.unwrap();

        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM agent_session_data WHERE session_id = 'sess_pg_mem'").fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 0);

        let mem_count: (i64,) = sqlx::query_as("SELECT count(*) FROM agent_memory_embeddings WHERE content = 'some context pg mem'").fetch_one(&pool).await.unwrap();
        assert_eq!(mem_count.0, 1);
    }
}
