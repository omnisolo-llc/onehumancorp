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
use super::translation::{translate_inbox_message_with_llm, InboxTranslation};

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub tenant_id: String,
    pub message: String,
    pub source: String,
    #[serde(default)]
    pub target_language: Option<String>,
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
        .route("/", post(handle_webhook))
        .with_state(orchestrator)
}

async fn handle_webhook(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    // For incoming Stripe webhooks for new orders, route to Operations to process the order
    if payload.source == "stripe" && payload.message == "order_placed" {
        // Trigger SMS notification for new orders
        tokio::spawn(async move {
            let _ = crate::dispatch_critical_sms("new_order", "You have received a new order!").await;
        });

        let event = crate::orchestration::departments::types::DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: payload.tenant_id.clone(),
            event_type: "tenant.order.created".to_string(),
            payload: serde_json::json!({"source": payload.source, "message": payload.message}),
        };

        match orchestrator.dispatch_event(event).await {
            Ok(_) => return (StatusCode::OK, Json(WebhookResponse { success: true, request_id: None })).into_response(),
            Err(e) => {
                if e.contains("AI Budget exhausted") {
                    return (StatusCode::TOO_MANY_REQUESTS, Json(WebhookResponse { success: false, request_id: None })).into_response();
                } else {
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response();
                }
            }
        }
    }

    if payload.source == "mercadopago" {
        if payload.message == "approved" {
            tokio::spawn(async move {
                let _ = crate::dispatch_critical_sms("new_order", "You have received a new order!").await;
            });

            let event = crate::orchestration::departments::types::DepartmentEvent {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: payload.tenant_id.clone(),
                event_type: "tenant.order.created".to_string(),
                payload: serde_json::json!({"source": payload.source, "message": payload.message}),
            };

            match orchestrator.dispatch_event(event).await {
                Ok(_) => return (StatusCode::OK, Json(WebhookResponse { success: true, request_id: None })).into_response(),
                Err(e) => {
                    if e.contains("AI Budget exhausted") {
                        return (StatusCode::TOO_MANY_REQUESTS, Json(WebhookResponse { success: false, request_id: None })).into_response();
                    } else {
                        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response();
                    }
                }
            }
        } else if payload.message == "pending" || payload.message == "rejected" {
            return (StatusCode::OK, Json(WebhookResponse { success: true, request_id: None })).into_response();
        }
    }

    let description = format!("Incoming message from {}: {}", payload.source, payload.message);

    // We route external messages (like DMs) to the Customer Success department
    let risk = ActionRisk::DraftForReview;

    let target_language = payload.target_language.as_deref().unwrap_or("en");
    let translation = match translate_inbox_message_with_llm(
        &payload.tenant_id,
        &payload.source,
        &payload.message,
        target_language,
    )
    .await
    {
        Ok(translation) => translation,
        Err(e) => {
            ::server_telemetry::record_error_signal("Inbox translation failed");
            tracing::error!("Inbox translation failed: {}", e);
            return (StatusCode::SERVICE_UNAVAILABLE, Json(WebhookResponse { success: false, request_id: None })).into_response();
        }
    };

    let draft_reply = match generate_inbox_draft_reply(&payload.tenant_id, &payload.source, &translation).await {
        Ok(reply) => reply,
        Err(e) => {
            ::server_telemetry::record_error_signal("Inbox draft generation failed");
            tracing::error!("Inbox draft generation failed: {}", e);
            return (StatusCode::SERVICE_UNAVAILABLE, Json(WebhookResponse { success: false, request_id: None })).into_response();
        }
    };

    // Save to inbox_messages
    let id = Uuid::new_v4().to_string();
    let status = "pending";
    let sender_id = "unknown"; // default to unknown for raw webhooks
    let pool = get_pool();
    if let Ok(mut tx) = pool.begin().await {
        if crate::common::auth_utils::set_org_context(&mut *tx, &payload.tenant_id).await.is_ok() {
            let _ = sqlx::query(
                "INSERT INTO inbox_messages
                    (id, tenant_id, source, content, original_content, translated_from_language, draft_reply, status, sender_id)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
            )
            .bind(&id)
            .bind(&payload.tenant_id)
            .bind(&payload.source)
            .bind(&translation.translated_content)
            .bind(&translation.original_content)
            .bind(translation.source_language.as_deref())
            .bind(&draft_reply)
            .bind(&status)
            .bind(&sender_id)
            .execute(&mut *tx)
            .await;
            let _ = tx.commit().await;
        }
    }

    let res = orchestrator.execute_action(
        DepartmentType::CustomerSuccess,
        description,
        payload.tenant_id.clone(),
        risk,
        serde_json::json!({
            "source": payload.source.clone(),
            "message": translation.translated_content.clone(),
            "original_content": translation.original_content.clone(),
            "translated_from_language": translation.source_language.clone(),
            "draft_reply": draft_reply.clone(),
            "inbox_message_id": id.clone(),
            "sender_id": sender_id,
        }),
    ).await;

    let _ = orchestrator.dispatch_event(crate::orchestration::departments::types::DepartmentEvent {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: serde_json::json!({
            "source": payload.source,
            "message": translation.translated_content,
            "original_content": translation.original_content,
            "translated_from_language": translation.source_language,
            "draft_reply": draft_reply,
            "inbox_message_id": id,
            "sender_id": sender_id,
        }),
    }).await;

    match res {
        Ok(req) => (StatusCode::OK, Json(WebhookResponse { success: true, request_id: Some(req.id) })).into_response(),
        Err(e) => {
            if e.contains("AI Budget exhausted") {
                return (StatusCode::TOO_MANY_REQUESTS, Json(WebhookResponse { success: false, request_id: None })).into_response();
            } else {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response();
            }
        }
    }
}

async fn generate_inbox_draft_reply(
    tenant_id: &str,
    source: &str,
    translation: &InboxTranslation,
) -> Result<String, String> {
    let prompt = format!(
        "Write one concise, warm customer-service reply in {} for an omnichannel SMB inbox. Do not invent policies, availability, prices, or order state. Tenant: {tenant_id}. Source: {source}. Customer message: {}",
        translation.target_language,
        translation.translated_content
    );
    let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

    match std::env::var("OHC_INBOX_DRAFT_LLM_PROVIDER")
        .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
        .as_deref()
    {
        Ok("minimax") => {
            let api_key = std::env::var("MINIMAX_API_KEY")
                .map_err(|_| "MINIMAX_API_KEY is required for minimax inbox draft generation".to_string())?;
            crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt).await
        }
        _ => crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await,
    }
}
