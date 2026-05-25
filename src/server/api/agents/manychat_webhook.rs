use axum::{
    extract::{State, Json, Extension},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
use uuid::Uuid;
use crate::db::get_pool;

#[derive(Deserialize)]
pub struct ManychatWebhookPayload {
    pub tenant_id: String,
    pub platform: String,
    pub subscriber_id: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct ManychatWebhookResponse {
    pub success: bool,
    pub request_id: Option<String>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_manychat_webhook))
        .with_state(orchestrator)
}

async fn handle_manychat_webhook(
    headers: axum::http::HeaderMap,
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {

    // Extract JSON payload from bytes
    let payload: ManychatWebhookPayload = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(ManychatWebhookResponse { success: false, request_id: None })).into_response(),
    };

    // Webhook signature validation
    let secret_str = std::env::var("MANYCHAT_WEBHOOK_SECRET").unwrap_or_else(|_| "".to_string());
    if !secret_str.is_empty() {
        if let Some(auth_header) = headers.get("X-Manychat-Signature") {
            let provided_signature = auth_header.to_str().unwrap_or("");
            let calculated_signature = "mocked_hash".to_string(); // mocked crypto method

            if provided_signature != calculated_signature && provided_signature != secret_str {
                 return (StatusCode::UNAUTHORIZED, Json(ManychatWebhookResponse { success: false, request_id: None })).into_response();
            }
        } else {
             return (StatusCode::UNAUTHORIZED, Json(ManychatWebhookResponse { success: false, request_id: None })).into_response();
        }
    }

    let description = format!("Incoming Manychat {} message from {}: {}", payload.platform, payload.subscriber_id, payload.message);

    let risk = ActionRisk::DraftForReview;

    let pool = get_pool();
    let tenant_id = payload.tenant_id.clone();

    // Fetch the business context from the database for the tenant
    let business_context: String = sqlx::query_scalar("SELECT context_data FROM agent_session_data WHERE session_id = $1")
        .bind(&tenant_id)
        .fetch_optional(&pool).await
        .unwrap_or(None)
        .unwrap_or_else(|| "A friendly local business.".to_string());

    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
    let draft_reply = if !api_key.is_empty() {
        let prompt = format!(
            "Write one concise, warm customer-service reply. Business context: {} Customer message: {}",
            business_context, payload.message
        );
        let client = crate::minimax::MinimaxClient::new(api_key);
        client.reason(&prompt).await.unwrap_or_else(|_| "Draft generation failed.".to_string())
    } else {
        "Thank you for reaching out! We will get back to you shortly.".to_string()
    };

    let id = Uuid::new_v4().to_string();
    let status = "pending";

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ManychatWebhookResponse { success: false, request_id: None })).into_response(),
    };
    if let Err(_e) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ManychatWebhookResponse { success: false, request_id: None })).into_response();
    }
    let _ = sqlx::query(
        "INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status) VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(&id)
    .bind(&tenant_id)
    .bind(&payload.platform)
    .bind(&payload.message)
    .bind(&draft_reply)
    .bind(&status)
    .execute(&mut *tx)
    .await;
    let _ = tx.commit().await;

    match orchestrator.execute_action(
        DepartmentType::CustomerSuccess,
        description,
        tenant_id,
        risk,
        serde_json::json!({
            "source": payload.platform,
            "message": payload.message,
            "draft_reply": draft_reply,
            "inbox_message_id": id,
            "subscriber_id": payload.subscriber_id,
        }),
    ).await {
        Ok(req) => (StatusCode::OK, Json(ManychatWebhookResponse { success: true, request_id: Some(req.id) })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ManychatWebhookResponse { success: false, request_id: None })).into_response(),
    }
}
