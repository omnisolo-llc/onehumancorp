use axum::{
    extract::{Extension, Json, Path},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;
use axum::http::StatusCode;
<<<<<<< HEAD
=======
use crate::services::subscription::service::SubscriptionService;
>>>>>>> 566ae988 (feat: Zero-Touch Subscription Engine scaffold)

#[derive(Serialize)]
pub struct SubscriptionPlanResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub amount: i64,
    pub interval: String,
    pub active: bool,
}

#[derive(Serialize)]
pub struct SubscriberResponse {
    pub id: String,
    pub customer_id: String,
    pub status: String,
<<<<<<< HEAD
}

#[derive(Serialize)]
pub struct FulfillmentBatchResponse {
    pub id: String,
    pub target_date: i64,
    pub status: String,
    pub subscriber_count: i64,
=======
    pub plan_id: String,
}

#[derive(Deserialize)]
pub struct SubscribeRequest {
    pub customer_id: String,
    pub plan_id: String,
}

#[derive(Serialize)]
pub struct SubscribeResponse {
    pub success: bool,
    pub subscription_id: Option<String>,
    pub message: Option<String>,
>>>>>>> 566ae988 (feat: Zero-Touch Subscription Engine scaffold)
}

async fn get_plans(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response(),
    };

    let result = sqlx::query(
<<<<<<< HEAD
        "SELECT id, name, description, amount, interval, active FROM subscription_plans WHERE tenant_id = $1"
=======
        "SELECT id, product_id as name, '' as description, discount_percentage as amount, interval, status FROM subscription_plans WHERE tenant_id = $1"
>>>>>>> 566ae988 (feat: Zero-Touch Subscription Engine scaffold)
    )
    .bind(tenant_id)
    .fetch_all(&mut *conn)
    .await;

    match result {
        Ok(rows) => {
            use sqlx::Row;
            let plans: Vec<SubscriptionPlanResponse> = rows.into_iter().map(|r| SubscriptionPlanResponse {
                id: r.try_get("id").unwrap_or_default(),
                name: r.try_get("name").unwrap_or_default(),
                description: r.try_get("description").unwrap_or_default(),
<<<<<<< HEAD
                amount: r.try_get("amount").unwrap_or(0),
                interval: r.try_get("interval").unwrap_or_default(),
                active: r.try_get("active").unwrap_or(true),
=======
                amount: r.try_get::<i32, _>("amount").unwrap_or(0) as i64,
                interval: r.try_get("interval").unwrap_or_default(),
                active: r.try_get::<String, _>("status").unwrap_or_default() == "active",
>>>>>>> 566ae988 (feat: Zero-Touch Subscription Engine scaffold)
            }).collect();
            (StatusCode::OK, Json(plans)).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to fetch subscription plans: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response()
        }
    }
}

async fn get_subscribers(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response(),
    };

    let result = sqlx::query(
<<<<<<< HEAD
        "SELECT id, customer_id, status FROM subscribers WHERE tenant_id = $1"
=======
        "SELECT id, customer_id, status, plan_id FROM subscriptions WHERE tenant_id = $1"
>>>>>>> 566ae988 (feat: Zero-Touch Subscription Engine scaffold)
    )
    .bind(tenant_id)
    .fetch_all(&mut *conn)
    .await;

    match result {
        Ok(rows) => {
            use sqlx::Row;
            let subscribers: Vec<SubscriberResponse> = rows.into_iter().map(|r| SubscriberResponse {
                id: r.try_get("id").unwrap_or_default(),
                customer_id: r.try_get("customer_id").unwrap_or_default(),
                status: r.try_get("status").unwrap_or_default(),
<<<<<<< HEAD
=======
                plan_id: r.try_get("plan_id").unwrap_or_default(),
>>>>>>> 566ae988 (feat: Zero-Touch Subscription Engine scaffold)
            }).collect();
            (StatusCode::OK, Json(subscribers)).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to fetch subscribers: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response()
        }
    }
}

<<<<<<< HEAD
async fn get_fulfillment_batches(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response(),
    };

    // Note: the count relies on subscriber_count logic, which we can join or approximate. For now we will return 0 if no subscribers exist for batch.
    // Usually target_date and batch are managed dynamically by Ops agent.
    let result = sqlx::query(
        "SELECT id, target_date, status FROM fulfillment_batches WHERE tenant_id = $1"
    )
    .bind(tenant_id)
    .fetch_all(&mut *conn)
    .await;

    match result {
        Ok(rows) => {
            use sqlx::Row;
            let batches: Vec<FulfillmentBatchResponse> = rows.into_iter().map(|r| FulfillmentBatchResponse {
                id: r.try_get("id").unwrap_or_default(),
                target_date: r.try_get("target_date").unwrap_or(0),
                status: r.try_get("status").unwrap_or_default(),
                subscriber_count: 0, // This should normally be computed via join
            }).collect();
            (StatusCode::OK, Json(batches)).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to fetch fulfillment batches: {}", e);
=======
async fn subscribe(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<SubscribeRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());
    let service = SubscriptionService::new(hub.pool.clone());

    match service.subscribe_customer(&tenant_id, &payload.plan_id, &payload.customer_id, "mock_stripe_id").await {
        Ok(subscriber) => (StatusCode::OK, Json(SubscribeResponse { success: true, subscription_id: Some(subscriber.id), message: Some("Subscribed successfully".to_string()) })).into_response(),
        Err(e) => {
            tracing::error!("Failed to insert subscription: {}", e);
>>>>>>> 566ae988 (feat: Zero-Touch Subscription Engine scaffold)
            (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct MagicLinkRequest {
    pub token: String,
    pub action: String, // "pause", "resume", "cancel"
}

#[derive(Serialize)]
pub struct MagicLinkResponse {
    pub success: bool,
}

<<<<<<< HEAD
// Simulated Magic Link - In reality, it would verify the token cryptographically
=======
>>>>>>> 566ae988 (feat: Zero-Touch Subscription Engine scaffold)
async fn handle_magic_link(
    Extension(hub): Extension<Arc<Hub>>,
    Json(payload): Json<MagicLinkRequest>,
) -> impl IntoResponse {
<<<<<<< HEAD
    // Basic verification - this is an insecure mock for the E2E.
=======
>>>>>>> 566ae988 (feat: Zero-Touch Subscription Engine scaffold)
    if payload.token.is_empty() {
        return (StatusCode::BAD_REQUEST, "Invalid token").into_response();
    }

<<<<<<< HEAD
    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response(),
    };

    let status = match payload.action.as_str() {
        "pause" => "Paused",
        "resume" => "Active",
        "cancel" => "Canceled",
        _ => return (StatusCode::BAD_REQUEST, "Invalid action").into_response(),
    };

    let update = sqlx::query(
        "UPDATE subscribers SET status = $1 WHERE id = $2"
    )
    .bind(status)
    .bind(payload.token) // Mock: using token as subscriber id
    .execute(&mut *conn)
    .await;

    match update {
=======
    let service = SubscriptionService::new(hub.pool.clone());

    let result = match payload.action.as_str() {
        "cancel" => service.cancel_subscription(&payload.token).await,
        _ => return (StatusCode::BAD_REQUEST, "Invalid action").into_response(),
    };

    match result {
>>>>>>> 566ae988 (feat: Zero-Touch Subscription Engine scaffold)
        Ok(_) => (StatusCode::OK, Json(MagicLinkResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to update subscription via magic link: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response()
        }
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/plans", get(get_plans))
        .route("/subscribers", get(get_subscribers))
<<<<<<< HEAD
        .route("/fulfillment-batches", get(get_fulfillment_batches))
=======
        .route("/subscribe", post(subscribe))
>>>>>>> 566ae988 (feat: Zero-Touch Subscription Engine scaffold)
        .route("/magic-link", post(handle_magic_link))
        .layer(Extension(hub))
}
