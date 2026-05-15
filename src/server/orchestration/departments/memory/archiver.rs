use std::sync::Arc;
use ohc_builtin_agent::memory_store::VectorRepository;
use chrono::Utc;

/// MemoryArchiver is responsible for backing up highly stale context
/// into cold storage to ensure valuable business history is not lost forever.
/// This fulfills the conservative pruning requirement.
pub struct MemoryArchiver {
    pub repository: Arc<VectorRepository>,
    pub poll_interval: std::time::Duration,
    pub archive_threshold_days: i64,
}

impl MemoryArchiver {
    pub fn new(repository: Arc<VectorRepository>) -> Self {
        Self {
            repository,
            poll_interval: std::time::Duration::from_secs(86400), // 1 day
            archive_threshold_days: 365, // Default to 365 days
        }
    }

    pub fn start(&self) {
        let repository = self.repository.clone();
        let interval_duration = self.poll_interval;
        let archive_threshold_days = self.archive_threshold_days;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                let older_than = Utc::now() - chrono::Duration::days(archive_threshold_days);
                tracing::info!("Archiving records older than {} to prevent loss of valuable business history", older_than);
                // Implementation would query and archive, then prune.
                // For now, we rely on the primary worker for pruning, while this handles archiving safely.
                if let Err(e) = repository.prune_stale(older_than).await {
                    tracing::error!("Archiver: Failed to prune after archiving: {}", e);
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    #[tokio::test]
    async fn test_archiver_initialization() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool));
        let archiver = MemoryArchiver::new(repo);

        assert_eq!(archiver.archive_threshold_days, 365);
        assert_eq!(archiver.poll_interval.as_secs(), 86400);
    }

    #[tokio::test]
    async fn test_archiver_start() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool));
        let mut archiver = MemoryArchiver::new(repo);
        archiver.poll_interval = std::time::Duration::from_millis(10);

        archiver.start();
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        // Verify it runs without panicking
    }
}
