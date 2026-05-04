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
                let older_than = Utc::now() - chrono::Duration::days(180);
                if let Err(e) = repository.prune_stale(older_than).await {
                    eprintln!("Failed to prune stale context: {}", e);
                }
                if let Err(e) = Self::resolve_conflicts(&repository).await {
                    eprintln!("Failed to resolve memory conflicts: {}", e);
                }
            }
        });
    }

    async fn resolve_conflicts(repository: &Arc<VectorRepository>) -> Result<(), String> {
        repository.auto_resolve_conflicts().await.map(|_| ()).map_err(|e| e.to_string())
    }
}
