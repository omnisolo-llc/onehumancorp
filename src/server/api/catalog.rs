use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;
use axum::http::StatusCode;


#[derive(Deserialize)]
pub struct SubscribeRequest {
    pub plan_id: String,
    pub customer_id: String,
}

#[derive(Deserialize)]
pub struct ManageSubscriptionRequest {
    pub subscription_id: String,
    pub action: String, // "pause", "resume", "cancel"
}

#[derive(Serialize)]
pub struct SubscriptionPlanItem {
    pub id: String,
    pub name: String,
    pub price_cents: i64,
    pub frequency: String,
}

#[derive(Serialize)]
pub struct SubscriberItem {
    pub id: String,
    pub customer_id: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct ListSubscriptionsResponse {
    pub plans: Vec<SubscriptionPlanItem>,
    pub subscribers: Vec<SubscriberItem>,
}

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub price: String,
    pub duration: Option<i32>,
    pub description: String,
    pub item_type: String,
    pub is_subscription: Option<bool>,
    pub subscription_interval: Option<String>,
    pub subscription_discount: Option<i32>,
}

#[derive(Serialize)]
pub struct CreateProductResponse {
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

async fn handle_create_product(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateProductRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    // Check product quota
    let quota_status = hub.tracker().check_product_quota(&tenant_id).await.unwrap_or_else(|e| {
        tracing::warn!("Failed to check product quota for tenant {}: {}", tenant_id, e);
        ::server_pricing::rate_limit::RateLimitStatus {
            is_allowed: true,
            soft_limit_reached: false,
            user_message: None,
        }
    });

    if quota_status.soft_limit_reached && !quota_status.is_allowed {
        let msg = quota_status.user_message.unwrap_or_else(|| "Tier limit reached. Please upgrade.".to_string());
        return (
            StatusCode::PAYMENT_REQUIRED,
            Json(ErrorResponse {
                error: "LIMIT_EXCEEDED".to_string(),
                message: msg,
            }),
        ).into_response();
    }

    // Record product addition
    let _ = hub.tracker().record_product_added(&tenant_id).await;

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to acquire DB connection: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "DATABASE_ERROR".to_string(),
                    message: "Failed to connect to database".to_string(),
                }),
            ).into_response();
        }
    };

    let product_id = uuid::Uuid::new_v4().to_string();

    let insert_product = sqlx::query(
        "INSERT INTO products (id, tenant_id, title, description, type, inventory_count) VALUES ($1, $2, $3, $4, $5, 100)"
    )
    .bind(&product_id)
    .bind(&tenant_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&payload.item_type)
    .execute(&mut *conn)
    .await;

    if let Err(e) = insert_product {
        tracing::error!("Failed to insert product: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "DATABASE_ERROR".to_string(),
                message: "Failed to create product".to_string(),
            }),
        ).into_response();
    }

    if payload.is_subscription.unwrap_or(false) {
        let plan_id = uuid::Uuid::new_v4().to_string();
        let interval = payload.subscription_interval.unwrap_or_else(|| "Monthly".to_string()).to_lowercase();
        let discount = payload.subscription_discount.unwrap_or(0);

        let insert_plan = sqlx::query(
            "INSERT INTO subscription_plans (id, tenant_id, product_id, interval, discount_percentage) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&plan_id)
        .bind(&tenant_id)
        .bind(&product_id)
        .bind(&interval)
        .bind(discount)
        .execute(&mut *conn)
        .await;

        if let Err(e) = insert_plan {
            tracing::error!("Failed to insert subscription plan: {}", e);
            // Non-fatal, just log it. The product was created.
        }
    }

    (StatusCode::OK, Json(CreateProductResponse { success: true, message: Some(format!("Created {}", payload.name)) })).into_response()
}


async fn handle_checkout_subscription(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<SubscribeRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to acquire DB connection: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "DATABASE_ERROR".to_string(),
                    message: "Failed to connect to database".to_string(),
                }),
            ).into_response();
        }
    };

    let subscription_id = uuid::Uuid::new_v4().to_string();
    let current_time = chrono::Utc::now();
    let end_time = current_time + chrono::Duration::days(30);

    let insert_result = sqlx::query(
        "INSERT INTO subscriptions (id, tenant_id, customer_id, plan_id, status, current_period_end) VALUES ($1, $2, $3, $4, 'active', $5)"
    )
    .bind(&subscription_id)
    .bind(&tenant_id)
    .bind(&payload.customer_id)
    .bind(&payload.plan_id)
    .bind(end_time)
    .execute(&mut *conn)
    .await;

    match insert_result {
        Ok(_) => (StatusCode::OK, Json(CreateProductResponse { success: true, message: Some("Subscribed successfully".to_string()) })).into_response(),
        Err(e) => {
            tracing::error!("Failed to create subscription: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "DATABASE_ERROR".to_string(),
                    message: "Failed to subscribe".to_string(),
                }),
            ).into_response()
        }
    }
}

async fn handle_manage_subscription(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(_claims): Extension<::server_common::Claims>,
    Json(payload): Json<ManageSubscriptionRequest>,
) -> impl IntoResponse {
    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to acquire DB connection: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "DATABASE_ERROR".to_string(),
                    message: "Failed to connect to database".to_string(),
                }),
            ).into_response();
        }
    };

    let status = match payload.action.as_str() {
        "pause" => "paused",
        "resume" => "active",
        "cancel" => "canceled",
        _ => return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "INVALID_ACTION".to_string(),
                message: "Action must be pause, resume, or cancel".to_string(),
            }),
        ).into_response()
    };

    let tenant_id = _claims.organization_id.unwrap_or_else(|| "system".to_string());

    let update_result = sqlx::query(
        "UPDATE subscriptions SET status = $1 WHERE id = $2 AND tenant_id = $3"
    )
    .bind(status)
    .bind(&payload.subscription_id)
    .bind(&tenant_id)
    .execute(&mut *conn)
    .await;

    match update_result {
        Ok(_) => (StatusCode::OK, Json(CreateProductResponse { success: true, message: Some("Subscription updated successfully".to_string()) })).into_response(),
        Err(e) => {
            tracing::error!("Failed to update subscription: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "DATABASE_ERROR".to_string(),
                    message: "Failed to update subscription".to_string(),
                }),
            ).into_response()
        }
    }
}

async fn handle_list_subscriptions(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to acquire DB connection: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "DATABASE_ERROR".to_string(),
                    message: "Failed to connect to database".to_string(),
                }),
            ).into_response();
        }
    };

    let plans_result = sqlx::query(
        "SELECT sp.id, p.title as name, sp.interval, sp.discount_percentage FROM subscription_plans sp JOIN products p ON sp.product_id = p.id WHERE sp.tenant_id = $1"
    )
    .bind(tenant_id.clone())
    .fetch_all(&mut *conn)
    .await;

    use sqlx::Row;
    let plans = match plans_result {
        Ok(records) => records.into_iter().map(|r| SubscriptionPlanItem {
            id: r.try_get("id").unwrap_or_default(),
            name: r.try_get("name").unwrap_or_default(),
            price_cents: 1999, // mock price based on original mock
            frequency: r.try_get("interval").unwrap_or_default(),
        }).collect(),
        Err(_) => vec![],
    };

    let subscribers_result = sqlx::query(
        "SELECT id, customer_id, status FROM subscriptions WHERE tenant_id = $1"
    )
    .bind(tenant_id)
    .fetch_all(&mut *conn)
    .await;

    let subscribers = match subscribers_result {
        Ok(records) => records.into_iter().map(|r| SubscriberItem {
            id: r.try_get("id").unwrap_or_default(),
            customer_id: r.try_get("customer_id").unwrap_or_default(),
            status: r.try_get("status").unwrap_or_default(),
        }).collect(),
        Err(_) => vec![],
    };

    (StatusCode::OK, Json(ListSubscriptionsResponse { plans, subscribers })).into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/product", post(handle_create_product))
        .route("/subscribe", post(handle_checkout_subscription))
        .route("/manage_subscription", post(handle_manage_subscription))
        .route("/subscriptions", axum::routing::get(handle_list_subscriptions))

        .layer(Extension(hub))
}
