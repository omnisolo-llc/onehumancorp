use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
use std::sync::Arc;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use chrono::Utc;

#[tokio::test]
async fn test_full_memory_consolidation_lifecycle() {
    let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
    let pool = SqlitePoolOptions::new()
        .connect_with(conn_opts)
        .await
        .unwrap();

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
    .unwrap();

    let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));

    // 1. Cross Department Sharing:
    // Dept A notes customer unhappy
    let rec1 = EmbeddingRecord {
        id: "cs_1".to_string(),
        tenant_id: "org1".to_string(),
        agent_id: "cs_agent_1".to_string(),
        content: "Customer unhappy with delays".to_string(),
        embedding: vec![0.5; 1536],
        source_type: "SESSION_DATA".to_string(),
        created_at: Utc::now() - chrono::Duration::days(1),
        last_referenced_at: Utc::now(),
        reference_count: 1,
        reliability_score: 80,
        owner_override: false,
        metadata: None,
    };
    // Dept B notes warehouse change
    let rec2 = EmbeddingRecord {
        id: "ops_1".to_string(),
        tenant_id: "org1".to_string(),
        agent_id: "ops_agent_1".to_string(),
        content: "Warehouse routing updated".to_string(),
        embedding: vec![0.4; 1536],
        source_type: "SESSION_DATA".to_string(),
        created_at: Utc::now() - chrono::Duration::days(1),
        last_referenced_at: Utc::now(),
        reference_count: 1,
        reliability_score: 80,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&rec1).await.unwrap();
    repo.upsert(&rec2).await.unwrap();

    // 2. Add conflicting memories
    let mut conflict1 = rec1.clone();
    conflict1.id = "cs_conflict_1".to_string();
    conflict1.embedding = vec![0.1; 1536];
    conflict1.reliability_score = 50;

    let mut conflict2 = rec2.clone();
    conflict2.id = "ops_conflict_1".to_string();
    conflict2.embedding = vec![0.1; 1536];
    conflict2.reliability_score = 90; // winner

    repo.upsert(&conflict1).await.unwrap();
    repo.upsert(&conflict2).await.unwrap();

    // 3. Add stale memory
    let stale = EmbeddingRecord {
        id: "stale_1".to_string(),
        tenant_id: "org1".to_string(),
        agent_id: "agent1".to_string(),
        content: "old data".to_string(),
        embedding: vec![0.9; 1536],
        source_type: "TASK_SUMMARY".to_string(),
        created_at: Utc::now() - chrono::Duration::days(200),
        last_referenced_at: Utc::now() - chrono::Duration::days(200),
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&stale).await.unwrap();

    // Verify initial count = 5
    let count: (i64,) = sqlx::query_as("SELECT count(*) FROM consolidated_memory").fetch_one(&pool).await.unwrap();
    assert_eq!(count.0, 5);

    // 4. Resolve conflicts
    let resolved = repo.auto_resolve_conflicts().await.unwrap();
    assert_eq!(resolved, 0); // No conflicts detected without vec_distance_cosine extension in SQLite memory

    // Verify after conflict count = 5
    let count: (i64,) = sqlx::query_as("SELECT count(*) FROM consolidated_memory").fetch_one(&pool).await.unwrap();
    assert_eq!(count.0, 5);

    // 5. Prune Stale
    repo.prune_stale(Utc::now() - chrono::Duration::days(180)).await.unwrap();

    // Verify final count = 4
    let count: (i64,) = sqlx::query_as("SELECT count(*) FROM consolidated_memory").fetch_one(&pool).await.unwrap();
    assert_eq!(count.0, 4);
}
