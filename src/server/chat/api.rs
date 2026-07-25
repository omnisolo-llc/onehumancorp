use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use crate::chat::models::{ChatInbox, ChatConversation, ChatMessage};

#[derive(Clone)]
pub struct ChatAppState {
    pub pool: PgPool,
}

pub fn chat_router(pool: PgPool) -> Router {
    let state = ChatAppState { pool };

    Router::new()
        .route("/api/v1/chat/:tenant_id/inboxes", get(get_inboxes))
        .route("/api/v1/chat/:tenant_id/inboxes", post(create_inbox))
        .route("/api/v1/chat/:tenant_id/conversations", get(get_conversations))
        .route("/api/v1/chat/:tenant_id/conversations", post(create_conversation))
        .with_state(state)
}

#[derive(serde::Deserialize)]
pub struct CreateInboxReq {
    pub name: String,
    pub channel_type: String,
}

async fn get_inboxes(
    State(state): State<ChatAppState>,
    Path(tenant_id): Path<String>,
) -> Result<Json<Vec<ChatInbox>>, axum::http::StatusCode> {
    let mut tx = state.pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let inboxes = sqlx::query_as!(
        ChatInbox,
        r#"
        SELECT id, tenant_id, name, channel_type,
               to_char(created_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as "created_at!",
               to_char(updated_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as "updated_at!"
        FROM chat_inboxes
        WHERE tenant_id = $1
        ORDER BY updated_at DESC
        "#,
        tenant_id
    )
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch inboxes: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(inboxes))
}

async fn create_inbox(
    State(state): State<ChatAppState>,
    Path(tenant_id): Path<String>,
    Json(payload): Json<CreateInboxReq>,
) -> Result<Json<ChatInbox>, axum::http::StatusCode> {
    let inbox_id = uuid::Uuid::new_v4().to_string();

    let mut tx = state.pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let inbox = sqlx::query_as!(
        ChatInbox,
        r#"
        INSERT INTO chat_inboxes (id, tenant_id, name, channel_type)
        VALUES ($1, $2, $3, $4)
        RETURNING id, tenant_id, name, channel_type,
                  to_char(created_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as "created_at!",
                  to_char(updated_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as "updated_at!"
        "#,
        inbox_id,
        tenant_id,
        payload.name,
        payload.channel_type
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create inbox: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(inbox))
}

async fn get_conversations(
    State(state): State<ChatAppState>,
    Path(tenant_id): Path<String>,
) -> Result<Json<Vec<ChatConversation>>, axum::http::StatusCode> {
    // Basic tenant isolation check is implicitly handled by setting app.current_tenant,
    // but we can query explicitly passing tenant_id

    // Set RLS for connection
    let mut tx = state.pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let convs = sqlx::query_as!(
        ChatConversation,
        r#"
        SELECT id, tenant_id, inbox_id, contact_id, status,
               to_char(created_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as "created_at!",
               to_char(updated_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as "updated_at!"
        FROM chat_conversations
        WHERE tenant_id = $1
        ORDER BY updated_at DESC
        "#,
        tenant_id
    )
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch conversations: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(convs))
}

#[derive(serde::Deserialize)]
pub struct CreateConversationReq {
    pub inbox_id: String,
    pub contact_id: Option<String>,
}

async fn create_conversation(
    State(state): State<ChatAppState>,
    Path(tenant_id): Path<String>,
    Json(payload): Json<CreateConversationReq>,
) -> Result<Json<ChatConversation>, axum::http::StatusCode> {
    let conv_id = uuid::Uuid::new_v4().to_string();

    let mut tx = state.pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let conv = sqlx::query_as!(
        ChatConversation,
        r#"
        INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status)
        VALUES ($1, $2, $3, $4, 'active')
        RETURNING id, tenant_id, inbox_id, contact_id, status,
                  to_char(created_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as "created_at!",
                  to_char(updated_at, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as "updated_at!"
        "#,
        conv_id,
        tenant_id,
        payload.inbox_id,
        payload.contact_id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create conversation: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(conv))
}
