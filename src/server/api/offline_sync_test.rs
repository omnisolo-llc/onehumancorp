use axum::{body::Body, http::Request, routing::post, Router};
use std::sync::Arc;
use tower::ServiceExt;

use crate::{
    api::offline_sync::{offline_sync_handler, OfflineSyncRequest},
    db::{DbStore, DB},
    hub::Hub,
};

#[tokio::test]
async fn test_offline_sync_handler_merges_inventory_correctly() {
    // 1. Setup mock database (SQLite)
    let db = Arc::new(DB::new().await.unwrap());
    db.run_migrations().await.unwrap();

    // 2. Setup mock hub
    let (event_tx, _event_rx) = tokio::sync::mpsc::channel(100);
    let hub = Arc::new(Hub::new(event_tx, db.pool.clone()));

    // 3. Insert initial product inventory state
    let product_id = "prod_123";
    let tenant_id = "tenant_1";

    // Add product to db
    sqlx::query("INSERT INTO products (id, tenant_id, name, inventory_count) VALUES (?, ?, ?, ?)")
        .bind(product_id)
        .bind(tenant_id)
        .bind("Test Product")
        .bind(10)
        .execute(&db.pool)
        .await
        .unwrap();

    // 4. Setup axum app route with states
    let app = Router::new().route(
        "/api/v1/sync/offline",
        post({
            let db = db.clone();
            let hub = hub.clone();
            move |headers: axum::http::HeaderMap, payload: axum::Json<OfflineSyncRequest>| async move {
                offline_sync_handler(db, hub, headers, payload).await
            }
        }),
    );

    // 5. Construct test request simulating a mobile Tap-to-Pay sync event deducting 2 items
    let payload = serde_json::json!({
        "mutations": [
            {
                "product_id": product_id,
                "action": "sale",
                "quantity": 2
            }
        ]
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/sync/offline")
        .header("content-type", "application/json")
        .header("x-spiffe-id", format!("spiffe://ohc/org/example.org/tenant/{}", tenant_id))
        .body(Body::from(serde_json::to_string(&payload).unwrap()))
        .unwrap();

    // 6. Execute request
    let response = app.oneshot(request).await.unwrap();

    // 7. Verify response
    assert_eq!(response.status(), axum::http::StatusCode::OK);

    // 8. Verify the DB inventory count was decremented from 10 to 8
    let row: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = ? AND tenant_id = ?")
        .bind(product_id)
        .bind(tenant_id)
        .fetch_one(&db.pool)
        .await
        .unwrap();

    assert_eq!(row.0, 8);
}

#[tokio::test]
async fn test_offline_sync_handler_prevents_negative_inventory() {
    let db = Arc::new(DB::new().await.unwrap());
    db.run_migrations().await.unwrap();

    let (event_tx, _event_rx) = tokio::sync::mpsc::channel(100);
    let hub = Arc::new(Hub::new(event_tx, db.pool.clone()));

    let product_id = "prod_999";
    let tenant_id = "tenant_1";

    // Setup initial product with only 1 in stock
    sqlx::query("INSERT INTO products (id, tenant_id, name, inventory_count) VALUES (?, ?, ?, ?)")
        .bind(product_id)
        .bind(tenant_id)
        .bind("Low Stock Item")
        .bind(1)
        .execute(&db.pool)
        .await
        .unwrap();

    let app = Router::new().route(
        "/api/v1/sync/offline",
        post({
            let db = db.clone();
            let hub = hub.clone();
            move |headers: axum::http::HeaderMap, payload: axum::Json<OfflineSyncRequest>| async move {
                offline_sync_handler(db, hub, headers, payload).await
            }
        }),
    );

    // Simulate an offline sync event for a larger quantity than what's available
    let payload = serde_json::json!({
        "mutations": [
            {
                "product_id": product_id,
                "action": "sale",
                "quantity": 5
            }
        ]
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/sync/offline")
        .header("content-type", "application/json")
        .header("x-spiffe-id", format!("spiffe://ohc/org/example.org/tenant/{}", tenant_id))
        .body(Body::from(serde_json::to_string(&payload).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), axum::http::StatusCode::OK);

    // Verify the DB inventory count is exactly 0 and NOT -4
    let row: (i32,) = sqlx::query_as("SELECT inventory_count FROM products WHERE id = ? AND tenant_id = ?")
        .bind(product_id)
        .bind(tenant_id)
        .fetch_one(&db.pool)
        .await
        .unwrap();

    assert_eq!(row.0, 0); // CRDT MUST clamp to 0 and avoid negative states
}
