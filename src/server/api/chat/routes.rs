use axum::{
    extract::{State, Path, Extension, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::DB;
use crate::services::chat::service::ChatService;
use crate::services::chat::models::*;

#[derive(Clone)]
pub struct ChatRouteState {
    pub db: Arc<DB>,
}

#[derive(Deserialize)]
pub struct CreateInboxReq {
    pub name: String,
}

#[derive(Deserialize)]
pub struct SendMessageReq {
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

pub fn router(db: Arc<DB>) -> Router {
    let state = ChatRouteState { db };
    Router::new()
        .route("/inbox", post(create_inbox))
        .route("/message", post(send_message))
        .with_state(state)
}

async fn create_inbox(
    State(state): State<ChatRouteState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateInboxReq>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(id) => match Uuid::parse_str(id) {
            Ok(u) => u,
            Err(_) => return (StatusCode::BAD_REQUEST, "Invalid tenant id").into_response(),
        },
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let service = ChatService::new(state.db.pool.clone());
            match service.create_inbox(tenant_id, payload.name).await {
                Ok(inbox) => (StatusCode::OK, Json(inbox)).into_response(),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create inbox").into_response(),
            }
        },
        _ => (StatusCode::NOT_IMPLEMENTED, "Only postgres is supported").into_response(),
    }
}

async fn send_message(
    State(state): State<ChatRouteState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<SendMessageReq>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(id) => match Uuid::parse_str(id) {
            Ok(u) => u,
            Err(_) => return (StatusCode::BAD_REQUEST, "Invalid tenant id").into_response(),
        },
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let service = ChatService::new(state.db.pool.clone());
            match service.send_message(tenant_id, payload.conversation_id, payload.sender_type, payload.sender_id, payload.content).await {
                Ok(msg) => (StatusCode::OK, Json(msg)).into_response(),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send message").into_response(),
            }
        },
        _ => (StatusCode::NOT_IMPLEMENTED, "Only postgres is supported").into_response(),
    }
}
