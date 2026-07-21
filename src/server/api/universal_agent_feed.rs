use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

// Assuming there's some shared AppState that holds DB connections, etc.
// For now we'll mock it for the basic API structure.
pub struct AppState {
    // db_pool: PgPool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FeedEvent {
    pub id: String,
    pub tenant_id: String,
    pub event_type: String,
    pub source: String,
    pub payload: serde_json::Value,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AgentActionDraft {
    pub id: String,
    pub tenant_id: String,
    pub feed_event_id: String,
    pub agent_type: String,
    pub proposed_action: serde_json::Value,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ActionApproval {
    pub id: String,
    pub tenant_id: String,
    pub agent_action_draft_id: String,
    pub decision: String,
    pub edited_action: Option<serde_json::Value>,
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/events", post(create_feed_event).get(list_feed_events))
        .route("/drafts", get(list_drafts))
        .route("/drafts/:id/approve", post(approve_draft))
        .with_state(state)
}

async fn create_feed_event(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<FeedEvent>,
) -> impl IntoResponse {
    // In a real implementation, this would insert into DB
    (StatusCode::CREATED, Json(payload))
}

async fn list_feed_events(
    State(_state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let events: Vec<FeedEvent> = vec![];
    (StatusCode::OK, Json(events))
}

async fn list_drafts(
    State(_state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let drafts: Vec<AgentActionDraft> = vec![];
    (StatusCode::OK, Json(drafts))
}

#[derive(Deserialize)]
struct ApprovalRequest {
    decision: String,
    edited_action: Option<serde_json::Value>,
}

async fn approve_draft(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<ApprovalRequest>,
) -> impl IntoResponse {
    let approval = ActionApproval {
        id: Uuid::new_v4().to_string(),
        tenant_id: "mock_tenant".to_string(), // In real app, extract from auth
        agent_action_draft_id: id,
        decision: payload.decision,
        edited_action: payload.edited_action,
    };

    // In real app, update draft status and insert approval
    (StatusCode::OK, Json(approval))
}
