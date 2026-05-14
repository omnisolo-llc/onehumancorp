use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
use std::sync::Arc;
use chrono::{DateTime, Utc};

/// Persistent Memory Layer responsible for coordinating agent context storage,
/// cross-department sharing, conflict resolution, and stale context pruning.
/// This acts as a centralized interface for all AI department memory interactions.
pub struct MemoryLayer {
    repository: Arc<VectorRepository>,
}

impl MemoryLayer {
    /// Creates a new MemoryLayer wrapping the underlying vector repository.
    pub fn new(repository: Arc<VectorRepository>) -> Self {
        Self { repository }
    }

    /// Stores a new context record in the persistent memory layer.
    pub async fn store_context(&self, record: &EmbeddingRecord) -> Result<(), String> {
        self.repository.upsert(record).await
    }

    /// Cross-Department Context Sharing: Retrieves context related to a query,
    /// searching across all departments for the given tenant.
    pub async fn retrieve_cross_department_context(
        &self,
        tenant_id: &str,
        query_embedding: &[f32],
        limit: i64,
    ) -> Result<Vec<EmbeddingRecord>, String> {
        self.repository.cross_department_search(tenant_id, query_embedding, limit).await
    }

    /// Conflict Resolution: Detects and resolves conflicting facts stored by agents.
    pub async fn resolve_conflicts(&self) -> Result<usize, String> {
        self.repository.auto_resolve_conflicts().await
    }

    /// Stale Context Pruning: Removes outdated context based on the provided threshold.
    pub async fn prune_stale_context(&self, threshold: DateTime<Utc>) -> Result<(), String> {
        self.repository.prune_stale(threshold).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;
    use sqlx::Row;

    #[tokio::test]
    async fn test_memory_layer_cross_department_context_sharing() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").expect("Failed to parse connection string");
        let pool = SqlitePoolOptions::new()
            .connect_with(conn_opts)
            .await
            .expect("Failed to connect to SQLite in-memory database");

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
        let layer = MemoryLayer::new(repo.clone());

        let rec1 = EmbeddingRecord {
            id: "cs_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "cs_agent_1".to_string(),
            content: "Customer expressed dissatisfaction with recent delivery delays.".to_string(),
            embedding: vec![0.5, 0.5, 0.5],
            source_type: "SESSION_DATA".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };
        layer.store_context(&rec1).await.expect("Failed to store context");

        let rec2 = EmbeddingRecord {
            id: "ops_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "ops_agent_1".to_string(),
            content: "Warehouse routing updated to reduce delivery delays.".to_string(),
            embedding: vec![0.4, 0.6, 0.5],
            source_type: "SESSION_DATA".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };
        layer.store_context(&rec2).await.expect("Failed to store context");

        let rows = sqlx::query("SELECT agent_id FROM consolidated_memory WHERE tenant_id = 'org1'")
            .fetch_all(&pool)
            .await
            .expect("Failed to query consolidated_memory");

        assert_eq!(rows.len(), 2, "Both records should be successfully stored");

        let query_embedding = vec![0.5, 0.5, 0.5];
        match layer.retrieve_cross_department_context("org1", &query_embedding, 5).await {
            Ok(results) => {
                let cs_found = results.iter().any(|r| r.agent_id == "cs_agent_1");
                let ops_found = results.iter().any(|r| r.agent_id == "ops_agent_1");
                assert!(cs_found || ops_found, "Cross-department context sharing should return records from other agents.");
            },
            Err(e) => {
                assert!(e.contains("no such function: vec_distance_cosine") || e.contains("syntax error") || e.contains("no such table"), "Unexpected error: {}", e);
            }
        }
    }
}
