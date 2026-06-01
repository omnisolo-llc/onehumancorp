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

#[derive(Deserialize, Debug)]
pub struct VoiceWebhookPayload {
    pub to_number: String,
    pub tenant_id: Option<String>,
    pub caller_id: String,
    pub call_id: String,
    pub status: String,
    pub transcript: String,
    pub actions_taken: Option<Vec<String>>,
    pub detected_language: Option<String>,
}

#[derive(Serialize)]
pub struct VoiceWebhookResponse {
    pub success: bool,
    pub request_id: Option<String>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_voice_webhook))
        .with_state(orchestrator)
}


async fn handle_voice_webhook(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<VoiceWebhookPayload>,
) -> impl IntoResponse {
    let pool = get_pool();

    // In a production system, we would verify the Vapi/Twilio signature here
    // using headers.get("x-vapi-signature") or similar.
    let _signature = headers.get("x-vapi-signature").and_then(|h| h.to_str().ok()).unwrap_or("");

    // Look up tenant context based on the dialed number (to_number)
    let tenant_id: Option<String> = sqlx::query_scalar(
        "SELECT id FROM tenants WHERE support_phone_number = $1"
    )
    .bind(&payload.to_number)
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);

    let tenant_id = match tenant_id {
        Some(t) => t,
        None => {
            // Fallback for tests or unassigned numbers
            payload.to_number.clone()
        }
    };
    let id = Uuid::new_v4().to_string();

    let actions_str = payload.actions_taken
        .unwrap_or_default()
        .join(", ");

    let language = payload.detected_language.unwrap_or_else(|| "English".to_string());

    let content = serde_json::json!({
        "type": "voice_call",
        "call_id": payload.call_id,
        "caller_id": payload.caller_id,
        "language": language,
        "transcript": payload.transcript,
        "actions_taken": actions_str,
    }).to_string();

    let draft_reply = format!("Call summarized: {}", payload.transcript);

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(VoiceWebhookResponse { success: false, request_id: None })).into_response(),
    };

    if let Err(_e) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(VoiceWebhookResponse { success: false, request_id: None })).into_response();
    }

    let source = "voice";
    let status = "completed";

    let _ = sqlx::query(
        "INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status) VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(&id)
    .bind(&tenant_id)
    .bind(&source)
    .bind(&content)
    .bind(&draft_reply)
    .bind(&status)
    .execute(&mut *tx)
    .await;

    let _ = tx.commit().await;

    let risk = ActionRisk::DraftForReview;
    let description = format!("Incoming voice call from {}: {}", payload.caller_id, payload.transcript);

    match orchestrator.execute_action(
        DepartmentType::Operations,
        description,
        tenant_id,
        risk,
        serde_json::json!({
            "source": source,
            "caller_id": payload.caller_id,
            "transcript": payload.transcript,
            "inbox_message_id": id,
            "actions_taken": actions_str,
        }),
    ).await {
        Ok(req) => (StatusCode::OK, Json(VoiceWebhookResponse { success: true, request_id: Some(req.id) })).into_response(),
        Err(e) => {
            if e.contains("AI Budget exhausted") {
                return (StatusCode::TOO_MANY_REQUESTS, Json(VoiceWebhookResponse { success: false, request_id: None })).into_response();
            } else {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(VoiceWebhookResponse { success: false, request_id: None })).into_response();
            }
        }
    }
}
