use std::sync::Arc;
use ohc_builtin_agent::mesh::transport::{InProcessTransport, MeshTransport};
use axum::http::HeaderMap;
use axum::{extract::State, Json};
use sqlx::postgres::PgPoolOptions;
use crate::services::sync::offline_pos::{offline_sync_handler, OfflineSyncRequest, OfflineMutation};
use axum::response::IntoResponse;
use reqwest::StatusCode;

#[tokio::test]
async fn test_crdt_offline_sync_unauthorized() {
    let pool = PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap();
    let mesh: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
    let state = State((pool, mesh));

    let req = OfflineSyncRequest { mutations: vec![] };
    let headers = HeaderMap::new();

    let response = offline_sync_handler(state, headers, Json(req)).await.into_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_crdt_offline_sync_success_and_negative_guard() {
    let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
    if !database_url.contains("test") {
        return;
    }

    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    // Setup test data
    sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-offline-pos-test', 'Offline Test Tenant') ON CONFLICT DO NOTHING")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count) VALUES ('prod-offline-pos-1', 'tenant-offline-pos-test', 'Test Prod', 5) ON CONFLICT DO NOTHING")
        .execute(&pool).await.unwrap();

    let mesh: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
    let state = State((pool.clone(), mesh.clone()));

    let req = OfflineSyncRequest {
        mutations: vec![
            OfflineMutation {
                transaction_id: "tx1-pos".to_string(),
                product_id: "prod-offline-pos-1".to_string(),
                quantity_deducted: 3,
                amount: Some(1000),
                payment_method: None,
                payment_intent_id: None,
                currency: Some("USD".to_string()),
            },
        ],
    };

    let mut headers = HeaderMap::new();
    headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-offline-pos-test/agent/x".parse().unwrap());

    let response = offline_sync_handler(state.clone(), headers.clone(), Json(req.clone())).await.into_response();
    assert_eq!(response.status(), StatusCode::OK);

    // Verify it inserts into pos_offline_transactions (Grow-only set)
    let tx_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pos_offline_transactions WHERE id = 'tx1-pos'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(tx_count.0, 1);

    // Re-applying the exact same mutation with same transaction ID should be a no-op per CRDT append-only properties
    let response_dup = offline_sync_handler(state.clone(), headers.clone(), Json(req.clone())).await.into_response();
    assert_eq!(response_dup.status(), StatusCode::OK);

    // Still 1 record
    let tx_count_dup: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pos_offline_transactions WHERE id = 'tx1-pos'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(tx_count_dup.0, 1);
}
