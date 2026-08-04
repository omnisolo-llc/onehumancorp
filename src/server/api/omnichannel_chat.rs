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
use crate::AppState;

#[derive(Serialize)]
pub struct ChatConversationResponse {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub inbox_id: Uuid,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct ChatMessageResponse {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub conversation_id: Uuid,
    pub content: String,
    pub sender_type: String,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/conversations", get(get_conversations))
        .route("/messages/:conversation_id", get(get_messages))
        .route("/messages", post(send_message))
}

async fn get_conversations(
    State(state): State<AppState>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref id_str) => match Uuid::parse_str(id_str) {
            Ok(id) => id,
            Err(_) => return (StatusCode::BAD_REQUEST, "Invalid tenant_id").into_response(),
        },
        None => return (StatusCode::UNAUTHORIZED, "Missing tenant_id").into_response(),
    };

    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let pool = state.db.pool.clone();
            let res = sqlx::query_as!(
                ChatConversationResponse,
                r#"SELECT id, contact_id, inbox_id, status, created_at FROM chat_conversations WHERE tenant_id = $1 ORDER BY created_at DESC"#,
                tenant_id
            )
            .fetch_all(&pool)
            .await;

            match res {
                Ok(conversations) => (StatusCode::OK, Json(conversations)).into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            }
        }
        _ => (StatusCode::NOT_IMPLEMENTED, "Not implemented for sqlite").into_response()
    }
}

async fn get_messages(
    State(state): State<AppState>,
    Path(conversation_id): Path<Uuid>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref id_str) => match Uuid::parse_str(id_str) {
            Ok(id) => id,
            Err(_) => return (StatusCode::BAD_REQUEST, "Invalid tenant_id").into_response(),
        },
        None => return (StatusCode::UNAUTHORIZED, "Missing tenant_id").into_response(),
    };

    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let pool = state.db.pool.clone();
            let res = sqlx::query_as!(
                ChatMessageResponse,
                r#"SELECT id, conversation_id, sender_type, sender_id, content, created_at FROM chat_messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC"#,
                tenant_id,
                conversation_id
            )
            .fetch_all(&pool)
            .await;

            match res {
                Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            }
        }
        _ => (StatusCode::NOT_IMPLEMENTED, "Not implemented for sqlite").into_response()
    }
}

async fn send_message(
    State(state): State<AppState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<SendMessageRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref id_str) => match Uuid::parse_str(id_str) {
            Ok(id) => id,
            Err(_) => return (StatusCode::BAD_REQUEST, "Invalid tenant_id").into_response(),
        },
        None => return (StatusCode::UNAUTHORIZED, "Missing tenant_id").into_response(),
    };

    let sender_id = match claims.user_id {
        Some(ref id_str) => Uuid::parse_str(id_str).ok(),
        None => None,
    };

    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let pool = state.db.pool.clone();
            let res = sqlx::query_as!(
                ChatMessageResponse,
                r#"
                INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id, conversation_id, sender_type, sender_id, content, created_at
                "#,
                Uuid::new_v4(),
                tenant_id,
                payload.conversation_id,
                payload.sender_type,
                sender_id,
                payload.content
            )
            .fetch_one(&pool)
            .await;

            match res {
                Ok(msg) => (StatusCode::OK, Json(msg)).into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            }
        }
        _ => (StatusCode::NOT_IMPLEMENTED, "Not implemented for sqlite").into_response()
    }
}
