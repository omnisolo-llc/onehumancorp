use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use crate::hub::Hub;
use crate::domain::repository::models::{DeliveryBatch, DeliveryStop, DriverSession};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Utc, Duration};
use ::server_common::Claims;

#[derive(Deserialize)]
pub struct CreateBatchRequest {
    pub order_ids: Vec<String>,
}

#[derive(Serialize)]
pub struct CreateBatchResponse {
    pub batch_id: String,
    pub status: String,
}

pub async fn create_batch_handler(
    axum::extract::Extension(user): axum::extract::Extension<Claims>,
    Json(payload): Json<CreateBatchRequest>,
) -> Result<Json<CreateBatchResponse>, axum::http::StatusCode> {
    let pool = crate::db::get_pool();
    let tenant_id = user.organization_id.unwrap_or_default();

    if payload.order_ids.is_empty() {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }

    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // RLS context
    if let Err(e) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        tracing::error!("Failed to set org context: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Verify orders belong to tenant and exist
    let mut valid_orders = Vec::new();
    for order_id in &payload.order_ids {
        let count: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM orders WHERE id = $1 AND tenant_id = $2"
        )
        .bind(order_id)
        .bind(&tenant_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        if count == 0 {
            return Err(axum::http::StatusCode::BAD_REQUEST);
        }
        valid_orders.push(order_id.clone());
    }

    let batch_id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO delivery_batches (id, tenant_id, status) VALUES ($1, $2, 'optimizing')"
    )
    .bind(&batch_id)
    .bind(&tenant_id)
    .execute(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create stops sequentially for now as a simple heuristic
    for (i, order_id) in valid_orders.iter().enumerate() {
        let stop_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO delivery_stops (id, tenant_id, batch_id, order_id, sequence_index, status) VALUES ($1, $2, $3, $4, $5, 'pending')"
        )
        .bind(&stop_id)
        .bind(&tenant_id)
        .bind(&batch_id)
        .bind(order_id)
        .bind(i as i32)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Simulating optimization complete
    sqlx::query(
        "UPDATE delivery_batches SET status = 'optimized', updated_at = CURRENT_TIMESTAMP WHERE id = $1"
    )
    .bind(&batch_id)
    .execute(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateBatchResponse {
        batch_id,
        status: "optimized".to_string(),
    }))
}

#[derive(Deserialize)]
pub struct DispatchRequest {
    pub phone_number: String,
}

#[derive(Serialize)]
pub struct DispatchResponse {
    pub session_id: String,
    pub magic_link_token: String,
}

pub async fn generate_driver_session_handler(
    axum::extract::Extension(user): axum::extract::Extension<Claims>,
    Path(batch_id): Path<String>,
    Json(payload): Json<DispatchRequest>,
) -> Result<Json<DispatchResponse>, axum::http::StatusCode> {
    let pool = crate::db::get_pool();
    let tenant_id = user.organization_id.unwrap_or_default();

    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // RLS context
    if let Err(e) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        tracing::error!("Failed to set org context: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Verify batch exists and belongs to tenant
    let count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM delivery_batches WHERE id = $1 AND tenant_id = $2"
    )
    .bind(&batch_id)
    .bind(&tenant_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if count == 0 {
        return Err(axum::http::StatusCode::NOT_FOUND);
    }

    let session_id = Uuid::new_v4().to_string();
    let magic_link_token = Uuid::new_v4().to_string(); // Simple secure random string for token
    let expires_at = Utc::now() + Duration::hours(24);

    sqlx::query(
        "INSERT INTO driver_sessions (id, tenant_id, batch_id, phone_number, magic_link_token, expires_at) VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(&session_id)
    .bind(&tenant_id)
    .bind(&batch_id)
    .bind(&payload.phone_number)
    .bind(&magic_link_token)
    .bind(expires_at)
    .execute(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query(
        "UPDATE delivery_batches SET status = 'dispatched', updated_at = CURRENT_TIMESTAMP WHERE id = $1"
    )
    .bind(&batch_id)
    .execute(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(DispatchResponse {
        session_id,
        magic_link_token,
    }))
}

#[derive(Deserialize)]
pub struct UpdateStopRequest {
    pub magic_link_token: String,
    pub status: String,
    pub proof_of_delivery: Option<serde_json::Value>,
}

pub async fn update_stop_status_handler(
    Path(stop_id): Path<String>,
    Json(payload): Json<UpdateStopRequest>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let pool = crate::db::get_pool();

    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // We use a system context because we don't know the tenant ID yet, we must derive it from the token
    if let Err(e) = crate::common::auth_utils::set_system_context(&mut *tx).await {
        tracing::error!("Failed to set system context: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Validate the token and find the session
    let session = sqlx::query_as::<_, DriverSession>(
        "SELECT * FROM driver_sessions WHERE magic_link_token = $1 AND expires_at > CURRENT_TIMESTAMP"
    )
    .bind(&payload.magic_link_token)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let session = match session {
        Some(s) => s,
        None => return Err(axum::http::StatusCode::UNAUTHORIZED),
    };

    // Set context to the session's tenant
    if let Err(e) = crate::common::auth_utils::set_org_context(&mut *tx, &session.tenant_id).await {
        tracing::error!("Failed to set org context: {}", e);
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Verify stop belongs to the batch from the session
    let stop_count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM delivery_stops WHERE id = $1 AND batch_id = $2 AND tenant_id = $3"
    )
    .bind(&stop_id)
    .bind(&session.batch_id)
    .bind(&session.tenant_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if stop_count == 0 {
        return Err(axum::http::StatusCode::NOT_FOUND);
    }

    // Update stop status
    sqlx::query(
        "UPDATE delivery_stops SET status = $1, proof_of_delivery = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3"
    )
    .bind(&payload.status)
    .bind(payload.proof_of_delivery.map(|v| sqlx::types::Json(v)))
    .bind(&stop_id)
    .execute(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({"success": true})))
}
