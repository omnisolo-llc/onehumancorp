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
use crate::services::subscription::service::SubscriptionService;

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
        "SELECT id, product_id as name, '' as description, discount_percentage as amount, interval, status FROM subscription_plans WHERE tenant_id = $1"
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
                amount: r.try_get::<i32, _>("amount").unwrap_or(0) as i64,
                interval: r.try_get("interval").unwrap_or_default(),
                active: r.try_get::<String, _>("status").unwrap_or_default() == "active",
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
        "SELECT id, customer_id, status, plan_id FROM subscriptions WHERE tenant_id = $1"
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
                plan_id: r.try_get("plan_id").unwrap_or_default(),
            }).collect();
            (StatusCode::OK, Json(subscribers)).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to fetch subscribers: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response()
        }
    }
}

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

async fn handle_magic_link(
    Extension(hub): Extension<Arc<Hub>>,
    Json(payload): Json<MagicLinkRequest>,
) -> impl IntoResponse {
    if payload.token.is_empty() {
        return (StatusCode::BAD_REQUEST, "Invalid token").into_response();
    }

    let service = SubscriptionService::new(hub.pool.clone());

    let result = match payload.action.as_str() {
        "cancel" => service.cancel_subscription(&payload.token).await,
        _ => return (StatusCode::BAD_REQUEST, "Invalid action").into_response(),
    };

    match result {
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
        .route("/subscribe", post(subscribe))
        .route("/magic-link", post(handle_magic_link))
        .layer(Extension(hub))
}
