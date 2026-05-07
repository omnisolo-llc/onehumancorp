use std::sync::Arc;
use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
use chrono::Utc;

pub struct CrossDepartmentMemoryLayer {
    pub repository: Arc<VectorRepository>,
}

impl CrossDepartmentMemoryLayer {
    pub fn new(repository: Arc<VectorRepository>) -> Self {
        Self { repository }
    }

    /// Stores a new memory into the repository in the background.
    pub async fn store_memory(
        &self,
        tenant_id: &str,
        agent_id: &str,
        content: &str,
        embedding: Vec<f32>,
        source_type: &str,
        reliability_score: i32,
    ) -> Result<(), String> {
        let record = EmbeddingRecord {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            agent_id: agent_id.to_string(),
            content: content.to_string(),
            embedding,
            source_type: source_type.to_string(),
            created_at: Utc::now(),
            last_referenced_at: Utc::now(),
            reference_count: 1,
            reliability_score,
            owner_override: false,
            metadata: None,
        };
        let repo = self.repository.clone();
        tokio::spawn(async move {
            if let Err(e) = repo.upsert(&record).await {
                tracing::error!("Failed to background store memory: {}", e);
            }
        });
        Ok(())
    }

    /// Searches for relevant memories across all departments for a given tenant.
    pub async fn search_cross_department(
        &self,
        tenant_id: &str,
        query_embedding: &[f32],
        limit: i64,
    ) -> Result<Vec<EmbeddingRecord>, String> {
        // We rely on VectorRepository's semantic_search, which is already tenant-scoped
        // and returns memories regardless of the agent_id.
        // semantic_search ALREADY updates the reference count and last_referenced_at in the database.
        self.repository.semantic_search(tenant_id, query_embedding, limit).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    async fn setup_test_layer() -> CrossDepartmentMemoryLayer {
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
        CrossDepartmentMemoryLayer::new(repo)
    }

    #[tokio::test]
    async fn test_store_and_search_cross_department() {
        let layer = setup_test_layer().await;

        let embedding1 = vec![0.1; 1536];
        layer.store_memory(
            "tenant_a",
            "operations_agent",
            "Vegan cake ingredients cost $10.",
            embedding1.clone(),
            "TASK_SUMMARY",
            80,
        ).await.unwrap();

        let embedding2 = vec![0.2; 1536];
        layer.store_memory(
            "tenant_a",
            "customer_success_agent",
            "Customer loves vegan cake.",
            embedding2.clone(),
            "SESSION_DATA",
            90,
        ).await.unwrap();

        // Wait for background tasks to execute
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Search from advisory_agent
        let results = layer.search_cross_department("tenant_a", &embedding1, 5).await.unwrap();

        assert_eq!(results.len(), 2);

        // Query directly to check reference count
        let after_update_results = layer.repository.semantic_search("tenant_a", &embedding1, 5).await.unwrap();

        assert_eq!(after_update_results[0].reference_count, 2);
    }
}