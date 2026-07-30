use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth::Claims,
    domain::chat::models::{ChatContact, ChatConversation, ChatInbox, ChatMessage},
    services::hub::HubState,
};

pub fn router() -> Router<HubState> {
    Router::new()
        .route("/inboxes", get(list_inboxes).post(create_inbox))
        .route("/contacts", get(list_contacts).post(create_contact))
        .route("/conversations", get(list_conversations).post(create_conversation))
        .route("/conversations/:id/messages", get(list_messages).post(create_message))
}

#[derive(Deserialize)]
pub struct CreateInboxReq {
    pub name: String,
    pub channel_type: String,
}

async fn list_inboxes(
    State(state): State<HubState>,
    claims: Claims,
) -> Result<Json<Vec<ChatInbox>>, (StatusCode, String)> {
    let inboxes: Vec<ChatInbox> = sqlx::query_as(
        "SELECT * FROM chat_inboxes WHERE tenant_id = $1"
    )
    .bind(claims.org_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(inboxes))
}

async fn create_inbox(
    State(state): State<HubState>,
    claims: Claims,
    Json(payload): Json<CreateInboxReq>,
) -> Result<Json<ChatInbox>, (StatusCode, String)> {
    let id = Uuid::new_v4();
    let inbox: ChatInbox = sqlx::query_as(
        r#"
        INSERT INTO chat_inboxes (id, tenant_id, name)
        VALUES ($1, $2, $3)
        RETURNING *
        "#
    )
    .bind(id)
    .bind(claims.org_id)
    .bind(&payload.name)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let channel_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO chat_channels (id, tenant_id, inbox_id, channel_type)
        VALUES ($1, $2, $3, $4)
        "#
    )
    .bind(channel_id)
    .bind(claims.org_id)
    .bind(inbox.id)
    .bind(&payload.channel_type)
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(inbox))
}

#[derive(Deserialize)]
pub struct CreateContactReq {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

async fn list_contacts(
    State(state): State<HubState>,
    claims: Claims,
) -> Result<Json<Vec<ChatContact>>, (StatusCode, String)> {
    let contacts: Vec<ChatContact> = sqlx::query_as(
        "SELECT * FROM chat_contacts WHERE tenant_id = $1"
    )
    .bind(claims.org_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(contacts))
}

async fn create_contact(
    State(state): State<HubState>,
    claims: Claims,
    Json(payload): Json<CreateContactReq>,
) -> Result<Json<ChatContact>, (StatusCode, String)> {
    let id = Uuid::new_v4();
    let contact: ChatContact = sqlx::query_as(
        r#"
        INSERT INTO chat_contacts (id, tenant_id, name, email, phone)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#
    )
    .bind(id)
    .bind(claims.org_id)
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&payload.phone)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(contact))
}

#[derive(Deserialize)]
pub struct CreateConversationReq {
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
}

async fn list_conversations(
    State(state): State<HubState>,
    claims: Claims,
) -> Result<Json<Vec<ChatConversation>>, (StatusCode, String)> {
    let convos: Vec<ChatConversation> = sqlx::query_as(
        "SELECT * FROM chat_conversations WHERE tenant_id = $1"
    )
    .bind(claims.org_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(convos))
}

async fn create_conversation(
    State(state): State<HubState>,
    claims: Claims,
    Json(payload): Json<CreateConversationReq>,
) -> Result<Json<ChatConversation>, (StatusCode, String)> {
    let id = Uuid::new_v4();
    let convo: ChatConversation = sqlx::query_as(
        r#"
        INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#
    )
    .bind(id)
    .bind(claims.org_id)
    .bind(payload.inbox_id)
    .bind(payload.contact_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(convo))
}

#[derive(Deserialize)]
pub struct CreateMessageReq {
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

async fn list_messages(
    State(state): State<HubState>,
    claims: Claims,
    Path(conversation_id): Path<Uuid>,
) -> Result<Json<Vec<ChatMessage>>, (StatusCode, String)> {
    let messages: Vec<ChatMessage> = sqlx::query_as(
        "SELECT * FROM chat_messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC"
    )
    .bind(claims.org_id)
    .bind(conversation_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(messages))
}

async fn create_message(
    State(state): State<HubState>,
    claims: Claims,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<CreateMessageReq>,
) -> Result<Json<ChatMessage>, (StatusCode, String)> {
    let id = Uuid::new_v4();
    let msg: ChatMessage = sqlx::query_as(
        r#"
        INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#
    )
    .bind(id)
    .bind(claims.org_id)
    .bind(conversation_id)
    .bind(&payload.sender_type)
    .bind(payload.sender_id)
    .bind(&payload.content)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(msg))
}
