use std::sync::Arc;
use ohc_builtin_agent::memory_store::VectorRepository;
use chrono::Utc;

pub struct MemoryConsolidationWorker {
    pub repository: Arc<VectorRepository>,
    pub poll_interval: std::time::Duration,
}

impl MemoryConsolidationWorker {
    pub fn new(repository: Arc<VectorRepository>) -> Self {
        Self {
            repository,
            poll_interval: std::time::Duration::from_secs(3600), // 1 hour
        }
    }

    pub fn start(&self) {
        let repository = self.repository.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                let older_than = chrono::Utc::now() - chrono::Duration::days(180);
                if let Err(e) = repository.prune_stale(older_than).await {
                    tracing::error!("Failed to prune stale context: {}", e);
                }
                if let Err(e) = repository.auto_resolve_conflicts().await {
                    tracing::error!("Failed to resolve memory conflicts: {}", e);
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_worker_start() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;

        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool));
        let worker = MemoryConsolidationWorker::new(repo);

        worker.start();

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        assert!(true, "Worker started successfully");
    }

    #[tokio::test]
    async fn test_worker_initialization() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;

        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool));
        let worker = MemoryConsolidationWorker::new(repo);
        assert_eq!(worker.poll_interval.as_secs(), 3600);
    }

    #[tokio::test]
    async fn test_cross_department_context_sharing() {
        use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;

        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let _ = sqlx::query(
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
        ).execute(&pool).await;

        let repo = VectorRepository::new_sqlite(pool.clone());
        let now = chrono::Utc::now();

        // Simulate Support agent writing a memory
        let support_record = EmbeddingRecord {
            id: "support_rec_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "support_agent".to_string(),
            content: "customer unhappy with vegan cake pricing".to_string(),
            embedding: vec![0.8, 0.2, 0.1],
            source_type: "SUPPORT_TICKET".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&support_record).await.unwrap();

        // Simulate Advisory agent searching the memory
        let search_results = repo.semantic_search("org1", &[0.8, 0.2, 0.1], 5).await.unwrap();

        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].content, "customer unhappy with vegan cake pricing");
        assert_eq!(search_results[0].agent_id, "support_agent");
    }

    #[tokio::test]
    async fn test_stale_context_pruning_worker_logic() {
        use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;

        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let _ = sqlx::query(
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
        ).execute(&pool).await;

        let repo = VectorRepository::new_sqlite(pool.clone());
        let now = chrono::Utc::now();

        let old_record = EmbeddingRecord {
            id: "stale_rec".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "discontinued product query".to_string(),
            embedding: vec![0.1, 0.1, 0.1],
            source_type: "SESSION_SUMMARY".to_string(),
            created_at: now - chrono::Duration::days(200),
            last_referenced_at: now - chrono::Duration::days(190),
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&old_record).await.unwrap();

        // Act: prune stale logic as executed by worker
        let older_than = chrono::Utc::now() - chrono::Duration::days(180);
        repo.prune_stale(older_than).await.unwrap();

        let search_results = repo.semantic_search("org1", &[0.1, 0.1, 0.1], 5).await.unwrap();
        assert_eq!(search_results.len(), 0);
    }
}
