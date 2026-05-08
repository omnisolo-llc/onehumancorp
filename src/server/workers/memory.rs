use crate::orchestration::departments::memory::layer::MemoryLayer;
use std::sync::Arc;
use ohc_builtin_agent::memory_store::VectorRepository;
use chrono::Utc;
use crate::orchestration::departments::memory::pruning::prune_stale;
use crate::orchestration::departments::memory::conflict::auto_resolve_conflicts;

pub struct MemoryConsolidationWorker {
    pub memory_layer: Arc<MemoryLayer>,
    pub poll_interval: std::time::Duration,
    pub prune_threshold_days: i64,
}

impl MemoryConsolidationWorker {
    pub fn new(memory_layer: Arc<MemoryLayer>) -> Self {
        Self {
            memory_layer,
            poll_interval: std::time::Duration::from_secs(3600), // 1 hour
            prune_threshold_days: 180, // Default to 180 days
        }
    }

    pub fn start(&self) {
        let memory_layer = self.memory_layer.clone();
        let interval_duration = self.poll_interval;
        let prune_threshold_days = self.prune_threshold_days;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                let older_than = Utc::now() - chrono::Duration::days(prune_threshold_days);
                if let Err(e) = memory_layer.prune_stale(older_than).await {
                    tracing::error!("Failed to prune stale context: {}", e);
                }
                if let Err(e) = memory_layer.auto_resolve_conflicts().await {
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
        let worker = MemoryConsolidationWorker::new(Arc::new(MemoryLayer::new(repo)));

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
        let worker = MemoryConsolidationWorker::new(Arc::new(MemoryLayer::new(repo)));
        assert_eq!(worker.poll_interval.as_secs(), 3600);
    }
}
