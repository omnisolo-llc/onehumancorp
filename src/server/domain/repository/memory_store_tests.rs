#[cfg(test)]
mod tests {
    use super::super::memory_store::{Memory, MemoryStore};
    use crate::db::{DB, DbStore};
    use std::sync::Arc;
    use sqlx::sqlite::SqlitePoolOptions;

    // We test with Sqlite for the test harness but ensuring tenant_id is correctly scoped
    #[tokio::test]
    async fn test_tenant_isolation_memory() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Setup schema
        sqlx::query(
            r#"
            CREATE TABLE consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                source_type TEXT NOT NULL,
                created_at DATETIME
            )
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(), // Won't be used
            store: DbStore::Sqlite(pool),
        });

        let store = MemoryStore::new(db);

        // Seed data for Tenant A
        store.create_memory(Memory {
            id: "mem1".to_string(),
            tenant_id: "tenantA".to_string(),
            agent_id: "agent1".to_string(),
            content: "Tenant A secret strategy".to_string(),
            embedding: Some(vec![0.1, 0.2]),
            source_type: "manual".to_string(),
            created_at: None,
        }).await.unwrap();

        // Seed data for Tenant B
        store.create_memory(Memory {
            id: "mem2".to_string(),
            tenant_id: "tenantB".to_string(),
            agent_id: "agent2".to_string(),
            content: "Tenant B public info".to_string(),
            embedding: Some(vec![0.3, 0.4]),
            source_type: "auto".to_string(),
            created_at: None,
        }).await.unwrap();

        // Tenant A searching
        let memories_a = store.search_similar_memories("tenantA", vec![0.1, 0.2], 10).await.unwrap();
        assert_eq!(memories_a.len(), 1);
        assert_eq!(memories_a[0].id, "mem1");
        assert_eq!(memories_a[0].tenant_id, "tenantA");

        // Tenant B searching
        let memories_b = store.search_similar_memories("tenantB", vec![0.3, 0.4], 10).await.unwrap();
        assert_eq!(memories_b.len(), 1);
        assert_eq!(memories_b[0].id, "mem2");
        assert_eq!(memories_b[0].tenant_id, "tenantB");
    }
}
