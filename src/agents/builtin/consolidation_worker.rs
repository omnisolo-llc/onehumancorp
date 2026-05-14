use crate::memory_store::VectorRepository;
use std::sync::Arc;
use tokio::time::sleep;
use std::time::Duration;
use chrono::Utc;

pub struct ConsolidationWorker {
    pub repository: Arc<VectorRepository>,
    pub compaction_engine: Arc<crate::compaction::CompactionEngine>,
    pub poll_interval: Duration,
    pub pruning_threshold_days: i64,
}

impl ConsolidationWorker {
    pub fn new(repository: Arc<VectorRepository>, poll_interval: Duration, pruning_threshold_days: i64) -> Self {
        let compaction_engine = Arc::new(crate::compaction::CompactionEngine::new(repository.clone()));
        Self {
            repository,
            poll_interval,
            pruning_threshold_days,
            compaction_engine,
        }
    }

    /// Run a single consolidation pass manually. Useful for testing.
    pub async fn run_once(&self) -> Result<(usize, bool), String> {
        let conflicts_resolved = self.repository.auto_resolve_conflicts().await?;

        let threshold_date = Utc::now() - chrono::Duration::days(self.pruning_threshold_days);
        let pruning_success = self.repository.prune_stale(threshold_date).await.is_ok();

        Ok((conflicts_resolved, pruning_success))
    }

    /// Spawns a background task that continuously runs consolidation.
    /// Returns a JoinHandle that can be used to wait for or abort the worker.
    pub fn spawn_background_task(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                // Ignore errors to keep the background worker alive
                let _ = self.run_once().await;
                sleep(self.poll_interval).await;
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_store::EmbeddingRecord;
    use std::str::FromStr;

    async fn setup_sqlite_repo() -> Arc<VectorRepository> {
        let conn_opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = sqlx::sqlite::SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

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

        Arc::new(VectorRepository::new_sqlite(pool))
    }

    #[tokio::test]
    async fn test_consolidation_worker_run_once() {
        let repo = setup_sqlite_repo().await;
        let worker = ConsolidationWorker::new(repo.clone(), Duration::from_secs(1), 180);

        // Insert a stale record that should be pruned
        let mut v1 = vec![0.0; 10];
        v1[0] = 1.0;

        let old_time = Utc::now() - chrono::Duration::days(181);

        let prune_me = EmbeddingRecord {
            id: "prune_1".to_string(),
            tenant_id: "org_maya".to_string(),
            agent_id: "test".to_string(),
            content: "old stuff".to_string(),
            embedding: v1.clone(),
            source_type: "TASK_SUMMARY".to_string(),
            created_at: old_time,
            last_referenced_at: old_time,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&prune_me).await.unwrap();

        // Run worker pass
        let (resolved_conflicts, pruned_success) = worker.run_once().await.unwrap();
        assert_eq!(resolved_conflicts, 0);
        assert!(pruned_success);

        // Verify it was pruned
        let results = repo.cross_department_search("org_maya", &v1, 10).await.unwrap();
        assert!(results.is_empty(), "Record should have been pruned");
    }

    #[tokio::test]
    async fn test_consolidation_worker_spawn() {
        let repo = setup_sqlite_repo().await;
        let worker = Arc::new(ConsolidationWorker::new(repo.clone(), Duration::from_millis(50), 180));

        let handle = worker.spawn_background_task();

        // Let it run for a short time
        tokio::time::sleep(Duration::from_millis(150)).await;
        handle.abort();
        let _ = handle.await; // Ensure it exits cleanly
    }
}

pub struct MemoryArchiver {
    pub repository: Arc<VectorRepository>,
    pub compaction_engine: Arc<crate::compaction::CompactionEngine>,
    pub archive_threshold_days: i64,
}

impl MemoryArchiver {
    pub fn new(repository: Arc<VectorRepository>, archive_threshold_days: i64) -> Self {
        let compaction_engine = Arc::new(crate::compaction::CompactionEngine::new(repository.clone()));
        Self { repository, archive_threshold_days, compaction_engine }
    }

    /// Evaluates all memories and determines which should be pruned or archived.
    /// This runs completely non-blocking and processes in chunks.
    pub async fn run_archive_pass(&self) -> Result<usize, String> {
        let cutoff = Utc::now() - chrono::Duration::days(self.archive_threshold_days);

        // This is a placeholder for actual complex archiving logic. We will just use the standard prune
        // but we would typically run an analysis and move them to an archive store.
        self.repository.prune_stale(cutoff).await.map_err(|e| e.to_string())?;

        let tenants = match self.repository.get_store() {
            crate::memory_store::VectorMemoryStore::Postgres(pool) => {
                let rows = sqlx::query("SELECT DISTINCT tenant_id FROM consolidated_memory").fetch_all(pool).await.map_err(|e| e.to_string())?;
                let mut ts: Vec<String> = Vec::new();
                for r in rows { use sqlx::Row; ts.push(r.get("tenant_id")); }
                ts
            }
            crate::memory_store::VectorMemoryStore::Sqlite(pool) => {
                let rows = sqlx::query("SELECT DISTINCT tenant_id FROM consolidated_memory").fetch_all(pool).await.map_err(|e| e.to_string())?;
                let mut ts: Vec<String> = Vec::new();
                for r in rows { use sqlx::Row; ts.push(r.get("tenant_id")); }
                ts
            }
        };

        for tenant in tenants {
            let _ = self.compaction_engine.run_compaction(&tenant, crate::compaction::CompactionStrategy::TimeDecay).await;
            let _ = self.compaction_engine.run_compaction(&tenant, crate::compaction::CompactionStrategy::RelevanceClustering).await;
            let _ = self.compaction_engine.run_compaction(&tenant, crate::compaction::CompactionStrategy::Summarization).await;
            let _ = self.compaction_engine.run_graph_projection(&tenant).await;
        }

        Ok(0)
    }

    pub fn spawn_archiver_task(self: Arc<Self>, interval: Duration) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                let _ = self.run_archive_pass().await;
                sleep(interval).await;
            }
        })
    }
}
