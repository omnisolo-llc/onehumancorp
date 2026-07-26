
use crate::domain::chat::{Contact, Conversation, Inbox, Message};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateContactRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

pub async fn create_contact(
    State(pool): State<PgPool>,
    axum::extract::Extension(tenant_id): axum::extract::Extension<Uuid>,
    Json(payload): Json<CreateContactRequest>,
) -> impl IntoResponse {
    match Contact::create(&pool, tenant_id, payload.name, payload.email, payload.phone).await {
        Ok(contact) => (StatusCode::CREATED, Json(contact)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create contact: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create contact",
            )
                .into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
    pub channel: String,
}

pub async fn create_inbox(
    State(pool): State<PgPool>,
    axum::extract::Extension(tenant_id): axum::extract::Extension<Uuid>,
    Json(payload): Json<CreateInboxRequest>,
) -> impl IntoResponse {
    match Inbox::create(&pool, tenant_id, payload.name, payload.channel).await {
        Ok(inbox) => (StatusCode::CREATED, Json(inbox)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create inbox: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create inbox").into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    pub contact_id: Uuid,
    pub inbox_id: Uuid,
    pub status: String,
}

pub async fn create_conversation(
    State(pool): State<PgPool>,
    axum::extract::Extension(tenant_id): axum::extract::Extension<Uuid>,
    Json(payload): Json<CreateConversationRequest>,
) -> impl IntoResponse {
    match Conversation::create(
        &pool,
        tenant_id,
        payload.contact_id,
        payload.inbox_id,
        payload.status,
    )
    .await
    {
        Ok(conversation) => (StatusCode::CREATED, Json(conversation)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create conversation: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create conversation",
            )
                .into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub content: String,
}

pub async fn create_message(
    State(pool): State<PgPool>,
    axum::extract::Extension(tenant_id): axum::extract::Extension<Uuid>,
    Json(payload): Json<CreateMessageRequest>,
) -> impl IntoResponse {
    match Message::create(
        &pool,
        tenant_id,
        payload.conversation_id,
        payload.sender_type,
        payload.content,
    )
    .await
    {
        Ok(message) => (StatusCode::CREATED, Json(message)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create message: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create message",
            )
                .into_response()
        }
    }
}

pub async fn list_conversations(
    State(pool): State<PgPool>,
    axum::extract::Extension(tenant_id): axum::extract::Extension<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Conversation,
        r#"
        SELECT id, tenant_id, contact_id, inbox_id, status, assignee_id, created_at, updated_at
        FROM conversations
        WHERE tenant_id = $1
        "#,
        tenant_id
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(conversations) => (StatusCode::OK, Json(conversations)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list conversations: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to list conversations",
            )
                .into_response()
        }
    }
}

pub async fn get_messages(
    State(pool): State<PgPool>,
    axum::extract::Extension(tenant_id): axum::extract::Extension<Uuid>,
    Path(conversation_id): Path<Uuid>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Message,
        r#"
        SELECT id, tenant_id, conversation_id, sender_type, content, created_at, updated_at
        FROM messages
        WHERE tenant_id = $1 AND conversation_id = $2
        ORDER BY created_at ASC
        "#,
        tenant_id,
        conversation_id
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => {
            tracing::error!("Failed to get messages: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get messages").into_response()
        }
    }
}

pub fn chat_router() -> Router<AppState> {
    Router::new()
        .route("/contacts", post(create_contact))
        .route("/inboxes", post(create_inbox))
        .route(
            "/conversations",
            post(create_conversation).get(list_conversations),
        )
        .route(
            "/conversations/:conversation_id/messages",
            post(create_message).get(get_messages),
        )
}
