use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
use std::sync::Arc;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

#[tokio::test]
async fn test_cross_department_memory_search_e2e() {
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

    // Sales Agent Record
    let record1 = EmbeddingRecord {
        id: "sales_1".to_string(),
        tenant_id: "maya_bakery".to_string(),
        agent_id: "sales_agent".to_string(),
        content: "Customer wants 100 vegan cupcakes for a wedding.".to_string(),
        embedding: vec![0.8, 0.2, 0.1], // Semantic meaning: vegan cupcakes wedding
        source_type: "SESSION_DATA".to_string(),
        created_at: now,
        last_referenced_at: now,
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record1).await.expect("Failed to upsert");

    // Marketing Agent Record
    let record2 = EmbeddingRecord {
        id: "marketing_1".to_string(),
        tenant_id: "maya_bakery".to_string(),
        agent_id: "marketing_agent".to_string(),
        content: "We should run a campaign on vegan weddings.".to_string(),
        embedding: vec![0.7, 0.3, 0.1], // Similar semantic meaning
        source_type: "SESSION_DATA".to_string(),
        created_at: now,
        last_referenced_at: now,
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record2).await.expect("Failed to upsert");

    // Inventory Agent Record (Different tenant to verify isolation)
    let record3 = EmbeddingRecord {
        id: "inventory_1".to_string(),
        tenant_id: "bobs_burgers".to_string(),
        agent_id: "inventory_agent".to_string(),
        content: "We are out of vegan patties.".to_string(),
        embedding: vec![0.8, 0.2, 0.1], // Similar embedding but different tenant
        source_type: "SESSION_DATA".to_string(),
        created_at: now,
        last_referenced_at: now,
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record3).await.expect("Failed to upsert");

    // Bakery Production Agent searching for vegan wedding context
    let results = repo.cross_department_search("maya_bakery", &[0.8, 0.2, 0.1], 5).await.expect("Failed to search");

    // SQLite mock semantic search might return all exact or fuzzy matches. We just ensure we get the right tenant ones.
    assert!(!results.is_empty(), "Should return results");

    let ids: Vec<String> = results.iter().map(|r| r.id.clone()).collect();
    assert!(ids.contains(&"sales_1".to_string()), "Should find sales context");
    assert!(ids.contains(&"marketing_1".to_string()), "Should find marketing context");
    assert!(!ids.contains(&"inventory_1".to_string()), "Should NOT find other tenant context");
}

#[tokio::test]
async fn test_cross_department_memory_search_limits() {
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

    for i in 0..10 {
        let record = EmbeddingRecord {
            id: format!("record_{}", i),
            tenant_id: "maya_bakery".to_string(),
            agent_id: "agent_a".to_string(),
            content: "Data".to_string(),
            embedding: vec![0.5, 0.5, 0.5],
            source_type: "SESSION_DATA".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.expect("Failed to upsert");
    }

    let results = repo.cross_department_search("maya_bakery", &[0.5, 0.5, 0.5], 3).await.expect("Failed to search");
    assert_eq!(results.len(), 3, "Should respect the limit parameter");

    let results_large = repo.cross_department_search("maya_bakery", &[0.5, 0.5, 0.5], 100).await.expect("Failed to search");
    assert_eq!(results_large.len(), 10, "Should return all available up to limit");
}

#[tokio::test]
async fn test_cross_department_memory_search_empty_tenant() {
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

    let record = EmbeddingRecord {
        id: "record_1".to_string(),
        tenant_id: "maya_bakery".to_string(),
        agent_id: "agent_a".to_string(),
        content: "Data".to_string(),
        embedding: vec![0.5, 0.5, 0.5],
        source_type: "SESSION_DATA".to_string(),
        created_at: now,
        last_referenced_at: now,
        reference_count: 1,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };
    repo.upsert(&record).await.expect("Failed to upsert");

    let results = repo.cross_department_search("unknown_tenant", &[0.5, 0.5, 0.5], 10).await.expect("Failed to search");
    assert!(results.is_empty(), "Should return empty for unknown tenant");
}
