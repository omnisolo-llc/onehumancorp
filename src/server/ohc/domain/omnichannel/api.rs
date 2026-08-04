use super::repository::OmnichannelRepository;
use axum::{
    extract::{State, Json},
    routing::post,
    Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::models::{Conversation, Message};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<OmnichannelRepository>,
}

#[derive(Deserialize)]
pub struct IngestWebhookRequest {
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_name: String,
    pub contact_identifier: String,
    pub content: String,
    pub sender_type: String,
}

#[derive(Serialize)]
pub struct IngestWebhookResponse {
    pub conversation: Conversation,
    pub message: Message,
}

pub fn create_router(pool: PgPool) -> Router {
    let repo = Arc::new(OmnichannelRepository::new(pool));
    let state = AppState { repo };

    Router::new()
        .route("/api/v1/omnichannel/webhook", post(ingest_webhook_handler))
        .with_state(state)
}

async fn ingest_webhook_handler(
    State(state): State<AppState>,
    Json(req): Json<IngestWebhookRequest>,
) -> Result<Json<IngestWebhookResponse>, (StatusCode, String)> {

    let contact = match state.repo.get_contact_by_identifier(req.tenant_id, &req.contact_identifier).await {
        Ok(Some(c)) => c,
        Ok(None) => match state.repo.create_contact(req.tenant_id, &req.contact_name, &req.contact_identifier).await {
            Ok(c) => c,
            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        },
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    let conversation = match state.repo.create_conversation(req.tenant_id, req.inbox_id, contact.id).await {
        Ok(c) => c,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    let message = match state.repo.create_message(req.tenant_id, conversation.id, &req.content, &req.sender_type).await {
        Ok(m) => m,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    Ok(Json(IngestWebhookResponse {
        conversation,
        message,
    }))
}
