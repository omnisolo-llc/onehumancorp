use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router, routing::{get, post},
};
use std::sync::Arc;
use crate::models::{ChatMessage, ChatConversation, ChatInbox, ChatContact};
use crate::websocket::WsState;
use sqlx::PgPool;

pub struct ApiState {
    pub db: PgPool,
    pub ws_state: Arc<WsState>,
}

pub fn router(state: Arc<ApiState>) -> Router {
    Router::new()
        .route("/api/v1/chat/messages", post(create_message))
        .with_state(state)
}

async fn create_message(
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<ChatMessage>,
) -> impl IntoResponse {
    // For now we just broadcast the message
    let _ = state.ws_state.broadcast(&payload.tenant_id, &payload).await;
    (StatusCode::CREATED, Json(payload))
}
