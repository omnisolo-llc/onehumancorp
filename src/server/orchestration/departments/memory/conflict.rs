use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
use std::sync::Arc;

pub async fn auto_resolve_conflicts(repository: Arc<VectorRepository>) -> Result<usize, String> {
    let conflicts = repository.get_conflicting_pairs().await?;
    let mut resolved_count = 0;

    for (a, b) in conflicts {
        let (winner, loser) = determine_conflict_winner(&a, &b);
        repository.resolve_conflict(winner, loser).await?;
        resolved_count += 1;
    }

    Ok(resolved_count)
}

pub fn determine_conflict_winner<'a>(a: &'a EmbeddingRecord, b: &'a EmbeddingRecord) -> (&'a EmbeddingRecord, &'a EmbeddingRecord) {
    if a.owner_override != b.owner_override {
        if a.owner_override {
            (a, b)
        } else {
            (b, a)
        }
    } else if a.reliability_score != b.reliability_score {
        if a.reliability_score > b.reliability_score {
            (a, b)
        } else {
            (b, a)
        }
    } else if a.created_at != b.created_at {
        if a.created_at > b.created_at {
            (a, b)
        } else {
            (b, a)
        }
    } else {
        (a, b) // Fallback, just pick 'a'
    }
}

#[cfg(test)]
mod determine_conflict_winner_tests {
    use super::*;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;
    use sqlx::Row;

    fn create_test_record(
        id: &str,
        owner_override: bool,
        reliability_score: i32,
        created_at_days_ago: i64,
    ) -> EmbeddingRecord {
        EmbeddingRecord {
            id: id.to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "test".to_string(),
            embedding: vec![1.0],
            source_type: "test".to_string(),
            created_at: chrono::Utc::now() - chrono::Duration::days(created_at_days_ago),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score,
            owner_override,
            metadata: None,
        }
    }

    #[test]
    fn test_winner_owner_override() {
        let a = create_test_record("a", true, 50, 10);
        let b = create_test_record("b", false, 90, 5); // b has better score and is newer, but a has override
        let (winner, loser) = determine_conflict_winner(&a, &b);
        assert_eq!(winner.id, "a");
        assert_eq!(loser.id, "b");

        let (winner2, loser2) = determine_conflict_winner(&b, &a);
        assert_eq!(winner2.id, "a");
        assert_eq!(loser2.id, "b");
    }

    #[test]
    fn test_winner_reliability_score() {
        let a = create_test_record("a", false, 80, 10);
        let b = create_test_record("b", false, 60, 5); // a has better score, b is newer
        let (winner, loser) = determine_conflict_winner(&a, &b);
        assert_eq!(winner.id, "a");
        assert_eq!(loser.id, "b");
    }

    #[test]
    fn test_winner_recency() {
        let a = create_test_record("a", false, 50, 2); // a is newer
        let b = create_test_record("b", false, 50, 10);
        let (winner, loser) = determine_conflict_winner(&a, &b);
        assert_eq!(winner.id, "a");
        assert_eq!(loser.id, "b");
    }

    #[test]
    fn test_winner_fallback() {
        let a = create_test_record("a", false, 50, 5);
        let mut b = create_test_record("b", false, 50, 5); // identical stats
        b.created_at = a.created_at; // Ensure created_at is identical
        let (winner, loser) = determine_conflict_winner(&a, &b);
        assert_eq!(winner.id, "a"); // fallback to a
        assert_eq!(loser.id, "b");
    }

    #[tokio::test]
    async fn test_auto_resolve_conflicts_with_override_new() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
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
        ).execute(&pool).await;

        let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));
        let mut rec1 = create_test_record("a", true, 50, 10);
        let mut rec2 = create_test_record("b", false, 90, 5);
        // Make embeddings similar enough to conflict
        rec1.embedding = vec![0.1; 1536];
        rec2.embedding = vec![0.1; 1536];

        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();

        let resolved = auto_resolve_conflicts(repo.clone()).await.unwrap();
        assert_eq!(resolved, 1);

        let query = "SELECT id, reference_count FROM consolidated_memory";
        let rows = sqlx::query(query).fetch_all(&pool).await.unwrap();

        assert_eq!(rows.len(), 1);
        let id: String = rows[0].try_get("id").unwrap();
        let ref_count: i32 = rows[0].try_get("reference_count").unwrap();

        assert_eq!(id, "a");
        assert_eq!(ref_count, 3);
    }

    #[tokio::test]
    async fn test_auto_resolve_conflicts() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
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
        ).execute(&pool).await;

        let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));
        let mut rec1 = create_test_record("a", false, 50, 10);
        let mut rec2 = create_test_record("b", false, 90, 5); // winner (better score)
        rec1.embedding = vec![0.1; 1536];
        rec2.embedding = vec![0.1; 1536];

        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();

        let resolved = auto_resolve_conflicts(repo.clone()).await.unwrap();
        assert_eq!(resolved, 1);

        let query = "SELECT id, reference_count FROM consolidated_memory";
        let rows = sqlx::query(query).fetch_all(&pool).await.unwrap();

        assert_eq!(rows.len(), 1);
        let id: String = rows[0].try_get("id").unwrap();
        assert_eq!(id, "b");
    }

    #[tokio::test]
    async fn test_auto_resolve_conflicts_fallback() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
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
        ).execute(&pool).await;

        let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));
        let mut rec1 = create_test_record("a", false, 50, 5);
        let mut rec2 = create_test_record("b", false, 50, 5);
        rec2.created_at = rec1.created_at; // Identical
        rec1.embedding = vec![0.1; 1536];
        rec2.embedding = vec![0.1; 1536];

        repo.upsert(&rec1).await.unwrap();
        repo.upsert(&rec2).await.unwrap();

        let resolved = auto_resolve_conflicts(repo.clone()).await.unwrap();
        assert_eq!(resolved, 1);

        let query = "SELECT id, reference_count FROM consolidated_memory";
        let rows = sqlx::query(query).fetch_all(&pool).await.unwrap();

        assert_eq!(rows.len(), 1);
        let id: String = rows[0].try_get("id").unwrap();
        assert_eq!(id, "a");
    }
}
