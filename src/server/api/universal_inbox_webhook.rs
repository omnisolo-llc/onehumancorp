use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use uuid::Uuid;
use crate::db::get_pool;

#[derive(Deserialize)]
pub struct UniversalWebhookPayload {
    pub tenant_id: String,
    pub customer_id: Option<String>,
    pub channel: String,
    pub message: String,
    pub sender_id: Option<String>,
    #[serde(default)]
    pub target_language: Option<String>,
}

#[derive(Serialize)]
pub struct UniversalWebhookResponse {
    pub success: bool,
    pub request_id: Option<String>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_universal_webhook))
        .with_state(orchestrator)
}

async fn handle_universal_webhook(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Json(payload): Json<UniversalWebhookPayload>,
) -> impl IntoResponse {
    let pool = get_pool();
    let thread_id = Uuid::new_v4().to_string(); // In a real implementation we would look up existing thread or create one
    let message_id = Uuid::new_v4().to_string();

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(UniversalWebhookResponse { success: false, request_id: None })).into_response();
        }
    };

    // Insert Thread
    let _ = sqlx::query(
        r#"
        INSERT INTO communication_threads (id, tenant_id, customer_id, channel, status, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 'open', NOW(), NOW())
        "#
    )
    .bind(&thread_id)
    .bind(&payload.tenant_id)
    .bind(&payload.customer_id)
    .bind(&payload.channel)
    .execute(&mut *tx)
    .await;

    // Insert Message
    let _ = sqlx::query(
        r#"
        INSERT INTO communication_messages (id, tenant_id, thread_id, direction, sender_id, original_content, content, status, created_at)
        VALUES ($1, $2, $3, 'inbound', $4, $5, $6, 'unread', NOW())
        "#
    )
    .bind(&message_id)
    .bind(&payload.tenant_id)
    .bind(&thread_id)
    .bind(&payload.sender_id)
    .bind(&payload.message)
    .bind(&payload.message)
    .execute(&mut *tx)
    .await;

    let _ = tx.commit().await;

    let target_language = payload.target_language.unwrap_or_else(|| "English".to_string());

    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: serde_json::json!({
            "source": payload.channel,
            "original_message": payload.message,
            "target_language": target_language,
            "inbox_message_id": message_id,
            "thread_id": thread_id,
        }),
    };

    match orchestrator.dispatch_event(event).await {
        Ok(_) => (StatusCode::OK, Json(UniversalWebhookResponse { success: true, request_id: Some(message_id) })).into_response(),
        Err(e) => {
            if e.contains("AI Budget exhausted") {
                (StatusCode::TOO_MANY_REQUESTS, Json(UniversalWebhookResponse { success: false, request_id: None })).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(UniversalWebhookResponse { success: false, request_id: None })).into_response()
            }
        }
    }
}
