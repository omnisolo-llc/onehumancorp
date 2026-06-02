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

    temp_env::async_with_vars(
        [("STRIPE_WEBHOOK_SECRET", Some("test_secret"))],
        async {
            let app = Router::new()
                .route("/api/v1/webhooks/stripe", post(stripe_webhook_handler))
                .route_layer(axum::middleware::from_fn_with_state(webhook_state.clone(), crate::api::billing_webhook::webhook_security_middleware))
                .with_state(webhook_state.clone());

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

            use hmac::{Hmac, Mac};
            use sha2::Sha256;
            type HmacSha256 = Hmac<Sha256>;

            let now = chrono::Utc::now().timestamp();
            let payload_str = payload.to_string();
            let signed_payload = format!("{}.{}", now, payload_str);
            let mut mac = HmacSha256::new_from_slice(b"test_secret").unwrap();
            mac.update(signed_payload.as_bytes());
            let sig = hex::encode(mac.finalize().into_bytes());
            let sig_header = format!("t={},v1={}", now, sig);

            let client_req = reqwest::Client::new();

            // Missing signature should return 401
            let response_unauth = client_req.post(format!("http://{}/api/v1/webhooks/stripe", addr))
                .json(&payload).send().await.unwrap();
            assert_eq!(response_unauth.status(), reqwest::StatusCode::UNAUTHORIZED);

            // Valid Request
            let response = client_req.post(format!("http://{}/api/v1/webhooks/stripe", addr))
                .header("Stripe-Signature", &sig_header)
                .json(&payload).send().await.unwrap();
            assert_eq!(response.status(), reqwest::StatusCode::OK);

            // Replay Request should also return 200 OK without executing db
            let response_replay = client_req.post(format!("http://{}/api/v1/webhooks/stripe", addr))
                .header("Stripe-Signature", &sig_header)
                .json(&payload).send().await.unwrap();
            assert_eq!(response_replay.status(), reqwest::StatusCode::OK);

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
    ).await;
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

    temp_env::async_with_vars(
        [("STRIPE_WEBHOOK_SECRET", Some("test_secret_deleted"))],
        async {
            let app = Router::new()
                .route("/api/v1/webhooks/stripe", post(stripe_webhook_handler))
                .route_layer(axum::middleware::from_fn_with_state(webhook_state.clone(), crate::api::billing_webhook::webhook_security_middleware))
                .with_state(webhook_state.clone());

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

            use hmac::{Hmac, Mac};
            use sha2::Sha256;
            type HmacSha256 = Hmac<Sha256>;

            let now = chrono::Utc::now().timestamp();
            let payload_str = payload.to_string();
            let signed_payload = format!("{}.{}", now, payload_str);
            let mut mac = HmacSha256::new_from_slice(b"test_secret_deleted").unwrap();
            mac.update(signed_payload.as_bytes());
            let sig = hex::encode(mac.finalize().into_bytes());
            let sig_header = format!("t={},v1={}", now, sig);

            let client_req = reqwest::Client::new();
            let response = client_req.post(format!("http://{}/api/v1/webhooks/stripe", addr))
                .header("Stripe-Signature", &sig_header)
                .json(&payload).send().await.unwrap();
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
    ).await;
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

    temp_env::async_with_vars(
        [("MERCADOPAGO_WEBHOOK_SECRET", Some("test_mp_secret"))],
        async {
            use axum::http::Request;
            use axum::body::Body;
            use axum::middleware::Next;
            use axum::routing::post;
            use axum::Router;

            let app = Router::new()
                .route("/api/v1/webhooks/mercadopago", post(mercadopago_webhook_handler))
                .route_layer(axum::middleware::from_fn_with_state(state.clone(), crate::api::billing_webhook::webhook_security_middleware))
                .with_state(state.clone());

            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let addr = listener.local_addr().unwrap();
            tokio::spawn(async move {
                axum::serve(listener, app).await.unwrap();
            });

            use hmac::{Hmac, Mac};
            use sha2::Sha256;
            type HmacSha256 = Hmac<Sha256>;

            let payload_str = serde_json::to_string(&event).unwrap();
            let mut mac = HmacSha256::new_from_slice(b"test_mp_secret").unwrap();
            mac.update(payload_str.as_bytes());
            let sig = hex::encode(mac.finalize().into_bytes());

            let client_req = reqwest::Client::new();
            let response = client_req.post(format!("http://{}/api/v1/webhooks/mercadopago", addr))
                .header("X-Signature", &sig)
                .header("Content-Type", "application/json")
                .body(payload_str)
                .send().await.unwrap();

            assert_eq!(response.status(), reqwest::StatusCode::OK);
        }
    ).await;
}
