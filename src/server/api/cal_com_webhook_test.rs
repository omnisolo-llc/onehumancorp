use axum::{
    routing::post,
    Router,
};
use serde_json::json;
use std::sync::Arc;
use crate::db::DB;
use crate::api::cal_com_webhook::cal_com_webhook_handler;
use crate::api::billing_webhook::WebhookState;

#[tokio::test]
async fn test_cal_com_webhook_handler_booking_created() {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    // Only run if redis is available
    let client = match redis::Client::open(redis_url) {
        Ok(c) => c,
        Err(_) => return,
    };

    if client.get_multiplexed_async_connection().await.is_err() {
        return;
    }

    let rate_limiter = Arc::new(::server_pricing::rate_limit::RedisRateLimiter::new(client));
    let db = match DB::new().await {
        Ok(d) => d,
        Err(_) => return,
    };

    let state = WebhookState {
        rate_limiter,
        db_pool: db.pool.clone(),
        db: Arc::new(db.clone()),
    };

    let app = Router::new()
        .route("/api/v1/webhooks/cal_com", post(cal_com_webhook_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let payload = json!({
        "triggerEvent": "BOOKING_CREATED",
        "payload": {
            "uid": "booking_123",
            "title": "Consultation",
            "startTime": "2024-06-01T10:00:00Z",
            "endTime": "2024-06-01T11:00:00Z",
            "status": "ACCEPTED",
            "metadata": null,
            "attendee": {
                "email": "test@example.com",
                "name": "Test User"
            }
        }
    });

    let client = reqwest::Client::new();
    let response = client.post(format!("http://{}/api/v1/webhooks/cal_com", addr)).json(&payload).send().await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    // Verify Database Tier
    let row: (String,) = sqlx::query_as("SELECT status FROM bookings WHERE id = 'booking_123'")
        .fetch_one(&db.pool)
        .await
        .expect("booking row not found");

    assert_eq!(row.0, "ACCEPTED");
}
