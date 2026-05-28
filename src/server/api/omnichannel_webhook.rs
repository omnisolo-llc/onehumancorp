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
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
use uuid::Uuid;
use crate::db::get_pool;

#[derive(Deserialize)]
pub struct OmnichannelWebhookPayload {
    pub tenant_id: String,
    pub message: String,
    pub channel: String,
    pub external_id: String,
    pub sender_id: String,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub request_id: Option<String>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_omnichannel_webhook))
        .with_state(orchestrator)
}

async fn handle_omnichannel_webhook(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Json(payload): Json<OmnichannelWebhookPayload>,
) -> impl IntoResponse {
    let pool = get_pool();
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response(),
    };
    if let Err(_) = crate::common::auth_utils::set_org_context(&mut *tx, &payload.tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response();
    }

    // Upsert thread
    let thread_id = Uuid::new_v4().to_string();
    if let Err(_) = sqlx::query(
        "INSERT INTO unified_thread (id, merchant_id, customer_id, requires_human_attention) VALUES ($1, $2, $3, $4)"
    )
    .bind(&thread_id)
    .bind(&payload.tenant_id)
    .bind(&payload.sender_id)
    .bind(true)
    .execute(&mut *tx)
    .await {
        let _ = tx.rollback().await;
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response();
    }

    // Insert message
    let msg_id = Uuid::new_v4().to_string();
    if let Err(_) = sqlx::query(
        "INSERT INTO unified_message (id, thread_id, channel, external_message_id, direction, sender_type, body) VALUES ($1, $2, $3, $4, 'INBOUND', 'CUSTOMER', $5)"
    )
    .bind(&msg_id)
    .bind(&thread_id)
    .bind(&payload.channel)
    .bind(&payload.external_id)
    .bind(&payload.message)
    .execute(&mut *tx)
    .await {
        let _ = tx.rollback().await;
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response();
    }

    // Generate AI draft
    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
    let draft_reply = if !api_key.is_empty() {
        let business_context = "A friendly business."; // mocked
        let prompt = format!(
            "Write a concise reply. Business context: {} Customer message: {}",
            business_context, payload.message
        );
        let client = crate::minimax::MinimaxClient::new(api_key);
        client.reason(&prompt).await.unwrap_or_else(|_| "Draft generation failed.".to_string())
    } else {
        "Thank you for reaching out! We will get back to you shortly.".to_string()
    };

    // Save draft as pending approval action in orchestration layer (mocked by executing action in CS dept)
    let description = format!("Incoming {} message", payload.channel);
    match orchestrator.execute_action(
        DepartmentType::CustomerSuccess,
        description,
        payload.tenant_id,
        ActionRisk::DraftForReview,
        serde_json::json!({
            "channel": payload.channel,
            "message": payload.message,
            "draft_reply": draft_reply,
            "thread_id": thread_id,
        }),
    ).await {
        Ok(req) => {
            let _ = tx.commit().await;
            (StatusCode::OK, Json(WebhookResponse { success: true, request_id: Some(req.id) })).into_response()
        },
        Err(_) => {
            let _ = tx.rollback().await;
            (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response()
        }
    }
}
