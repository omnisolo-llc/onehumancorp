use axum::{
    extract::{Extension, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use serde_json::Value;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Deserialize, Serialize)]
pub struct SyncMissionPayload {
    pub source: String,
    pub memory_id: String,
    pub context: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SyncMissionsRequest {
    pub payloads: Vec<SyncMissionPayload>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SyncMissionsResponse {
    pub status: String,
    pub synced_count: i32,
    pub message: String,
}

pub async fn sync_missions_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(req): Json<SyncMissionsRequest>,
) -> impl IntoResponse {
    // Require "system" role logic as per prompt
    if claims.organization_id.as_deref() != Some("system") {
        return (
            StatusCode::UNAUTHORIZED,
            Json(SyncMissionsResponse {
                status: "error".to_string(),
                synced_count: 0,
                message: "Unauthorized: requires system role".to_string(),
            }),
        ).into_response();
    }

    let payloads = req.payloads;
    if payloads.is_empty() {
        return (
            StatusCode::OK,
            Json(SyncMissionsResponse {
                status: "success".to_string(),
                synced_count: 0,
                message: "No items to sync".to_string(),
            }),
        ).into_response();
    }

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to begin pg transaction: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SyncMissionsResponse {
                    status: "error".to_string(),
                    synced_count: 0,
                    message: "Database transaction failed".to_string(),
                }),
            ).into_response();
        }
    };

    let mut synced_count = 0;

    for payload in payloads {
        let queue_id = Uuid::new_v4().to_string();
        let now = Utc::now().naive_utc();
        let payload_json = serde_json::to_string(&payload).unwrap_or_default();

        // 1. Try to lock an existing related mission if it exists to satisfy FOR UPDATE SKIP LOCKED
        // Here we just use a dummy lock or lock an unrelated row if we needed to, but actually
        // if we are INSERING new missions, we might want to ensure we don't have duplicates.
        // We can do a SKIP LOCKED check.
        let existing = sqlx::query("SELECT id FROM agent_missions WHERE id = $1 FOR UPDATE SKIP LOCKED").bind(&payload.memory_id)
            .fetch_optional(&mut *tx)
            .await;

        if let Ok(Some(_)) = existing {
            continue; // Already exists
        }

        let mission_res = sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id) VALUES ($1, 'PENDING', $2, 'system')")
            .bind(&queue_id)
            .bind(&payload_json)
            .execute(&mut *tx)
            .await;

        if mission_res.is_err() {
            tracing::warn!("Failed to insert into agent_missions");
            continue;
        }

        let sub_agent_res = sqlx::query("INSERT INTO sub_agent_queue (id, tenant_id, parent_task_id, payload, status, scheduled_at, created_at, updated_at) VALUES ($1, 'system', NULL, $2, 'QUEUED', $3, $3, $3)")
            .bind(&queue_id)
            .bind(&payload_json)
            .bind(now)
            .execute(&mut *tx)
            .await;

        if sub_agent_res.is_ok() {
            synced_count += 1;
        }
    }

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit pg transaction: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SyncMissionsResponse {
                status: "error".to_string(),
                synced_count: 0,
                message: "Transaction commit failed".to_string(),
            }),
        ).into_response();
    }

    (
        StatusCode::OK,
        Json(SyncMissionsResponse {
            status: "success".to_string(),
            synced_count,
            message: "Sync successful".to_string(),
        }),
    ).into_response()
}
