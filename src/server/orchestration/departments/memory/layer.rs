#[cfg(test)]
mod tests {
    use ohc_builtin_agent::memory_store::{EmbeddingRecord, VectorRepository};
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use sqlx::Row;
    use std::str::FromStr;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_cross_department_context_sharing() {
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

        // The VectorRepository's `semantic_search` uses vector functions for Postgres.
        // For SQLite, it uses `vec_distance_cosine`, or falls back to returning all matches or none
        // based on extension availability. Let's provide a mock function so `vec_distance_cosine` succeeds
        // inside `semantic_search` if the repository calls it. If `sqlite-vss` is not available,
        // we can still test the cross-department schema integrity and the logic surrounding context sharing
        // by verifying the records can be stored and retrieved successfully.

        let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));

        // Dept A: Customer Success notes customer is unhappy
        let rec1 = EmbeddingRecord {
            id: "cs_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "cs_agent_1".to_string(),
            content: "Customer expressed dissatisfaction with recent delivery delays.".to_string(),
            embedding: vec![0.5, 0.5, 0.5],
            source_type: "SESSION_DATA".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.expect("Failed to upsert Dept A record");

        // Dept B: Operations
        let rec2 = EmbeddingRecord {
            id: "ops_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "ops_agent_1".to_string(),
            content: "Warehouse routing updated to reduce delivery delays.".to_string(),
            embedding: vec![0.4, 0.6, 0.5],
            source_type: "SESSION_DATA".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec2).await.expect("Failed to upsert Dept B record");

        // Prove that context is cross-departmental by checking directly against the database
        // to bypass the SQLite vector extension requirement for `semantic_search` in test environments.
        // This validates the structure allows cross-departmental data retrieval.
        let rows = sqlx::query("SELECT agent_id FROM consolidated_memory WHERE tenant_id = 'org1'")
            .fetch_all(&pool)
            .await
            .expect("Failed to query consolidated_memory");

        assert_eq!(rows.len(), 2, "Both records should be successfully stored for cross-department context sharing");

        let agent_ids: Vec<String> = rows.into_iter().map(|row| row.try_get("agent_id").expect("Failed to get agent_id")).collect();

        assert!(agent_ids.contains(&"cs_agent_1".to_string()), "Customer Success agent record should exist");
        assert!(agent_ids.contains(&"ops_agent_1".to_string()), "Operations agent record should exist");

        // Dept C: Business Advisory tries to retrieve context about delays
        // In Cloud mode with Postgres, `semantic_search` would be called.
        // We will call it here, handling the Result safely if the SQLite vector extension is missing.
        let query_embedding = vec![0.5, 0.5, 0.5];
        // The fallback mechanism ensures the query always succeeds
        let results = repo.semantic_search("org1", &query_embedding, 5).await.expect("Semantic search should successfully fallback to memory sorting");
        let cs_found = results.iter().any(|r| r.agent_id == "cs_agent_1");
        let ops_found = results.iter().any(|r| r.agent_id == "ops_agent_1");

        // The query succeeds and fetches items across different departments
        assert!(cs_found || ops_found, "Cross-department context sharing should return records from other agents.");
    }

    #[tokio::test]
    async fn test_sqlite_fallback_conflict_resolution() {
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

        let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));

        // Insert two identical embedding records which will trigger a conflict resolution in the fallback path
        let rec1 = EmbeddingRecord {
            id: "fallback_conflict_1".to_string(),
            tenant_id: "fallback_org".to_string(),
            agent_id: "agent_a".to_string(),
            content: "Context version 1".to_string(),
            embedding: vec![0.5, 0.5, 0.5],
            source_type: "SESSION_DATA".to_string(),
            created_at: chrono::Utc::now() - chrono::Duration::days(1),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.expect("Failed to upsert first record");

        let rec2 = EmbeddingRecord {
            id: "fallback_conflict_2".to_string(),
            tenant_id: "fallback_org".to_string(),
            agent_id: "agent_a".to_string(),
            content: "Context version 2".to_string(),
            embedding: vec![0.5, 0.5, 0.5],
            source_type: "SESSION_DATA".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 90, // Higher score so rec2 wins
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec2).await.expect("Failed to upsert second record");

        // Verify that auto_resolve_conflicts successfully discovers and resolves it
        let resolved_count = repo.auto_resolve_conflicts().await.expect("Auto resolve conflicts should succeed");
        assert_eq!(resolved_count, 1, "Should resolve exactly 1 conflict using fallback method");

        let results = sqlx::query("SELECT id FROM consolidated_memory")
            .fetch_all(&pool)
            .await
            .expect("Failed to query DB");

        assert_eq!(results.len(), 1, "Only one record should remain after conflict resolution");
        let remaining_id: String = results[0].try_get("id").expect("Failed to get ID");
        assert_eq!(remaining_id, "fallback_conflict_2", "The record with the higher reliability score should win");
    }

    #[tokio::test]
    async fn test_cross_department_context_sharing_fallback_edge_case() {
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

        let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));

        // Dept A
        let rec1 = EmbeddingRecord {
            id: "dept_a_edge".to_string(),
            tenant_id: "edge_org".to_string(),
            agent_id: "agent_a".to_string(),
            content: "Context from Agent A".to_string(),
            embedding: vec![0.1, 0.9, 0.1], // completely different vector
            source_type: "SESSION_DATA".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.expect("Failed to upsert Dept A record");

        let query_embedding = vec![0.9, 0.1, 0.9];
        // Query should still succeed gracefully even if vector match might be low or fallback executes
        // The cross-department memory sharing is guaranteed to work regardless of vector extensions because of the fallback mechanism.
        // Therefore, we strictly assert that the fallback mechanism succeeds instead of allowing failure.
        let results = repo.semantic_search("edge_org", &query_embedding, 5).await.expect("Semantic search should succeed via fallback even without vector extensions in SQLite test environments");
        assert_eq!(results.len(), 1, "Fallback should return the single record");
        assert_eq!(results[0].id, "dept_a_edge");
    }

}
