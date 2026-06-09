use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DbStore;

#[derive(Serialize, Deserialize)]
pub struct ProactiveAction {
    pub id: String,
    pub tenant_id: String,
    pub title: String,
    pub description: String,
    pub action_type: String,
    pub payload: serde_json::Value,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProactiveActionResponse {
    pub actions: Vec<ProactiveAction>,
}

pub fn proactive_routes<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/api/proactive/actions", get(get_proactive_actions))
        .route("/api/proactive/actions/{action_id}/approve", post(approve_proactive_action))
        .route("/api/proactive/actions/{action_id}/reject", post(reject_proactive_action))
}

async fn get_proactive_actions(
    Extension(db): Extension<Arc<crate::db::DB>>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let tenant_id = headers
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default")
        .to_string();

    let mut actions = Vec::new();

    if matches!(&db.store, DbStore::Postgres) {
        use sqlx::Row;
        if let Ok(rows) = sqlx::query(
            "SELECT id, tenant_id, department, description, status, payload FROM agent_approvals WHERE tenant_id = $1 AND department = 'proactive' AND status = 'DRAFT' ORDER BY created_at DESC",
        )
        .bind(&tenant_id)
        .fetch_all(&db.pool)
        .await
        {
            for row in rows {
                let id: String = row.try_get("id").unwrap_or_default();
                let tid: String = row.try_get("tenant_id").unwrap_or_default();
                let description: String = row.try_get("description").unwrap_or_default();
                let status: String = row.try_get("status").unwrap_or_default();
                let payload: serde_json::Value = row.try_get("payload").unwrap_or(serde_json::json!({}));

                let title = match &payload {
                    serde_json::Value::Object(map) => map.get("title").and_then(|t| t.as_str()).unwrap_or("Proactive Alert").to_string(),
                    _ => "Proactive Alert".to_string(),
                };

                actions.push(ProactiveAction {
                    id,
                    tenant_id: tid,
                    title,
                    description,
                    action_type: "proactive".to_string(),
                    payload,
                    status,
                });
            }
        }
    }

    Json(ProactiveActionResponse { actions })
}

async fn approve_proactive_action(
    Path(action_id): Path<String>,
    Extension(db): Extension<Arc<crate::db::DB>>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let tenant_id = headers
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default")
        .to_string();

    if matches!(&db.store, DbStore::Postgres) {
        let _ = sqlx::query(
            "UPDATE agent_approvals SET status = 'APPROVED' WHERE id = $1 AND tenant_id = $2"
        )
        .bind(&action_id)
        .bind(&tenant_id)
        .execute(&db.pool)
        .await;
    }

    Json(serde_json::json!({"status": "approved", "action_id": action_id}))
}

async fn reject_proactive_action(
    Path(action_id): Path<String>,
    Extension(db): Extension<Arc<crate::db::DB>>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let tenant_id = headers
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default")
        .to_string();

    if matches!(&db.store, DbStore::Postgres) {
        let _ = sqlx::query(
            "UPDATE agent_approvals SET status = 'REJECTED' WHERE id = $1 AND tenant_id = $2"
        )
        .bind(&action_id)
        .bind(&tenant_id)
        .execute(&db.pool)
        .await;
    }

    Json(serde_json::json!({"status": "rejected", "action_id": action_id}))
}
