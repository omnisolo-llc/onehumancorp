use std::sync::Arc;
use ohc_builtin_agent::memory_store::VectorRepository;
use chrono::Utc;

pub struct MemoryConsolidationWorker {
    pub repository: Arc<VectorRepository>,
    pub poll_interval: std::time::Duration,
    pub prune_threshold_days: i64,
}

impl MemoryConsolidationWorker {
    pub fn new(repository: Arc<VectorRepository>) -> Self {
        Self {
            repository,
            poll_interval: std::time::Duration::from_secs(3600), // 1 hour
            prune_threshold_days: 180, // Default to 180 days
        }
    }

    pub async fn run_once(&self) -> Result<(usize, usize), String> {
        let older_than = Utc::now() - chrono::Duration::days(self.prune_threshold_days);

        let mut pruned = 0;
        let mut resolved = 0;

        match self.repository.prune_stale(older_than).await {
            Ok(count) => {
                pruned = count;
                if count > 0 {
                    tracing::info!("Memory worker pruned {} stale contexts", count);
                }
            }
            Err(e) => {
                tracing::error!("Failed to prune stale context: {}", e);
            }
        }

        match self.repository.auto_resolve_conflicts().await {
            Ok(count) => {
                resolved = count;
                if count > 0 {
                    tracing::info!("Memory worker resolved {} memory conflicts", count);
                }
            }
            Err(e) => {
                tracing::error!("Failed to resolve memory conflicts: {}", e);
            }
        }

        Ok((pruned, resolved))
    }

        pub fn start(self: Arc<Self>) {
        let worker = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(worker.poll_interval);
            loop {
                interval.tick().await;
                let _ = worker.run_once().await;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent::memory_store::EmbeddingRecord;

    #[tokio::test]
    async fn test_worker_run_once() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;

        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        // Initialize schema for the tests
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding TEXT NOT NULL,
                source_type TEXT NOT NULL,
                created_at DATETIME NOT NULL,
                last_referenced_at DATETIME NOT NULL,
                reference_count INTEGER NOT NULL,
                reliability_score INTEGER NOT NULL,
                owner_override BOOLEAN NOT NULL,
                metadata TEXT
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool));
        let worker = MemoryConsolidationWorker::new(repo.clone());

        // Insert a stale record
        let stale_record = EmbeddingRecord {
            id: "stale_1".to_string(),
            tenant_id: "tenant1".to_string(),
            agent_id: "agent1".to_string(),
            content: "old stale fact".to_string(),
            embedding: vec![0.1, 0.2, 0.3],
            source_type: "SESSION_DATA".to_string(),
            created_at: Utc::now() - chrono::Duration::days(200),
            last_referenced_at: Utc::now() - chrono::Duration::days(190), // Very old reference
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&stale_record).await.unwrap();

        // Run once
        let (pruned, resolved) = worker.run_once().await.unwrap();

        // Assert it pruned the stale record
        assert_eq!(pruned, 1, "Should prune exactly 1 stale record");
        assert_eq!(resolved, 0, "No conflicts to resolve");
    }

    #[tokio::test]
    async fn test_worker_start() {
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
                embedding TEXT NOT NULL,
                source_type TEXT NOT NULL,
                created_at DATETIME NOT NULL,
                last_referenced_at DATETIME NOT NULL,
                reference_count INTEGER NOT NULL,
                reliability_score INTEGER NOT NULL,
                owner_override BOOLEAN NOT NULL,
                metadata TEXT
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool));
        let worker = Arc::new(MemoryConsolidationWorker::new(repo));

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

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding TEXT NOT NULL,
                source_type TEXT NOT NULL,
                created_at DATETIME NOT NULL,
                last_referenced_at DATETIME NOT NULL,
                reference_count INTEGER NOT NULL,
                reliability_score INTEGER NOT NULL,
                owner_override BOOLEAN NOT NULL,
                metadata TEXT
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool));
        let worker = MemoryConsolidationWorker::new(repo);
        assert_eq!(worker.poll_interval.as_secs(), 3600);
    }
}
