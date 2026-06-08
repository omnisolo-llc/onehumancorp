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
use crate::minimax::{LocalLLMClient, MinimaxClient};

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

async fn analyze_intake_inquiry(inquiry: &str) -> Result<(f64, String, String), String> {
    let prompt = format!(
        "Analyze the following client intake inquiry and extract the key project parameters. Provide a suggested price (as a number), a short service name (e.g., 'Logo Refresh'), and a brief scope description.\n\nInquiry: {}\n\nRespond with a JSON object containing keys: 'suggested_price' (number), 'service_name' (string), and 'scope' (string).",
        inquiry
    );

    let raw_response = match std::env::var("OHC_SALES_LLM_PROVIDER").or_else(|_| std::env::var("OHC_LLM_PROVIDER")).as_deref() {
        Ok("minimax") => {
            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            if api_key.trim().is_empty() {
                LocalLLMClient::new().reason(&prompt).await
            } else {
                MinimaxClient::new(api_key).reason(&prompt).await
            }
        },
        _ => LocalLLMClient::new().reason(&prompt).await,
    }?;

    let parsed: serde_json::Value = serde_json::from_str(&raw_response).unwrap_or(serde_json::json!({}));

    let price = parsed.get("suggested_price").and_then(|v| v.as_f64()).unwrap_or(1500.0);
    let name = parsed.get("service_name").and_then(|v| v.as_str()).unwrap_or("Custom Project Scope").to_string();
    let scope = parsed.get("scope").and_then(|v| v.as_str()).unwrap_or("Custom requirements based on inquiry.").to_string();

    Ok((price, name, scope))
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
            let _ = orchestrator.dispatch_event(event).await;
        }
        return (StatusCode::OK, Json(WebhookResponse { success: true, request_id: None })).into_response();
    }

    // Client intake flow via email or form webhook
    if payload.source == "intake_form" || payload.source == "email_inquiry" {
        let tenant_id = payload.tenant_id.clone();
        let inquiry = payload.message.clone();

        let (suggested_price, service_name, scope) = match analyze_intake_inquiry(&inquiry).await {
            Ok(res) => res,
            Err(_) => (1500.00, "Custom Project Scope".to_string(), "Custom requirements based on inquiry.".to_string()),
        };

        // Generate embedding for memory query
        let prompt_for_embedding = format!("Past proposals for {}", service_name);
        let query_embedding = match std::env::var("OHC_SALES_LLM_PROVIDER").or_else(|_| std::env::var("OHC_LLM_PROVIDER")).as_deref() {
            Ok("minimax") => {
                let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                if api_key.trim().is_empty() {
                    LocalLLMClient::new().generate_embedding(&prompt_for_embedding).await
                } else {
                    MinimaxClient::new(api_key).generate_embedding(&prompt_for_embedding).await
                }
            },
            _ => LocalLLMClient::new().generate_embedding(&prompt_for_embedding).await,
        }.unwrap_or_else(|_| vec![0.0; 1536]);

        let context = match orchestrator.query_long_term_memory(&tenant_id, &query_embedding, 5).await {
            Ok(c) => c,
            Err(_) => vec![],
        };

        let drafted_message = format!(
            "Hi there! Based on your request for '{}', I've put together a drafted proposal. The estimated scope will cost around ${}, including standard services.",
            inquiry, suggested_price
        );

        let action_payload = serde_json::json!({
            "feature_type": "quote_draft",
            "customer_inquiry": inquiry,
            "suggested_price": suggested_price,
            "scope": scope,
            "suggested_time": "Next Week",
            "generated_response": drafted_message,
            "service": service_name,
            "price": suggested_price,
            "context": context,
        });

        match orchestrator.execute_action(
            DepartmentType::Sales,
            format!("Draft proposal for new intake: {}", service_name),
            tenant_id,
            ActionRisk::DraftForReview,
            action_payload,
        ).await {
            Ok(_) => return (StatusCode::OK, Json(WebhookResponse { success: true, request_id: None })).into_response(),
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response(),
        }
    }

    let translation = match translate_inbox_message_with_llm(
        &payload.tenant_id,
        &payload.source,
        &payload.message,
        payload.target_language.as_deref().unwrap_or("English"),
    ).await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Translation failed: {}", e);
            InboxTranslation {
                translated_content: payload.message.clone(),
                source_language: Some("Unknown".to_string()),
                target_language: payload.target_language.unwrap_or_else(|| "English".to_string()),
                original_content: payload.message.clone(),
            }
        }
    };

    let draft_reply = match generate_inbox_draft_reply(&payload.tenant_id, &payload.source, &translation).await {
        Ok(d) => d,
        Err(e) => {
            tracing::error!("Failed to generate draft reply: {}", e);
            "Thanks for reaching out! We will review this and get back to you soon.".to_string()
        }
    };

    let pool = get_pool();
    let id = Uuid::new_v4().to_string();
    let _ = sqlx::query(
        r#"
        INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, source_language, target_language, draft_reply, status, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'unread', NOW(), NOW())
        "#
    )
    .bind(&id)
    .bind(&payload.tenant_id)
    .bind(&payload.source)
    .bind(&translation.original_content)
    .bind(&translation.translated_content)
    .bind(&translation.source_language)
    .bind(&translation.target_language)
    .bind(&draft_reply)
    .execute(&pool)
    .await;

    let res = orchestrator.execute_action(
        DepartmentType::CustomerSuccess,
        format!("New {} message from {} (Language: {:?})", payload.source, payload.tenant_id, translation.source_language),
        payload.tenant_id.clone(),
        ActionRisk::DraftForReview,
        serde_json::json!({
            "source": payload.source.clone(),
            "message": translation.translated_content.clone(),
            "original_content": translation.original_content.clone(),
            "translated_from_language": translation.source_language.clone(),
            "draft_reply": draft_reply.clone(),
            "inbox_message_id": id.clone(),
        }),
    ).await;

    let _ = orchestrator.dispatch_event(crate::orchestration::departments::types::DepartmentEvent {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: serde_json::json!({
            "source": payload.source,
            "message": translation.translated_content,
            "original_message": translation.original_content,
            "translated_from_language": translation.source_language,
            "generated_response": draft_reply,
            "feature_type": "ambassador_reply",
            "inbox_message_id": id,
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
