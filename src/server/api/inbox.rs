use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow, Debug)]
pub struct Customer {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, FromRow, Debug)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub customer_id: Uuid,
    pub subject: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, FromRow, Debug)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub channel: String,
    pub direction: String,
    pub content: String,
    pub ai_draft: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateCustomerReq {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateConversationReq {
    pub customer_id: Uuid,
    pub subject: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateMessageReq {
    pub channel: String,
    pub direction: String,
    pub content: String,
    pub ai_draft: Option<bool>,
}

pub fn router() -> Router<Arc<crate::harness::ServerState>> {
    Router::new()
        .route("/customers", post(create_customer).get(list_customers))
        .route("/conversations", post(create_conversation).get(list_conversations))
        .route("/conversations/:id/messages", post(create_message).get(list_messages))
}

async fn create_customer(
    State(state): State<Arc<crate::harness::ServerState>>,
    axum::Extension(tenant_id): axum::Extension<crate::auth::TenantId>,
    Json(payload): Json<CreateCustomerReq>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let customer = sqlx::query_as::<_, Customer>(
        r#"
        INSERT INTO customers (tenant_id, name, email, phone)
        VALUES ($1, $2, $3, $4)
        RETURNING id, tenant_id, name, email, phone, created_at, updated_at
        "#
    )
    .bind(tenant_id.0)
    .bind(payload.name)
    .bind(payload.email)
    .bind(payload.phone)
    .fetch_one(&state.db)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(customer)))
}

async fn list_customers(
    State(state): State<Arc<crate::harness::ServerState>>,
    axum::Extension(tenant_id): axum::Extension<crate::auth::TenantId>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let customers = sqlx::query_as::<_, Customer>(
        "SELECT id, tenant_id, name, email, phone, created_at, updated_at FROM customers WHERE tenant_id = $1 ORDER BY created_at DESC"
    )
    .bind(tenant_id.0)
    .fetch_all(&state.db)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(customers))
}

async fn create_conversation(
    State(state): State<Arc<crate::harness::ServerState>>,
    axum::Extension(tenant_id): axum::Extension<crate::auth::TenantId>,
    Json(payload): Json<CreateConversationReq>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let conversation = sqlx::query_as::<_, Conversation>(
        r#"
        INSERT INTO conversations (tenant_id, customer_id, subject)
        VALUES ($1, $2, $3)
        RETURNING id, tenant_id, customer_id, subject, created_at, updated_at
        "#
    )
    .bind(tenant_id.0)
    .bind(payload.customer_id)
    .bind(payload.subject)
    .fetch_one(&state.db)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(conversation)))
}

async fn list_conversations(
    State(state): State<Arc<crate::harness::ServerState>>,
    axum::Extension(tenant_id): axum::Extension<crate::auth::TenantId>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let conversations = sqlx::query_as::<_, Conversation>(
        "SELECT id, tenant_id, customer_id, subject, created_at, updated_at FROM conversations WHERE tenant_id = $1 ORDER BY created_at DESC"
    )
    .bind(tenant_id.0)
    .fetch_all(&state.db)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(conversations))
}

async fn create_message(
    State(state): State<Arc<crate::harness::ServerState>>,
    Path(conversation_id): Path<Uuid>,
    axum::Extension(tenant_id): axum::Extension<crate::auth::TenantId>,
    Json(payload): Json<CreateMessageReq>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let ai_draft = payload.ai_draft.unwrap_or(false);

    // Begin transaction to insert message
    let mut tx = state.db.begin().await.map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let message = sqlx::query_as::<_, Message>(
        r#"
        INSERT INTO messages (tenant_id, conversation_id, channel, direction, content, ai_draft)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, tenant_id, conversation_id, channel, direction, content, ai_draft, created_at
        "#
    )
    .bind(tenant_id.0)
    .bind(conversation_id)
    .bind(payload.channel.clone())
    .bind(payload.direction.clone())
    .bind(payload.content)
    .bind(ai_draft)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // AI agent hook: If it's an inbound message, optionally generate a draft response
    if payload.direction == "INBOUND" {
        // Trigger customer success agent to generate a draft
        let _ = sqlx::query(
            r#"
            INSERT INTO messages (tenant_id, conversation_id, channel, direction, content, ai_draft)
            VALUES ($1, $2, $3, 'OUTBOUND', 'Generated AI draft reply', true)
            "#
        )
        .bind(tenant_id.0)
        .bind(conversation_id)
        .bind(payload.channel)
        .execute(&mut *tx)
        .await
        .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    tx.commit().await.map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(message)))
}

async fn list_messages(
    State(state): State<Arc<crate::harness::ServerState>>,
    Path(conversation_id): Path<Uuid>,
    axum::Extension(tenant_id): axum::Extension<crate::auth::TenantId>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let messages = sqlx::query_as::<_, Message>(
        "SELECT id, tenant_id, conversation_id, channel, direction, content, ai_draft, created_at FROM messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC"
    )
    .bind(tenant_id.0)
    .bind(conversation_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(messages))
}
