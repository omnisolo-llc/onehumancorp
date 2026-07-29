use axum::{
    extract::{State, Json},
    routing::post,
    Router,
    http::{StatusCode, HeaderMap},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub webhook_secret: String,
}

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub status: String,
    pub message_id: Option<String>,
}

pub fn webhook_router(state: AppState) -> Router {
    Router::new()
        .route("/webhooks/chat", post(handle_webhook))
        .with_state(state)
}

async fn handle_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<WebhookPayload>,
) -> Result<Json<WebhookResponse>, (StatusCode, String)> {
    let auth_header = headers.get("Authorization").ok_or((StatusCode::UNAUTHORIZED, "Missing auth".into()))?;
    let expected_header = format!("Bearer {}", state.webhook_secret);
    let provided_header = auth_header.to_str().unwrap_or("");

    // Constant time comparison
    if state.webhook_secret.is_empty() || provided_header.len() != expected_header.len() {
        return Err((StatusCode::UNAUTHORIZED, "Invalid auth".into()));
    }

    // Manual constant time comparison
    let mut result = 0;
    for (a, b) in provided_header.bytes().zip(expected_header.bytes()) {
        result |= a ^ b;
    }

    if result != 0 {
        return Err((StatusCode::UNAUTHORIZED, "Invalid auth".into()));
    }

    let tenant_id = Uuid::parse_str(&payload.tenant_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid tenant_id".into()))?;
    let inbox_id = Uuid::parse_str(&payload.inbox_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid inbox_id".into()))?;
    let contact_id = Uuid::parse_str(&payload.contact_id).map_err(|_| (StatusCode::BAD_REQUEST, "Invalid contact_id".into()))?;

    let mut tx = state.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)").bind(tenant_id.to_string()).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let conv = sqlx::query(
        "SELECT id FROM chat_conversation WHERE tenant_id = $1 AND inbox_id = $2 AND contact_id = $3 AND status = 'open' LIMIT 1 FOR UPDATE")
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    use sqlx::Row;
    let conversation_id = match conv {
        Some(row) => row.get("id"),
        None => {
            let new_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO chat_conversation (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, 'open')")
                .bind(new_id)
                .bind(tenant_id)
                .bind(inbox_id)
                .bind(contact_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            new_id
        }
    };

    let message_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO chat_message (id, tenant_id, conversation_id, content, sender_type, sender_id) VALUES ($1, $2, $3, $4, 'contact', $5)")
        .bind(message_id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(&payload.content)
        .bind(contact_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(WebhookResponse {
        status: "success".into(),
        message_id: Some(message_id.to_string()),
    }))
}
