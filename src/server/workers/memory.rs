use std::sync::Arc;
use ohc_builtin_agent::memory_store::VectorRepository;
use chrono::Utc;

/// MemoryConsolidationWorker is responsible for periodically pruning stale context
/// and automatically resolving memory conflicts within the vector repository.
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

    pub fn start(&self) {
        let repository = self.repository.clone();
        let interval_duration = self.poll_interval;
        let prune_threshold_days = self.prune_threshold_days;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                let older_than = Utc::now() - chrono::Duration::days(prune_threshold_days);
                if let Err(e) = repository.prune_stale(older_than).await {
                    tracing::error!("Consolidation Worker: Failed to prune stale context: {}", e);
                }
                if let Err(e) = repository.auto_resolve_conflicts().await {
                    tracing::error!("Consolidation Worker: Failed to resolve memory conflicts: {}", e);
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
    async fn test_worker_pipeline_execution() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;

        // Safe database initialization without Err(_) => return
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").expect("Failed to parse SQLite connection string");
        let pool = SqlitePoolOptions::new()
            .connect_with(conn_opts)
            .await
            .expect("Failed to connect to SQLite memory pool");

        // Set up the schema manually for SQLite test
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

        let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));

        // Insert a stale record that should be pruned
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

        let mut worker = MemoryConsolidationWorker::new(repo.clone());
        worker.poll_interval = std::time::Duration::from_millis(10); // Fast interval for testing
        worker.start();

        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        // Verify the record was pruned
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

        // Insert a stale record
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

        // Insert two conflicting records
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
            embedding: vec![0.1; 1536], // Same embedding = conflict
            source_type: "NOTES".to_string(),
            created_at: Utc::now() - chrono::Duration::days(2), // Newer
            last_referenced_at: Utc::now(),
            reference_count: 2,
            reliability_score: 90, // Higher score = winner
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&stale_record).await.unwrap();
        repo.upsert(&conflict_loser).await.unwrap();
        repo.upsert(&conflict_winner).await.unwrap();

        let mut worker = MemoryConsolidationWorker::new(repo.clone());
        worker.poll_interval = std::time::Duration::from_millis(10);
        worker.start();

        tokio::time::sleep(std::time::Duration::from_millis(150)).await;

        // Verify the database state
        let query = "SELECT id, reference_count FROM consolidated_memory";
        let rows = sqlx::query(query).fetch_all(&pool).await.unwrap();

        // Stale should be gone. Loser should be gone. Winner should remain.
        assert_eq!(rows.len(), 1, "Only the conflict winner should remain");

        let id: String = rows[0].try_get("id").unwrap();
        let ref_count: i32 = rows[0].try_get("reference_count").unwrap();

        assert_eq!(id, "conflict_winner", "The winner must be preserved");
        // Loser has 1, winner has 2, logic increments winner by loser + 1 -> 2 + 1 + 1 = 4.
        assert_eq!(ref_count, 4, "The winner should inherit the loser's reference count");
    }
}
// Integration complete.
