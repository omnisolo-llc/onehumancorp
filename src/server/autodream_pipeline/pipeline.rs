use crate::db::DB;
use std::sync::Arc;
use sqlx::Row;
use super::llm_client::LLMClient;
use tokio::time::{sleep, Duration};

pub struct AutoDreamPipeline {
    db: Arc<DB>,
    llm_client: Arc<dyn LLMClient>,
    pub cache: Option<Arc<crate::pricing::cache::LocalEmbeddingCache>>,
}

impl AutoDreamPipeline {
    pub fn new(db: Arc<DB>, llm_client: Arc<dyn LLMClient>) -> Self {
        AutoDreamPipeline { db, llm_client, cache: None }
    }

    pub fn new_with_cache(db: Arc<DB>, llm_client: Arc<dyn LLMClient>, cache: Arc<crate::pricing::cache::LocalEmbeddingCache>) -> Self {
        AutoDreamPipeline { db, llm_client, cache: Some(cache) }
    }

    pub fn start_worker(&self) {
        let db = self.db.clone();
        let llm_client = self.llm_client.clone();

        tokio::spawn(async move {
            loop {
                let pipeline = AutoDreamPipeline::new(db.clone(), llm_client.clone());
                if let Err(e) = pipeline.process_closed_tasks().await {
                    tracing::error!("AutoDreamPipeline worker error: {}", e);
                }
                sleep(Duration::from_secs(60)).await;
            }
        });
    }

    fn chunk_content(content: &str, chunk_size: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let words: Vec<&str> = content.split_whitespace().collect();
        let mut current_chunk = String::new();
        let mut current_size = 0;

        for word in words {
            if current_size + word.len() + 1 > chunk_size && !current_chunk.is_empty() {
                chunks.push(current_chunk.trim().to_string());
                current_chunk.clear();
                current_size = 0;
            }
            current_chunk.push_str(word);
            current_chunk.push(' ');
            current_size += word.len() + 1;
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk.trim().to_string());
        }

        chunks
    }

    pub async fn process_closed_tasks(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Find tasks that are COMPLETED but not yet in consolidated_memory
        let query = "
            SELECT t.id, t.organization_id, t.assigned_agent_id, t.payload, t.deliberation_log
            FROM shared_tasks t
            LEFT JOIN consolidated_memory m ON t.id = m.task_id
            WHERE t.status = 'COMPLETED' AND m.id IS NULL
            LIMIT 100
        ";

        let mut rows_data = Vec::new();
        match &self.db.store {
            crate::db::DbStore::Sqlite(pool) => {
                let tasks = sqlx::query(query).fetch_all(pool).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                for row in tasks {
                    let task_id: String = row.get("id");
                    let tenant_id: String = row.get("organization_id");
                    let agent_id: Option<String> = row.try_get("assigned_agent_id").unwrap_or(None);
                    let payload: String = row.try_get("payload").unwrap_or_default();
                    let log: Option<String> = row.try_get("deliberation_log").unwrap_or(None);
                    rows_data.push((task_id, tenant_id, agent_id, payload, log));
                }
            }
            crate::db::DbStore::Postgres => {
                let tasks = sqlx::query(query).fetch_all(&self.db.pool).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                for row in tasks {
                    let task_id: String = row.get("id");
                    let tenant_id: String = row.get("organization_id");
                    let agent_id: Option<String> = row.try_get("assigned_agent_id").unwrap_or(None);
                    let payload: String = row.try_get("payload").unwrap_or_default();
                    let log: Option<String> = row.try_get("deliberation_log").unwrap_or(None);
                    rows_data.push((task_id, tenant_id, agent_id, payload, log));
                }
            }
        };

        for (task_id, tenant_id, agent_id, payload, log) in rows_data {

            let content = format!("Task Payload:\n{}\nDeliberation Log:\n{}", payload, log.unwrap_or_default());

            // Chunk the content to avoid token limits (e.g., 2000 chars roughly to tokens)
            let chunks = Self::chunk_content(&content, 2000);

            for chunk in chunks {
                let cached_embedding = if let Some(cache) = &self.cache {
                    cache.get(&chunk)
                } else {
                    None
                };

                let embedding_res = if let Some(emb_str) = cached_embedding {
                    Ok(emb_str)
                } else {
                    match self.llm_client.generate_embedding(&chunk).await {
                        Ok(embedding) => {
                            let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                            if let Some(cache) = &self.cache {
                                cache.set(&chunk, &emb_str);
                            }
                            Ok(emb_str)
                        }
                        Err(e) => Err(e),
                    }
                };

                match embedding_res {
                    Ok(emb_str) => {
                        let mem_id = uuid::Uuid::new_v4().to_string();

                        match &self.db.store {
                            crate::db::DbStore::Sqlite(pool) => {
                                sqlx::query("INSERT INTO consolidated_memory (id, tenant_id, agent_id, content, embedding, source_type, task_id) VALUES (?, ?, ?, ?, ?, ?, ?)")
                                    .bind(&mem_id)
                                    .bind(&tenant_id)
                                    .bind(&agent_id)
                                    .bind(&chunk)
                                    .bind(&emb_str)
                                    .bind("TASK_SUMMARY")
                                    .bind(&task_id)
                                    .execute(pool)
                                    .await
                                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                            }
                            crate::db::DbStore::Postgres => {
                                sqlx::query("INSERT INTO consolidated_memory (id, tenant_id, agent_id, content, embedding, source_type, task_id) VALUES ($1, $2, $3, $4, $5::vector, $6, $7)")
                                    .bind(&mem_id)
                                    .bind(&tenant_id)
                                    .bind(&agent_id)
                                    .bind(&chunk)
                                    .bind(&emb_str)
                                    .bind("TASK_SUMMARY")
                                    .bind(&task_id)
                                    .execute(&self.db.pool)
                                    .await
                                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("AutoDreamPipeline: Failed to generate embedding for task {}: {}", task_id, e);
                    }
                }
            }
            tracing::info!("AutoDreamPipeline: Consolidated task {}", task_id);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DbStore;
    use super::super::llm_client::MockLLMClient;

    #[test]
    fn test_chunk_content() {
        let content = "this is a very long string that needs to be chunked properly to avoid token limits during embeddings";
        let chunks = AutoDreamPipeline::chunk_content(content, 25);
        assert!(chunks.len() > 1);
        for chunk in chunks {
            assert!(chunk.len() <= 25);
        }
    }

    #[tokio::test]
    async fn test_process_closed_tasks() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "dummy".to_string());
        if database_url == "dummy" {
            return;
        }

        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect(&database_url)
            .await
            .unwrap();

        let db = Arc::new(DB { pool: pool.clone(), store: DbStore::Postgres });
        let mock_llm = Arc::new(MockLLMClient {
            embedding: vec![0.1, 0.2, 0.3],
        });

        // Clean up
        sqlx::query("DELETE FROM consolidated_memory").execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM shared_tasks").execute(&pool).await.unwrap();

        let task_id = "test-task-1";
        sqlx::query("INSERT INTO shared_tasks (id, organization_id, mission_id, title, status, priority, payload) VALUES ($1, 'org1', 'm1', 'title', 'COMPLETED', 'HIGH', 'some payload')")
            .bind(task_id)
            .execute(&pool)
            .await
            .unwrap();

        let pipeline = AutoDreamPipeline::new(db.clone(), mock_llm);
        let res = pipeline.process_closed_tasks().await;
        assert!(res.is_ok());

        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM consolidated_memory WHERE task_id = $1")
            .bind(task_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(count.0, 1);
    }

    #[tokio::test]
    async fn test_process_closed_tasks_sqlite() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;

        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool_sqlite = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let db = Arc::new(crate::db::DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap(),
            store: crate::db::DbStore::Sqlite(pool_sqlite.clone())
        });

        if let crate::db::DbStore::Sqlite(ref pool) = db.store {
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS consolidated_memory (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, agent_id TEXT, content TEXT NOT NULL, embedding TEXT, source_type TEXT NOT NULL, task_id TEXT);").execute(pool).await;
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS shared_tasks (id TEXT PRIMARY KEY, organization_id TEXT NOT NULL, mission_id TEXT, title TEXT, status TEXT, priority TEXT, payload TEXT, assigned_agent_id TEXT, deliberation_log TEXT);").execute(pool).await;

            let _ = sqlx::query("DELETE FROM consolidated_memory").execute(pool).await;
            let _ = sqlx::query("DELETE FROM shared_tasks").execute(pool).await;

            let task_id = "test-task-1-sqlite";
            let _ = sqlx::query("INSERT INTO shared_tasks (id, organization_id, status, payload) VALUES (?, 'org1', 'COMPLETED', 'some payload')")
                .bind(task_id)
                .execute(pool)
                .await;

            let mock_llm = Arc::new(MockLLMClient {
                embedding: vec![0.1, 0.2, 0.3],
            });

            let pipeline = AutoDreamPipeline::new(db.clone(), mock_llm);
            let res = pipeline.process_closed_tasks().await;
            assert!(res.is_ok());

            let count: (i64,) = sqlx::query_as("SELECT count(*) FROM consolidated_memory WHERE task_id = ?")
                .bind(task_id)
                .fetch_one(pool)
                .await
                .unwrap();

            assert_eq!(count.0, 1);
        }
    }
}
