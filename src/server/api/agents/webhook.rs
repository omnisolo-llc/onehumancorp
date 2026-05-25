use axum::{
    extract::{State, Json, Query},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
use uuid::Uuid;
use crate::db::get_pool;

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub tenant_id: String,
    pub message: String,
    pub source: String,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub request_id: Option<String>,
}

#[derive(Deserialize)]
pub struct MetaWebhookVerification {
    #[serde(rename = "hub.mode")]
    pub hub_mode: Option<String>,
    #[serde(rename = "hub.verify_token")]
    pub hub_verify_token: Option<String>,
    #[serde(rename = "hub.challenge")]
    pub hub_challenge: Option<String>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(verify_webhook).post(handle_webhook))
        .with_state(orchestrator)
}

async fn verify_webhook(
    Query(params): Query<MetaWebhookVerification>,
) -> impl IntoResponse {
    let verify_token = std::env::var("META_VERIFY_TOKEN").unwrap_or_else(|_| "default_token".to_string());

    if let (Some(mode), Some(token), Some(challenge)) = (params.hub_mode, params.hub_verify_token, params.hub_challenge) {
        if mode == "subscribe" && token == verify_token {
            return (StatusCode::OK, challenge).into_response();
        }
    }

    (StatusCode::FORBIDDEN, "Forbidden".to_string()).into_response()
}

async fn handle_webhook(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    // For incoming Stripe webhooks for new orders, route to Operations to process the order
    if payload.source == "stripe" && payload.message == "order_placed" {
        let event = crate::orchestration::departments::types::DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: payload.tenant_id.clone(),
            event_type: "tenant.quote.accepted".to_string(),
            payload: serde_json::json!({"source": payload.source, "message": payload.message}),
        };

        match orchestrator.dispatch_event(event).await {
            Ok(_) => return (StatusCode::OK, Json(WebhookResponse { success: true, request_id: None })).into_response(),
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response(),
        }
    }

    let description = format!("Incoming message from {}: {}", payload.source, payload.message);

    // We route external messages (like DMs) to the Customer Success department
    let risk = ActionRisk::DraftForReview;

    // Generate a draft reply
    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
    let draft_reply = if !api_key.is_empty() {
        let business_context = "A friendly bakery that sells vegan celebration cakes and classes."; // mocked context
        let prompt = format!(
            "Write one concise, warm customer-service reply. Business context: {} Customer message: {}",
            business_context, payload.message
        );
        let client = crate::minimax::MinimaxClient::new(api_key);
        client.reason(&prompt).await.unwrap_or_else(|_| "Draft generation failed.".to_string())
    } else {
        "Thank you for reaching out! We will get back to you shortly.".to_string()
    };

    // Save to inbox_messages
    let id = Uuid::new_v4().to_string();
    let status = "pending";
    let pool = get_pool();
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response(),
    };
    if let Err(_e) = crate::common::auth_utils::set_org_context(&mut *tx, &payload.tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response();
    }
    let _ = sqlx::query(
        "INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status) VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(&id)
    .bind(&payload.tenant_id)
    .bind(&payload.source)
    .bind(&payload.message)
    .bind(&draft_reply)
    .bind(&status)
    .execute(&mut *tx)
    .await;
    let _ = tx.commit().await;

    match orchestrator.execute_action(
        DepartmentType::CustomerSuccess,
        description,
        payload.tenant_id,
        risk,
        serde_json::json!({
            "source": payload.source,
            "message": payload.message,
            "draft_reply": draft_reply,
            "inbox_message_id": id,
        }),
    ).await {
        Ok(req) => (StatusCode::OK, Json(WebhookResponse { success: true, request_id: Some(req.id) })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response(),
    }
}
