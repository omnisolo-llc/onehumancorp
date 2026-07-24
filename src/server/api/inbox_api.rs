use axum::{
    extract::{Path, State},
    routing::{get, post, put},
    Json, Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;
use server_common::Result;
use ohc_mono::app::AppState;
use server_auth::extractors::RequireUser;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_inboxes))
        .route("/conversations", get(list_conversations))
        .route("/conversations/:id/messages", get(list_messages))
        .route("/conversations/:id/messages", post(send_message))
        .route("/conversations/:id/approve_draft", post(approve_draft))
}

async fn list_inboxes(
    State(state): State<Arc<AppState>>,
    RequireUser(user): RequireUser,
) -> Result<Json<Value>> {
    let inboxes = sqlx::query!(
        r#"
        SELECT id, name, channel_type
        FROM inboxes
        WHERE tenant_id = $1
        "#,
        user.tenant_id
    )
    .fetch_all(&*state.db)
    .await?;

    let result: Vec<_> = inboxes.into_iter().map(|i| json!({
        "id": i.id,
        "name": i.name,
        "channel_type": i.channel_type
    })).collect();

    Ok(Json(json!({ "inboxes": result })))
}

async fn list_conversations(
    State(state): State<Arc<AppState>>,
    RequireUser(user): RequireUser,
) -> Result<Json<Value>> {
    let convs = sqlx::query!(
        r#"
        SELECT c.id, c.status, i.name as inbox_name, cust.name as contact_name
        FROM conversations c
        JOIN inboxes i ON c.inbox_id = i.id
        JOIN customers cust ON c.contact_id = cust.id
        WHERE c.tenant_id = $1
        ORDER BY c.updated_at DESC
        "#,
        user.tenant_id
    )
    .fetch_all(&*state.db)
    .await?;

    let result: Vec<_> = convs.into_iter().map(|c| json!({
        "id": c.id,
        "status": c.status,
        "inbox_name": c.inbox_name,
        "contact_name": c.contact_name,
    })).collect();

    Ok(Json(json!({ "conversations": result })))
}

async fn list_messages(
    State(state): State<Arc<AppState>>,
    RequireUser(user): RequireUser,
    Path(conversation_id): Path<Uuid>,
) -> Result<Json<Value>> {
    let msgs = sqlx::query!(
        r#"
        SELECT id, content, message_type, status, sender_type
        FROM chat_messages
        WHERE tenant_id = $1 AND conversation_id = $2
        ORDER BY created_at ASC
        "#,
        user.tenant_id,
        conversation_id
    )
    .fetch_all(&*state.db)
    .await?;

    let result: Vec<_> = msgs.into_iter().map(|m| json!({
        "id": m.id,
        "content": m.content,
        "message_type": m.message_type,
        "status": m.status,
        "sender_type": m.sender_type,
    })).collect();

    Ok(Json(json!({ "messages": result })))
}

#[derive(serde::Deserialize)]
pub struct SendMessageReq {
    pub content: String,
}

async fn send_message(
    State(state): State<Arc<AppState>>,
    RequireUser(user): RequireUser,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<SendMessageReq>,
) -> Result<Json<Value>> {
    let conv = sqlx::query!(
        "SELECT inbox_id FROM conversations WHERE id = $1 AND tenant_id = $2",
        conversation_id, user.tenant_id
    )
    .fetch_one(&*state.db)
    .await?;

    let msg_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO chat_messages (id, tenant_id, conversation_id, inbox_id, content, message_type, status, sender_type)
        VALUES ($1, $2, $3, $4, $5, 'outgoing', 'sent', 'user')
        "#,
        msg_id, user.tenant_id, conversation_id, conv.inbox_id, payload.content
    )
    .execute(&*state.db)
    .await?;

    Ok(Json(json!({ "id": msg_id, "status": "sent" })))
}

#[derive(serde::Deserialize)]
pub struct ApproveDraftReq {
    pub message_id: Uuid,
}

async fn approve_draft(
    State(state): State<Arc<AppState>>,
    RequireUser(user): RequireUser,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<ApproveDraftReq>,
) -> Result<Json<Value>> {
    sqlx::query!(
        r#"
        UPDATE chat_messages
        SET status = 'sent'
        WHERE id = $1 AND conversation_id = $2 AND tenant_id = $3 AND status = 'draft'
        "#,
        payload.message_id, conversation_id, user.tenant_id
    )
    .execute(&*state.db)
    .await?;

    Ok(Json(json!({ "status": "approved" })))
}
