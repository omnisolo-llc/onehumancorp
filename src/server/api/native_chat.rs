use axum::{
    extract::{Extension, Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
use crate::db::DB;
use crate::services::chat::service::ChatService;

pub struct AppState {
    pub db: Arc<DB>,
    pub chat_service: Arc<ChatService>,
}

pub fn router(db: Arc<DB>, auth_store: Arc<::server_auth::Store>) -> Router {
    let chat_service = Arc::new(ChatService::new(db.clone()));
    let state = Arc::new(AppState {
        db,
        chat_service,
    });

    Router::new()
        .route("/inboxes", post(create_inbox).get(list_inboxes))
        .route("/contacts", post(create_contact).get(list_contacts))
        .route("/conversations", post(start_conversation).get(list_conversations))
        .route("/conversations/:conversation_id/messages", get(list_messages))
        .route("/messages", post(send_message))
        .layer(axum::middleware::from_fn_with_state(
            auth_store,
            ::server_auth::strict_bearer_auth_middleware,
        ))
        .with_state(state)
}

fn claim_tenant_id(claims: &::server_common::Claims) -> Result<Uuid, axum::response::Response> {
    claims
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|tenant_id| !tenant_id.is_empty() && !tenant_id.eq_ignore_ascii_case("system"))
        .and_then(|tenant_id| Uuid::parse_str(tenant_id).ok())
        .ok_or_else(|| {
            (
                axum::http::StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Unauthorized"})),
            )
                .into_response()
        })
}

// 1. Inboxes CRUD Handlers

#[derive(Deserialize)]
pub struct CreateInboxPayload {
    pub name: String,
}

async fn create_inbox(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateInboxPayload>,
) -> impl IntoResponse {
    let tenant_id = match claim_tenant_id(&claims) {
        Ok(t) => t,
        Err(r) => return r,
    };

    if payload.name.trim().is_empty() {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({"error": "Inbox name is required"})),
        )
            .into_response();
    }

    match state.chat_service.create_inbox(tenant_id, payload.name).await {
        Ok(inbox) => (axum::http::StatusCode::CREATED, Json(inbox)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create inbox: {e:?}");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to create inbox"})),
            )
                .into_response()
        }
    }
}

async fn list_inboxes(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match claim_tenant_id(&claims) {
        Ok(t) => t,
        Err(r) => return r,
    };

    match state.chat_service.get_inboxes(tenant_id).await {
        Ok(inboxes) => (axum::http::StatusCode::OK, Json(inboxes)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list inboxes: {e:?}");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to list inboxes"})),
            )
                .into_response()
        }
    }
}

// 2. Contacts CRUD Handlers

#[derive(Deserialize)]
pub struct CreateContactPayload {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

async fn create_contact(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateContactPayload>,
) -> impl IntoResponse {
    let tenant_id = match claim_tenant_id(&claims) {
        Ok(t) => t,
        Err(r) => return r,
    };

    match state.chat_service.create_contact(tenant_id, payload.name, payload.email, payload.phone).await {
        Ok(contact) => (axum::http::StatusCode::CREATED, Json(contact)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create contact: {e:?}");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to create contact"})),
            )
                .into_response()
        }
    }
}

async fn list_contacts(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match claim_tenant_id(&claims) {
        Ok(t) => t,
        Err(r) => return r,
    };

    match state.chat_service.get_contacts(tenant_id).await {
        Ok(contacts) => (axum::http::StatusCode::OK, Json(contacts)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list contacts: {e:?}");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to list contacts"})),
            )
                .into_response()
        }
    }
}

// 3. Conversations CRUD Handlers

#[derive(Deserialize)]
pub struct StartConversationPayload {
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
}

async fn start_conversation(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<StartConversationPayload>,
) -> impl IntoResponse {
    let tenant_id = match claim_tenant_id(&claims) {
        Ok(t) => t,
        Err(r) => return r,
    };

    match state.chat_service.start_conversation(tenant_id, payload.inbox_id, payload.contact_id, payload.assignee_id).await {
        Ok(conversation) => (axum::http::StatusCode::CREATED, Json(conversation)).into_response(),
        Err(sqlx::Error::RowNotFound) => {
            (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({"error": "Inbox or contact not found or access denied"})),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("Failed to start conversation: {e:?}");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to start conversation"})),
            )
                .into_response()
        }
    }
}

async fn list_conversations(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match claim_tenant_id(&claims) {
        Ok(t) => t,
        Err(r) => return r,
    };

    match state.chat_service.get_conversations(tenant_id).await {
        Ok(conversations) => (axum::http::StatusCode::OK, Json(conversations)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list conversations: {e:?}");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to list conversations"})),
            )
                .into_response()
        }
    }
}

// 4. Messages Handlers

#[derive(Deserialize)]
pub struct SendMessagePayload {
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

async fn send_message(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<SendMessagePayload>,
) -> impl IntoResponse {
    let tenant_id = match claim_tenant_id(&claims) {
        Ok(t) => t,
        Err(r) => return r,
    };

    if payload.content.trim().is_empty() {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({"error": "Message content cannot be empty"})),
        )
            .into_response();
    }

    match state.chat_service.send_message(tenant_id, payload.conversation_id, payload.sender_type, payload.sender_id, payload.content).await {
        Ok(message) => (axum::http::StatusCode::CREATED, Json(message)).into_response(),
        Err(sqlx::Error::RowNotFound) => {
            (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({"error": "Conversation not found or access denied"})),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("Failed to send message: {e:?}");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to send message"})),
            )
                .into_response()
        }
    }
}

async fn list_messages(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(conversation_id_str): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match claim_tenant_id(&claims) {
        Ok(t) => t,
        Err(r) => return r,
    };

    let conversation_id = match Uuid::parse_str(&conversation_id_str) {
        Ok(id) => id,
        Err(_) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid conversation ID"})),
            )
                .into_response()
        }
    };

    match state.chat_service.get_messages(tenant_id, conversation_id).await {
        Ok(messages) => (axum::http::StatusCode::OK, Json(messages)).into_response(),
        Err(sqlx::Error::RowNotFound) => {
            (
                axum::http::StatusCode::NOT_FOUND,
                Json(json!({"error": "Conversation not found or access denied"})),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list messages: {e:?}");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to list messages"})),
            )
                .into_response()
        }
    }
}
