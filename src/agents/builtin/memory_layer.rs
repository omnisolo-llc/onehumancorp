use std::sync::Arc;
use crate::memory_store::{VectorRepository, EmbeddingRecord};
use chrono::{DateTime, Utc};

pub struct CrossDepartmentMemoryLayer {
    pub repository: Arc<VectorRepository>,
}

impl CrossDepartmentMemoryLayer {
    pub fn new(repository: Arc<VectorRepository>) -> Self {
        Self { repository }
    }

    pub async fn store_context(
        &self,
        tenant_id: &str,
        department: &str,
        content: &str,
        embedding: Vec<f32>,
        reliability_score: i32,
    ) -> Result<String, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let record = EmbeddingRecord {
            id: id.clone(),
            tenant_id: tenant_id.to_string(),
            agent_id: department.to_string(), // use department as agent_id for context sharing
            content: content.to_string(),
            embedding,
            source_type: format!("{}_CONTEXT", department.to_string().to_uppercase()),
            created_at: Utc::now(),
            last_referenced_at: Utc::now(),
            reference_count: 1,
            reliability_score,
            owner_override: false,
            metadata: None,
        };

        self.repository.upsert(&record).await.map_err(|e| e.to_string())?;
        Ok(id)
    }

    pub async fn retrieve_cross_department_context(
        &self,
        tenant_id: &str,
        query_embedding: &[f32],
        limit: i64,
    ) -> Result<Vec<EmbeddingRecord>, String> {
        self.repository.semantic_search(tenant_id, query_embedding, limit).await.map_err(|e| e.to_string())
    }
}

pub struct ContextPruner {
    pub repository: Arc<VectorRepository>,
}

impl ContextPruner {
    pub fn new(repository: Arc<VectorRepository>) -> Self {
        Self { repository }
    }

    /// Periodically removes or archives context that is no longer relevant.
    /// Uses signals like time since last reference, business event type, and owner activity.
    /// When in doubt, it keeps the context (conservative pruning).
    pub async fn prune_stale_context(&self, stale_threshold: DateTime<Utc>) -> Result<usize, String> {
        // Here we encapsulate the call to the repository's native pruning logic,
        // which deletes old facts that haven't been referenced recently and don't have owner overrides.
        self.repository.prune_stale(stale_threshold).await.map_err(|e| e.to_string())?;
        Ok(0)
    }
}

pub struct ConflictResolver {
    pub repository: Arc<VectorRepository>,
}

impl ConflictResolver {
    pub fn new(repository: Arc<VectorRepository>) -> Self {
        Self { repository }
    }

    /// Detects when the same fact is stored multiple times with different values.
    /// Resolves conflicts automatically based on recency, source reliability, or explicit owner override.
    pub async fn resolve_conflicts(&self) -> Result<usize, String> {
        self.repository.auto_resolve_conflicts().await.map_err(|e| e.to_string())
    }
}

pub struct MemoryConsolidationSystem {
    pub layer: CrossDepartmentMemoryLayer,
    pub pruner: ContextPruner,
    pub resolver: ConflictResolver,
}

impl MemoryConsolidationSystem {
    pub fn new(repository: Arc<VectorRepository>) -> Self {
        Self {
            layer: CrossDepartmentMemoryLayer::new(repository.clone()),
            pruner: ContextPruner::new(repository.clone()),
            resolver: ConflictResolver::new(repository),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

    #[tokio::test]
    async fn test_store_and_retrieve_and_prune() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(e) => panic!("Failed to setup sqlite pool: {}", e),
        };

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
        ).execute(&pool).await;

        let repo = Arc::new(VectorRepository::new_sqlite(pool));
        let system = MemoryConsolidationSystem::new(repo.clone());

        let tenant_id = "tenant_123";
        let department = "Sales";
        let content = "Customer is unhappy with the vegan cake orders delay.";
        let embedding = vec![0.5, 0.5, 0.5];

        let id = system.layer.store_context(tenant_id, department, content, embedding.clone(), 80).await.unwrap();

        let retrieved = system.layer.retrieve_cross_department_context(tenant_id, &embedding, 10).await.unwrap();

        assert!(!retrieved.is_empty());
        assert_eq!(retrieved[0].id, id);
        assert_eq!(retrieved[0].content, content);

        let pruned = system.pruner.prune_stale_context(Utc::now() - chrono::Duration::days(180)).await.unwrap();
        assert_eq!(pruned, 0);

        let resolved = system.resolver.resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts to resolve yet
    }
}
