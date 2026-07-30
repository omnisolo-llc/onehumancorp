use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::inbox::Message;

pub struct AppState {
    pub db: PgPool,
}

#[derive(Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
}

pub async fn list_messages(
    State(state): State<Arc<AppState>>,
    Path((tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    let mut tx = state.db.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
    .bind(tenant_id.to_string())
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let messages = sqlx::query_as::<_, Message>(
        r#"
        SELECT id, tenant_id, conversation_id, content, message_type, created_at, updated_at
        FROM messages
        WHERE conversation_id = $1
        ORDER BY created_at ASC
        "#,
    )
    .bind(conversation_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(messages))
}

pub async fn send_message(
    State(state): State<Arc<AppState>>,
    Path((tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<Message>, StatusCode> {
    let mut tx = state.db.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
    .bind(tenant_id.to_string())
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let message = sqlx::query_as::<_, Message>(
        r#"
        INSERT INTO messages (tenant_id, conversation_id, content, message_type)
        VALUES ($1, $2, $3, 'outgoing')
        RETURNING id, tenant_id, conversation_id, content, message_type, created_at, updated_at
        "#,
    )
    .bind(tenant_id)
    .bind(conversation_id)
    .bind(payload.content)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(message))
}

pub fn inbox_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/tenants/:tenant_id/conversations/:conversation_id/messages", get(list_messages))
        .route("/api/tenants/:tenant_id/conversations/:conversation_id/messages", post(send_message))
        .with_state(state)
}
