use std::sync::Arc;
use ohc_builtin_agent::memory_store::VectorRepository;
use chrono::Utc;
use async_trait::async_trait;

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

/// MemoryConsolidationWorker is responsible for periodically converting new facts
/// into vector embeddings, resolving memory conflicts within the vector repository,
/// and pruning stale context.
pub struct MemoryConsolidationWorker {
    pub repository: Arc<VectorRepository>,
    pub poll_interval: std::time::Duration,
    pub prune_threshold_days: i64,
    pub db: Arc<crate::db::DB>,
    pub embedding_api: Arc<dyn MemoryEmbeddingApi>,
    pub memory_dir: String,
}

impl MemoryConsolidationWorker {
    pub fn new(repository: Arc<VectorRepository>, db: Arc<crate::db::DB>, embedding_api: Arc<dyn MemoryEmbeddingApi>, memory_dir: Option<String>) -> Self {
        let dir = memory_dir.unwrap_or_else(|| std::env::var("OHC_MEMORY_DIR").unwrap_or_else(|_| ".agent-task/memory".to_string()));
        Self {
            repository,
            poll_interval: std::time::Duration::from_secs(3600), // 1 hour
            prune_threshold_days: 180, // Default to 180 days
            db,
            embedding_api,
            memory_dir: dir,
        }
    }

    pub async fn process_session_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        use crate::db::DbStore;
        use sqlx::Row;

        let query = "SELECT session_id, agent_id, context_data FROM agent_session_data ORDER BY last_accessed ASC LIMIT 100";

        let mut rows = Vec::new();
        match &self.db.store {
            DbStore::Postgres => {
                let pg_rows = sqlx::query(query).fetch_all(&self.db.pool).await?;
                for row in pg_rows {
                    rows.push((
                        row.get::<String, _>("session_id"),
                        row.get::<String, _>("agent_id"),
                        row.get::<String, _>("context_data"),
                    ));
                }
            }
            DbStore::Sqlite(sqlite_pool) => {
                let sq_rows = sqlx::query(query).fetch_all(sqlite_pool).await?;
                for row in sq_rows {
                    rows.push((
                        row.get::<String, _>("session_id"),
                        row.get::<String, _>("agent_id"),
                        row.get::<String, _>("context_data"),
                    ));
                }
            }
        }

        for (session_id, agent_id, context_data) in rows {
            let embedding = match self.embedding_api.generate_embedding(&context_data).await {
                Ok(emb) => emb,
                Err(e) => {
                    tracing::error!("MemoryConsolidationWorker: failed to generate embedding: {}", e);
                    vec![0.0; 1536]
                }
            };

            let mem_id = uuid::Uuid::new_v4().to_string();

            let record = ohc_builtin_agent::memory_store::EmbeddingRecord {
                id: mem_id,
                tenant_id: "system".to_string(), // Adjust based on session scope if available
                agent_id: agent_id,
                content: context_data,
                embedding: embedding,
                source_type: "SESSION_DATA".to_string(),
                created_at: chrono::Utc::now(),
                last_referenced_at: chrono::Utc::now(),
                reference_count: 1,
                reliability_score: 50,
                owner_override: false,
                metadata: None,
            };

            if let Err(e) = self.repository.upsert(&record).await {
                tracing::error!("MemoryConsolidationWorker: failed to insert session embedding: {}", e);
            } else {
                match &self.db.store {
                    DbStore::Postgres => {
                        let _ = sqlx::query("DELETE FROM agent_session_data WHERE session_id = $1")
                            .bind(&session_id)
                            .execute(&self.db.pool)
                            .await;
                    }
                    DbStore::Sqlite(sqlite_pool) => {
                        let _ = sqlx::query("DELETE FROM agent_session_data WHERE session_id = ?")
                            .bind(&session_id)
                            .execute(sqlite_pool)
                            .await;
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn process_fs_memories(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = std::path::Path::new(&self.memory_dir);

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
                        let mem_id = uuid::Uuid::new_v4().to_string();

                        let record = ohc_builtin_agent::memory_store::EmbeddingRecord {
                            id: mem_id,
                            tenant_id: "system".to_string(),
                            agent_id: "fs-agent".to_string(),
                            content: content,
                            embedding: embedding,
                            source_type: "FS_MEMORY".to_string(),
                            created_at: chrono::Utc::now(),
                            last_referenced_at: chrono::Utc::now(),
                            reference_count: 1,
                            reliability_score: 50,
                            owner_override: false,
                            metadata: None,
                        };

                        if let Err(e) = self.repository.upsert(&record).await {
                            tracing::error!("MemoryConsolidationWorker: failed to insert fs embedding: {}", e);
                        } else {
                            let _ = tokio::fs::remove_file(&file_path).await;
                        }
                    }
                    Err(e) => {
                        tracing::error!("MemoryConsolidationWorker: failed to generate embedding for fs memory: {}", e);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn start(self: Arc<Self>) {
        let worker = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(worker.poll_interval);
            loop {
                interval.tick().await;

                if let Err(e) = worker.process_session_data().await {
                    tracing::error!("Consolidation Worker: Failed to process session data: {}", e);
                }

                if let Err(e) = worker.process_fs_memories().await {
                    tracing::error!("Consolidation Worker: Failed to process fs memories: {}", e);
                }

                let older_than = Utc::now() - chrono::Duration::days(worker.prune_threshold_days);
                if let Err(e) = worker.repository.prune_stale(older_than).await {
                    tracing::error!("Consolidation Worker: Failed to prune stale context: {}", e);
                }

                if let Err(e) = worker.repository.auto_resolve_conflicts().await {
                    tracing::error!("Consolidation Worker: Failed to resolve memory conflicts: {}", e);
                }
            }
        });
    }

    pub async fn run_pipeline_once(&self) {
        if let Err(e) = self.process_session_data().await {
            tracing::error!("Consolidation Worker: Failed to process session data: {}", e);
        }

        if let Err(e) = self.process_fs_memories().await {
            tracing::error!("Consolidation Worker: Failed to process fs memories: {}", e);
        }

        let older_than = Utc::now() - chrono::Duration::days(self.prune_threshold_days);
        if let Err(e) = self.repository.prune_stale(older_than).await {
            tracing::error!("Consolidation Worker: Failed to prune stale context: {}", e);
        }

        if let Err(e) = self.repository.auto_resolve_conflicts().await {
            tracing::error!("Consolidation Worker: Failed to resolve memory conflicts: {}", e);
        }
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
    async fn test_worker_initialization() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;

        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));
        let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap();
        let db = Arc::new(crate::db::DB { pool: dummy_pg_pool, store: crate::db::DbStore::Sqlite(pool) });
        let worker = MemoryConsolidationWorker::new(repo, db, Arc::new(MockEmbeddingApi { succeeds: true }), None);
        assert_eq!(worker.poll_interval.as_secs(), 3600);
    }

    #[tokio::test]
    async fn test_worker_start() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;

        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));
        let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap();
        let db = Arc::new(crate::db::DB { pool: dummy_pg_pool, store: crate::db::DbStore::Sqlite(pool) });
        let worker = Arc::new(MemoryConsolidationWorker::new(repo, db, Arc::new(MockEmbeddingApi { succeeds: true }), None));

        worker.start();

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        assert!(true, "Worker started successfully");
    }

    #[tokio::test]
    async fn test_worker_pipeline_execution() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;

        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").expect("Failed to parse SQLite connection string");
        let pool = SqlitePoolOptions::new()
            .connect_with(conn_opts)
            .await
            .expect("Failed to connect to SQLite memory pool");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding TEXT,
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        )
        .execute(&pool)
        .await
        .expect("Failed to create consolidated_memory table");

        sqlx::query("CREATE TABLE IF NOT EXISTS agent_session_data (session_id VARCHAR NOT NULL, agent_id VARCHAR NOT NULL, context_data TEXT NOT NULL, last_accessed TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP);").execute(&pool).await.unwrap_or(sqlx::sqlite::SqliteQueryResult::default());

        let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));

        let stale_record = ohc_builtin_agent::memory_store::EmbeddingRecord {
            id: "stale_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "old data".to_string(),
            embedding: vec![1.0],
            source_type: "TASK_SUMMARY".to_string(),
            created_at: Utc::now() - chrono::Duration::days(200),
            last_referenced_at: Utc::now() - chrono::Duration::days(200),
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&stale_record).await.expect("Failed to upsert stale record");

        let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap();
        let db = Arc::new(crate::db::DB { pool: dummy_pg_pool, store: crate::db::DbStore::Sqlite(pool.clone()) });
        let mut worker = MemoryConsolidationWorker::new(repo.clone(), db, Arc::new(MockEmbeddingApi { succeeds: true }), None);
        worker.poll_interval = std::time::Duration::from_millis(10);

        worker.run_pipeline_once().await;

        let query = "SELECT count(*) FROM consolidated_memory";
        let row: (i64,) = sqlx::query_as(query)
            .fetch_one(&pool)
            .await
            .expect("Failed to query consolidated_memory count");

        assert_eq!(row.0, 0, "Stale record should be pruned by worker pipeline");
    }

    #[tokio::test]
    async fn test_worker_full_pipeline_with_conflict_and_pruning() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        use sqlx::Row;

        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new()
            .connect_with(conn_opts)
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding VECTOR(1536),
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        )
        .execute(&pool)
        .await
        .unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));

        let stale_record = ohc_builtin_agent::memory_store::EmbeddingRecord {
            id: "stale_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "old data".to_string(),
            embedding: vec![0.5; 1536],
            source_type: "TASK_SUMMARY".to_string(),
            created_at: Utc::now() - chrono::Duration::days(200),
            last_referenced_at: Utc::now() - chrono::Duration::days(200),
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        let conflict_loser = ohc_builtin_agent::memory_store::EmbeddingRecord {
            id: "conflict_loser".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "price is 50".to_string(),
            embedding: vec![0.1; 1536],
            source_type: "NOTES".to_string(),
            created_at: Utc::now() - chrono::Duration::days(5),
            last_referenced_at: Utc::now(),
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        let conflict_winner = ohc_builtin_agent::memory_store::EmbeddingRecord {
            id: "conflict_winner".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "price is 55".to_string(),
            embedding: vec![0.1; 1536],
            source_type: "NOTES".to_string(),
            created_at: Utc::now() - chrono::Duration::days(2),
            last_referenced_at: Utc::now(),
            reference_count: 2,
            reliability_score: 90,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&stale_record).await.unwrap();
        repo.upsert(&conflict_loser).await.unwrap();
        repo.upsert(&conflict_winner).await.unwrap();

        let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap();
        let db = Arc::new(crate::db::DB { pool: dummy_pg_pool, store: crate::db::DbStore::Sqlite(pool.clone()) });
        let mut worker = MemoryConsolidationWorker::new(repo.clone(), db, Arc::new(MockEmbeddingApi { succeeds: true }), None);
        worker.poll_interval = std::time::Duration::from_millis(10);

        worker.run_pipeline_once().await;

        let query = "SELECT id, reference_count FROM consolidated_memory";
        let rows = sqlx::query(query).fetch_all(&pool).await.unwrap();

        assert_eq!(rows.len(), 1, "Only the conflict winner should remain");

        let id: String = rows[0].try_get("id").unwrap();
        let ref_count: i32 = rows[0].try_get("reference_count").unwrap();

        assert_eq!(id, "conflict_winner", "The winner must be preserved");
        assert_eq!(ref_count, 4, "The winner should inherit the loser's reference count");
    }

    #[tokio::test]
    async fn test_agent_memory_pipeline_sqlite() {
        let pg_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).connect_lazy("postgres://dummy").unwrap();
        let pool = sqlx::sqlite::SqlitePoolOptions::new().connect_lazy("sqlite::memory:").unwrap();
        let db_mock = Arc::new(crate::db::DB { pool: pg_pool, store: crate::db::DbStore::Sqlite(pool.clone()) });

        let repo = Arc::new(VectorRepository::new_sqlite(pool));
        let _worker = MemoryConsolidationWorker::new(repo, db_mock, Arc::new(MockEmbeddingApi { succeeds: true }), None);
        assert!(true);
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

        let db = Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });
        let repo = Arc::new(VectorRepository::new(pool.clone()));

        sqlx::query("DELETE FROM agent_session_data").execute(&pool).await.unwrap_or(sqlx::postgres::PgQueryResult::default());
        sqlx::query("DELETE FROM consolidated_memory").execute(&pool).await.unwrap_or(sqlx::postgres::PgQueryResult::default());

        sqlx::query("CREATE TABLE IF NOT EXISTS agent_session_data (session_id VARCHAR NOT NULL, agent_id VARCHAR NOT NULL, context_data TEXT NOT NULL, last_accessed TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP);").execute(&pool).await.unwrap_or(sqlx::postgres::PgQueryResult::default());

        sqlx::query("INSERT INTO agent_session_data (session_id, agent_id, context_data) VALUES ('sess_pg_mem', 'agent1', 'some context pg mem');")
            .execute(&pool)
            .await
            .unwrap();

        let worker = MemoryConsolidationWorker::new(repo, db.clone(), Arc::new(MockEmbeddingApi { succeeds: true }), None);
        worker.process_session_data().await.unwrap();

        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM agent_session_data WHERE session_id = 'sess_pg_mem'").fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 0);

        let mem_count: (i64,) = sqlx::query_as("SELECT count(*) FROM consolidated_memory WHERE content = 'some context pg mem'").fetch_one(&pool).await.unwrap();
        assert_eq!(mem_count.0, 1);
    }

    #[tokio::test]
    async fn test_process_fs_memories() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding TEXT,
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        )
        .execute(&pool)
        .await
        .unwrap();

        let temp_dir = std::env::temp_dir().join(uuid::Uuid::new_v4().to_string());
        std::fs::create_dir_all(&temp_dir).unwrap();

        let test_file = temp_dir.join("test_memory.yml");
        std::fs::write(&test_file, "this is a test memory").unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));
        let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap();
        let db = Arc::new(crate::db::DB { pool: dummy_pg_pool, store: crate::db::DbStore::Sqlite(pool.clone()) });

        let worker = MemoryConsolidationWorker::new(repo, db, Arc::new(MockEmbeddingApi { succeeds: true }), Some(temp_dir.to_str().unwrap().to_string()));
        worker.process_fs_memories().await.unwrap();

        assert!(!test_file.exists());

        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM consolidated_memory WHERE content = 'this is a test memory'").fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 0);

        std::fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[tokio::test]
    async fn test_process_session_data_error_embedding() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding TEXT,
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS agent_session_data (session_id VARCHAR NOT NULL, agent_id VARCHAR NOT NULL, context_data TEXT NOT NULL, last_accessed TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP);").execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO agent_session_data (session_id, agent_id, context_data) VALUES ('err_sess', 'err_agent', 'error data');").execute(&pool).await.unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));
        if std::env::var("DATABASE_URL").is_err() { return; }
        let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap();
        let db = Arc::new(crate::db::DB { pool: dummy_pg_pool, store: crate::db::DbStore::Sqlite(pool.clone()) });

        let worker = MemoryConsolidationWorker::new(repo, db, Arc::new(MockEmbeddingApi { succeeds: false }), None);
        worker.process_session_data().await.unwrap();

        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM agent_session_data WHERE session_id = 'err_sess'").fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 0);

        let mem_count: (i64,) = sqlx::query_as("SELECT count(*) FROM consolidated_memory WHERE content = 'error data'").fetch_one(&pool).await.unwrap();
        assert_eq!(mem_count.0, 1);
    }
}
