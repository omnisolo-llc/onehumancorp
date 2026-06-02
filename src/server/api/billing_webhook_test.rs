use axum::{
    routing::post,
    Router,
};
use serde_json::json;

use ::server_pricing::rate_limit::{PlanTier, RedisRateLimiter};
use crate::api::billing_webhook::{stripe_webhook_handler, WebhookState};
use crate::db::DB;

#[tokio::test]
async fn test_stripe_webhook_handler_completed() {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    // Only run if redis is available
    let client = match redis::Client::open(redis_url) {
        Ok(c) => c,
        Err(_) => return,
    };

    if client.get_multiplexed_async_connection().await.is_err() {
        return;
    }

    let rate_limiter = std::sync::Arc::new(RedisRateLimiter::new(client.clone()));
    let db = match DB::new().await {
        Ok(d) => d,
        Err(_) => return,
    };

    let webhook_state = WebhookState {
        rate_limiter: rate_limiter.clone(),
        db_pool: db.pool.clone(),
        db: std::sync::Arc::new(db.clone()),
    };

    // Seed the database with a test tenant
    if sqlx::query("INSERT INTO tenants (tenant_id, tier) VALUES ('test_tenant', 'Starter') ON CONFLICT DO NOTHING")
        .execute(&db.pool).await.is_err() {
        return; // Skip if we can't seed the database
    }

    let app = Router::new()
        .route("/api/v1/webhooks/stripe", post(stripe_webhook_handler))
        .route_layer(axum::middleware::from_fn_with_state(webhook_state.clone(), crate::api::billing_webhook::webhook_security_middleware))
        .with_state(webhook_state);

    let payload = json!({
        "id": "evt_test",
        "type": "checkout.session.completed",
        "data": {
            "object": {
                "metadata": {
                    "tenant_id": "test_tenant",
                    "tier": "Pro"
                }
            }
        }
    });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client_req = reqwest::Client::new();
    let now = chrono::Utc::now().timestamp();
    let valid_sig = format!("t={},v1=valid_sig", now);
    let response = client_req.post(format!("http://{}/api/v1/webhooks/stripe", addr)).header("X-Signature", valid_sig).json(&payload).send().await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    // Verify Redis Tier
    let current_tier = rate_limiter.get_tenant_tier("test_tenant").await.unwrap();
    assert_eq!(current_tier, PlanTier::Pro);

    // Verify Database Tier
    let row: (String,) = sqlx::query_as("SELECT tier FROM tenants WHERE tenant_id = 'test_tenant'")
        .fetch_one(&db.pool)
        .await
        .expect("tenant row not found");

    assert_eq!(row.0, "Pro");
}

#[tokio::test]
async fn test_stripe_webhook_handler_deleted() {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    // Only run if redis is available
    let client = match redis::Client::open(redis_url) {
        Ok(c) => c,
        Err(_) => return,
    };

    if client.get_multiplexed_async_connection().await.is_err() {
        return;
    }

    let rate_limiter = std::sync::Arc::new(RedisRateLimiter::new(client.clone()));
    let db = match DB::new().await {
        Ok(d) => d,
        Err(_) => return,
    };

    let webhook_state = WebhookState {
        rate_limiter: rate_limiter.clone(),
        db_pool: db.pool.clone(),
        db: std::sync::Arc::new(db.clone()),
    };

    // Seed the database with a test tenant
    if sqlx::query("INSERT INTO tenants (tenant_id, tier) VALUES ('test_tenant', 'Pro') ON CONFLICT DO NOTHING")
        .execute(&db.pool).await.is_err() {
        return; // Skip if we can't seed the database
    }

    let app = Router::new()
        .route("/api/v1/webhooks/stripe", post(stripe_webhook_handler))
        .route_layer(axum::middleware::from_fn_with_state(webhook_state.clone(), crate::api::billing_webhook::webhook_security_middleware))
        .with_state(webhook_state);

    let payload = json!({
        "id": "evt_test_deleted",
        "type": "customer.subscription.deleted",
        "data": {
            "object": {
                "metadata": {
                    "tenant_id": "test_tenant",
                }
            }
        }
    });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client_req = reqwest::Client::new();
    let now = chrono::Utc::now().timestamp();
    let valid_sig = format!("t={},v1=valid_sig", now);
    let response = client_req.post(format!("http://{}/api/v1/webhooks/stripe", addr)).header("X-Signature", valid_sig).json(&payload).send().await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    // Verify Redis Tier
    let current_tier = rate_limiter.get_tenant_tier("test_tenant").await.unwrap();
    assert_eq!(current_tier, PlanTier::Free);

    // Verify Database Tier
    let row: (String,) = sqlx::query_as("SELECT tier FROM tenants WHERE tenant_id = 'test_tenant'")
        .fetch_one(&db.pool)
        .await
        .expect("tenant row not found");

    assert_eq!(row.0, "Free");
}

#[tokio::test]
async fn test_mercadopago_webhook_handler_payment_created() {
    use axum::http::StatusCode;
    use axum::extract::{State, Json};
    use axum::response::IntoResponse;
    use crate::api::billing_webhook::{mercadopago_webhook_handler, WebhookState, MercadoPagoEvent, MercadoPagoEventData};
    use std::sync::Arc;

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let client = match redis::Client::open(redis_url) {
        Ok(c) => c,
        Err(_) => return,
    };
    if client.get_multiplexed_async_connection().await.is_err() {
        return;
    }

    let rate_limiter = Arc::new(::server_pricing::rate_limit::RedisRateLimiter::new(client));
    let db = match crate::db::DB::new().await {
        Ok(d) => d,
        Err(_) => return,
    };

    let state = WebhookState {
        rate_limiter,
        db_pool: db.pool.clone(),
        db: Arc::new(db),
    };

    let event = MercadoPagoEvent {
        id: 12345,
        live_mode: true,
        r#type: "payment".to_string(),
        date_created: "2024-05-10T12:00:00Z".to_string(),
        application_id: 123,
        user_id: 456,
        version: 1,
        api_version: "v1".to_string(),
        action: "payment.created".to_string(),
        data: MercadoPagoEventData {
            id: "pay_123".to_string(),
        },
    };

    let response = mercadopago_webhook_handler(State(state), Json(event)).await.into_response();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_webhook_security_invalid_signature() {
    use axum::routing::post;
    use crate::api::billing_webhook::{stripe_webhook_handler, WebhookState};

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let client = match redis::Client::open(redis_url) {
        Ok(c) => c,
        Err(_) => return,
    };
    if client.get_multiplexed_async_connection().await.is_err() { return; }

    let rate_limiter = std::sync::Arc::new(::server_pricing::rate_limit::RedisRateLimiter::new(client.clone()));
    let db = match DB::new().await {
        Ok(d) => d,
        Err(_) => return,
    };

    let webhook_state = WebhookState {
        rate_limiter,
        db_pool: db.pool.clone(),
        db: std::sync::Arc::new(db),
    };

    let app = Router::new()
        .route("/api/v1/webhooks/stripe", post(stripe_webhook_handler))
        .route_layer(axum::middleware::from_fn_with_state(webhook_state.clone(), crate::api::billing_webhook::webhook_security_middleware))
        .with_state(webhook_state);

    let payload = json!({ "id": "evt_invalid_sig", "type": "checkout.session.completed", "data": {} });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap(); });

    let client_req = reqwest::Client::new();
    let response = client_req.post(format!("http://{}/api/v1/webhooks/stripe", addr))
        .header("X-Signature", "invalid")
        .json(&payload).send().await.unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_webhook_security_expired_timestamp() {
    use axum::routing::post;
    use crate::api::billing_webhook::{stripe_webhook_handler, WebhookState};

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let client = match redis::Client::open(redis_url) {
        Ok(c) => c,
        Err(_) => return,
    };
    if client.get_multiplexed_async_connection().await.is_err() { return; }

    let rate_limiter = std::sync::Arc::new(::server_pricing::rate_limit::RedisRateLimiter::new(client.clone()));
    let db = match DB::new().await {
        Ok(d) => d,
        Err(_) => return,
    };

    let webhook_state = WebhookState {
        rate_limiter,
        db_pool: db.pool.clone(),
        db: std::sync::Arc::new(db),
    };

    let app = Router::new()
        .route("/api/v1/webhooks/stripe", post(stripe_webhook_handler))
        .route_layer(axum::middleware::from_fn_with_state(webhook_state.clone(), crate::api::billing_webhook::webhook_security_middleware))
        .with_state(webhook_state);

    let payload = json!({ "id": "evt_expired_ts", "type": "checkout.session.completed", "data": {} });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap(); });

    let expired_ts = chrono::Utc::now().timestamp() - 600; // 10 minutes ago
    let client_req = reqwest::Client::new();
    let response = client_req.post(format!("http://{}/api/v1/webhooks/stripe", addr))
        .header("X-Signature", format!("t={},v1=abc", expired_ts))
        .json(&payload).send().await.unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_webhook_security_replay_protection() {
    use axum::routing::post;
    use crate::api::billing_webhook::{stripe_webhook_handler, WebhookState};

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let client = match redis::Client::open(redis_url) {
        Ok(c) => c,
        Err(_) => return,
    };
    if client.get_multiplexed_async_connection().await.is_err() { return; }

    let rate_limiter = std::sync::Arc::new(::server_pricing::rate_limit::RedisRateLimiter::new(client.clone()));
    let db = match DB::new().await {
        Ok(d) => d,
        Err(_) => return,
    };

    let webhook_state = WebhookState {
        rate_limiter,
        db_pool: db.pool.clone(),
        db: std::sync::Arc::new(db),
    };

    let app = Router::new()
        .route("/api/v1/webhooks/stripe", post(stripe_webhook_handler))
        .route_layer(axum::middleware::from_fn_with_state(webhook_state.clone(), crate::api::billing_webhook::webhook_security_middleware))
        .with_state(webhook_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap(); });

    let payload = json!({ "id": "evt_replay_test", "type": "checkout.session.completed", "data": {} });
    let ts = chrono::Utc::now().timestamp();
    let sig = format!("t={},v1=abc", ts);

    let client_req = reqwest::Client::new();

    // First request should be 200 OK
    let response1 = client_req.post(format!("http://{}/api/v1/webhooks/stripe", addr))
        .header("X-Signature", &sig)
        .json(&payload).send().await.unwrap();
    assert_eq!(response1.status(), reqwest::StatusCode::OK);

    // Second request with same ID should also be 200 OK (idempotent ignore)
    let response2 = client_req.post(format!("http://{}/api/v1/webhooks/stripe", addr))
        .header("X-Signature", &sig)
        .json(&payload).send().await.unwrap();
    assert_eq!(response2.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_stripe_webhook_handler_payment_succeeded() {
    use axum::routing::post;
    use crate::api::billing_webhook::{stripe_webhook_handler, WebhookState};
    use sqlx::Row;

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let client = match redis::Client::open(redis_url) {
        Ok(c) => c,
        Err(_) => return,
    };
    if client.get_multiplexed_async_connection().await.is_err() { return; }

    let rate_limiter = std::sync::Arc::new(::server_pricing::rate_limit::RedisRateLimiter::new(client.clone()));
    let db = match DB::new().await {
        Ok(d) => d,
        Err(_) => return,
    };

    let webhook_state = WebhookState {
        rate_limiter,
        db_pool: db.pool.clone(),
        db: std::sync::Arc::new(db.clone()),
    };

    // Seed the database with a test product
    let _ = match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS products (id TEXT PRIMARY KEY, organization_id TEXT, tenant_id TEXT, name TEXT, inventory_count INT, supplier_name TEXT, supplier_contact TEXT)").execute(pool).await;
            sqlx::query("INSERT INTO products (id, organization_id, tenant_id, name, inventory_count) VALUES ('test_prod_sync', 'test_tenant_sync', 'test_tenant_sync', 'Sync Item', 10)")
                .execute(pool).await
        },
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO products (id, organization_id, tenant_id, name, inventory_count) VALUES ('test_prod_sync', 'test_tenant_sync', 'test_tenant_sync', 'Sync Item', 10) ON CONFLICT DO NOTHING")
                .execute(&db.pool).await
        }
    };

    let app = Router::new()
        .route("/api/v1/webhooks/stripe", post(stripe_webhook_handler))
        .route_layer(axum::middleware::from_fn_with_state(webhook_state.clone(), crate::api::billing_webhook::webhook_security_middleware))
        .with_state(webhook_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap(); });

    let items_str = serde_json::to_string(&vec![
        serde_json::json!({"product_id": "test_prod_sync", "quantity": 3})
    ]).unwrap();

    let payload = json!({
        "id": "evt_payment_intent_succeeded",
        "type": "payment_intent.succeeded",
        "data": {
            "object": {
                "metadata": {
                    "tenant_id": "test_tenant_sync",
                    "items": items_str
                }
            }
        }
    });

    let ts = chrono::Utc::now().timestamp();
    let sig = format!("t={},v1=abc", ts);

    let client_req = reqwest::Client::new();
    let response = client_req.post(format!("http://{}/api/v1/webhooks/stripe", addr))
        .header("X-Signature", &sig)
        .json(&payload).send().await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    // Verify Database Tier
    let row = match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("SELECT inventory_count FROM products WHERE id = 'test_prod_sync'").fetch_one(pool).await.expect("product row not found")
        },
        crate::db::DbStore::Postgres => {
            sqlx::query("SELECT inventory_count FROM products WHERE id = 'test_prod_sync'").fetch_one(&db.pool).await.expect("product row not found")
        }
    };

    let count: i32 = row.try_get("inventory_count").unwrap_or(0);
    assert_eq!(count, 7);

    // Verify Ledger Entry
    let row = match &db.store {
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("SELECT action_type FROM ohc_universal_ledger WHERE tenant_id = 'test_tenant_sync'").fetch_one(pool).await.expect("ledger row not found")
        },
        crate::db::DbStore::Postgres => {
            sqlx::query("SELECT action_type FROM ohc_universal_ledger WHERE tenant_id = 'test_tenant_sync'").fetch_one(&db.pool).await.expect("ledger row not found")
        }
    };
    let action_type: String = row.try_get("action_type").unwrap_or_default();
    assert_eq!(action_type, "inventory_decrement");
}
