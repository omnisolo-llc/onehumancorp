use axum::{
    extract::{State, Json},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize};
use uuid::Uuid;
use crate::domain::chat::service::ChatService;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[derive(Debug, Deserialize)]
pub struct WebhookPayload {
    pub channel: String,
    pub sender: SenderInfo,
    pub message: MessageInfo,
}

#[derive(Debug, Deserialize)]
pub struct SenderInfo {
    pub name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MessageInfo {
    pub text: String,
}

pub fn router(pool: PgPool) -> Router {
    let state = AppState { pool };
    Router::new()
        .route("/api/v1/webhooks/chat/:channel", post(handle_webhook))
        .with_state(state)
}

async fn handle_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    let tenant_id_str = match headers.get("X-Tenant-Id") {
        Some(val) => match val.to_str() {
            Ok(s) => s,
            Err(_) => return (StatusCode::BAD_REQUEST, "Invalid X-Tenant-Id header format").into_response(),
        },
        None => return (StatusCode::BAD_REQUEST, "Missing X-Tenant-Id header").into_response(),
    };

    let tenant_id = match Uuid::parse_str(tenant_id_str) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid Tenant ID").into_response(),
    };

    // For now we mock the inbox resolution. In reality this would query the DB to find the Inbox associated with this channel integration
    let inbox_id = Uuid::new_v4();

    let service = ChatService::new(state.pool.clone());

    match service.process_incoming_message(
        tenant_id,
        inbox_id,
        payload.sender.name.or(payload.sender.username),
        payload.sender.email,
        payload.sender.phone,
        payload.message.text,
    ).await {
        Ok(_) => (StatusCode::OK, "Message processed successfully").into_response(),
        Err(e) => {
            tracing::error!("Error processing message: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error processing message").into_response()
        }
    }
}
