use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use serde::Deserialize;

use super::service::ChatEngine;
use super::models::{Conversation, Message};
use super::websocket::{ws_handler, WsState};

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<dyn ChatEngine>,
    pub ws_state: Arc<WsState>,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/inboxes/:inbox_id/conversations", get(list_conversations))
        .route("/conversations/:conversation_id/messages", get(get_messages))
        .route("/ws/:tenant_id", get(ws_handler))
        .with_state(state)
}

async fn list_conversations(
    State(state): State<AppState>,
    Path(inbox_id): Path<Uuid>,
    // In a real app tenant_id would come from auth middleware
    // Here we'll just use a dummy or require it in headers/query for simplicity in the demo
) -> Json<Vec<Conversation>> {
    // Dummy tenant
    let tenant_id = Uuid::new_v4();
    let convos = state.engine.list_conversations(tenant_id, inbox_id).await.unwrap_or_default();
    Json(convos)
}

async fn get_messages(
    State(state): State<AppState>,
    Path(conversation_id): Path<Uuid>,
    Query(pagination): Query<Pagination>,
) -> Json<Vec<Message>> {
    let tenant_id = Uuid::new_v4();
    let limit = pagination.limit.unwrap_or(50);
    let offset = pagination.offset.unwrap_or(0);

    let msgs = state.engine.get_messages(tenant_id, conversation_id, limit, offset).await.unwrap_or_default();
    Json(msgs)
}
