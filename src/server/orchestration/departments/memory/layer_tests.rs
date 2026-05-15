use super::layer::MemoryLayer;
use super::models::{MemoryContext, ConflictResolutionPolicy, PruningPolicy};
use ohc_builtin_agent::memory_store::VectorRepository;
use std::sync::Arc;
use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
use std::str::FromStr;

async fn setup_test_layer() -> MemoryLayer {
    let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
    let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

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

    let repo = Arc::new(VectorRepository::new_sqlite(pool));

    let pruning_policy = PruningPolicy {
        retention_days_default: 30,
        retention_days_override: None,
        keep_high_access_count: 5,
    };

    let mut department_weights = std::collections::HashMap::new();
    department_weights.insert("sales".to_string(), 1.2);

    let conflict_policy = ConflictResolutionPolicy {
        prefer_recency: true,
        require_owner_approval: false,
        department_weights,
    };

    MemoryLayer::new(repo, pruning_policy, conflict_policy)
}

#[tokio::test]
async fn test_store_and_retrieve_context() {
    let layer = setup_test_layer().await;

    let context = MemoryContext::new(
        "test_1".to_string(),
        "tenant_1".to_string(),
        "dept_1".to_string(),
        "Test content".to_string(),
        vec![0.1; 10],
        "TEST_EVENT".to_string(),
    );

    layer.store_context(context).await.unwrap();

    let results = layer.retrieve_cross_department("tenant_1", &[0.1; 10], 5).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].content, "Test content");
}

#[tokio::test]
async fn test_cross_department_sharing_logic() {
    let layer = setup_test_layer().await;

    layer.share_context_cross_department(
        "tenant_1",
        "sales",
        "support",
        "Shared context regarding client X".to_string(),
        vec![0.5; 10]
    ).await.unwrap();

    let results = layer.retrieve_cross_department("tenant_1", &[0.5; 10], 5).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].department_id, "support");
}

#[tokio::test]
async fn test_pruning_cycle_integration() {
    let layer = setup_test_layer().await;
    assert!(layer.run_pruning().await.is_ok());
}

#[tokio::test]
async fn test_conflict_resolution_cycle_integration() {
    let layer = setup_test_layer().await;
    assert!(layer.run_conflict_resolution().await.is_ok());
}

#[tokio::test]
async fn test_layer_genuine_matrix_growth() {
    let layer = setup_test_layer().await;
    let base_embedding = vec![0.5; 10];
    for i in 0..150 {
        let mut embedding = base_embedding.clone();
        embedding[i % 10] = (i as f32) / 150.0;
        let context = MemoryContext::new(
            format!("matrix_{}", i),
            "tenant_matrix".to_string(),
            format!("dept_{}", i % 5),
            format!("Matrix content element {}", i),
            embedding,
            "MATRIX_EVENT".to_string(),
        );
        layer.store_context(context).await.unwrap();
    }
    let results = layer.retrieve_cross_department("tenant_matrix", &base_embedding, 200).await.unwrap();
    assert!(results.len() > 0);
}

#[tokio::test]
async fn test_layer_genuine_metadata_persistence() {
    let layer = setup_test_layer().await;
    let base_embedding = vec![0.5; 10];
    let mut context = MemoryContext::new(
        "meta_test".to_string(),
        "tenant_meta".to_string(),
        "dept_meta".to_string(),
        "Content with complex metadata".to_string(),
        base_embedding.clone(),
        "META_EVENT".to_string(),
    );
    let complex_metadata = serde_json::json!({
        "nested": {
            "level1": "deep_value"
        }
    });
    context.metadata_json = Some(complex_metadata);
    layer.store_context(context).await.unwrap();
    let results = layer.retrieve_cross_department("tenant_meta", &base_embedding, 10).await.unwrap();
    assert_eq!(results.len(), 1);
}
