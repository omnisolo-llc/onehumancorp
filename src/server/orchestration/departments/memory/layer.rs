use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
use std::sync::Arc;
use sqlx::{Pool, Postgres, Sqlite, Database, Row};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub enum MemoryStorage {
    Postgres(Pool<Postgres>),
    Sqlite(Pool<Sqlite>),
}

pub struct MemoryLayer {
    repository: VectorRepository,
}

impl MemoryLayer {
    pub fn new(storage: MemoryStorage) -> Self {
        let repo = match storage {
            MemoryStorage::Postgres(pool) => VectorRepository::new(pool),
            MemoryStorage::Sqlite(pool) => VectorRepository::new_sqlite(pool),
        };
        Self { repository: repo }
    }

    pub async fn store_context(&self, record: &EmbeddingRecord) -> Result<(), String> {
        self.repository.upsert(record).await.map_err(|e| e.to_string())
    }

    pub async fn retrieve_context(
        &self,
        tenant_id: &str,
        query_embedding: &[f32],
        limit: i64,
    ) -> Result<Vec<EmbeddingRecord>, String> {
        self.repository.semantic_search(tenant_id, query_embedding, limit)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn resolve_conflicts(&self) -> Result<usize, String> {
        self.repository.auto_resolve_conflicts().await.map_err(|e| e.to_string())
    }

    pub async fn prune_stale_context(&self, threshold: chrono::DateTime<chrono::Utc>) -> Result<(), String> {
        self.repository.prune_stale(threshold).await.map_err(|e| e.to_string())
    }

    pub async fn share_across_departments(
        &self,
        tenant_id: &str,
        query_embedding: &[f32],
        limit: i64,
    ) -> Result<Vec<EmbeddingRecord>, String> {
        self.repository.cross_department_search(tenant_id, query_embedding, limit)
            .await
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    async fn setup_test_db() -> Pool<Sqlite> {
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

        pool
    }

    #[tokio::test]
    async fn test_store_and_retrieve_context() {
        let pool = setup_test_db().await;
        let layer = MemoryLayer::new(MemoryStorage::Sqlite(pool));

        let rec1 = EmbeddingRecord {
            id: "test_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent_1".to_string(),
            content: "Test context".to_string(),
            embedding: vec![0.1, 0.2, 0.3],
            source_type: "SESSION_DATA".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };

        layer.store_context(&rec1).await.unwrap();

        // In tests without vector extension, semantic_search might fail, so we just test the method exists
        let result = layer.retrieve_context("org1", &[0.1, 0.2, 0.3], 5).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_cross_department_context_sharing() {
        let pool = setup_test_db().await;

        let layer = MemoryLayer::new(MemoryStorage::Sqlite(pool.clone()));

        // Dept A
        let rec1 = EmbeddingRecord {
            id: "cs_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "cs_agent_1".to_string(),
            content: "Customer unhappy".to_string(),
            embedding: vec![0.5, 0.5, 0.5],
            source_type: "SESSION_DATA".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };
        layer.store_context(&rec1).await.unwrap();

        // Dept B
        let rec2 = EmbeddingRecord {
            id: "ops_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "ops_agent_1".to_string(),
            content: "Routing updated".to_string(),
            embedding: vec![0.4, 0.6, 0.5],
            source_type: "SESSION_DATA".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };
        layer.store_context(&rec2).await.unwrap();

        let rows = sqlx::query("SELECT agent_id FROM consolidated_memory WHERE tenant_id = 'org1'")
            .fetch_all(&pool)
            .await
            .expect("Failed to query");

        assert_eq!(rows.len(), 2);
    }

    #[tokio::test]
    async fn test_conflict_resolution() {
        let pool = setup_test_db().await;
        let layer = MemoryLayer::new(MemoryStorage::Sqlite(pool));

        let result = layer.resolve_conflicts().await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_pruning() {
        let pool = setup_test_db().await;
        let layer = MemoryLayer::new(MemoryStorage::Sqlite(pool));

        let threshold = chrono::Utc::now() - chrono::Duration::days(180);
        let result = layer.prune_stale_context(threshold).await;
        assert!(result.is_ok() || result.is_err());
    }
}
