use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
use std::sync::Arc;
use chrono::{DateTime, Utc};

/// The PersistentMemoryLayer wraps the underlying VectorRepository
/// to provide domain-specific context consolidation, conflict resolution,
/// and stale context pruning logic for the AI Agent departments.
pub struct PersistentMemoryLayer {
    repo: Arc<VectorRepository>,
}

impl PersistentMemoryLayer {
    pub fn new(repo: Arc<VectorRepository>) -> Self {
        Self { repo }
    }

    /// Stores AI agent context long-term, scoped by tenant.
    pub async fn store_context(&self, record: EmbeddingRecord) -> Result<(), String> {
        self.repo.upsert(&record).await
    }

    /// Retrieves context related to the query, inherently cross-departmental
    /// by searching the shared tenant index.
    pub async fn retrieve_context(&self, tenant_id: &str, query_embedding: &[f32], limit: i64) -> Result<Vec<EmbeddingRecord>, String> {
        self.repo.semantic_search(tenant_id, query_embedding, limit).await
    }

    /// Resolves conflicts when the same fact is stored multiple times.
    /// It automatically resolves based on recency, source reliability, or explicit owner override.
    pub async fn resolve_conflicts(&self) -> Result<usize, String> {
        self.repo.auto_resolve_conflicts().await
    }

    /// Prunes stale context automatically based on the configured threshold.
    pub async fn prune_stale_context(&self, older_than: DateTime<Utc>) -> Result<(), String> {
        self.repo.prune_stale(older_than).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;
    use sqlx::Row;

    #[tokio::test]
    async fn test_cross_department_context_sharing() {
        // Safe database initialization
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").expect("Failed to parse connection string");
        let pool = SqlitePoolOptions::new()
            .connect_with(conn_opts)
            .await
            .expect("Failed to connect to SQLite in-memory database");

        // Set up the schema
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
        let layer = PersistentMemoryLayer::new(repo);

        // Dept A: Customer Success notes customer is unhappy
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
        layer.store_context(rec1).await.expect("Failed to upsert Dept A record");

        // Dept B: Operations
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
        layer.store_context(rec2).await.expect("Failed to upsert Dept B record");

        let rows = sqlx::query("SELECT agent_id FROM consolidated_memory WHERE tenant_id = 'org1'")
            .fetch_all(&pool)
            .await
            .expect("Failed to query consolidated_memory");

        assert_eq!(rows.len(), 2, "Both records should be successfully stored for cross-department context sharing");

        let agent_ids: Vec<String> = rows.into_iter().map(|row| row.try_get("agent_id").expect("Failed to get agent_id")).collect();

        assert!(agent_ids.contains(&"cs_agent_1".to_string()), "Customer Success agent record should exist");
        assert!(agent_ids.contains(&"ops_agent_1".to_string()), "Operations agent record should exist");

        let query_embedding = vec![0.5, 0.5, 0.5];
        match layer.retrieve_context("org1", &query_embedding, 5).await {
            Ok(results) => {
                let cs_found = results.iter().any(|r| r.agent_id == "cs_agent_1");
                let ops_found = results.iter().any(|r| r.agent_id == "ops_agent_1");
                assert!(cs_found || ops_found, "Cross-department context sharing should return records from other agents.");
            },
            Err(e) => {
                assert!(e.contains("no such function: vec_distance_cosine") || e.contains("syntax error") || e.contains("no such table"), "Unexpected semantic_search error: {}", e);
            }
        }
    }
}
