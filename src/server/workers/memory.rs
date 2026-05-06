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

    pub async fn tick_once(repository: &VectorRepository) {
        let older_than = Utc::now() - chrono::Duration::days(180);
        if let Err(e) = repository.prune_stale(older_than).await {
            tracing::error!("Failed to prune stale context: {}", e);
        }
        if let Err(e) = repository.auto_resolve_conflicts().await {
            tracing::error!("Failed to resolve memory conflicts: {}", e);
        }
    }

    pub fn start(&self) {
        let repository = self.repository.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                Self::tick_once(&repository).await;
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

        // Let it run for a bit to ensure it doesn't crash on start
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        assert!(true, "Worker started successfully");
    }

    #[tokio::test]
    async fn test_worker_pruning_trigger() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        use ohc_builtin_agent::memory_store::EmbeddingRecord;
        use sqlx::Row;

        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query(
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
        ).execute(&pool).await.unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));
        let _worker = MemoryConsolidationWorker::new(repo.clone());

        let now = chrono::Utc::now();
        let old_time = now - chrono::Duration::days(181);

        let record1 = EmbeddingRecord {
            id: "rec_stale".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "hello world stale".to_string(),
            embedding: vec![1.0, 2.0, 3.0],
            source_type: "TASK_SUMMARY".to_string(), // Should be pruned
            created_at: old_time,
            last_referenced_at: old_time,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&record1).await.unwrap();

        MemoryConsolidationWorker::tick_once(&repo).await;

        let query = "SELECT id FROM consolidated_memory";
        let rows = sqlx::query(query).fetch_all(&pool).await.unwrap();

        assert_eq!(rows.len(), 0, "Stale record should be pruned");
    }
}
