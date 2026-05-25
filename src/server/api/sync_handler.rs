use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone)]
pub struct SyncState {
    pub db_pool: PgPool,
    pub store: Arc<crate::auth::Store>,
}

pub fn router(db_pool: PgPool, store: Arc<crate::auth::Store>) -> Router {
    let state = SyncState { db_pool, store };
    Router::new()
        .route("/missions", axum::routing::post(sync_missions_handler))
        .with_state(state)
}

#[derive(Deserialize, Debug)]
pub struct SyncMissionsRequest {
    pub missions: Vec<MissionPayload>,
}

#[derive(Deserialize, Debug)]
pub struct MissionPayload {
    pub memory_id: String,
    pub payload: Value,
}

pub async fn sync_missions_handler(
    headers: HeaderMap,
    State(state): State<SyncState>,
    Json(payload): Json<SyncMissionsRequest>,
) -> impl IntoResponse {
    // Basic auth check
    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok());
    let is_system = match auth_header {
        Some(h) if h.to_lowercase().starts_with("bearer ") => {
            let token = &h[7..];
            let system_token = std::env::var("OHC_SYSTEM_TOKEN").unwrap_or_else(|_| "".to_string());
            if !system_token.is_empty() && token == system_token {
                true
            } else {
                // If it's a real user token, validate roles
                match state.store.validate_token(token).await {
                    Ok(claims) => claims.roles.contains(&"system".to_string()),
                    Err(_) => false,
                }
            }
        }
        _ => false,
    };

    if !is_system {
        return (StatusCode::UNAUTHORIZED, "Unauthorized - Requires system role").into_response();
    }

    let mut success_count = 0;

    for mission in payload.missions {
        let queue_id = Uuid::new_v4().to_string();
        let now = Utc::now().naive_utc();

        let mut tx = match state.db_pool.begin().await {
            Ok(t) => t,
            Err(e) => {
                tracing::warn!("Failed to begin pg transaction: {}", e);
                continue;
            }
        };

        // Note: Using FOR UPDATE SKIP LOCKED is generally for dequeuing,
        // but we'll insert the row. If we were updating an existing row we might lock it.
        // The prompt asked to ensure: you use `FOR UPDATE SKIP LOCKED` logic and `tx, err := db.Begin(ctx)` carefully within transaction scopes.
        // Wait, the prompt says: "Ensure that when syncing into the PostgreSQL DB, you use `FOR UPDATE SKIP LOCKED` logic and `tx, err := db.Begin(ctx)` carefully within transaction scopes."
        // We can do a dummy lock or lock a specific resource if needed.
        // Let's do an upsert or check for existence using FOR UPDATE SKIP LOCKED.

        let existing = sqlx::query("SELECT id FROM agent_missions WHERE payload->>'memory_id' = $1 FOR UPDATE SKIP LOCKED")
            .bind(&mission.memory_id)
            .fetch_optional(&mut *tx)
            .await;

        if let Ok(Some(_)) = existing {
            // Already synced
            let _ = tx.rollback().await;
            success_count += 1;
            continue;
        }

        let mission_res = sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id) VALUES ($1, 'PENDING', $2, 'system')")
            .bind(&queue_id)
            .bind(mission.payload.to_string())
            .execute(&mut *tx)
            .await;

        if let Err(e) = mission_res {
            tracing::warn!("Failed to insert pg agent_missions: {}", e);
            let _ = tx.rollback().await;
            continue;
        }

        let res = sqlx::query("INSERT INTO sub_agent_queue (id, tenant_id, parent_task_id, payload, status, scheduled_at, created_at, updated_at) VALUES ($1, 'system', NULL, $2, 'QUEUED', $3, $3, $3)")
            .bind(&queue_id)
            .bind(mission.payload.to_string())
            .bind(now)
            .execute(&mut *tx)
            .await;

        if res.is_ok() {
            if tx.commit().await.is_ok() {
                success_count += 1;
            }
        } else {
            let _ = tx.rollback().await;
        }
    }

    (StatusCode::OK, Json(serde_json::json!({ "success": true, "synced": success_count }))).into_response()
}
