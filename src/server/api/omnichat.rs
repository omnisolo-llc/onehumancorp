use axum::{
    extract::{State, Path},
    routing::{post, get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;
use crate::domain::omnichat::repository::OmnichatRepository;
use sqlx::PgPool;

pub struct AppState {
    pub repo: OmnichatRepository,
}

#[derive(Deserialize)]
pub struct CreateInboxReq {
    pub tenant_id: Uuid,
    pub name: String,
}

#[derive(Deserialize)]
pub struct LinkAdapterReq {
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub channel_type: String,
    pub config: serde_json::Value,
}

#[derive(Deserialize)]
pub struct IngestMessageReq {
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub content: String,
}

pub fn omnichat_router(pool: PgPool) -> Router {
    let repo = OmnichatRepository::new(pool);
    let state = Arc::new(AppState { repo });

    Router::new()
        .route("/inboxes", post(create_inbox))
        .route("/adapters", post(link_adapter))
        .route("/messages/ingest", post(ingest_message))
        .with_state(state)
}

async fn create_inbox(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateInboxReq>,
) -> axum::response::Json<serde_json::Value> {
    match state.repo.create_inbox(payload.tenant_id, &payload.name).await {
        Ok(inbox) => axum::response::Json(serde_json::json!({ "status": "success", "inbox": inbox })),
        Err(_) => axum::response::Json(serde_json::json!({ "status": "error" })),
    }
}

async fn link_adapter(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LinkAdapterReq>,
) -> axum::response::Json<serde_json::Value> {
    match state.repo.link_channel_adapter(payload.tenant_id, payload.inbox_id, &payload.channel_type, payload.config).await {
        Ok(adapter) => axum::response::Json(serde_json::json!({ "status": "success", "adapter": adapter })),
        Err(_) => axum::response::Json(serde_json::json!({ "status": "error" })),
    }
}

async fn ingest_message(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<IngestMessageReq>,
) -> axum::response::Json<serde_json::Value> {
    // get or create conversation
    let conv = match state.repo.get_or_create_conversation(payload.tenant_id, payload.inbox_id, payload.contact_id).await {
        Ok(c) => c,
        Err(_) => return axum::response::Json(serde_json::json!({ "status": "error", "message": "Failed to create conversation" })),
    };

    match state.repo.ingest_message(payload.tenant_id, conv.id, Some(payload.contact_id), &payload.content).await {
        Ok(msg) => axum::response::Json(serde_json::json!({ "status": "success", "message": msg })),
        Err(_) => axum::response::Json(serde_json::json!({ "status": "error" })),
    }
}
