use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use serde_json::json;

use crate::db::DB;
use server_common::Claims;

#[derive(serde::Deserialize)]
pub struct ApproveActionRequest {
    pub approved: bool,
}

pub async fn list_triage_items_handler(
    State(db): State<Arc<DB>>,
    axum::extract::Extension(user): axum::extract::Extension<Claims>,
) -> axum::response::Response {
    let tenant_id = user.organization_id.unwrap_or_else(|| "default".to_string());

    let items = match &db.store {
        crate::db::DbStore::Postgres => {
            match sqlx::query(
                "SELECT id, source, priority, context, draft_response, proposed_action, status, created_at::text
                 FROM triage_items
                 WHERE tenant_id = $1 AND status = 'OPEN'
                 ORDER BY created_at DESC LIMIT 50"
            )
            .bind(&tenant_id)
            .fetch_all(&db.pool)
            .await
            {
                Ok(rows) => {
                    use sqlx::Row;
                    rows.into_iter().map(|row| {
                        let proposed_action: Option<serde_json::Value> = row.try_get("proposed_action").unwrap_or_default();
                        json!({
                            "id": row.try_get::<String, _>("id").unwrap_or_default(),
                            "source": row.try_get::<String, _>("source").unwrap_or_default(),
                            "priority": row.try_get::<String, _>("priority").unwrap_or_default(),
                            "context": row.try_get::<String, _>("context").unwrap_or_default(),
                            "draft_response": row.try_get::<Option<String>, _>("draft_response").unwrap_or_default(),
                            "proposed_action": proposed_action.unwrap_or_default(),
                            "status": row.try_get::<String, _>("status").unwrap_or_default(),
                            "created_at": row.try_get::<Option<String>, _>("created_at").unwrap_or_default(),
                        })
                    }).collect::<Vec<_>>()
                }
                Err(e) => {
                    ::server_telemetry::record_error_signal("Failed to fetch triage items");
                    tracing::error!("Failed to fetch triage items: {}", e);
                    return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to load triage items from database").into_response();
                }
            }
        }
        _ => vec![],
    };

    (axum::http::StatusCode::OK, Json(items)).into_response()
}

pub async fn approve_triage_action_handler(
    State(db): State<Arc<DB>>,
    axum::extract::Extension(user): axum::extract::Extension<Claims>,
    Path(id): Path<String>,
    axum::extract::Json(req): axum::extract::Json<ApproveActionRequest>,
) -> axum::response::Response {
    let tenant_id = user.organization_id.unwrap_or_else(|| "default".to_string());
    let new_status = if req.approved { "RESOLVED" } else { "DISMISSED" };

    match &db.store {
        crate::db::DbStore::Postgres => {
            match sqlx::query(
                "UPDATE triage_items SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3"
            )
            .bind(new_status)
            .bind(&id)
            .bind(&tenant_id)
            .execute(&db.pool)
            .await
            {
                Ok(_) => {
                    (axum::http::StatusCode::OK, Json(json!({"success": true, "status": new_status}))).into_response()
                }
                Err(e) => {
                    ::server_telemetry::record_error_signal("Failed to update triage item");
                    tracing::error!("Failed to update triage item: {}", e);
                    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to update triage item").into_response()
                }
            }
        }
        _ => (axum::http::StatusCode::OK, Json(json!({"success": true, "status": new_status}))).into_response(),
    }
}
