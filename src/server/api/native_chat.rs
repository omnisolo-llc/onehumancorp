use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::repository::omnichannel_repo::OmniChannelRepo;

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
    pub channel_type: String,
}

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub conversation_id: Uuid,
    pub contact_id: Uuid,
    pub content: String,
}

pub async fn create_inbox(
    Extension(repo): Extension<Arc<OmniChannelRepo>>,
    Extension(tenant_id): Extension<Uuid>,
    Json(payload): Json<CreateInboxRequest>,
) -> impl axum::response::IntoResponse {
    let inbox = repo.create_omni_inbox(tenant_id, payload.name, payload.channel_type).await.unwrap();
    Json(inbox)
}

pub async fn create_message(
    Extension(repo): Extension<Arc<OmniChannelRepo>>,
    Extension(tenant_id): Extension<Uuid>,
    Json(payload): Json<CreateMessageRequest>,
) -> impl axum::response::IntoResponse {
    let msg = repo.create_omni_message(
        tenant_id,
        payload.conversation_id,
        payload.contact_id,
        "User".to_string(),
        Uuid::new_v4(), // Placeholder for sender id
        payload.content,
        "Text".to_string(),
    ).await.unwrap();
    Json(msg)
}

pub fn router() -> Router {
    Router::new()
        .route("/inbox", post(create_inbox))
        .route("/message", post(create_message))
}
