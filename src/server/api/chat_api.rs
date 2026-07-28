use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{db::DB, services::chat::models::*, services::chat::service::ChatService};

#[derive(Clone)]
pub struct ChatApiState {
    pub db: Arc<DB>,
}

pub fn router<S>(state: ChatApiState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/inboxes", get(list_inboxes).post(create_inbox))
        .route("/channels", get(list_channels).post(create_channel))
        .route("/contacts", get(list_contacts).post(create_contact))
        .route("/conversations", get(list_conversations).post(start_conversation))
        .route("/conversations/:conversation_id/messages", get(list_messages).post(send_message))
        .with_state(state)
}

fn get_tenant_id(claims: &server_common::Claims) -> Option<Uuid> {
    claims
        .organization_id
        .as_deref()
        .and_then(|id| Uuid::parse_str(id).ok())
}

#[derive(Deserialize)]
pub struct CreateInboxReq {
    pub name: String,
}

pub async fn list_inboxes(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&claims) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(Vec::<ChatInbox>::new())).into_response(),
    };

    let svc = ChatService::new(state.db.pool.clone());
    match svc.list_inboxes(tenant_id).await {
        Ok(inboxes) => (StatusCode::OK, Json(inboxes)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list inboxes: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Vec::<ChatInbox>::new()),
            )
                .into_response()
        }
    }
}

pub async fn create_inbox(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<server_common::Claims>,
    Json(payload): Json<CreateInboxReq>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&claims) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let svc = ChatService::new(state.db.pool.clone());
    match svc.create_inbox(tenant_id, payload.name).await {
        Ok(inbox) => (StatusCode::CREATED, Json(inbox)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create inbox: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct CreateChannelReq {
    pub inbox_id: Uuid,
    pub channel_type: String,
    pub config: serde_json::Value,
}

pub async fn list_channels(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&claims) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(Vec::<ChatChannel>::new())).into_response(),
    };

    let svc = ChatService::new(state.db.pool.clone());
    match svc.list_channels(tenant_id).await {
        Ok(channels) => (StatusCode::OK, Json(channels)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list channels: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Vec::<ChatChannel>::new()),
            )
                .into_response()
        }
    }
}

pub async fn create_channel(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<server_common::Claims>,
    Json(payload): Json<CreateChannelReq>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&claims) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let svc = ChatService::new(state.db.pool.clone());
    match svc.create_channel(tenant_id, payload.inbox_id, payload.channel_type, payload.config).await {
        Ok(channel) => (StatusCode::CREATED, Json(channel)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create channel: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct CreateContactReq {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

pub async fn list_contacts(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&claims) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(Vec::<ChatContact>::new())).into_response(),
    };

    let svc = ChatService::new(state.db.pool.clone());
    match svc.list_contacts(tenant_id).await {
        Ok(contacts) => (StatusCode::OK, Json(contacts)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list contacts: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Vec::<ChatContact>::new()),
            )
                .into_response()
        }
    }
}

pub async fn create_contact(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<server_common::Claims>,
    Json(payload): Json<CreateContactReq>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&claims) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let svc = ChatService::new(state.db.pool.clone());
    match svc.create_contact(tenant_id, payload.name, payload.email, payload.phone).await {
        Ok(contact) => (StatusCode::CREATED, Json(contact)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create contact: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}


#[derive(Deserialize)]
pub struct StartConversationReq {
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
}

pub async fn list_conversations(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&claims) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(Vec::<ChatConversation>::new())).into_response(),
    };

    let svc = ChatService::new(state.db.pool.clone());
    match svc.list_conversations(tenant_id).await {
        Ok(conversations) => (StatusCode::OK, Json(conversations)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list conversations: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Vec::<ChatConversation>::new()),
            )
                .into_response()
        }
    }
}

pub async fn start_conversation(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<server_common::Claims>,
    Json(payload): Json<StartConversationReq>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&claims) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let svc = ChatService::new(state.db.pool.clone());
    match svc.start_conversation(tenant_id, payload.inbox_id, payload.contact_id, payload.assignee_id).await {
        Ok(conversation) => (StatusCode::CREATED, Json(conversation)).into_response(),
        Err(e) => {
            tracing::error!("Failed to start conversation: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct SendMessageReq {
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

pub async fn list_messages(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<server_common::Claims>,
    Path(conversation_id): Path<Uuid>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&claims) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(Vec::<ChatMessage>::new())).into_response(),
    };

    let svc = ChatService::new(state.db.pool.clone());
    match svc.list_messages(tenant_id, conversation_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list messages: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Vec::<ChatMessage>::new()),
            )
                .into_response()
        }
    }
}

pub async fn send_message(
    State(state): State<ChatApiState>,
    Extension(claims): Extension<server_common::Claims>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<SendMessageReq>,
) -> impl IntoResponse {
    let tenant_id = match get_tenant_id(&claims) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let svc = ChatService::new(state.db.pool.clone());
    match svc.send_message(tenant_id, conversation_id, payload.sender_type, payload.sender_id, payload.content).await {
        Ok(message) => (StatusCode::CREATED, Json(message)).into_response(),
        Err(e) => {
            tracing::error!("Failed to send message: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}
