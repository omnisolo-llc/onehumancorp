use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum::http::StatusCode;
use std::sync::Arc;
use crate::orchestration::feed::AgentFeedService;
use crate::common::Claims;
use crate::db::DB;
use crate::msgbus::DistributedLock;

pub struct FeedState {
    pub service: AgentFeedService,
}

pub fn router(db: Arc<DB>, lock_service: Arc<dyn DistributedLock>) -> Router {
    let state = Arc::new(FeedState {
        service: AgentFeedService::new(db, lock_service),
    });

    Router::new()
        .route("/", get(get_agent_feed))
        .route("/:id/approve", post(approve_action))
        .route("/:id/dismiss", post(dismiss_action))
        .with_state(state)
}

async fn get_agent_feed(
    State(state): State<Arc<FeedState>>,
    axum::extract::Extension(claims): axum::extract::Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    match state.service.get_pending_actions(&tenant_id).await {
        Ok(actions) => (StatusCode::OK, Json(actions)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response(),
    }
}

async fn approve_action(
    State(state): State<Arc<FeedState>>,
    Path(id): Path<String>,
    axum::extract::Extension(claims): axum::extract::Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    match state.service.update_action_status(&tenant_id, &id, "APPROVED").await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({ "success": true }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response(),
    }
}

async fn dismiss_action(
    State(state): State<Arc<FeedState>>,
    Path(id): Path<String>,
    axum::extract::Extension(claims): axum::extract::Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    match state.service.update_action_status(&tenant_id, &id, "DISMISSED").await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({ "success": true }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response(),
    }
}
