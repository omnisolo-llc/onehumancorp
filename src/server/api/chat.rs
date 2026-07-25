use axum::{extract::Extension, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::hub::Hub;

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct SendMessageResponse {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_type: String,
    pub sender_id: String,
    pub content: String,
    pub created_at: String,
}

pub async fn send_message(
    Extension(hub): Extension<Arc<Hub>>,
    Json(req): Json<SendMessageRequest>,
) -> impl IntoResponse {
    let tenant_id = uuid::Uuid::parse_str(&req.tenant_id).unwrap_or_default();
    let conversation_id = uuid::Uuid::parse_str(&req.conversation_id).unwrap_or_default();
    let sender_id = uuid::Uuid::parse_str(&req.sender_id).unwrap_or_default();

    let row_result = sqlx::query(
        r#"
        INSERT INTO chat_messages (tenant_id, conversation_id, sender_type, sender_id, content)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, created_at
        "#)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind("agent")
        .bind(sender_id)
        .bind(&req.content)
        .fetch_one(&hub.pool)
        .await;

    match row_result {
        Ok(row) => {
            use sqlx::Row;
            let id: uuid::Uuid = row.get("id");
            let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");

            let res = SendMessageResponse {
                id: id.to_string(),
                tenant_id: req.tenant_id,
                conversation_id: req.conversation_id,
                sender_type: "agent".to_string(),
                sender_id: req.sender_id,
                content: req.content,
                created_at: created_at.to_string(),
            };
            (StatusCode::OK, Json(res)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn router() -> Router {
    Router::new().route("/send", post(send_message))
}
