use axum::{
    routing::post,
    Router,
};
use serde_json::json;

use crate::pricing::rate_limit::{PlanTier, RedisRateLimiter};
use crate::api::billing_webhook::{stripe_webhook_handler, WebhookState};
use crate::db::DB;

#[tokio::test]
async fn test_stripe_webhook_handler_completed() {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    // Only run if redis is available
    let client = match redis::Client::open(redis_url) {
        Ok(c) => c,
        Err(_) => return, // Skip test if no redis
    };

    if client.get_multiplexed_async_connection().await.is_err() {
        return; // Skip if can't connect
    }

    let rate_limiter = std::sync::Arc::new(RedisRateLimiter::new(client.clone()));
    let db = DB::new().await.unwrap();

    let webhook_state = WebhookState {
        rate_limiter: rate_limiter.clone(),
        db_pool: db.pool.clone(),
        db: std::sync::Arc::new(db.clone()),
    };

    // Seed the database with a test tenant
    sqlx::query("INSERT INTO tenants (tenant_id, tier) VALUES ('test_tenant', 'Starter') ON CONFLICT DO NOTHING")
        .execute(&db.pool)
        .await
        .unwrap();

    let app = Router::new()
        .route("/api/v1/webhooks/stripe", post(stripe_webhook_handler))
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
    let response = client_req.post(format!("http://{}/api/v1/webhooks/stripe", addr)).json(&payload).send().await.unwrap();
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
        Err(_) => return, // Skip test if no redis
    };

    if client.get_multiplexed_async_connection().await.is_err() {
        return; // Skip if can't connect
    }

    let rate_limiter = std::sync::Arc::new(RedisRateLimiter::new(client.clone()));
    let db = DB::new().await.unwrap();

    let webhook_state = WebhookState {
        rate_limiter: rate_limiter.clone(),
        db_pool: db.pool.clone(),
        db: std::sync::Arc::new(db.clone()),
    };

    // Seed the database with a test tenant
    sqlx::query("INSERT INTO tenants (tenant_id, tier) VALUES ('test_tenant', 'Pro') ON CONFLICT DO NOTHING")
        .execute(&db.pool)
        .await
        .unwrap();

    let app = Router::new()
        .route("/api/v1/webhooks/stripe", post(stripe_webhook_handler))
        .with_state(webhook_state);

    let payload = json!({
        "id": "evt_test",
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
    let response = client_req.post(format!("http://{}/api/v1/webhooks/stripe", addr)).json(&payload).send().await.unwrap();
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

    let rate_limiter = Arc::new(crate::pricing::rate_limit::RedisRateLimiter::new(client));
    let db = match crate::db::DB::new().await {
        Ok(d) => d,
        Err(_) => return, // skip
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
