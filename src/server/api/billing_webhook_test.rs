use axum::{
    routing::post,
    Router,
};
use serde_json::json;

use ::server_pricing::rate_limit::{PlanTier, RedisRateLimiter};
use crate::api::billing_webhook::{stripe_webhook_handler, WebhookState};
use crate::db::DB;

#[test]
fn payment_failure_extracts_subscription_and_customer_refs() {
    let object = json!({
        "customer": "cus_123",
        "parent": {
            "subscription_details": {
                "subscription": "sub_456"
            }
        }
    });

    let lookup = crate::api::billing_webhook::payment_failure_lookup(&object);

    assert_eq!(lookup.customer_id.as_deref(), Some("cus_123"));
    assert_eq!(lookup.stripe_subscription_id.as_deref(), Some("sub_456"));
}

#[test]
fn payment_success_extracts_inventory_locks_for_release() {
    let object = json!({
        "metadata": {
            "inventory_lock_id": "ohc:lock:tenant-1:inventory:prod-1:sess-1",
            "inventory_lock_ids": [
                "ohc:lock:tenant-1:inventory:prod-2:sess-1",
                "ohc:lock:tenant-1:inventory:prod-3:sess-1"
            ]
        }
    });

    let locks = crate::api::billing_webhook::inventory_locks_for_payment_success(&object);

    assert_eq!(locks, vec![
        "ohc:lock:tenant-1:inventory:prod-1:sess-1".to_string(),
        "ohc:lock:tenant-1:inventory:prod-2:sess-1".to_string(),
        "ohc:lock:tenant-1:inventory:prod-3:sess-1".to_string(),
    ]);
}

#[tokio::test]
async fn payment_failure_marks_subscriber_past_due_and_sends_dunning() {
    use crate::api::billing_webhook::{PaymentFailureMessageGenerator, PaymentFailureNotifier};
    use std::sync::{Arc, Mutex};

    struct RecordingNotifier {
        sent: Arc<Mutex<Vec<(String, String)>>>,
    }

    struct FixedGenerator;

    #[async_trait::async_trait]
    impl PaymentFailureNotifier for RecordingNotifier {
        async fn send_payment_failure_sms(&self, subscriber_id: &str, message: &str) -> Result<(), String> {
            self.sent.lock().unwrap().push((subscriber_id.to_string(), message.to_string()));
            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl PaymentFailureMessageGenerator for FixedGenerator {
        async fn generate_payment_failure_message(&self, subscriber_id: &str, business_name: &str) -> String {
            format!("{business_name}:{subscriber_id}:update payment")
        }
    }

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let client = match redis::Client::open(redis_url) {
        Ok(c) => c,
        Err(_) => return,
    };
    if client.get_multiplexed_async_connection().await.is_err() {
        return;
    }

    let db = match DB::new().await {
        Ok(d) => d,
        Err(_) => return,
    };

    if sqlx::query(
        "CREATE TABLE IF NOT EXISTS subscribers (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            customer_id TEXT NOT NULL,
            subscription_plan_id TEXT,
            plan_id TEXT,
            status TEXT NOT NULL DEFAULT 'ACTIVE',
            stripe_subscription_id TEXT,
            created_at BIGINT DEFAULT 0
        )",
    )
    .execute(&db.pool)
    .await
    .is_err()
    {
        return;
    }

    if sqlx::query(
        "INSERT INTO subscribers (id, tenant_id, customer_id, subscription_plan_id, status, stripe_subscription_id)
         VALUES ('subscriber_failed_payment', 'tenant_1', 'cus_failed', 'plan_1', 'ACTIVE', 'sub_failed')
         ON CONFLICT DO NOTHING",
    )
    .execute(&db.pool)
    .await
    .is_err()
    {
        return;
    }

    let state = WebhookState {
        rate_limiter: std::sync::Arc::new(RedisRateLimiter::new(client)),
        db_pool: db.pool.clone(),
        db: std::sync::Arc::new(db),
    };
    let sent = Arc::new(Mutex::new(Vec::new()));
    let notifier = RecordingNotifier { sent: sent.clone() };
    let generator = FixedGenerator;
    let object = json!({
        "customer": "cus_failed",
        "subscription": "sub_failed",
        "metadata": {
            "business_name": "Maya Cakes"
        }
    });

    let processed = crate::api::billing_webhook::process_invoice_payment_failed(
        &state,
        &object,
        &notifier,
        &generator,
    )
    .await
    .expect("failed-payment processing should succeed");

    assert_eq!(processed.as_deref(), Some("subscriber_failed_payment"));
    let row: (String,) = sqlx::query_as("SELECT status FROM subscribers WHERE id = 'subscriber_failed_payment'")
        .fetch_one(&state.db.pool)
        .await
        .expect("subscriber status should be readable");
    assert_eq!(row.0, "PAST_DUE");
    assert_eq!(
        sent.lock().unwrap().as_slice(),
        &[("subscriber_failed_payment".to_string(), "Maya Cakes:subscriber_failed_payment:update payment".to_string())]
    );
}

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
async fn test_stripe_webhook_payment_intent_succeeded_pos() {
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
        db: std::sync::Arc::new(db),
    };

    let app = Router::new()
        .route("/api/v1/webhooks/stripe", post(stripe_webhook_handler))
        .with_state(webhook_state);

    let payload = json!({
        "id": "evt_test_pi",
        "type": "payment_intent.succeeded",
        "data": {
            "object": {
                "amount": 1500,
                "metadata": {
                    "source": "in_person",
                    "tenant_id": "test_tenant",
                    "product_id": "test_product",
                    "quantity": "2",
                    "order_id": "test_order"
                }
            }
        }
    });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Wait for server to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let client_req = reqwest::Client::new();

    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let valid_sig = format!("t={},v1=valid_sig", now);

    let response = client_req.post(format!("http://{}/api/v1/webhooks/stripe", addr))
        .header("Stripe-Signature", valid_sig)
        .json(&payload).send().await.unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_stripe_webhook_pos_transaction() {
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
        db: std::sync::Arc::new(db),
    };

    let app = Router::new()
        .route("/api/v1/webhooks/stripe", post(stripe_webhook_handler))
        .with_state(webhook_state);

    let payload = json!({
        "id": "evt_test_pos",
        "type": "pos_transaction",
        "data": {
            "object": {
                "metadata": {
                    "tenant_id": "test_tenant",
                    "product_id": "test_product",
                    "quantity": "2",
                    "order_id": "test_order"
                }
            }
        }
    });

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client_req = reqwest::Client::new();
    let response = client_req.post(format!("http://{}/api/v1/webhooks/stripe", addr))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);
}
