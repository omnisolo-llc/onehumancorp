use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::CookieJar;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
use crate::{auth, db::DB, domain::repository::action_required_queue_repo::ActionRequiredQueueRepo};

pub struct AppState {
    pub db: Arc<DB>,
}

pub fn router(db: Arc<DB>) -> Router {
    Router::new()
        .route("/", get(list_pending_drafts))
        .route("/:id/approve", post(approve_draft))
        .with_state(Arc::new(AppState { db }))
}

async fn list_pending_drafts(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let tenant_id_str = match auth::get_tenant_id_from_jar(&jar) {
        Some(t) => t,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))).into_response(),
    };

    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(t) => t,
        Err(_) => return (axum::http::StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid tenant ID"}))).into_response(),
    };

    let repo = ActionRequiredQueueRepo::new(state.db.clone());
    match repo.get_pending_drafts(tenant_id).await {
        Ok(drafts) => (axum::http::StatusCode::OK, Json(drafts)).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

async fn approve_draft(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Path(draft_id_str): Path<String>,
) -> impl IntoResponse {
    let tenant_id_str = match auth::get_tenant_id_from_jar(&jar) {
        Some(t) => t,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(json!({"error": "Unauthorized"}))).into_response(),
    };

    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(t) => t,
        Err(_) => return (axum::http::StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid tenant ID"}))).into_response(),
    };

    let draft_id = match Uuid::parse_str(&draft_id_str) {
        Ok(t) => t,
        Err(_) => return (axum::http::StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid draft ID"}))).into_response(),
    };

    let repo = ActionRequiredQueueRepo::new(state.db.clone());
    match repo.approve_draft(draft_id, tenant_id).await {
        Ok(_) => {
            // Here we would trigger the omnichannel dispatch logic.
            // For now, returning success.
            (axum::http::StatusCode::OK, Json(json!({"status": "approved"}))).into_response()
        },
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}
