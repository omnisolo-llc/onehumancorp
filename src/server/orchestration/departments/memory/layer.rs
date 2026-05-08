use std::sync::Arc;
use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
use chrono::Utc;
use uuid::Uuid;
use async_trait::async_trait;

#[async_trait]
pub trait PersistentMemoryLayer: Send + Sync {
    async fn store_context(
        &self,
        tenant_id: &str,
        agent_id: &str,
        content: &str,
        source_type: &str,
        embedding: Vec<f32>,
    ) -> Result<(), String>;

    async fn search_context(
        &self,
        tenant_id: &str,
        query_embedding: &[f32],
        limit: i64,
    ) -> Result<Vec<EmbeddingRecord>, String>;
}

pub struct MemoryLayerImpl {
    pub repository: Arc<VectorRepository>,
}

impl MemoryLayerImpl {
    pub fn new(repository: Arc<VectorRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl PersistentMemoryLayer for MemoryLayerImpl {
    async fn store_context(
        &self,
        tenant_id: &str,
        agent_id: &str,
        content: &str,
        source_type: &str,
        embedding: Vec<f32>,
    ) -> Result<(), String> {
        let record = EmbeddingRecord {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            agent_id: agent_id.to_string(),
            content: content.to_string(),
            embedding,
            source_type: source_type.to_string(),
            created_at: Utc::now(),
            last_referenced_at: Utc::now(),
            reference_count: 0,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        self.repository.upsert(&record).await
    }

    async fn search_context(
        &self,
        tenant_id: &str,
        query_embedding: &[f32],
        limit: i64,
    ) -> Result<Vec<EmbeddingRecord>, String> {
        self.repository.semantic_search(tenant_id, query_embedding, limit).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    #[tokio::test]
    async fn test_memory_layer_store_and_search() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => panic!("Failed to connect to sqlite memory database"),
        };

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
        ).execute(&pool).await.expect("Failed to create table");

        let repo = Arc::new(VectorRepository::new_sqlite(pool));
        let layer = MemoryLayerImpl::new(repo);

        let tenant_id = "test_tenant";
        let agent_id = "test_agent";
        let content = "Maya's cake price is $50";
        let source_type = "SESSION_DATA";
        let embedding = vec![0.1; 1536];

        layer.store_context(tenant_id, agent_id, content, source_type, embedding.clone()).await.unwrap();

        let results = layer.search_context(tenant_id, &embedding, 5).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].content, content);
    }
}
