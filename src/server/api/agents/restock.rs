use axum::{
    extract::{Extension, State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use ::server_common::Claims;

#[derive(Deserialize)]
pub struct RestockRequest {
    pub target_product_id: String,
    pub restock_qty: i32,
    pub approval_id: String,
}

#[derive(Serialize)]
pub struct RestockResponse {
    pub success: bool,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(restock_action))
        .with_state(orchestrator)
}

async fn restock_action(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RestockRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(RestockResponse { success: false })).into_response(),
    };

    let db = &orchestrator.db;

    match &db.store {
        crate::db::DbStore::Postgres => {
            if let Err(e) = sqlx::query("UPDATE products SET inventory_count = COALESCE(inventory_count, 0) + $1 WHERE id = $2 AND tenant_id = $3")
                .bind(payload.restock_qty)
                .bind(&payload.target_product_id)
                .bind(&tenant_id)
                .execute(&db.pool)
                .await {
                    tracing::error!("Failed to update products inventory: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(RestockResponse { success: false })).into_response();
                }

            if let Err(e) = sqlx::query("UPDATE products SET inventory_count = COALESCE(inventory_count, 0) + $1 WHERE id = $2 AND organization_id = $3")
                .bind(payload.restock_qty)
                .bind(&payload.target_product_id)
                .bind(&tenant_id)
                .execute(&db.pool)
                .await {
                    tracing::error!("Failed to update products inventory (org_id): {}", e);
                }

            if let Err(e) = sqlx::query("UPDATE agent_approvals SET status = 'APPROVED' WHERE id = $1 AND tenant_id = $2")
                .bind(&payload.approval_id)
                .bind(&tenant_id)
                .execute(&db.pool)
                .await {
                    tracing::error!("Failed to update agent_approvals: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(RestockResponse { success: false })).into_response();
                }

            if let Err(e) = sqlx::query("UPDATE inventory_forecasts SET status = 'COMPLETED' WHERE item_id = $1 AND tenant_id = $2")
                .bind(&payload.target_product_id)
                .bind(&tenant_id)
                .execute(&db.pool)
                .await {
                    tracing::error!("Failed to update inventory_forecasts: {}", e);
                }
        },
        crate::db::DbStore::Sqlite(pool) => {
            if let Err(e) = sqlx::query("UPDATE products SET inventory_count = COALESCE(inventory_count, 0) + ? WHERE id = ? AND tenant_id = ?")
                .bind(payload.restock_qty)
                .bind(&payload.target_product_id)
                .bind(&tenant_id)
                .execute(pool)
                .await {
                    tracing::error!("Failed to update products inventory: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(RestockResponse { success: false })).into_response();
                }

            if let Err(e) = sqlx::query("UPDATE products SET inventory_count = COALESCE(inventory_count, 0) + ? WHERE id = ? AND organization_id = ?")
                .bind(payload.restock_qty)
                .bind(&payload.target_product_id)
                .bind(&tenant_id)
                .execute(pool)
                .await {
                    tracing::error!("Failed to update products inventory (org_id): {}", e);
                }

            if let Err(e) = sqlx::query("UPDATE agent_approvals SET status = 'APPROVED' WHERE id = ? AND tenant_id = ?")
                .bind(&payload.approval_id)
                .bind(&tenant_id)
                .execute(pool)
                .await {
                    tracing::error!("Failed to update agent_approvals: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(RestockResponse { success: false })).into_response();
                }

            if let Err(e) = sqlx::query("UPDATE inventory_forecasts SET status = 'COMPLETED' WHERE item_id = ? AND tenant_id = ?")
                .bind(&payload.target_product_id)
                .bind(&tenant_id)
                .execute(pool)
                .await {
                    tracing::error!("Failed to update inventory_forecasts: {}", e);
                }
        }
    }

    (StatusCode::OK, Json(RestockResponse { success: true })).into_response()
}
