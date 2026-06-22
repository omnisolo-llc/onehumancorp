use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{db::DB};
use crate::harness::{UiTenantQuery, ui_tenant_id};
use axum::extract::Query;

#[derive(Deserialize)]
pub struct CreateAgentDraftRequest {
    pub work_item_id: String,
    pub proposed_action: serde_json::Value,
    pub context: serde_json::Value,
}

#[derive(Serialize)]
pub struct AgentDraftResponse {
    pub id: String,
    pub work_item_id: String,
    pub status: String,
    pub proposed_action: serde_json::Value,
    pub context: serde_json::Value,
}

#[derive(Deserialize)]
pub struct ApproveAgentDraftRequest {
    pub action_status: String, // e.g. "APPROVED" or "DISMISSED"
}

use axum::routing::{get, post};
use axum::Router;

pub fn routes() -> Router<Arc<DB>> {
    Router::new()
        .route("/api/v1/agent_drafts", post(create_agent_draft_handler))
        .route("/api/v1/agent_drafts", get(get_agent_drafts_handler))
        .route("/api/v1/agent_drafts/:id/approve", post(approve_agent_draft_handler))
}

pub async fn create_agent_draft_handler(
    State(db): State<Arc<DB>>,
    Query(query): Query<UiTenantQuery>,
    Json(payload): Json<CreateAgentDraftRequest>,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    let tenant_id = ui_tenant_id(&query);
    let draft_id = Uuid::new_v4().to_string();

    match &db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                "INSERT INTO agent_drafts (id, tenant_id, work_item_id, proposed_action, context, status) VALUES ($1, $2, $3, $4, $5, 'PENDING')"
            )
            .bind(&draft_id)
            .bind(&tenant_id)
            .bind(&payload.work_item_id)
            .bind(&payload.proposed_action)
            .bind(&payload.context)
            .execute(&db.pool).await;

            if res.is_ok() {
                (axum::http::StatusCode::CREATED, Json(serde_json::json!({"id": draft_id}))).into_response()
            } else {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "INSERT INTO agent_drafts (id, tenant_id, work_item_id, proposed_action, context, status) VALUES (?, ?, ?, ?, ?, 'PENDING')"
            )
            .bind(&draft_id)
            .bind(&tenant_id)
            .bind(&payload.work_item_id)
            .bind(&payload.proposed_action)
            .bind(&payload.context)
            .execute(pool).await;

            if res.is_ok() {
                (axum::http::StatusCode::CREATED, Json(serde_json::json!({"id": draft_id}))).into_response()
            } else {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        }
    }
}

pub async fn get_agent_drafts_handler(
    State(db): State<Arc<DB>>,
    Query(query): Query<UiTenantQuery>,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    let tenant_id = ui_tenant_id(&query);

    match &db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                "SELECT id, work_item_id, proposed_action, context, status FROM agent_drafts WHERE tenant_id = $1 AND status = 'PENDING' ORDER BY created_at DESC"
            )
            .bind(&tenant_id)
            .fetch_all(&db.pool).await;

            match res {
                Ok(rows) => {
                    use sqlx::Row;
                    let items: Vec<serde_json::Value> = rows.into_iter().map(|r| {
                        let id: String = r.get("id");
                        let work_item_id: String = r.get("work_item_id");
                        let proposed_action_str: Option<String> = r.try_get("proposed_action").ok();
                        let proposed_action: Option<serde_json::Value> = proposed_action_str.and_then(|s| serde_json::from_str(&s).ok());
                        let context_str: Option<String> = r.try_get("context").ok();
                        let context: Option<serde_json::Value> = context_str.and_then(|s| serde_json::from_str(&s).ok());
                        let status: String = r.get("status");

                        serde_json::json!({
                            "id": id,
                            "work_item_id": work_item_id,
                            "proposed_action": proposed_action,
                            "context": context,
                            "status": status
                        })
                    }).collect();
                    (axum::http::StatusCode::OK, Json(serde_json::json!({"items": items}))).into_response()
                },
                Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
             let res = sqlx::query(
                "SELECT id, work_item_id, proposed_action, context, status FROM agent_drafts WHERE tenant_id = ? AND status = 'PENDING' ORDER BY created_at DESC"
            )
            .bind(&tenant_id)
            .fetch_all(pool).await;

            match res {
                Ok(rows) => {
                    use sqlx::Row;
                    let items: Vec<serde_json::Value> = rows.into_iter().map(|r| {
                        let proposed_action_str: Option<String> = r.try_get("proposed_action").ok();
                        let proposed_action: Option<serde_json::Value> = proposed_action_str.and_then(|s| serde_json::from_str(&s).ok());

                        let context_str: Option<String> = r.try_get("context").ok();
                        let context: Option<serde_json::Value> = context_str.and_then(|s| serde_json::from_str(&s).ok());

                        let id: String = r.get("id");
                        let work_item_id: String = r.get("work_item_id");
                        let status: String = r.get("status");

                        serde_json::json!({
                            "id": id,
                            "work_item_id": work_item_id,
                            "proposed_action": proposed_action,
                            "context": context,
                            "status": status
                        })
                    }).collect();
                    (axum::http::StatusCode::OK, Json(serde_json::json!({"items": items}))).into_response()
                },
                Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
            }
        }
    }
}

pub async fn approve_agent_draft_handler(
    State(db): State<Arc<DB>>,
    Path(id): Path<String>,
    Query(query): Query<UiTenantQuery>,
    Json(payload): Json<ApproveAgentDraftRequest>,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    let tenant_id = ui_tenant_id(&query);

    let target_status = if payload.action_status == "DISMISSED" {
        "DISMISSED"
    } else {
        "APPROVED"
    };

    match &db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                "UPDATE agent_drafts SET status = $1, updated_at = NOW() WHERE id = $2 AND tenant_id = $3"
            )
            .bind(target_status)
            .bind(&id)
            .bind(&tenant_id)
            .execute(&db.pool).await;

            if res.is_ok() {
                // Here we would enqueue the action execution. For now just ack success.
                (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
            } else {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        },
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "UPDATE agent_drafts SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?"
            )
            .bind(target_status)
            .bind(&id)
            .bind(&tenant_id)
            .execute(pool).await;

            if res.is_ok() {
                (axum::http::StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
            } else {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        }
    }
}
