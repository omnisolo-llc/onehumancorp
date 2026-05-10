pub mod layer; // Persistent memory layer module
pub mod pruning;
pub mod conflict;

#[cfg(test)]
mod tests {
    use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
    use std::sync::Arc;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;
    use chrono::Utc;

    #[tokio::test]
    async fn test_full_consolidation_flow() {
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
                embedding VECTOR(1536),
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

        // Dept A: Customer Success notes customer is unhappy
        let rec1 = EmbeddingRecord {
            id: "cs_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "cs_agent_1".to_string(),
            content: "Customer expressed dissatisfaction with recent delivery delays.".to_string(),
            embedding: vec![0.5; 1536],
            source_type: "SESSION_DATA".to_string(),
            created_at: Utc::now(),
            last_referenced_at: Utc::now(),
            reference_count: 1,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.expect("Failed to upsert Dept A record");

        let rows = sqlx::query("SELECT COUNT(*) FROM consolidated_memory")
            .fetch_one(&pool)
            .await
            .expect("Failed to query consolidated_memory count");
        use sqlx::Row;
        let count: i64 = rows.try_get(0).unwrap();
        assert_eq!(count, 1);
    }
}
