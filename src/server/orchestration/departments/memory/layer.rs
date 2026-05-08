use std::sync::Arc;
use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
use chrono::{DateTime, Utc};

pub struct MemoryLayer {
    pub repository: Arc<VectorRepository>,
}

impl MemoryLayer {
    pub fn new(repository: Arc<VectorRepository>) -> Self {
        Self { repository }
    }

    /// Stores a new piece of context in the memory layer.
    /// Uses a background worker (tokio::spawn) to avoid blocking the main AI request path.
    /// This context becomes available across all departments within the same tenant.
    pub fn store_context(&self, record: EmbeddingRecord) {
        let repo = self.repository.clone();
        tokio::spawn(async move {
            if let Err(e) = repo.upsert(&record).await {
                tracing::error!("Failed to store context in background: {}", e);
            }
        });
    }

    /// Retrieves context related to the given embedding for the specified tenant.
    /// This ignores agent_id and department boundaries, enabling cross-department sharing.
    pub async fn retrieve_cross_department_context(&self, tenant_id: &str, query_embedding: &[f32], limit: i64) -> Result<Vec<EmbeddingRecord>, String> {
        self.repository.semantic_search(tenant_id, query_embedding, limit).await
    }

    /// Resolves conflicts automatically based on recency, source reliability, or explicit owner override.
    pub async fn auto_resolve_conflicts(&self) -> Result<usize, String> {
        crate::orchestration::departments::memory::conflict::auto_resolve_conflicts(self.repository.clone()).await
    }

    /// Periodically removes or archives context that is no longer relevant.
    pub async fn prune_stale(&self, older_than: DateTime<Utc>) -> Result<(), String> {
        crate::orchestration::departments::memory::pruning::prune_stale(self.repository.clone(), older_than).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;
    use ohc_builtin_agent::memory_store::VectorRepository;

    #[tokio::test]
    async fn test_memory_layer_cross_department_sharing() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query(
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
        ).execute(&pool).await.unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool));
        let layer = MemoryLayer::new(repo.clone());

        let record_dept_a = EmbeddingRecord {
            id: "1".to_string(),
            tenant_id: "tenant_1".to_string(),
            agent_id: "dept_a".to_string(),
            content: "Customer is unhappy".to_string(),
            embedding: vec![1.0],
            source_type: "SUPPORT_TICKET".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };

        layer.store_context(record_dept_a.clone());

        // Wait for background task to complete
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let results = layer.retrieve_cross_department_context("tenant_1", &[1.0], 5).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].content, "Customer is unhappy");
        assert_eq!(results[0].agent_id, "dept_a");

        let results_other_tenant = layer.retrieve_cross_department_context("tenant_2", &[1.0], 5).await.unwrap();
        assert_eq!(results_other_tenant.len(), 0);
    }
}
