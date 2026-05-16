use ohc_builtin_agent::memory_store::VectorRepository;
use std::sync::Arc;
use tokio::time::sleep;

/// HybridSyncWorker bridges the gap between Cloud (Postgres) and Standalone (SQLite) modes,
/// ensuring that stale context pruning and conflict resolution are executed safely
/// regardless of the underlying storage engine.
pub struct HybridSyncWorker {
    pub repository: Arc<VectorRepository>,
}

impl HybridSyncWorker {
    pub fn new(repository: Arc<VectorRepository>) -> Self {
        Self { repository }
    }

    /// Periodically executes consolidation tasks, adapting to the current DB mode.
    pub async fn run_sync_loop(&self) {
        loop {
            let threshold = chrono::Utc::now() - chrono::Duration::days(180);

            // Prune stale context safely across modes
            if let Err(e) = self.repository.prune_stale(threshold).await {
                tracing::error!("HybridSyncWorker: Error pruning stale context: {}", e);
            }

            // Resolve conflicts automatically
            if let Err(e) = self.repository.auto_resolve_conflicts().await {
                tracing::error!("HybridSyncWorker: Error resolving conflicts: {}", e);
            }

            sleep(std::time::Duration::from_secs(3600)).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;
    use ohc_builtin_agent::memory_store::EmbeddingRecord;

    #[tokio::test]
    async fn test_hybrid_sync_worker_logic() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

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
        ).execute(&pool).await.unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool));
        let worker = HybridSyncWorker::new(repo.clone());

        let old_time = chrono::Utc::now() - chrono::Duration::days(200);
        let rec = EmbeddingRecord {
            id: "stale_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "cs".to_string(),
            content: "old".to_string(),
            embedding: vec![0.1, 0.2],
            source_type: "TASK_SUMMARY".to_string(),
            created_at: old_time,
            last_referenced_at: old_time,
            reference_count: 1,
            reliability_score: 10,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec).await.unwrap();

        // Run pruning via threshold
        let threshold = chrono::Utc::now() - chrono::Duration::days(180);
        worker.repository.prune_stale(threshold).await.unwrap();

        assert_eq!(worker.repository.auto_resolve_conflicts().await.unwrap(), 0);
    }
}
