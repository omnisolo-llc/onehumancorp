use crate::memory_store::VectorRepository;
use crate::consolidation_agent::ConsolidationAgent;
use ohc_builtin_agent_core::types::EmbeddingRecord;
use std::sync::Arc;
use tokio::time::sleep;
use std::time::Duration;
use chrono::Utc;

pub struct ConsolidationWorker {
    pub repository: Arc<VectorRepository>,
    pub consolidation_agent: Option<Arc<ConsolidationAgent>>,
    pub poll_interval: Duration,
    pub pruning_threshold_days: i64,
}

impl ConsolidationWorker {
    pub fn new(repository: Arc<VectorRepository>, poll_interval: Duration, pruning_threshold_days: i64) -> Self {
        Self { repository, consolidation_agent: None, poll_interval, pruning_threshold_days }
    }

    pub fn with_agent(mut self, agent: Arc<ConsolidationAgent>) -> Self {
        self.consolidation_agent = Some(agent);
        self
    }

    pub async fn run_once(&self) -> Result<(usize, bool, u64), String> {
        let mut resolved_count = 0;

        // 1. Intelligent consolidation (merging) runs FIRST to preserve information
        if let Some(agent) = &self.consolidation_agent {
            if let Ok(tenants) = self.repository.get_active_tenants().await {
                for tenant in tenants {
                    if let Ok(count) = agent.auto_consolidate(&tenant).await {
                        resolved_count += count;
                    }
                }
            }
        }

        // 2. Rule-based resolution (picking winners) runs SECOND for remaining simple conflicts
        resolved_count += self.repository.auto_resolve_conflicts().await?;

        let threshold_date = Utc::now() - chrono::Duration::days(self.pruning_threshold_days);
        let archived_count = self.repository.archive_stale(threshold_date).await.unwrap_or(0);
        let pruning_success = self.repository.prune_stale(threshold_date).await.is_ok();
        Ok((resolved_count, pruning_success, archived_count))
    }

    pub fn spawn_background_task(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                let _ = self.run_once().await;
                sleep(self.poll_interval).await;
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
                archived BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await.unwrap();
        Arc::new(VectorRepository::new_sqlite(pool))
    }

    #[tokio::test]
    async fn test_consolidation_worker_run_once() {
        let repo = setup_sqlite_repo().await;
        let worker = ConsolidationWorker::new(repo.clone(), Duration::from_secs(1), 180);
        let mut v1 = vec![0.0; 10]; v1[0] = 1.0;
        let old_time = Utc::now() - chrono::Duration::days(181);
        let prune_me = EmbeddingRecord {
            id: "prune_1".to_string(), tenant_id: "org_maya".to_string(), agent_id: "test".to_string(),
            content: "old stuff".to_string(), embedding: v1.clone(), source_type: "TASK_SUMMARY".to_string(),
            created_at: old_time, last_referenced_at: old_time, reference_count: 1, reliability_score: 50,
            owner_override: false, archived: true, metadata: None,
        };
        repo.upsert(&prune_me).await.unwrap();
        let (_, pruned, _) = worker.run_once().await.unwrap();
        assert!(pruned);
        let results = repo.semantic_search("org_maya", &v1, 10).await.unwrap();
        assert!(results.is_empty());
    }
}
