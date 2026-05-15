use crate::memory::vector::*;
use crate::memory::long_term::*;
use chrono::{DateTime, Utc, TimeZone};
use sqlx::Row;
use async_trait::async_trait;
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_embedding_record_serialization() {
        let now = Utc.with_ymd_and_hms(2026, 4, 26, 0, 0, 0).unwrap();
        let record = EmbeddingRecord {
            id: "rec1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "Hello world".to_string(),
            embedding: vec![1.0, 2.0, 3.0],
            source_type: "TEXT".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 0,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        let json = serde_json::to_string(&record).unwrap();
        let deserialized: EmbeddingRecord = serde_json::from_str(&json).unwrap();

        assert_eq!(record.id, deserialized.id);
        assert_eq!(record.embedding, deserialized.embedding);
        assert_eq!(record.created_at, deserialized.created_at);
    }

    #[tokio::test]
    async fn test_file_based_memory() {
        let dir = "/tmp/test_memory";
        let mem = FileBasedMemory::new(dir);
        let namespace = "test_ns";
        let key = "test_key";
        let data = b"hello memory";

        mem.write(namespace, key, data).await.unwrap();

        let read_data = mem.read(namespace, key).await.unwrap();
        assert_eq!(read_data, data);

        let _ = tokio::fs::remove_dir_all(dir).await;
    }

    #[tokio::test]
    async fn test_anthropic_3_tier_memory_store() {
        let base_dir = "/tmp/test_anthropic_3_tier";
        let _ = tokio::fs::remove_dir_all(base_dir).await;

        let store = Anthropic3TierMemoryStore::new(base_dir).unwrap();

        // Test lightweight index
        store.update_index("Sample index content").await.unwrap();
        let index = store.get_lightweight_index().await.unwrap();
        assert_eq!(index, "Sample index content");

        // Test topic retrieve
        store.write_topic("system_architecture", "Detailed DB schema").await.unwrap();
        let topic_content = store.retrieve_topic("system_architecture").await.unwrap();
        assert_eq!(topic_content, "Detailed DB schema");
        assert!(store.retrieve_topic("nonexistent").await.is_err());

        // Test transcript search
        store.append_transcript("session1", "User asked about memory.\n\nAgent replied 3-tier is better.").await.unwrap();
        store.append_transcript("session2", "User requested weather.\n\nAgent gave forecast.").await.unwrap();

        let res = store.search_transcripts("3-tier is better", 10).await.unwrap();
        assert_eq!(res.len(), 1);
        assert!(res[0].contains("Agent replied 3-tier is better."));

        let _ = tokio::fs::remove_dir_all(base_dir).await;
    }

    }



mod additional_tests {
    use super::*;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_cross_department_search_sqlite() {
        let conn_opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = sqlx::sqlite::SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

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
        ).execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);

        let v1 = vec![0.5; 1536];
        let record = EmbeddingRecord {
            id: "rec_cross_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent_sales".to_string(),
            content: "Sales context".to_string(),
            embedding: v1.clone(),
            source_type: "NOTES".to_string(),
            created_at: Utc::now(),
            last_referenced_at: Utc::now(),
            reference_count: 0,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let results = repo.cross_department_search("org1", &v1, 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].content, "Sales context");
    }
}

#[cfg(test)]
mod override_tests_resolve {
    use super::*;

    #[tokio::test]
    async fn test_resolve_conflict_propagates_override() {
        // Setup SQLite repository
        use std::str::FromStr;
        let conn_opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = sqlx::sqlite::SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

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
        ).execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);

        let mut v1 = vec![0.0; 10];
        v1[0] = 1.0;
        let mut v2 = vec![0.0; 10];
        v2[0] = 0.99;

        let timestamp = chrono::Utc::now();

        // record_a is winner, but has NO owner_override
        let record_a = EmbeddingRecord {
            id: "winner_a".to_string(),
            tenant_id: "org_override".to_string(),
            agent_id: "test".to_string(),
            content: "Newer info".to_string(),
            embedding: v1.clone(),
            source_type: "NOTE".to_string(),
            created_at: timestamp + chrono::Duration::days(1),
            last_referenced_at: timestamp + chrono::Duration::days(1),
            reference_count: 1,
            reliability_score: 90,
            owner_override: false,
            metadata: None,
        };

        // record_b is loser, but HAS owner_override
        let record_b = EmbeddingRecord {
            id: "loser_b".to_string(),
            tenant_id: "org_override".to_string(),
            agent_id: "test".to_string(),
            content: "Older info".to_string(),
            embedding: v2.clone(),
            source_type: "NOTE".to_string(),
            created_at: timestamp,
            last_referenced_at: timestamp,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true,
            metadata: None,
        };

        repo.upsert(&record_a).await.unwrap();
        repo.upsert(&record_b).await.unwrap();

        // Directly call resolve_conflict, bypassing determine_conflict_winner
        // since determine_conflict_winner already naturally picks the one with owner_override as winner.
        repo.resolve_conflict(&record_a, &record_b).await.unwrap();

        let results = repo.cross_department_search("org_override", &v1, 10).await.unwrap();
        assert_eq!(results.len(), 1, "Only winner_a should remain");
        assert_eq!(results[0].id, "winner_a");
        assert!(results[0].owner_override, "Winner should have inherited owner_override");
    }
}
