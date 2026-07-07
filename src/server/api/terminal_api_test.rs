use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use std::sync::Arc;
use crate::hub::Hub;

#[tokio::test]
async fn test_get_terminal_connection_token_unauthenticated() {
    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let pool_tmp = crate::db::get_pool();
    let hub = Arc::new(Hub::new(tx, pool_tmp));
    let app = crate::api::terminal_api::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/token")
                .method("POST")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    let json_body: serde_json::Value = serde_json::from_str(&body_str).unwrap();
    assert_eq!(json_body["success"], false);
    assert!(json_body["error_message"].as_str().unwrap().contains("Unauthenticated"));
}

#[tokio::test]
async fn test_get_terminal_connection_token_authenticated() {
    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let pool_tmp = crate::db::get_pool();
    let hub = Arc::new(Hub::new(tx, pool_tmp));

    let app_with_auth = axum::Router::new()
        .route("/token", axum::routing::get(crate::api::terminal_api::get_terminal_connection_token_handler))
        .with_state(hub)
        .layer(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
            spiffe_id: "spiffe://test".to_string(),
            agent_id: "agent_1".to_string(),
            org_id: "test_tenant".to_string(),
        }));

    let response = app_with_auth
        .oneshot(
            Request::builder()
                .uri("/token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Stripe API key is required") || body_str.contains("Stripe API error") || body_str.contains("Stripe Terminal connection token request failed"));
}

#[tokio::test]
async fn test_capture_payment_intent_unauthenticated() {
    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let pool_tmp = crate::db::get_pool();
    let hub = Arc::new(Hub::new(tx, pool_tmp));
    let app = crate::api::terminal_api::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/intent/capture")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"payment_intent_id": "pi_12345"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Unauthenticated"));
}

#[tokio::test]
async fn test_capture_payment_intent_authenticated() {
    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let pool_tmp = crate::db::get_pool();
    let hub = Arc::new(Hub::new(tx, pool_tmp));

    let app_with_auth = axum::Router::new()
        .route("/intent/capture", axum::routing::post(crate::api::terminal_api::capture_payment_intent_handler))
        .with_state(hub)
        .layer(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
            spiffe_id: "spiffe://test".to_string(),
            agent_id: "agent_1".to_string(),
            org_id: "test_tenant".to_string(),
        }));

    let response = app_with_auth
        .oneshot(
            Request::builder()
                .uri("/intent/capture")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"payment_intent_id": "pi_12345"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Stripe API key is required") || body_str.contains("Stripe API capture request failed") || body_str.contains("Stripe API error"));
}

#[tokio::test]
async fn test_create_payment_intent_authenticated() {
    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let pool_tmp = crate::db::get_pool();
    let hub = Arc::new(Hub::new(tx, pool_tmp));

    let app_with_auth = axum::Router::new()
        .route("/intent", axum::routing::post(crate::api::terminal_api::create_payment_intent_handler))
        .with_state(hub)
        .layer(axum::extract::Extension(::server_auth::orchestration::AuthInfo {
            spiffe_id: "spiffe://test".to_string(),
            agent_id: "agent_1".to_string(),
            org_id: "test_tenant".to_string(),
        }));

    let response = app_with_auth
        .oneshot(
            Request::builder()
                .uri("/intent")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"amount_cents": 1500, "currency": "usd", "product_id": "prod_1", "quantity": 1, "order_id": "ord_1", "idempotency_key": "idem-key-1"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Stripe API key is required") || body_str.contains("Stripe API error") || body_str.contains("Stripe Terminal connection token request failed"));
}

#[tokio::test]
async fn test_create_payment_intent_unauthenticated() {
    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let pool_tmp = crate::db::get_pool();
    let hub = Arc::new(Hub::new(tx, pool_tmp));
    let app = crate::api::terminal_api::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/intent")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"amount_cents": 1000, "currency": "usd", "product_id": null, "quantity": null, "order_id": null}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Unauthenticated"));
}

#[tokio::test]
async fn test_get_terminal_connection_token_authenticated_via_router() {
    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let pool_tmp = crate::db::get_pool();
    let hub = Arc::new(Hub::new(tx, pool_tmp));
    let mut app = crate::api::terminal_api::router(hub);

    let mut req = Request::builder()
        .uri("/token")
        .method("POST")
        .body(Body::empty())
        .unwrap();

    req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "spiffe://test".to_string(),
        agent_id: "agent_1".to_string(),
        org_id: "test_tenant".to_string(),
    });

    let response = app
        .oneshot(req)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Stripe API key is required") || body_str.contains("Stripe API error") || body_str.contains("Stripe Terminal connection token request failed"));
}

#[tokio::test]
async fn test_start_terminal_session_unauthenticated() {
    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let pool_tmp = crate::db::get_pool();
    let hub = Arc::new(Hub::new(tx, pool_tmp));
    let app = crate::api::terminal_api::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/session/start")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"device_id": "test_device"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Unauthenticated"));
}

#[tokio::test]
async fn test_sync_offline_transactions_unauthenticated() {
    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let pool_tmp = crate::db::get_pool();
    let hub = Arc::new(Hub::new(tx, pool_tmp));
    let app = crate::api::terminal_api::router(hub);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/sync_offline")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"session_id": "test_session", "transactions": []}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Unauthenticated"));
}

#[tokio::test]
async fn test_create_payment_intent_authenticated_via_router() {
    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let pool_tmp = crate::db::get_pool();
    let hub = Arc::new(Hub::new(tx, pool_tmp));
    let mut app = crate::api::terminal_api::router(hub);

    let mut req = Request::builder()
        .uri("/intent")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"amount_cents": 1500, "currency": "usd", "product_id": "prod_2", "quantity": 2, "order_id": "ord_2", "idempotency_key": "idem-key-2"}"#))
        .unwrap();

    req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "spiffe://test".to_string(),
        agent_id: "agent_1".to_string(),
        org_id: "test_tenant".to_string(),
    });

    let response = app
        .oneshot(req)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Stripe API key is required") || body_str.contains("Stripe API error") || body_str.contains("Stripe Terminal connection token request failed"));
}

#[tokio::test]
async fn test_offline_sync_reconciliation() {
    let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
    if !database_url.contains("test") {
        return;
    }

    let pool = crate::db::secure_pg_pool_options().connect(&database_url).await.unwrap();
    let tenant_id = "tenant-pos-test-offline-sync";
    let product_id = "prod-pos-test-offline-sync";
    let device_id = "test-device-123";

    sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'POS Test Tenant') ON CONFLICT DO NOTHING")
        .bind(tenant_id).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count, available_quantity) VALUES ($2, $1, 'POS Test Prod', 10, 10) ON CONFLICT DO NOTHING")
        .bind(tenant_id).bind(product_id).execute(&pool).await.unwrap();

    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let pool_tmp = crate::db::get_pool();
    let hub = Arc::new(Hub::new(tx, pool_tmp));
    let mut app = crate::api::terminal_api::router(hub);

    let payload = serde_json::json!([{ "product_id": product_id, "quantity": 100 }]); // Cause shortage
    let req_body = serde_json::json!({
        "session_id": "test_session",
        "transactions": [{
            "id": "tx_offline_sync_1",
            "client_id": device_id,
            "amount_cents": 1000,
            "currency": "USD",
            "payload": payload.to_string(),
            "device_signature": "sig_123"
        }]
    });

    let mut req = Request::builder()
        .uri("/sync_offline")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(req_body.to_string()))
        .unwrap();

    req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "spiffe://test".to_string(),
        agent_id: "agent_1".to_string(),
        org_id: tenant_id.to_string(),
    });

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Verify session updated with pending reconciliation
    let session_row: (String, serde_json::Value) = sqlx::query_as("SELECT sync_status, pending_reconciliation FROM pos_terminal_sessions WHERE tenant_id = $1 AND device_id = $2")
        .bind(tenant_id)
        .bind(device_id)
        .fetch_one(&pool).await.unwrap();

    assert_eq!(session_row.0, "CONFLICTS_PENDING");

    let pending: Vec<serde_json::Value> = serde_json::from_value(session_row.1).unwrap_or_default();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0]["product_id"].as_str().unwrap(), product_id);
    assert_eq!(pending[0]["shortage"].as_i64().unwrap(), 90); // 100 - 10
}

#[tokio::test]
async fn test_reserve_inventory_handler() {
    let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
    if !database_url.contains("test") {
        return;
    }

    let pool = crate::db::secure_pg_pool_options().connect(&database_url).await.unwrap();
    let tenant_id = "tenant-pos-reserve";
    let product_id = "prod-pos-reserve";

    sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'POS Reserve Test') ON CONFLICT DO NOTHING")
        .bind(tenant_id).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count, available_quantity) VALUES ($2, $1, 'POS Test Prod', 5, 5) ON CONFLICT DO NOTHING")
        .bind(tenant_id).bind(product_id).execute(&pool).await.unwrap();

    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let pool_tmp = crate::db::get_pool();
    let hub = Arc::new(Hub::new(tx, pool_tmp));
    let mut app = crate::api::terminal_api::router(hub);

    let req_body = serde_json::json!({
        "tenant_id": tenant_id,
        "product_id": product_id,
        "quantity": 1,
        "ttl_seconds": 15
    });

    let mut req = Request::builder()
        .uri("/reserve")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(req_body.to_string()))
        .unwrap();

    req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "spiffe://test".to_string(),
        agent_id: "agent_1".to_string(),
        org_id: tenant_id.to_string(),
    });

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    let json_body: serde_json::Value = serde_json::from_str(&body_str).unwrap();

    assert_eq!(json_body["success"], true);
    assert!(json_body["lock_id"].as_str().unwrap().len() > 0);
}
