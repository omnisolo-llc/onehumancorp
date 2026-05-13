use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
use std::sync::Arc;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

#[tokio::test]
async fn test_auto_resolve_conflicts_e2e_comprehensive() {
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
    let now = chrono::Utc::now();

    // Base Record
    let record1 = EmbeddingRecord {
        id: "conflict_base_1".to_string(),
        tenant_id: "org_conflict".to_string(),
        agent_id: "agent_a".to_string(),
        content: "The main office is located in New York.".to_string(),
        embedding: vec![0.9, 0.9, 0.9],
        source_type: "SESSION_DATA".to_string(),
        created_at: now - chrono::Duration::days(10),
        last_referenced_at: now,
        reference_count: 5,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record1).await.expect("Failed to upsert");

    // Conflict 1: Higher reliability score
    let record2 = EmbeddingRecord {
        id: "conflict_higher_rel_1".to_string(),
        tenant_id: "org_conflict".to_string(),
        agent_id: "agent_b".to_string(),
        content: "The main office is located in New Jersey.".to_string(),
        embedding: vec![0.9, 0.9, 0.9],
        source_type: "SESSION_DATA".to_string(),
        created_at: now - chrono::Duration::days(5),
        last_referenced_at: now,
        reference_count: 2,
        reliability_score: 80, // Higher score wins
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record2).await.expect("Failed to upsert");

    // Conflict 2: Owner override
    let record3 = EmbeddingRecord {
        id: "conflict_owner_over_1".to_string(),
        tenant_id: "org_conflict".to_string(),
        agent_id: "agent_c".to_string(),
        content: "The main office is located in California.".to_string(),
        embedding: vec![0.9, 0.9, 0.9],
        source_type: "MANUAL_ENTRY".to_string(),
        created_at: now - chrono::Duration::days(1),
        last_referenced_at: now,
        reference_count: 1,
        reliability_score: 30, // Lower score but overridden
        owner_override: true, // This wins all
        metadata: None,
    };
    repo.upsert(&record3).await.expect("Failed to upsert");

    // Verify initial count is 3
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM consolidated_memory")
        .fetch_one(&pool).await.expect("Failed to query count");
    assert_eq!(count, 3, "Initial state should have 3 records");

    // Trigger auto-resolution
    let resolved = repo.auto_resolve_conflicts().await.expect("Auto resolve failed");

    // There are 3 records -> 3 pairs of conflicts. In a full run, it will compare them and resolve.
    assert!(resolved > 0, "Conflicts should be resolved");

    // Check final count. Should be 1 (the owner override one)
    let final_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM consolidated_memory")
        .fetch_one(&pool).await.expect("Failed to query count");
    assert_eq!(final_count, 1, "Only one record should remain after resolution");

    // Verify the winner is the owner override
    let winner_id: String = sqlx::query_scalar("SELECT id FROM consolidated_memory LIMIT 1")
        .fetch_one(&pool).await.expect("Failed to query winner ID");
    assert_eq!(winner_id, "conflict_owner_over_1", "Owner override should win");
}

#[tokio::test]
async fn test_auto_resolve_conflicts_recency_win() {
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
    let now = chrono::Utc::now();

    // Older Record
    let record1 = EmbeddingRecord {
        id: "conflict_old".to_string(),
        tenant_id: "org_recency".to_string(),
        agent_id: "agent_a".to_string(),
        content: "Store hours are 9am to 5pm.".to_string(),
        embedding: vec![0.8, 0.8, 0.8],
        source_type: "SESSION_DATA".to_string(),
        created_at: now - chrono::Duration::days(10),
        last_referenced_at: now,
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record1).await.expect("Failed to upsert");

    // Newer Record (same reliability, no override)
    let record2 = EmbeddingRecord {
        id: "conflict_new".to_string(),
        tenant_id: "org_recency".to_string(),
        agent_id: "agent_b".to_string(),
        content: "Store hours are 10am to 6pm.".to_string(),
        embedding: vec![0.8, 0.8, 0.8],
        source_type: "SESSION_DATA".to_string(),
        created_at: now - chrono::Duration::days(1),
        last_referenced_at: now,
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record2).await.expect("Failed to upsert");

    // Trigger auto-resolution
    let resolved = repo.auto_resolve_conflicts().await.expect("Auto resolve failed");
    assert_eq!(resolved, 1, "One conflict should be resolved");

    // Check final count
    let final_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM consolidated_memory")
        .fetch_one(&pool).await.expect("Failed to query count");
    assert_eq!(final_count, 1, "Only one record should remain after resolution");

    // Verify the winner is the newer record
    let winner_id: String = sqlx::query_scalar("SELECT id FROM consolidated_memory LIMIT 1")
        .fetch_one(&pool).await.expect("Failed to query winner ID");
    assert_eq!(winner_id, "conflict_new", "Newer record should win");
}

#[tokio::test]
async fn test_auto_resolve_conflicts_multiple_tenants() {
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
    let now = chrono::Utc::now();

    // Tenant A conflicts
    let record_a1 = EmbeddingRecord {
        id: "tenant_a_1".to_string(),
        tenant_id: "tenant_a".to_string(),
        agent_id: "agent_a".to_string(),
        content: "Data A1".to_string(),
        embedding: vec![0.7, 0.7, 0.7],
        source_type: "SESSION_DATA".to_string(),
        created_at: now - chrono::Duration::days(10),
        last_referenced_at: now,
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record_a1).await.expect("Failed to upsert");

    let record_a2 = EmbeddingRecord {
        id: "tenant_a_2".to_string(),
        tenant_id: "tenant_a".to_string(),
        agent_id: "agent_a".to_string(),
        content: "Data A2".to_string(),
        embedding: vec![0.7, 0.7, 0.7],
        source_type: "SESSION_DATA".to_string(),
        created_at: now - chrono::Duration::days(1),
        last_referenced_at: now,
        reference_count: 1,
        reliability_score: 90, // Wins
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record_a2).await.expect("Failed to upsert");

    // Tenant B conflicts
    let record_b1 = EmbeddingRecord {
        id: "tenant_b_1".to_string(),
        tenant_id: "tenant_b".to_string(),
        agent_id: "agent_b".to_string(),
        content: "Data B1".to_string(),
        embedding: vec![0.7, 0.7, 0.7],
        source_type: "SESSION_DATA".to_string(),
        created_at: now - chrono::Duration::days(10),
        last_referenced_at: now,
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record_b1).await.expect("Failed to upsert");

    let record_b2 = EmbeddingRecord {
        id: "tenant_b_2".to_string(),
        tenant_id: "tenant_b".to_string(),
        agent_id: "agent_b".to_string(),
        content: "Data B2".to_string(),
        embedding: vec![0.7, 0.7, 0.7],
        source_type: "SESSION_DATA".to_string(),
        created_at: now - chrono::Duration::days(1),
        last_referenced_at: now,
        reference_count: 1,
        reliability_score: 50,
        owner_override: true, // Wins
        metadata: None,
    };
    repo.upsert(&record_b2).await.expect("Failed to upsert");

    let resolved = repo.auto_resolve_conflicts().await.expect("Auto resolve failed");
    assert_eq!(resolved, 2, "Two conflicts should be resolved across tenants");

    let final_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM consolidated_memory")
        .fetch_one(&pool).await.expect("Failed to query count");
    assert_eq!(final_count, 2, "Two records should remain");

    let winner_a_id: String = sqlx::query_scalar("SELECT id FROM consolidated_memory WHERE tenant_id = 'tenant_a'")
        .fetch_one(&pool).await.expect("Failed to query winner A ID");
    assert_eq!(winner_a_id, "tenant_a_2", "Tenant A should resolve correctly");

    let winner_b_id: String = sqlx::query_scalar("SELECT id FROM consolidated_memory WHERE tenant_id = 'tenant_b'")
        .fetch_one(&pool).await.expect("Failed to query winner B ID");
    assert_eq!(winner_b_id, "tenant_b_2", "Tenant B should resolve correctly");
}

#[tokio::test]
async fn test_auto_resolve_conflicts_fallback() {
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
    let now = chrono::Utc::now();

    // Two identical records to trigger the fallback logic
    let record1 = EmbeddingRecord {
        id: "conflict_ident_1".to_string(),
        tenant_id: "org_ident".to_string(),
        agent_id: "agent_a".to_string(),
        content: "Data".to_string(),
        embedding: vec![0.6, 0.6, 0.6],
        source_type: "SESSION_DATA".to_string(),
        created_at: now,
        last_referenced_at: now,
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record1).await.expect("Failed to upsert");

    let record2 = EmbeddingRecord {
        id: "conflict_ident_2".to_string(),
        tenant_id: "org_ident".to_string(),
        agent_id: "agent_b".to_string(),
        content: "Data".to_string(),
        embedding: vec![0.6, 0.6, 0.6],
        source_type: "SESSION_DATA".to_string(),
        created_at: now,
        last_referenced_at: now,
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record2).await.expect("Failed to upsert");

    let resolved = repo.auto_resolve_conflicts().await.expect("Auto resolve failed");
    assert_eq!(resolved, 1, "One conflict should be resolved");

    let final_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM consolidated_memory")
        .fetch_one(&pool).await.expect("Failed to query count");
    assert_eq!(final_count, 1, "Only one record should remain after resolution");
}
