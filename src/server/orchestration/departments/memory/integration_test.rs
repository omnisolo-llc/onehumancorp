#[cfg(test)]
mod tests {
    use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
    use crate::orchestration::departments::memory::pruning::prune_stale;
    use crate::orchestration::departments::memory::conflict::auto_resolve_conflicts;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;
    use std::sync::Arc;
    use chrono::Utc;
    use sqlx::Row;

    #[tokio::test]
    async fn test_full_memory_consolidation_lifecycle_deterministic() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new()
            .connect_with(conn_opts)
            .await
            .unwrap();

        // Initialize proper schema supporting vectors for SQLite
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
        .unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));

        // 1. Store context from Dept A (Customer Success)
        let rec1 = EmbeddingRecord {
            id: "cs_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "cs_agent_1".to_string(),
            content: "Maya's vegan cake price is $50".to_string(),
            embedding: vec![0.1; 1536],
            source_type: "SESSION_DATA".to_string(),
            created_at: Utc::now() - chrono::Duration::days(5),
            last_referenced_at: Utc::now() - chrono::Duration::days(5),
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();

        // 2. Conflict context from Dept B (Operations)
        let rec2 = EmbeddingRecord {
            id: "ops_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "ops_agent_1".to_string(),
            content: "Maya's vegan cake price is $55".to_string(),
            embedding: vec![0.1; 1536], // Same embedding -> conflict
            source_type: "SESSION_DATA".to_string(),
            created_at: Utc::now() - chrono::Duration::days(2), // Newer
            last_referenced_at: Utc::now() - chrono::Duration::days(2),
            reference_count: 1,
            reliability_score: 80, // Higher score -> winner
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec2).await.unwrap();

        // 3. Stale context
        let rec_stale = EmbeddingRecord {
            id: "stale_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "marketing_1".to_string(),
            content: "Old promotion from 6 months ago".to_string(),
            embedding: vec![0.5; 1536],
            source_type: "TASK_SUMMARY".to_string(), // Prunable source type
            created_at: Utc::now() - chrono::Duration::days(200),
            last_referenced_at: Utc::now() - chrono::Duration::days(200),
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec_stale).await.unwrap();

        // 4. Run exact pipeline functions deterministically (no sleep/worker)
        let threshold = Utc::now() - chrono::Duration::days(180);
        prune_stale(repo.clone(), threshold).await.unwrap();

        let resolved_count = auto_resolve_conflicts(repo.clone()).await.unwrap();
        assert_eq!(resolved_count, 1, "Should resolve exactly one conflict");

        // 5. Assertions
        let query = "SELECT id, reference_count, agent_id FROM consolidated_memory";
        let rows = sqlx::query(query).fetch_all(&pool).await.unwrap();

        // Stale is pruned. CS and Ops conflict: Ops wins and inherits ref count.
        assert_eq!(rows.len(), 1, "Only the winning conflict record should remain");

        let id: String = rows[0].try_get("id").unwrap();
        let ref_count: i32 = rows[0].try_get("reference_count").unwrap();
        let agent_id: String = rows[0].try_get("agent_id").unwrap();

        assert_eq!(id, "ops_1", "The newer, more reliable record should win");
        assert_eq!(agent_id, "ops_agent_1", "Cross-departmental context preserved to winner");

        // Winner inherits loser's reference count. winner(1) + loser(1) + 1 = 3
        assert_eq!(ref_count, 3, "Winner inherits reference count");
    }
}
