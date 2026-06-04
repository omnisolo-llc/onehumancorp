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
            ::server_telemetry::record_error_signal("Failed to acquire DB connection");
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
        ::server_telemetry::record_error_signal("Failed to insert product");
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
            ::server_telemetry::record_error_signal("Failed to insert subscription plan");
            tracing::error!("Failed to insert subscription plan: {}", e);
            // Non-fatal, just log it. The product was created.
        }
    }

    let event_payload = serde_json::json!({
        "product_id": product_id,
        "name": payload.name,
        "organization_id": tenant_id,
    });

    let event = ::server_ohc::orchestration::TeammateMeshEvent {
        agent_id: "system".to_string(),
        action: "ProductCreated".to_string(),
        status: "success".to_string(),
        payload: serde_json::to_vec(&event_payload).unwrap_or_default(),
        msg_id: uuid::Uuid::new_v4().to_string(),
    };

    let _ = hub.publish_teammate_event("products_inbox".to_string(), event);

    (StatusCode::OK, Json(CreateProductResponse { success: true, message: Some(format!("Created {}", payload.name)) })).into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/product", post(handle_create_product))
        .layer(Extension(hub))
}
