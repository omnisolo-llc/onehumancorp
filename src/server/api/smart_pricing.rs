use crate::hub::Hub;
use axum::http::StatusCode;
use axum::{
    Router,
    extract::{Extension, Json},
    response::IntoResponse,
    routing::post,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ApproveDiscountRequest {
    pub policy_id: String,
    pub product_id: String,
    pub discount_amount: f64,
}

#[derive(Serialize)]
pub struct ApproveDiscountResponse {
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

async fn handle_approve_discount(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<ApproveDiscountRequest>,
) -> impl IntoResponse {
    let tenant_id = claims
        .organization_id
        .unwrap_or_else(|| ::server_common::auth_utils::get_default_tenant());

    let mut conn = match hub.pool.begin().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to acquire DB connection: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "DATABASE_ERROR".to_string(),
                    message: "Failed to connect to database".to_string(),
                }),
            )
                .into_response();
        }
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *conn, &tenant_id).await {
        tracing::error!("Failed to set RLS context: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "DATABASE_ERROR".to_string(),
                message: "Failed to connect to database".to_string(),
            }),
        )
            .into_response();
    }

    let discount_id = uuid::Uuid::new_v4().to_string();

    // Expires in 3 days for example
    let expires_at = chrono::Utc::now() + chrono::Duration::days(3);

    let insert_discount = sqlx::query(
        "INSERT INTO active_discounts (id, tenant_id, policy_id, discount_amount, expires_at) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(&discount_id)
    .bind(&tenant_id)
    .bind(&payload.policy_id)
    .bind(payload.discount_amount)
    .bind(expires_at)
    .execute(&mut *conn)
    .await;

    let _ = conn.commit().await;
    if let Err(e) = insert_discount {
        tracing::error!("Failed to insert active discount: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "DATABASE_ERROR".to_string(),
                message: "Failed to apply discount".to_string(),
            }),
        )
            .into_response();
    }

    // Invalidate edge cache (HybridCache in catalog.rs)
    let cache = super::catalog::CATALOG_CACHE.get_or_init(|| crate::utils::cache::HybridCache::new(None));
    cache.invalidate(&tenant_id).await;

    (
        StatusCode::OK,
        Json(ApproveDiscountResponse {
            success: true,
            message: Some("Discount applied successfully".to_string()),
        }),
    )
        .into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/approve", post(handle_approve_discount))
        .layer(Extension(hub))
}
