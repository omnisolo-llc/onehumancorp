use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
use std::sync::Arc;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

#[tokio::test]
async fn test_pruning_stale_context_e2e_comprehensive() {
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

    // 1. Very stale, no override -> Pruned
    let record1 = EmbeddingRecord {
        id: "prune_stale_1".to_string(),
        tenant_id: "org_prune".to_string(),
        agent_id: "agent_a".to_string(),
        content: "Old data 1".to_string(),
        embedding: vec![0.1, 0.1, 0.1],
        source_type: "TASK_SUMMARY".to_string(),
        created_at: now - chrono::Duration::days(200),
        last_referenced_at: now - chrono::Duration::days(190),
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record1).await.expect("Failed to upsert");

    // 2. Very stale, WITH override -> Kept
    let record2 = EmbeddingRecord {
        id: "keep_stale_override_1".to_string(),
        tenant_id: "org_prune".to_string(),
        agent_id: "agent_b".to_string(),
        content: "Old data 2 but overridden".to_string(),
        embedding: vec![0.2, 0.2, 0.2],
        source_type: "MANUAL_ENTRY".to_string(),
        created_at: now - chrono::Duration::days(250),
        last_referenced_at: now - chrono::Duration::days(200),
        reference_count: 1,
        reliability_score: 50,
        owner_override: true, // Prevents pruning
        metadata: None,
    };
    repo.upsert(&record2).await.expect("Failed to upsert");

    // 3. Recently referenced -> Kept
    let record3 = EmbeddingRecord {
        id: "keep_recent_1".to_string(),
        tenant_id: "org_prune".to_string(),
        agent_id: "agent_c".to_string(),
        content: "Recent data".to_string(),
        embedding: vec![0.3, 0.3, 0.3],
        source_type: "TASK_SUMMARY".to_string(),
        created_at: now - chrono::Duration::days(200), // Created old
        last_referenced_at: now - chrono::Duration::days(10), // But referenced recently
        reference_count: 5,
        reliability_score: 80,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record3).await.expect("Failed to upsert");

    // Verify initial count is 3
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM consolidated_memory")
        .fetch_one(&pool).await.expect("Failed to query count");
    assert_eq!(count, 3, "Initial state should have 3 records");

    // Run Pruning process (threshold 180 days)
    repo.prune_stale(now - chrono::Duration::days(180)).await.expect("Failed to prune stale context");

    // Check final count. Should be 2
    let final_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM consolidated_memory")
        .fetch_one(&pool).await.expect("Failed to query count");
    assert_eq!(final_count, 2, "Only 2 records should remain after pruning");

    // Verify the correct ones were kept
    let ids: Vec<String> = sqlx::query_scalar("SELECT id FROM consolidated_memory")
        .fetch_all(&pool).await.expect("Failed to query remaining IDs");
    assert!(ids.contains(&"keep_stale_override_1".to_string()), "Overridden record should be kept");
    assert!(ids.contains(&"keep_recent_1".to_string()), "Recently referenced record should be kept");
    assert!(!ids.contains(&"prune_stale_1".to_string()), "Stale un-overridden record should be pruned");
}

#[tokio::test]
async fn test_pruning_stale_context_multi_tenant() {
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

    // Tenant A: Stale
    let record1 = EmbeddingRecord {
        id: "tenant_a_stale".to_string(),
        tenant_id: "tenant_a".to_string(),
        agent_id: "agent_a".to_string(),
        content: "Data".to_string(),
        embedding: vec![0.1, 0.1, 0.1],
        source_type: "TASK_SUMMARY".to_string(),
        created_at: now - chrono::Duration::days(365),
        last_referenced_at: now - chrono::Duration::days(300),
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record1).await.expect("Failed to upsert");

    // Tenant B: Stale
    let record2 = EmbeddingRecord {
        id: "tenant_b_stale".to_string(),
        tenant_id: "tenant_b".to_string(),
        agent_id: "agent_b".to_string(),
        content: "Data".to_string(),
        embedding: vec![0.1, 0.1, 0.1],
        source_type: "TASK_SUMMARY".to_string(),
        created_at: now - chrono::Duration::days(365),
        last_referenced_at: now - chrono::Duration::days(300),
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record2).await.expect("Failed to upsert");

    // Run Pruning process
    repo.prune_stale(now - chrono::Duration::days(180)).await.expect("Failed to prune stale context");

    let final_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM consolidated_memory")
        .fetch_one(&pool).await.expect("Failed to query count");
    assert_eq!(final_count, 0, "All stale records should be pruned across all tenants");
}

#[tokio::test]
async fn test_pruning_stale_context_varying_thresholds() {
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

    // 1 Year old
    let record1 = EmbeddingRecord {
        id: "1_year_old".to_string(),
        tenant_id: "org".to_string(),
        agent_id: "agent".to_string(),
        content: "Data".to_string(),
        embedding: vec![0.1, 0.1, 0.1],
        source_type: "TASK_SUMMARY".to_string(),
        created_at: now - chrono::Duration::days(365),
        last_referenced_at: now - chrono::Duration::days(365),
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record1).await.expect("Failed to upsert");

    // 6 Months old
    let record2 = EmbeddingRecord {
        id: "6_months_old".to_string(),
        tenant_id: "org".to_string(),
        agent_id: "agent".to_string(),
        content: "Data".to_string(),
        embedding: vec![0.1, 0.1, 0.1],
        source_type: "TASK_SUMMARY".to_string(),
        created_at: now - chrono::Duration::days(180),
        last_referenced_at: now - chrono::Duration::days(180),
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record2).await.expect("Failed to upsert");

    // 1 Month old
    let record3 = EmbeddingRecord {
        id: "1_month_old".to_string(),
        tenant_id: "org".to_string(),
        agent_id: "agent".to_string(),
        content: "Data".to_string(),
        embedding: vec![0.1, 0.1, 0.1],
        source_type: "TASK_SUMMARY".to_string(),
        created_at: now - chrono::Duration::days(30),
        last_referenced_at: now - chrono::Duration::days(30),
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record3).await.expect("Failed to upsert");

    // Prune older than 200 days
    repo.prune_stale(now - chrono::Duration::days(200)).await.expect("Failed to prune");

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM consolidated_memory")
        .fetch_one(&pool).await.expect("Failed to query count");
    assert_eq!(count, 2, "Only the 1 year old record should be pruned");

    // Prune older than 100 days
    repo.prune_stale(now - chrono::Duration::days(100)).await.expect("Failed to prune");

    let count2: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM consolidated_memory")
        .fetch_one(&pool).await.expect("Failed to query count");
    assert_eq!(count2, 1, "The 6 months old record should be pruned now");

    // Prune older than 10 days
    repo.prune_stale(now - chrono::Duration::days(10)).await.expect("Failed to prune");

    let count3: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM consolidated_memory")
        .fetch_one(&pool).await.expect("Failed to query count");
    assert_eq!(count3, 0, "The 1 month old record should be pruned now");
}
