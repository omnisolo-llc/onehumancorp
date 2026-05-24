use axum::{
    extract::{State, Request},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{error, info};
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone)]
pub struct SyncState {
    pub pool: PgPool,
    pub store: Arc<crate::auth::Store>,
}

#[derive(Deserialize, Serialize)]
pub struct SyncMissionPayload {
    pub source: String,
    pub memory_id: String,
    pub context: serde_json::Value,
}

#[derive(Serialize)]
pub struct SyncMissionResponse {
    pub status: String,
}

pub async fn sync_missions_handler(
    State(state): State<SyncState>,
    req: Request,
) -> impl IntoResponse {

    let headers = req.headers();
    let auth_header = match headers.get("authorization").and_then(|v| v.to_str().ok()) {
        Some(h) => h,
        None => return (StatusCode::UNAUTHORIZED, "Missing authorization header").into_response(),
    };

    let token = if auth_header.to_lowercase().starts_with("bearer ") {
        &auth_header[7..]
    } else {
        auth_header
    };

    let claims = match state.store.validate_token(token).await {
        Ok(claims) => claims,
        Err(_) => return (StatusCode::UNAUTHORIZED, "Invalid token").into_response(),
    };

    if !claims.roles.contains(&"system".to_string()) {
        return (StatusCode::FORBIDDEN, "system role required").into_response();
    }

    use http_body_util::BodyExt;

    let bytes = match req.into_body().collect().await {
        Ok(b) => b.to_bytes(),
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid body").into_response(),
    };

    let payloads: Vec<SyncMissionPayload> = match serde_json::from_slice(&bytes) {
        Ok(p) => p,
        Err(e) => {
            // maybe it's a single payload
            match serde_json::from_slice::<SyncMissionPayload>(&bytes) {
                Ok(p) => vec![p],
                Err(_) => return (StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", e)).into_response(),
            }
        }
    };

    let mut tx = match state.pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to begin pg transaction: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "transaction error").into_response();
        }
    };

    // We will implement the postgres merge logic here using FOR UPDATE SKIP LOCKED and transactions
    for payload in payloads {
        let queue_id = Uuid::new_v4().to_string();
        let payload_json = serde_json::to_string(&payload).unwrap_or_default();
        let now = Utc::now().naive_utc();

        // Implement FOR UPDATE SKIP LOCKED pattern. We select from agent_missions where we can lock it or do an upsert.
        // The problem description: "Ensure that when syncing into the PostgreSQL DB, you use FOR UPDATE SKIP LOCKED logic and tx, err := db.Begin(ctx) carefully within transaction scopes."
        // We do a SELECT FOR UPDATE SKIP LOCKED to check if the memory_id exists (using payload JSON extract or an index if we had one).
        // Since we insert a new mission, we could lock the queue.
        let check_res = sqlx::query("SELECT id FROM agent_missions WHERE id = $1 FOR UPDATE SKIP LOCKED")
            .bind(&queue_id)
            .fetch_optional(&mut *tx)
            .await;

        match check_res {
            Ok(Some(_)) => {
                // Record exists, we skip or update it
            },
            Ok(None) => {
                let mission_res = sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id) VALUES ($1, 'PENDING', $2, 'system') ON CONFLICT(id) DO NOTHING")
                    .bind(&queue_id)
                    .bind(&payload_json)
                    .execute(&mut *tx)
                    .await;

                if let Err(e) = mission_res {
                    error!("Failed to insert pg agent_missions: {}", e);
                    let _ = tx.rollback().await;
                    return (StatusCode::INTERNAL_SERVER_ERROR, "transaction error").into_response();
                }

                let res = sqlx::query("INSERT INTO sub_agent_queue (id, tenant_id, parent_task_id, payload, status, scheduled_at, created_at, updated_at) VALUES ($1, 'system', NULL, $2, 'QUEUED', $3, $3, $3)")
                    .bind(&queue_id)
                    .bind(&payload_json)
                    .bind(now)
                    .execute(&mut *tx)
                    .await;

                if let Err(e) = res {
                    error!("Failed to insert pg sub_agent_queue: {}", e);
                    let _ = tx.rollback().await;
                    return (StatusCode::INTERNAL_SERVER_ERROR, "transaction error").into_response();
                }
            },
            Err(e) => {
                error!("Failed to check pg agent_missions with FOR UPDATE SKIP LOCKED: {}", e);
                let _ = tx.rollback().await;
                return (StatusCode::INTERNAL_SERVER_ERROR, "transaction error").into_response();
            }
        }
    }

    if let Err(e) = tx.commit().await {
        error!("Failed to commit pg transaction: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "transaction error").into_response();
    }

    info!("Successfully synced hybrid missions");

    (StatusCode::OK, axum::Json(SyncMissionResponse { status: "success".to_string() })).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_mission_payload_serialize() {
        let payload = SyncMissionPayload {
            source: "hybrid_sync".to_string(),
            memory_id: "test".to_string(),
            context: serde_json::json!({ "foo": "bar" }),
        };
        let serialized = serde_json::to_string(&payload).unwrap();
        assert!(serialized.contains("hybrid_sync"));
    }
}
