pub mod layer; // Persistent memory layer module
pub mod pruning;
pub mod conflict;

use std::sync::Arc;
use chrono::{DateTime, Utc};
use ohc_builtin_agent::memory_store::VectorRepository;

pub struct MemoryConsolidator {
    pub layer: layer::CrossDepartmentMemoryLayer,
    repository: Arc<VectorRepository>,
}

impl MemoryConsolidator {
    pub fn new(repository: Arc<VectorRepository>) -> Self {
        Self {
            layer: layer::CrossDepartmentMemoryLayer::new(repository.clone()),
            repository,
        }
    }

    pub async fn run_consolidation_pipeline(&self, older_than: DateTime<Utc>) -> Result<(), String> {
        pruning::prune_stale(self.repository.clone(), older_than).await?;
        conflict::auto_resolve_conflicts(self.repository.clone()).await.map(|_| ())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    #[tokio::test]
    async fn test_memory_consolidator_pipeline() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

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
        ).execute(&pool).await.unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool));
        let consolidator = MemoryConsolidator::new(repo);
        let older_than = Utc::now() - chrono::Duration::days(180);
        let res = consolidator.run_consolidation_pipeline(older_than).await;
        assert!(res.is_ok());
    }
}
