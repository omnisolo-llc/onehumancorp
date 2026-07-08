use axum::{
    extract::{State, Json},
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_postgres::Error as PgError;
use uuid::Uuid;

use crate::server::db::Pool;
use crate::server::api::auth::require_tenant_id;
use crate::server::api::error::ApiError;

#[derive(Debug, Serialize, Deserialize)]
pub struct OfflineSyncRequest {
    pub idempotency_key: String,
    pub action_type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfflineSyncResponse {
    pub status: String,
    pub job_id: Option<String>,
    pub message: String,
}

pub async fn handle_offline_sync(
    State(pool): State<Arc<Pool>>,
    cookies: CookieJar,
    Json(payload): Json<OfflineSyncRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let tenant_id = require_tenant_id(&cookies)?;

    let db = pool.get().await.map_err(|e| {
        tracing::error!("DB pool error in handle_offline_sync: {}", e);
        ApiError::InternalError("Database unavailable".to_string())
    })?;

    let offline_job_id = Uuid::new_v4().to_string();

    // Begin transaction for idempotent insert
    let tx = db.transaction().await.map_err(|e| {
        tracing::error!("Failed to begin tx: {}", e);
        ApiError::InternalError("Database error".to_string())
    })?;

    let insert_stmt = "
        INSERT INTO offline_action_queue (id, tenant_id, idempotency_key, action_type, payload, status)
        VALUES ($1, $2, $3, $4, $5, 'pending')
        ON CONFLICT (tenant_id, idempotency_key) DO NOTHING
        RETURNING id
    ";

    let result = tx.query_opt(insert_stmt, &[
        &offline_job_id,
        &tenant_id,
        &payload.idempotency_key,
        &payload.action_type,
        &payload.payload
    ]).await.map_err(|e| {
        tracing::error!("Failed to insert into offline queue: {}", e);
        ApiError::InternalError("Database error".to_string())
    })?;

    if result.is_none() {
        // Was a duplicate idempotency key, meaning it's already queued/processed
        tx.rollback().await.map_err(|_| ApiError::InternalError("Rollback failed".to_string()))?;
        return Ok(Json(OfflineSyncResponse {
            status: "already_queued".to_string(),
            job_id: None,
            message: "Action already received".to_string(),
        }));
    }

    // Now insert into the actual job queue
    let ai_job_id = Uuid::new_v4().to_string();
    let insert_ai_job = "
        INSERT INTO ai_job_queue (id, tenant_id, action_type, payload, status)
        VALUES ($1, $2, $3, $4, 'pending')
    ";

    tx.execute(insert_ai_job, &[
        &ai_job_id,
        &tenant_id,
        &payload.action_type,
        &payload.payload
    ]).await.map_err(|e| {
        tracing::error!("Failed to enqueue AI job: {}", e);
        ApiError::InternalError("Failed to process job".to_string())
    })?;

    // Mark offline queue item as processed so we know it transitioned successfully
    let update_offline = "
        UPDATE offline_action_queue SET status = 'processed'
        WHERE id = $1 AND tenant_id = $2
    ";

    tx.execute(update_offline, &[&offline_job_id, &tenant_id]).await.map_err(|e| {
         tracing::error!("Failed to update offline status: {}", e);
         ApiError::InternalError("Failed to process job".to_string())
    })?;

    tx.commit().await.map_err(|e| {
        tracing::error!("Failed to commit tx: {}", e);
        ApiError::InternalError("Database error".to_string())
    })?;

    Ok(Json(OfflineSyncResponse {
        status: "success".to_string(),
        job_id: Some(ai_job_id),
        message: "Action queued successfully".to_string(),
    }))
}
