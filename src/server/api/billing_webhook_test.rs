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

    let client = match redis::Client::open(redis_url) {
        Ok(c) => c,
        Err(_) => return,
    };

    if client.get_multiplexed_async_connection().await.is_err() {
        return;
    }

    let rate_limiter = std::sync::Arc::new(RedisRateLimiter::new(client.clone()));
    let db = DB::new().await.unwrap();

    sqlx::query("INSERT INTO tenants (tenant_id, tier, organization_id, created_at, updated_at) VALUES ('test_tenant', 'Free', 'org1', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
        .execute(&db.pool)
        .await
        .unwrap();

    let webhook_state = WebhookState {
        rate_limiter: rate_limiter.clone(),
        db_pool: db.pool.clone(),
        db: std::sync::Arc::new(db.clone()),
    };

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

    let current_tier = rate_limiter.get_tenant_tier("test_tenant").await.unwrap();
    assert_eq!(current_tier, PlanTier::Pro);

    let row: (String,) = sqlx::query_as("SELECT tier FROM tenants WHERE tenant_id = 'test_tenant'")
        .fetch_one(&db.pool)
        .await
        .unwrap();

    assert_eq!(row.0, "Pro");
}

#[tokio::test]
async fn test_stripe_webhook_handler_deleted() {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    let client = match redis::Client::open(redis_url) {
        Ok(c) => c,
        Err(_) => return,
    };

    if client.get_multiplexed_async_connection().await.is_err() {
        return;
    }

    let rate_limiter = std::sync::Arc::new(RedisRateLimiter::new(client.clone()));
    let db = DB::new().await.unwrap();

    sqlx::query("INSERT INTO tenants (tenant_id, tier, organization_id, created_at, updated_at) VALUES ('test_tenant', 'Pro', 'org1', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
        .execute(&db.pool)
        .await
        .unwrap();

    let webhook_state = WebhookState {
        rate_limiter: rate_limiter.clone(),
        db_pool: db.pool.clone(),
        db: std::sync::Arc::new(db.clone()),
    };

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

    let current_tier = rate_limiter.get_tenant_tier("test_tenant").await.unwrap();
    assert_eq!(current_tier, PlanTier::Free);

    let row: (String,) = sqlx::query_as("SELECT tier FROM tenants WHERE tenant_id = 'test_tenant'")
        .fetch_one(&db.pool)
        .await
        .unwrap();

    assert_eq!(row.0, "Free");
}
