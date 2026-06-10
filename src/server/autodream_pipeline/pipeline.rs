use crate::db::{DB, DbStore};
use std::sync::Arc;
use sqlx::Row;
use super::llm_client::LLMClient;
use tokio::time::{sleep, Duration};

pub struct AutoDreamPipeline {
    db: Arc<DB>,
    llm_client: Arc<dyn LLMClient>,
    pub cache: Option<Arc<crate::pricing::cache::LocalEmbeddingCache>>,
}

struct PendingAutoDreamTask {
    id: String,
    tenant_id: String,
    agent_id: Option<String>,
    payload: String,
    deliberation_log: Option<String>,
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
        let cache = self.cache.clone();

        tokio::spawn(async move {
            loop {
                let pipeline = AutoDreamPipeline {
                    db: db.clone(),
                    llm_client: llm_client.clone(),
                    cache: cache.clone(),
                };
                if let Err(e) = pipeline.process_closed_tasks().await {
                    ::server_telemetry::record_error_signal("AutoDreamPipeline worker error");
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
        // Find tasks that are COMPLETED but not yet in autodream_memories
        let query = "
            SELECT t.id, t.organization_id, t.assigned_agent_id, t.payload, t.deliberation_log
            FROM shared_tasks t
            LEFT JOIN autodream_memories m ON t.id = m.task_id
            WHERE t.status = 'COMPLETED' AND m.id IS NULL
            LIMIT 100
        ";

        let tasks: Vec<PendingAutoDreamTask> = match &self.db.store {
            DbStore::Postgres => sqlx::query(query)
                .fetch_all(&self.db.pool)
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
                .into_iter()
                .map(|row| PendingAutoDreamTask {
                    id: row.get("id"),
                    tenant_id: row.get("organization_id"),
                    agent_id: row.try_get("assigned_agent_id").unwrap_or(None),
                    payload: row.try_get("payload").unwrap_or_default(),
                    deliberation_log: row.try_get("deliberation_log").unwrap_or(None),
                })
                .collect(),
            DbStore::Sqlite(pool) => sqlx::query(query)
                .fetch_all(pool)
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
                .into_iter()
                .map(|row| PendingAutoDreamTask {
                    id: row.get("id"),
                    tenant_id: row.get("organization_id"),
                    agent_id: row.try_get("assigned_agent_id").unwrap_or(None),
                    payload: row.try_get("payload").unwrap_or_default(),
                    deliberation_log: row.try_get("deliberation_log").unwrap_or(None),
                })
                .collect(),
        };

        for task in tasks {
            let content = format!(
                "Task Payload:\n{}\nDeliberation Log:\n{}",
                task.payload,
                task.deliberation_log.unwrap_or_default()
            );

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

                        self.db.insert_autodream_memory(
                            &mem_id,
                            &task.tenant_id,
                            task.agent_id.as_deref().unwrap_or("system"),
                            &task.id,
                            &chunk,
                            &emb_str,
                            "TASK_SUMMARY"
                        ).await.map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error + Send + Sync>)?;

                        if let Err(telemetry_err) = crate::telemetry::record_autodream_consolidation(&self.db.pool, 1.0).await {
                            ::server_telemetry::record_error_signal("AutoDreamPipeline: Failed to record telemetry");
                            tracing::error!("AutoDreamPipeline: Failed to record telemetry: {}", telemetry_err);
                        }
                    }
                    Err(e) => {
                        ::server_telemetry::record_error_signal("AutoDreamPipeline: Failed to generate embedding for task ");
                        tracing::error!("AutoDreamPipeline: Failed to generate embedding for task {}: {}", task.id, e);
                    }
                }
            }
            tracing::info!("AutoDreamPipeline: Consolidated task {}", task.id);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    use std::sync::atomic::{AtomicUsize, Ordering};
    use async_trait::async_trait;
    use crate::pricing::cache::LocalEmbeddingCache;

    struct TrackingMockLLMClient {
        embedding: Vec<f32>,
        call_count: AtomicUsize,
    }

    impl TrackingMockLLMClient {
        fn new(embedding: Vec<f32>) -> Self {
            Self {
                embedding,
                call_count: AtomicUsize::new(0),
            }
        }

        fn get_call_count(&self) -> usize {
            self.call_count.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl LLMClient for TrackingMockLLMClient {
        async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>, String> {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            Ok(self.embedding.clone())
        }
    }

    #[tokio::test]
    async fn test_process_closed_tasks_with_cache() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "dummy".to_string());
        if database_url == "dummy" {
            return;
        }

        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(&database_url)
            .await
            .unwrap();

        let db = Arc::new(DB { pool: pool.clone(), store: DbStore::Postgres });

        let tracking_llm = Arc::new(TrackingMockLLMClient::new(vec![0.5, 0.6, 0.7]));

        let cache = Arc::new(LocalEmbeddingCache::new(Duration::from_secs(60)));

        // Clean up
        sqlx::query("DELETE FROM autodream_memories").execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM shared_tasks").execute(&pool).await.unwrap();

        let task_id_1 = "test-task-cache-1";
        let task_id_2 = "test-task-cache-2";

        // Insert two tasks with identical payload/log so their chunk text is exactly the same.
        sqlx::query("INSERT INTO shared_tasks (id, organization_id, mission_id, title, status, priority, payload, deliberation_log) VALUES ($1, 'org1', 'm1', 'title', 'COMPLETED', 'HIGH', 'identical payload', 'identical log')")
            .bind(task_id_1)
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO shared_tasks (id, organization_id, mission_id, title, status, priority, payload, deliberation_log) VALUES ($1, 'org1', 'm1', 'title', 'COMPLETED', 'HIGH', 'identical payload', 'identical log')")
            .bind(task_id_2)
            .execute(&pool)
            .await
            .unwrap();

        let pipeline = AutoDreamPipeline::new_with_cache(db.clone(), tracking_llm.clone(), cache);

        let res = pipeline.process_closed_tasks().await;
        assert!(res.is_ok());

        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM autodream_memories WHERE task_id IN ($1, $2)")
            .bind(task_id_1)
            .bind(task_id_2)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(count.0, 2);

        // Since both tasks generated the exact same text chunk, the LLM should have only been called once.
        assert_eq!(tracking_llm.get_call_count(), 1);
    }

    #[tokio::test]
    async fn test_process_closed_tasks_with_sqlite_and_local_cache() {
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE shared_tasks (
                id TEXT PRIMARY KEY,
                organization_id TEXT NOT NULL,
                assigned_agent_id TEXT,
                payload TEXT,
                deliberation_log TEXT,
                status TEXT NOT NULL
            )",
        )
        .execute(&sqlite_pool)
        .await
        .unwrap();
        sqlx::query(
            "CREATE TABLE autodream_memories (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT NOT NULL,
                task_id TEXT NOT NULL UNIQUE,
                content TEXT NOT NULL,
                embedding TEXT NOT NULL,
                source_type TEXT NOT NULL
            )",
        )
        .execute(&sqlite_pool)
        .await
        .unwrap();

        for task_id in ["sqlite-cache-task-1", "sqlite-cache-task-2"] {
            sqlx::query(
                "INSERT INTO shared_tasks
                    (id, organization_id, assigned_agent_id, payload, deliberation_log, status)
                 VALUES (?, 'org-sqlite', 'agent-local', 'identical local payload', 'identical local log', 'COMPLETED')",
            )
            .bind(task_id)
            .execute(&sqlite_pool)
            .await
            .unwrap();
        }

        let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        let db = Arc::new(DB {
            pool: pg_pool,
            store: DbStore::Sqlite(sqlite_pool.clone()),
        });
        let tracking_llm = Arc::new(TrackingMockLLMClient::new(vec![0.5, 0.6, 0.7]));
        let cache = Arc::new(LocalEmbeddingCache::new(Duration::from_secs(60)));
        let pipeline = AutoDreamPipeline::new_with_cache(db, tracking_llm.clone(), cache);

        pipeline.process_closed_tasks().await.unwrap();

        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM autodream_memories")
            .fetch_one(&sqlite_pool)
            .await
            .unwrap();
        assert_eq!(count, 2);
        assert_eq!(tracking_llm.get_call_count(), 1);
    }

    #[tokio::test]
    async fn test_process_closed_tasks() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "dummy".to_string());
        if database_url == "dummy" {
            return;
        }

        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(&database_url)
            .await
            .unwrap();

        let db = Arc::new(DB { pool: pool.clone(), store: DbStore::Postgres });
        let mock_llm = Arc::new(MockLLMClient {
            embedding: vec![0.1, 0.2, 0.3],
        });

        // Clean up
        sqlx::query("DELETE FROM autodream_memories").execute(&pool).await.unwrap();
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

        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM autodream_memories WHERE task_id = $1")
            .bind(task_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(count.0, 1);
    }

    #[tokio::test]
    async fn test_process_closed_tasks_concurrently() {
        let _ = crate::telemetry::get_error_signal_counter();
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "dummy".to_string());
        if database_url == "dummy" { return; }

        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(&database_url)
            .await
            .unwrap();

        let db = Arc::new(DB { pool: pool.clone(), store: DbStore::Postgres });
        let mock_llm = Arc::new(MockLLMClient { embedding: vec![0.1, 0.2] });

        sqlx::query("DELETE FROM autodream_memories").execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM shared_tasks").execute(&pool).await.unwrap();

        let task_id = "test-task-concurrent";
        sqlx::query("INSERT INTO shared_tasks (id, organization_id, mission_id, title, status, priority, payload) VALUES ($1, 'org1', 'm1', 'title', 'COMPLETED', 'HIGH', 'some payload')")
            .bind(task_id)
            .execute(&pool)
            .await
            .unwrap();

        let pipeline = Arc::new(AutoDreamPipeline::new(db.clone(), mock_llm));
        let mut handles = vec![];

        for _ in 0..5 {
            let p = pipeline.clone();
            handles.push(tokio::spawn(async move {
                let _ = p.process_closed_tasks().await;
            }));
        }

        for h in handles {
            let _ = h.await;
        }

        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM autodream_memories WHERE task_id = $1")
            .bind(task_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(count.0, 1);
    }
}
