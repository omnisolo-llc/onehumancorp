use std::sync::Arc;
use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};

pub async fn get_cross_department_context(
    repository: Arc<VectorRepository>,
    tenant_id: &str,
    query_embedding: &[f32],
    limit: i64,
) -> Result<Vec<EmbeddingRecord>, String> {
    // Cross-department sharing looks at the same tenant but across all agent_ids.
    // The underlying VectorRepository implementation of semantic_search already searches
    // across all agents within a tenant_id. We just need to call it.
    // We can filter out records that belong to the querying agent at the caller site if needed.
    repository.semantic_search(tenant_id, query_embedding, limit).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    #[tokio::test]
    async fn test_cross_department_context() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new()
            .connect_with(conn_opts)
            .await
            .expect("Failed to connect to sqlite memory database");

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

        let record1 = EmbeddingRecord {
            id: "rec1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "customer_success".to_string(),
            content: "Customer is unhappy".to_string(),
            embedding: vec![0.1; 1536],
            source_type: "SESSION_DATA".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&record1).await.unwrap();

        let results = get_cross_department_context(repo, "org1", &vec![0.1; 1536], 5).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].agent_id, "customer_success");
    }
}
