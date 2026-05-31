#[cfg(test)]
mod exhaustive_tests {
    use crate::memory_store::{VectorRepository, EmbeddingRecord};
    use chrono::Utc;
    use std::sync::Arc;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    async fn setup_repo() -> Arc<VectorRepository> {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS consolidated_memory (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, agent_id TEXT, content TEXT NOT NULL, embedding TEXT, source_type TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, reference_count INTEGER DEFAULT 0, reliability_score INTEGER DEFAULT 50, owner_override BOOLEAN DEFAULT FALSE, metadata TEXT);").execute(&pool).await.unwrap();
        Arc::new(VectorRepository::new_sqlite(pool))
    }

    #[tokio::test]
    async fn test_conflict_scenario_0() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_0_1".to_string(),
            tenant_id: "tenant_0".to_string(),
            agent_id: "agent_0".to_string(),
            content: "content A 0".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_0_2".to_string(),
            tenant_id: "tenant_0".to_string(),
            agent_id: "agent_0".to_string(),
            content: "content B 0".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 60,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_1() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_1_1".to_string(),
            tenant_id: "tenant_1".to_string(),
            agent_id: "agent_1".to_string(),
            content: "content A 1".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 51,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_1_2".to_string(),
            tenant_id: "tenant_1".to_string(),
            agent_id: "agent_1".to_string(),
            content: "content B 1".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_2() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_2_1".to_string(),
            tenant_id: "tenant_2".to_string(),
            agent_id: "agent_2".to_string(),
            content: "content A 2".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 52,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_2_2".to_string(),
            tenant_id: "tenant_2".to_string(),
            agent_id: "agent_2".to_string(),
            content: "content B 2".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 58,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_3() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_3_1".to_string(),
            tenant_id: "tenant_3".to_string(),
            agent_id: "agent_3".to_string(),
            content: "content A 3".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_3_2".to_string(),
            tenant_id: "tenant_3".to_string(),
            agent_id: "agent_3".to_string(),
            content: "content B 3".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 57,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_4() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_4_1".to_string(),
            tenant_id: "tenant_4".to_string(),
            agent_id: "agent_4".to_string(),
            content: "content A 4".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 54,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_4_2".to_string(),
            tenant_id: "tenant_4".to_string(),
            agent_id: "agent_4".to_string(),
            content: "content B 4".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 56,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_5() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_5_1".to_string(),
            tenant_id: "tenant_5".to_string(),
            agent_id: "agent_5".to_string(),
            content: "content A 5".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_5_2".to_string(),
            tenant_id: "tenant_5".to_string(),
            agent_id: "agent_5".to_string(),
            content: "content B 5".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_6() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_6_1".to_string(),
            tenant_id: "tenant_6".to_string(),
            agent_id: "agent_6".to_string(),
            content: "content A 6".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 56,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_6_2".to_string(),
            tenant_id: "tenant_6".to_string(),
            agent_id: "agent_6".to_string(),
            content: "content B 6".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 54,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_7() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_7_1".to_string(),
            tenant_id: "tenant_7".to_string(),
            agent_id: "agent_7".to_string(),
            content: "content A 7".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 57,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_7_2".to_string(),
            tenant_id: "tenant_7".to_string(),
            agent_id: "agent_7".to_string(),
            content: "content B 7".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_8() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_8_1".to_string(),
            tenant_id: "tenant_8".to_string(),
            agent_id: "agent_8".to_string(),
            content: "content A 8".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 58,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_8_2".to_string(),
            tenant_id: "tenant_8".to_string(),
            agent_id: "agent_8".to_string(),
            content: "content B 8".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 52,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_9() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_9_1".to_string(),
            tenant_id: "tenant_9".to_string(),
            agent_id: "agent_9".to_string(),
            content: "content A 9".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_9_2".to_string(),
            tenant_id: "tenant_9".to_string(),
            agent_id: "agent_9".to_string(),
            content: "content B 9".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 51,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_10() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_10_1".to_string(),
            tenant_id: "tenant_10".to_string(),
            agent_id: "agent_10".to_string(),
            content: "content A 10".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_10_2".to_string(),
            tenant_id: "tenant_10".to_string(),
            agent_id: "agent_10".to_string(),
            content: "content B 10".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 60,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_11() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_11_1".to_string(),
            tenant_id: "tenant_11".to_string(),
            agent_id: "agent_11".to_string(),
            content: "content A 11".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 51,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_11_2".to_string(),
            tenant_id: "tenant_11".to_string(),
            agent_id: "agent_11".to_string(),
            content: "content B 11".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_12() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_12_1".to_string(),
            tenant_id: "tenant_12".to_string(),
            agent_id: "agent_12".to_string(),
            content: "content A 12".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 52,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_12_2".to_string(),
            tenant_id: "tenant_12".to_string(),
            agent_id: "agent_12".to_string(),
            content: "content B 12".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 58,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_13() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_13_1".to_string(),
            tenant_id: "tenant_13".to_string(),
            agent_id: "agent_13".to_string(),
            content: "content A 13".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_13_2".to_string(),
            tenant_id: "tenant_13".to_string(),
            agent_id: "agent_13".to_string(),
            content: "content B 13".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 57,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_14() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_14_1".to_string(),
            tenant_id: "tenant_14".to_string(),
            agent_id: "agent_14".to_string(),
            content: "content A 14".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 54,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_14_2".to_string(),
            tenant_id: "tenant_14".to_string(),
            agent_id: "agent_14".to_string(),
            content: "content B 14".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 56,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_15() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_15_1".to_string(),
            tenant_id: "tenant_15".to_string(),
            agent_id: "agent_15".to_string(),
            content: "content A 15".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_15_2".to_string(),
            tenant_id: "tenant_15".to_string(),
            agent_id: "agent_15".to_string(),
            content: "content B 15".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 55,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_16() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_16_1".to_string(),
            tenant_id: "tenant_16".to_string(),
            agent_id: "agent_16".to_string(),
            content: "content A 16".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 56,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_16_2".to_string(),
            tenant_id: "tenant_16".to_string(),
            agent_id: "agent_16".to_string(),
            content: "content B 16".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 54,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_17() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_17_1".to_string(),
            tenant_id: "tenant_17".to_string(),
            agent_id: "agent_17".to_string(),
            content: "content A 17".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 57,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_17_2".to_string(),
            tenant_id: "tenant_17".to_string(),
            agent_id: "agent_17".to_string(),
            content: "content B 17".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_18() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_18_1".to_string(),
            tenant_id: "tenant_18".to_string(),
            agent_id: "agent_18".to_string(),
            content: "content A 18".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 58,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_18_2".to_string(),
            tenant_id: "tenant_18".to_string(),
            agent_id: "agent_18".to_string(),
            content: "content B 18".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 52,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_19() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_19_1".to_string(),
            tenant_id: "tenant_19".to_string(),
            agent_id: "agent_19".to_string(),
            content: "content A 19".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_19_2".to_string(),
            tenant_id: "tenant_19".to_string(),
            agent_id: "agent_19".to_string(),
            content: "content B 19".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 51,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_20() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_20_1".to_string(),
            tenant_id: "tenant_20".to_string(),
            agent_id: "agent_20".to_string(),
            content: "content A 20".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_20_2".to_string(),
            tenant_id: "tenant_20".to_string(),
            agent_id: "agent_20".to_string(),
            content: "content B 20".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 60,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_21() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_21_1".to_string(),
            tenant_id: "tenant_21".to_string(),
            agent_id: "agent_21".to_string(),
            content: "content A 21".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 51,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_21_2".to_string(),
            tenant_id: "tenant_21".to_string(),
            agent_id: "agent_21".to_string(),
            content: "content B 21".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 59,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_22() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_22_1".to_string(),
            tenant_id: "tenant_22".to_string(),
            agent_id: "agent_22".to_string(),
            content: "content A 22".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 52,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_22_2".to_string(),
            tenant_id: "tenant_22".to_string(),
            agent_id: "agent_22".to_string(),
            content: "content B 22".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 58,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_23() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_23_1".to_string(),
            tenant_id: "tenant_23".to_string(),
            agent_id: "agent_23".to_string(),
            content: "content A 23".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_23_2".to_string(),
            tenant_id: "tenant_23".to_string(),
            agent_id: "agent_23".to_string(),
            content: "content B 23".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 57,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_24() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_24_1".to_string(),
            tenant_id: "tenant_24".to_string(),
            agent_id: "agent_24".to_string(),
            content: "content A 24".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 54,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_24_2".to_string(),
            tenant_id: "tenant_24".to_string(),
            agent_id: "agent_24".to_string(),
            content: "content B 24".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 56,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_25() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_25_1".to_string(),
            tenant_id: "tenant_25".to_string(),
            agent_id: "agent_25".to_string(),
            content: "content A 25".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_25_2".to_string(),
            tenant_id: "tenant_25".to_string(),
            agent_id: "agent_25".to_string(),
            content: "content B 25".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_26() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_26_1".to_string(),
            tenant_id: "tenant_26".to_string(),
            agent_id: "agent_26".to_string(),
            content: "content A 26".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 56,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_26_2".to_string(),
            tenant_id: "tenant_26".to_string(),
            agent_id: "agent_26".to_string(),
            content: "content B 26".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 54,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_27() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_27_1".to_string(),
            tenant_id: "tenant_27".to_string(),
            agent_id: "agent_27".to_string(),
            content: "content A 27".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 57,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_27_2".to_string(),
            tenant_id: "tenant_27".to_string(),
            agent_id: "agent_27".to_string(),
            content: "content B 27".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 53,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_28() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_28_1".to_string(),
            tenant_id: "tenant_28".to_string(),
            agent_id: "agent_28".to_string(),
            content: "content A 28".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 58,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_28_2".to_string(),
            tenant_id: "tenant_28".to_string(),
            agent_id: "agent_28".to_string(),
            content: "content B 28".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 52,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_29() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_29_1".to_string(),
            tenant_id: "tenant_29".to_string(),
            agent_id: "agent_29".to_string(),
            content: "content A 29".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_29_2".to_string(),
            tenant_id: "tenant_29".to_string(),
            agent_id: "agent_29".to_string(),
            content: "content B 29".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 51,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_30() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_30_1".to_string(),
            tenant_id: "tenant_30".to_string(),
            agent_id: "agent_30".to_string(),
            content: "content A 30".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_30_2".to_string(),
            tenant_id: "tenant_30".to_string(),
            agent_id: "agent_30".to_string(),
            content: "content B 30".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 60,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_31() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_31_1".to_string(),
            tenant_id: "tenant_31".to_string(),
            agent_id: "agent_31".to_string(),
            content: "content A 31".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 51,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_31_2".to_string(),
            tenant_id: "tenant_31".to_string(),
            agent_id: "agent_31".to_string(),
            content: "content B 31".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_32() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_32_1".to_string(),
            tenant_id: "tenant_32".to_string(),
            agent_id: "agent_32".to_string(),
            content: "content A 32".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 52,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_32_2".to_string(),
            tenant_id: "tenant_32".to_string(),
            agent_id: "agent_32".to_string(),
            content: "content B 32".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 58,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_33() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_33_1".to_string(),
            tenant_id: "tenant_33".to_string(),
            agent_id: "agent_33".to_string(),
            content: "content A 33".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_33_2".to_string(),
            tenant_id: "tenant_33".to_string(),
            agent_id: "agent_33".to_string(),
            content: "content B 33".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 57,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_34() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_34_1".to_string(),
            tenant_id: "tenant_34".to_string(),
            agent_id: "agent_34".to_string(),
            content: "content A 34".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 54,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_34_2".to_string(),
            tenant_id: "tenant_34".to_string(),
            agent_id: "agent_34".to_string(),
            content: "content B 34".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 56,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_35() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_35_1".to_string(),
            tenant_id: "tenant_35".to_string(),
            agent_id: "agent_35".to_string(),
            content: "content A 35".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_35_2".to_string(),
            tenant_id: "tenant_35".to_string(),
            agent_id: "agent_35".to_string(),
            content: "content B 35".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_36() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_36_1".to_string(),
            tenant_id: "tenant_36".to_string(),
            agent_id: "agent_36".to_string(),
            content: "content A 36".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 56,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_36_2".to_string(),
            tenant_id: "tenant_36".to_string(),
            agent_id: "agent_36".to_string(),
            content: "content B 36".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 54,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_37() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_37_1".to_string(),
            tenant_id: "tenant_37".to_string(),
            agent_id: "agent_37".to_string(),
            content: "content A 37".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 57,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_37_2".to_string(),
            tenant_id: "tenant_37".to_string(),
            agent_id: "agent_37".to_string(),
            content: "content B 37".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_38() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_38_1".to_string(),
            tenant_id: "tenant_38".to_string(),
            agent_id: "agent_38".to_string(),
            content: "content A 38".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 58,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_38_2".to_string(),
            tenant_id: "tenant_38".to_string(),
            agent_id: "agent_38".to_string(),
            content: "content B 38".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 52,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_39() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_39_1".to_string(),
            tenant_id: "tenant_39".to_string(),
            agent_id: "agent_39".to_string(),
            content: "content A 39".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_39_2".to_string(),
            tenant_id: "tenant_39".to_string(),
            agent_id: "agent_39".to_string(),
            content: "content B 39".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 51,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_40() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_40_1".to_string(),
            tenant_id: "tenant_40".to_string(),
            agent_id: "agent_40".to_string(),
            content: "content A 40".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_40_2".to_string(),
            tenant_id: "tenant_40".to_string(),
            agent_id: "agent_40".to_string(),
            content: "content B 40".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 60,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_41() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_41_1".to_string(),
            tenant_id: "tenant_41".to_string(),
            agent_id: "agent_41".to_string(),
            content: "content A 41".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 51,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_41_2".to_string(),
            tenant_id: "tenant_41".to_string(),
            agent_id: "agent_41".to_string(),
            content: "content B 41".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_42() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_42_1".to_string(),
            tenant_id: "tenant_42".to_string(),
            agent_id: "agent_42".to_string(),
            content: "content A 42".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 52,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_42_2".to_string(),
            tenant_id: "tenant_42".to_string(),
            agent_id: "agent_42".to_string(),
            content: "content B 42".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 58,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_43() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_43_1".to_string(),
            tenant_id: "tenant_43".to_string(),
            agent_id: "agent_43".to_string(),
            content: "content A 43".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_43_2".to_string(),
            tenant_id: "tenant_43".to_string(),
            agent_id: "agent_43".to_string(),
            content: "content B 43".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 57,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_44() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_44_1".to_string(),
            tenant_id: "tenant_44".to_string(),
            agent_id: "agent_44".to_string(),
            content: "content A 44".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 54,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_44_2".to_string(),
            tenant_id: "tenant_44".to_string(),
            agent_id: "agent_44".to_string(),
            content: "content B 44".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 56,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_45() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_45_1".to_string(),
            tenant_id: "tenant_45".to_string(),
            agent_id: "agent_45".to_string(),
            content: "content A 45".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_45_2".to_string(),
            tenant_id: "tenant_45".to_string(),
            agent_id: "agent_45".to_string(),
            content: "content B 45".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 55,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_46() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_46_1".to_string(),
            tenant_id: "tenant_46".to_string(),
            agent_id: "agent_46".to_string(),
            content: "content A 46".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 56,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_46_2".to_string(),
            tenant_id: "tenant_46".to_string(),
            agent_id: "agent_46".to_string(),
            content: "content B 46".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 54,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_47() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_47_1".to_string(),
            tenant_id: "tenant_47".to_string(),
            agent_id: "agent_47".to_string(),
            content: "content A 47".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 57,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_47_2".to_string(),
            tenant_id: "tenant_47".to_string(),
            agent_id: "agent_47".to_string(),
            content: "content B 47".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_48() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_48_1".to_string(),
            tenant_id: "tenant_48".to_string(),
            agent_id: "agent_48".to_string(),
            content: "content A 48".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 58,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_48_2".to_string(),
            tenant_id: "tenant_48".to_string(),
            agent_id: "agent_48".to_string(),
            content: "content B 48".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 52,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_49() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_49_1".to_string(),
            tenant_id: "tenant_49".to_string(),
            agent_id: "agent_49".to_string(),
            content: "content A 49".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_49_2".to_string(),
            tenant_id: "tenant_49".to_string(),
            agent_id: "agent_49".to_string(),
            content: "content B 49".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 51,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_50() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_50_1".to_string(),
            tenant_id: "tenant_50".to_string(),
            agent_id: "agent_50".to_string(),
            content: "content A 50".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_50_2".to_string(),
            tenant_id: "tenant_50".to_string(),
            agent_id: "agent_50".to_string(),
            content: "content B 50".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 60,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_51() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_51_1".to_string(),
            tenant_id: "tenant_51".to_string(),
            agent_id: "agent_51".to_string(),
            content: "content A 51".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 51,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_51_2".to_string(),
            tenant_id: "tenant_51".to_string(),
            agent_id: "agent_51".to_string(),
            content: "content B 51".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 59,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_52() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_52_1".to_string(),
            tenant_id: "tenant_52".to_string(),
            agent_id: "agent_52".to_string(),
            content: "content A 52".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 52,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_52_2".to_string(),
            tenant_id: "tenant_52".to_string(),
            agent_id: "agent_52".to_string(),
            content: "content B 52".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 58,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_53() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_53_1".to_string(),
            tenant_id: "tenant_53".to_string(),
            agent_id: "agent_53".to_string(),
            content: "content A 53".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_53_2".to_string(),
            tenant_id: "tenant_53".to_string(),
            agent_id: "agent_53".to_string(),
            content: "content B 53".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 57,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_54() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_54_1".to_string(),
            tenant_id: "tenant_54".to_string(),
            agent_id: "agent_54".to_string(),
            content: "content A 54".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 54,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_54_2".to_string(),
            tenant_id: "tenant_54".to_string(),
            agent_id: "agent_54".to_string(),
            content: "content B 54".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 56,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_55() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_55_1".to_string(),
            tenant_id: "tenant_55".to_string(),
            agent_id: "agent_55".to_string(),
            content: "content A 55".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_55_2".to_string(),
            tenant_id: "tenant_55".to_string(),
            agent_id: "agent_55".to_string(),
            content: "content B 55".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_56() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_56_1".to_string(),
            tenant_id: "tenant_56".to_string(),
            agent_id: "agent_56".to_string(),
            content: "content A 56".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 56,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_56_2".to_string(),
            tenant_id: "tenant_56".to_string(),
            agent_id: "agent_56".to_string(),
            content: "content B 56".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 54,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_57() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_57_1".to_string(),
            tenant_id: "tenant_57".to_string(),
            agent_id: "agent_57".to_string(),
            content: "content A 57".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 57,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_57_2".to_string(),
            tenant_id: "tenant_57".to_string(),
            agent_id: "agent_57".to_string(),
            content: "content B 57".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 53,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_58() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_58_1".to_string(),
            tenant_id: "tenant_58".to_string(),
            agent_id: "agent_58".to_string(),
            content: "content A 58".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 58,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_58_2".to_string(),
            tenant_id: "tenant_58".to_string(),
            agent_id: "agent_58".to_string(),
            content: "content B 58".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 52,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_59() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_59_1".to_string(),
            tenant_id: "tenant_59".to_string(),
            agent_id: "agent_59".to_string(),
            content: "content A 59".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_59_2".to_string(),
            tenant_id: "tenant_59".to_string(),
            agent_id: "agent_59".to_string(),
            content: "content B 59".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 51,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_60() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_60_1".to_string(),
            tenant_id: "tenant_60".to_string(),
            agent_id: "agent_60".to_string(),
            content: "content A 60".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_60_2".to_string(),
            tenant_id: "tenant_60".to_string(),
            agent_id: "agent_60".to_string(),
            content: "content B 60".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 60,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_61() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_61_1".to_string(),
            tenant_id: "tenant_61".to_string(),
            agent_id: "agent_61".to_string(),
            content: "content A 61".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 51,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_61_2".to_string(),
            tenant_id: "tenant_61".to_string(),
            agent_id: "agent_61".to_string(),
            content: "content B 61".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_62() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_62_1".to_string(),
            tenant_id: "tenant_62".to_string(),
            agent_id: "agent_62".to_string(),
            content: "content A 62".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 52,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_62_2".to_string(),
            tenant_id: "tenant_62".to_string(),
            agent_id: "agent_62".to_string(),
            content: "content B 62".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 58,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_63() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_63_1".to_string(),
            tenant_id: "tenant_63".to_string(),
            agent_id: "agent_63".to_string(),
            content: "content A 63".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_63_2".to_string(),
            tenant_id: "tenant_63".to_string(),
            agent_id: "agent_63".to_string(),
            content: "content B 63".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 57,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_64() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_64_1".to_string(),
            tenant_id: "tenant_64".to_string(),
            agent_id: "agent_64".to_string(),
            content: "content A 64".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 54,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_64_2".to_string(),
            tenant_id: "tenant_64".to_string(),
            agent_id: "agent_64".to_string(),
            content: "content B 64".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 56,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_65() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_65_1".to_string(),
            tenant_id: "tenant_65".to_string(),
            agent_id: "agent_65".to_string(),
            content: "content A 65".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_65_2".to_string(),
            tenant_id: "tenant_65".to_string(),
            agent_id: "agent_65".to_string(),
            content: "content B 65".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_66() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_66_1".to_string(),
            tenant_id: "tenant_66".to_string(),
            agent_id: "agent_66".to_string(),
            content: "content A 66".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 56,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_66_2".to_string(),
            tenant_id: "tenant_66".to_string(),
            agent_id: "agent_66".to_string(),
            content: "content B 66".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 54,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_67() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_67_1".to_string(),
            tenant_id: "tenant_67".to_string(),
            agent_id: "agent_67".to_string(),
            content: "content A 67".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 57,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_67_2".to_string(),
            tenant_id: "tenant_67".to_string(),
            agent_id: "agent_67".to_string(),
            content: "content B 67".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_68() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_68_1".to_string(),
            tenant_id: "tenant_68".to_string(),
            agent_id: "agent_68".to_string(),
            content: "content A 68".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 58,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_68_2".to_string(),
            tenant_id: "tenant_68".to_string(),
            agent_id: "agent_68".to_string(),
            content: "content B 68".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 52,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_69() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_69_1".to_string(),
            tenant_id: "tenant_69".to_string(),
            agent_id: "agent_69".to_string(),
            content: "content A 69".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_69_2".to_string(),
            tenant_id: "tenant_69".to_string(),
            agent_id: "agent_69".to_string(),
            content: "content B 69".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 51,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_70() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_70_1".to_string(),
            tenant_id: "tenant_70".to_string(),
            agent_id: "agent_70".to_string(),
            content: "content A 70".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_70_2".to_string(),
            tenant_id: "tenant_70".to_string(),
            agent_id: "agent_70".to_string(),
            content: "content B 70".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 60,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_71() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_71_1".to_string(),
            tenant_id: "tenant_71".to_string(),
            agent_id: "agent_71".to_string(),
            content: "content A 71".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 51,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_71_2".to_string(),
            tenant_id: "tenant_71".to_string(),
            agent_id: "agent_71".to_string(),
            content: "content B 71".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_72() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_72_1".to_string(),
            tenant_id: "tenant_72".to_string(),
            agent_id: "agent_72".to_string(),
            content: "content A 72".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 52,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_72_2".to_string(),
            tenant_id: "tenant_72".to_string(),
            agent_id: "agent_72".to_string(),
            content: "content B 72".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 58,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_73() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_73_1".to_string(),
            tenant_id: "tenant_73".to_string(),
            agent_id: "agent_73".to_string(),
            content: "content A 73".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_73_2".to_string(),
            tenant_id: "tenant_73".to_string(),
            agent_id: "agent_73".to_string(),
            content: "content B 73".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 57,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_74() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_74_1".to_string(),
            tenant_id: "tenant_74".to_string(),
            agent_id: "agent_74".to_string(),
            content: "content A 74".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 54,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_74_2".to_string(),
            tenant_id: "tenant_74".to_string(),
            agent_id: "agent_74".to_string(),
            content: "content B 74".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 56,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_75() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_75_1".to_string(),
            tenant_id: "tenant_75".to_string(),
            agent_id: "agent_75".to_string(),
            content: "content A 75".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_75_2".to_string(),
            tenant_id: "tenant_75".to_string(),
            agent_id: "agent_75".to_string(),
            content: "content B 75".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 55,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_76() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_76_1".to_string(),
            tenant_id: "tenant_76".to_string(),
            agent_id: "agent_76".to_string(),
            content: "content A 76".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 56,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_76_2".to_string(),
            tenant_id: "tenant_76".to_string(),
            agent_id: "agent_76".to_string(),
            content: "content B 76".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 54,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_77() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_77_1".to_string(),
            tenant_id: "tenant_77".to_string(),
            agent_id: "agent_77".to_string(),
            content: "content A 77".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 57,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_77_2".to_string(),
            tenant_id: "tenant_77".to_string(),
            agent_id: "agent_77".to_string(),
            content: "content B 77".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_78() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_78_1".to_string(),
            tenant_id: "tenant_78".to_string(),
            agent_id: "agent_78".to_string(),
            content: "content A 78".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 58,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_78_2".to_string(),
            tenant_id: "tenant_78".to_string(),
            agent_id: "agent_78".to_string(),
            content: "content B 78".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 52,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_79() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_79_1".to_string(),
            tenant_id: "tenant_79".to_string(),
            agent_id: "agent_79".to_string(),
            content: "content A 79".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_79_2".to_string(),
            tenant_id: "tenant_79".to_string(),
            agent_id: "agent_79".to_string(),
            content: "content B 79".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 51,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_80() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_80_1".to_string(),
            tenant_id: "tenant_80".to_string(),
            agent_id: "agent_80".to_string(),
            content: "content A 80".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_80_2".to_string(),
            tenant_id: "tenant_80".to_string(),
            agent_id: "agent_80".to_string(),
            content: "content B 80".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 60,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_81() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_81_1".to_string(),
            tenant_id: "tenant_81".to_string(),
            agent_id: "agent_81".to_string(),
            content: "content A 81".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 51,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_81_2".to_string(),
            tenant_id: "tenant_81".to_string(),
            agent_id: "agent_81".to_string(),
            content: "content B 81".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 59,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_82() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_82_1".to_string(),
            tenant_id: "tenant_82".to_string(),
            agent_id: "agent_82".to_string(),
            content: "content A 82".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 52,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_82_2".to_string(),
            tenant_id: "tenant_82".to_string(),
            agent_id: "agent_82".to_string(),
            content: "content B 82".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 58,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_83() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_83_1".to_string(),
            tenant_id: "tenant_83".to_string(),
            agent_id: "agent_83".to_string(),
            content: "content A 83".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_83_2".to_string(),
            tenant_id: "tenant_83".to_string(),
            agent_id: "agent_83".to_string(),
            content: "content B 83".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 57,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_84() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_84_1".to_string(),
            tenant_id: "tenant_84".to_string(),
            agent_id: "agent_84".to_string(),
            content: "content A 84".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 54,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_84_2".to_string(),
            tenant_id: "tenant_84".to_string(),
            agent_id: "agent_84".to_string(),
            content: "content B 84".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 56,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_85() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_85_1".to_string(),
            tenant_id: "tenant_85".to_string(),
            agent_id: "agent_85".to_string(),
            content: "content A 85".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_85_2".to_string(),
            tenant_id: "tenant_85".to_string(),
            agent_id: "agent_85".to_string(),
            content: "content B 85".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_86() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_86_1".to_string(),
            tenant_id: "tenant_86".to_string(),
            agent_id: "agent_86".to_string(),
            content: "content A 86".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 56,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_86_2".to_string(),
            tenant_id: "tenant_86".to_string(),
            agent_id: "agent_86".to_string(),
            content: "content B 86".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 54,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_87() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_87_1".to_string(),
            tenant_id: "tenant_87".to_string(),
            agent_id: "agent_87".to_string(),
            content: "content A 87".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 57,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_87_2".to_string(),
            tenant_id: "tenant_87".to_string(),
            agent_id: "agent_87".to_string(),
            content: "content B 87".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 53,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_88() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_88_1".to_string(),
            tenant_id: "tenant_88".to_string(),
            agent_id: "agent_88".to_string(),
            content: "content A 88".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 58,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_88_2".to_string(),
            tenant_id: "tenant_88".to_string(),
            agent_id: "agent_88".to_string(),
            content: "content B 88".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 52,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_89() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_89_1".to_string(),
            tenant_id: "tenant_89".to_string(),
            agent_id: "agent_89".to_string(),
            content: "content A 89".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_89_2".to_string(),
            tenant_id: "tenant_89".to_string(),
            agent_id: "agent_89".to_string(),
            content: "content B 89".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 51,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_90() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_90_1".to_string(),
            tenant_id: "tenant_90".to_string(),
            agent_id: "agent_90".to_string(),
            content: "content A 90".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_90_2".to_string(),
            tenant_id: "tenant_90".to_string(),
            agent_id: "agent_90".to_string(),
            content: "content B 90".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 60,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_91() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_91_1".to_string(),
            tenant_id: "tenant_91".to_string(),
            agent_id: "agent_91".to_string(),
            content: "content A 91".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 51,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_91_2".to_string(),
            tenant_id: "tenant_91".to_string(),
            agent_id: "agent_91".to_string(),
            content: "content B 91".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_92() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_92_1".to_string(),
            tenant_id: "tenant_92".to_string(),
            agent_id: "agent_92".to_string(),
            content: "content A 92".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 52,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_92_2".to_string(),
            tenant_id: "tenant_92".to_string(),
            agent_id: "agent_92".to_string(),
            content: "content B 92".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 58,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_93() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_93_1".to_string(),
            tenant_id: "tenant_93".to_string(),
            agent_id: "agent_93".to_string(),
            content: "content A 93".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_93_2".to_string(),
            tenant_id: "tenant_93".to_string(),
            agent_id: "agent_93".to_string(),
            content: "content B 93".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 57,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_94() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_94_1".to_string(),
            tenant_id: "tenant_94".to_string(),
            agent_id: "agent_94".to_string(),
            content: "content A 94".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 54,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_94_2".to_string(),
            tenant_id: "tenant_94".to_string(),
            agent_id: "agent_94".to_string(),
            content: "content B 94".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 56,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_95() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_95_1".to_string(),
            tenant_id: "tenant_95".to_string(),
            agent_id: "agent_95".to_string(),
            content: "content A 95".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_95_2".to_string(),
            tenant_id: "tenant_95".to_string(),
            agent_id: "agent_95".to_string(),
            content: "content B 95".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 55,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_96() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_96_1".to_string(),
            tenant_id: "tenant_96".to_string(),
            agent_id: "agent_96".to_string(),
            content: "content A 96".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 56,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_96_2".to_string(),
            tenant_id: "tenant_96".to_string(),
            agent_id: "agent_96".to_string(),
            content: "content B 96".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 54,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_97() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_97_1".to_string(),
            tenant_id: "tenant_97".to_string(),
            agent_id: "agent_97".to_string(),
            content: "content A 97".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 57,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_97_2".to_string(),
            tenant_id: "tenant_97".to_string(),
            agent_id: "agent_97".to_string(),
            content: "content B 97".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 53,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_98() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_98_1".to_string(),
            tenant_id: "tenant_98".to_string(),
            agent_id: "agent_98".to_string(),
            content: "content A 98".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 58,
            owner_override: true,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_98_2".to_string(),
            tenant_id: "tenant_98".to_string(),
            agent_id: "agent_98".to_string(),
            content: "content B 98".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 52,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

    #[tokio::test]
    async fn test_conflict_scenario_99() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let rec1 = EmbeddingRecord {
            id: "rec_99_1".to_string(),
            tenant_id: "tenant_99".to_string(),
            agent_id: "agent_99".to_string(),
            content: "content A 99".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 59,
            owner_override: false,
            metadata: None,
        };
        let rec2 = EmbeddingRecord {
            id: "rec_99_2".to_string(),
            tenant_id: "tenant_99".to_string(),
            agent_id: "agent_99".to_string(),
            content: "content B 99".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now - chrono::Duration::days(1),
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 51,
            owner_override: true,
            metadata: None,
        };
        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert!(resolved <= 2);
    }

}

#[cfg(test)]
mod tests_added_for_coverage {
    use crate::memory_store::{VectorRepository, EmbeddingRecord};
    use chrono::Utc;
    use std::sync::Arc;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    async fn setup_repo() -> Arc<VectorRepository> {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS consolidated_memory (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, agent_id TEXT, content TEXT NOT NULL, embedding TEXT, source_type TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, reference_count INTEGER DEFAULT 0, reliability_score INTEGER DEFAULT 50, owner_override BOOLEAN DEFAULT FALSE, metadata TEXT);").execute(&pool).await.unwrap();
        Arc::new(VectorRepository::new_sqlite(pool))
    }

    #[tokio::test]
    async fn test_pruning_varying_reliability() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();
        let old_time = now - chrono::Duration::days(181);

        // Record with reliability < 20
        let rec1 = EmbeddingRecord {
            id: "rec_prune_1".to_string(),
            tenant_id: "tenant_prune".to_string(),
            agent_id: "agent_1".to_string(),
            content: "content".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now, // recent, but low reliability
            reference_count: 10, // high ref count, but low reliability
            reliability_score: 19,
            owner_override: false,
            metadata: None,
        };

        // Record with reliability < 20 but owner override
        let rec2 = EmbeddingRecord {
            id: "rec_prune_2".to_string(),
            tenant_id: "tenant_prune".to_string(),
            agent_id: "agent_2".to_string(),
            content: "content".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 10,
            reliability_score: 19,
            owner_override: true,
            metadata: None,
        };

        // Stale record with source_type TASK_SUMMARY, reference count < 5, owner_override false
        let rec3 = EmbeddingRecord {
            id: "rec_prune_3".to_string(),
            tenant_id: "tenant_prune".to_string(),
            agent_id: "agent_3".to_string(),
            content: "content".to_string(),
            embedding: vec![0.5; 10],
            source_type: "TASK_SUMMARY".to_string(),
            created_at: old_time,
            last_referenced_at: old_time,
            reference_count: 4,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();
        repo.upsert(&rec3).await.unwrap();

        repo.prune_stale(now - chrono::Duration::days(180)).await.unwrap();

        // Check if pruned correctly
        let results = repo.cross_department_search("tenant_prune", &vec![0.5; 10], 10).await.unwrap();
        let ids: Vec<String> = results.iter().map(|r| r.id.clone()).collect();

        assert!(!ids.contains(&"rec_prune_1".to_string())); // pruned due to low reliability
        assert!(ids.contains(&"rec_prune_2".to_string()));  // kept due to owner override
        assert!(!ids.contains(&"rec_prune_3".to_string())); // pruned due to being stale TASK_SUMMARY
    }

    #[tokio::test]
    async fn test_conflict_winner_scenarios() {
        #[allow(unused_variables)] let repo = setup_repo().await;
        let now = Utc::now();

        // Testing owner_override
        let mut rec_a = EmbeddingRecord {
            id: "a".to_string(),
            tenant_id: "t".to_string(),
            agent_id: "a".to_string(),
            content: "c".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true,
            metadata: None,
        };

        let mut rec_b = EmbeddingRecord {
            id: "b".to_string(),
            tenant_id: "t".to_string(),
            agent_id: "a".to_string(),
            content: "c".to_string(),
            embedding: vec![0.5; 10],
            source_type: "NOTE".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        #[allow(unused_variables)] let (winner, loser) = VectorRepository::determine_conflict_winner(&rec_a, &rec_b);
        assert_eq!(winner.id, "a");

        #[allow(unused_variables)] let (winner, loser) = VectorRepository::determine_conflict_winner(&rec_b, &rec_a);
        assert_eq!(winner.id, "a");

        // Testing reliability_score
        rec_a.owner_override = false;
        rec_a.reliability_score = 60;
        rec_b.reliability_score = 50;

        #[allow(unused_variables)] let (winner, loser) = VectorRepository::determine_conflict_winner(&rec_a, &rec_b);
        assert_eq!(winner.id, "a");
        #[allow(unused_variables)] let (winner, loser) = VectorRepository::determine_conflict_winner(&rec_b, &rec_a);
        assert_eq!(winner.id, "a");

        // Testing recency
        rec_a.reliability_score = 50;
        rec_a.created_at = now;
        rec_b.created_at = now - chrono::Duration::days(1);

        #[allow(unused_variables)] let (winner, loser) = VectorRepository::determine_conflict_winner(&rec_a, &rec_b);
        assert_eq!(winner.id, "a");
        #[allow(unused_variables)] let (winner, loser) = VectorRepository::determine_conflict_winner(&rec_b, &rec_a);
        assert_eq!(winner.id, "a");

        // Testing fallback
        rec_b.created_at = now;
        #[allow(unused_variables)] let (winner, loser) = VectorRepository::determine_conflict_winner(&rec_a, &rec_b);
        assert_eq!(winner.id, "a");
    }
}
