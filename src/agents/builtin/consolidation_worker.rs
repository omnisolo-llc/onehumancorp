use crate::memory_store::VectorRepository;
use chrono::Utc;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub struct ConsolidationWorker {
    pub repository: Arc<VectorRepository>,
    pub poll_interval: Duration,
    pub pruning_threshold_days: i64,
    pub telemetry_error_callback: Option<Arc<dyn Fn(&str, &str) + Send + Sync>>,
}

impl ConsolidationWorker {
    pub fn new(
        repository: Arc<VectorRepository>,
        poll_interval: Duration,
        pruning_threshold_days: i64,
        telemetry_error_callback: Option<Arc<dyn Fn(&str, &str) + Send + Sync>>,
    ) -> Self {
        Self {
            repository,
            poll_interval,
            pruning_threshold_days,
            telemetry_error_callback,
        }
    }

    /// Run a single consolidation pass manually. Useful for testing.
    pub async fn run_once(&self) -> Result<(usize, bool), String> {
        let conflicts_resolved = match self.repository.auto_resolve_conflicts().await {
            Ok(count) => count,
            Err(e) => {
                tracing::error!("Consolidation Worker: Failed to resolve memory conflicts: {}", e);
                if let Some(ref cb) = self.telemetry_error_callback {
                    cb("Consolidation Worker: Failed to resolve memory conflicts", &e);
                }
                return Err(e);
            }
        };

        let threshold_date = Utc::now() - chrono::Duration::days(self.pruning_threshold_days);
        let pruning_success = match self.repository.prune_stale(threshold_date).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("Consolidation Worker: Failed to prune stale context: {}", e);
                if let Some(ref cb) = self.telemetry_error_callback {
                    cb("Consolidation Worker: Failed to prune stale context", &e);
                }
                return Err(e);
            }
        };

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
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect_with(conn_opts)
            .await
            .unwrap();

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
            );",
        )
        .execute(&pool)
        .await
        .unwrap();

        Arc::new(VectorRepository::new_sqlite(pool))
    }

    #[tokio::test]
    async fn test_consolidation_worker_run_once() {
        let repo = setup_sqlite_repo().await;
        let worker = ConsolidationWorker::new(repo.clone(), Duration::from_secs(1), 180, None);

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
        let results = repo
            .cross_department_search("org_maya", &v1, 10)
            .await
            .unwrap();
        assert!(results.is_empty(), "Record should have been pruned");
    }

    #[test]
    fn test_determine_conflict_winner_all_branches() {
        use chrono::{Utc, TimeZone};
        let ts_early = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let ts_late = Utc.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap();

        let base_record = EmbeddingRecord {
            id: "1".to_string(),
            tenant_id: "t1".to_string(),
            agent_id: "a1".to_string(),
            content: "c".to_string(),
            embedding: vec![0.0],
            source_type: "s".to_string(),
            created_at: ts_early,
            last_referenced_at: ts_early,
            reference_count: 0,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        // Branch 1: Owner override wins
        let mut rec_a = base_record.clone();
        rec_a.id = "A".to_string();
        rec_a.owner_override = true;

        let mut rec_b = base_record.clone();
        rec_b.id = "B".to_string();
        rec_b.owner_override = false;
        rec_b.reliability_score = 100; // Even with higher score, override wins

        let (winner, loser) = VectorRepository::determine_conflict_winner(&rec_a, &rec_b);
        assert_eq!(winner.id, "A");
        assert_eq!(loser.id, "B");

        // Swap order
        let (winner, loser) = VectorRepository::determine_conflict_winner(&rec_b, &rec_a);
        assert_eq!(winner.id, "A");
        assert_eq!(loser.id, "B");

        // Branch 2: Reliability score wins (no overrides)
        let mut rec_a2 = base_record.clone();
        rec_a2.id = "A2".to_string();
        rec_a2.reliability_score = 60;

        let mut rec_b2 = base_record.clone();
        rec_b2.id = "B2".to_string();
        rec_b2.reliability_score = 90;

        let (winner, loser) = VectorRepository::determine_conflict_winner(&rec_a2, &rec_b2);
        assert_eq!(winner.id, "B2");
        assert_eq!(loser.id, "A2");

        // Branch 3: Recency wins (no overrides, tied score)
        let mut rec_a3 = base_record.clone();
        rec_a3.id = "A3".to_string();
        rec_a3.created_at = ts_late; // newer

        let mut rec_b3 = base_record.clone();
        rec_b3.id = "B3".to_string();
        rec_b3.created_at = ts_early; // older

        let (winner, loser) = VectorRepository::determine_conflict_winner(&rec_a3, &rec_b3);
        assert_eq!(winner.id, "A3");
        assert_eq!(loser.id, "B3");

        // Branch 4: ID tiebreaker (no overrides, tied score, tied time)
        let mut rec_a4 = base_record.clone();
        rec_a4.id = "Z".to_string();

        let mut rec_b4 = base_record.clone();
        rec_b4.id = "A".to_string();

        let (winner, loser) = VectorRepository::determine_conflict_winner(&rec_a4, &rec_b4);
        assert_eq!(winner.id, "A");
        assert_eq!(loser.id, "Z");
    }

    #[tokio::test]
    async fn test_consolidation_worker_spawn() {
        let repo = setup_sqlite_repo().await;
        let worker = Arc::new(ConsolidationWorker::new(
            repo.clone(),
            Duration::from_millis(50),
            180,
            None
        ));

        let handle = worker.spawn_background_task();

        // Let it run for a short time
        tokio::time::sleep(Duration::from_millis(150)).await;
        handle.abort();
        let _ = handle.await; // Ensure it exits cleanly
    }

    #[tokio::test]
    async fn test_worker_full_pipeline_with_conflict_and_pruning() {
        use sqlx::Row;

        let repo = setup_sqlite_repo().await;

        // Insert a stale record
        let stale_record = crate::memory_store::EmbeddingRecord {
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
        let conflict_loser = crate::memory_store::EmbeddingRecord {
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

        let conflict_winner = crate::memory_store::EmbeddingRecord {
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

        let worker = Arc::new(ConsolidationWorker::new(repo.clone(), std::time::Duration::from_millis(10), 180, None));
        let handle = worker.spawn_background_task();

        tokio::time::sleep(std::time::Duration::from_millis(150)).await;

        // Verify the database state
        let pool = match &repo.get_store() {
            crate::memory_store::VectorMemoryStore::Sqlite(p) => p.clone(),
            _ => panic!("Expected Sqlite pool"),
        };
        let query = "SELECT id, reference_count FROM consolidated_memory";
        let rows = sqlx::query(query).fetch_all(&pool).await.unwrap();

        // Stale should be gone. Loser should be gone. Winner should remain.
        assert_eq!(rows.len(), 1, "Only the conflict winner should remain");

        let id: String = rows[0].try_get("id").unwrap();
        let ref_count: i32 = rows[0].try_get("reference_count").unwrap();

        assert_eq!(id, "conflict_winner", "The winner must be preserved");
        // Loser has 1, winner has 2, logic increments winner by loser + 1 -> 2 + 1 + 1 = 4.
        assert_eq!(ref_count, 4, "The winner should inherit the loser's reference count");
        handle.abort();
    }
}
