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

    let description = format!("Incoming message from {}: {}", payload.source, payload.message);

    // We route external messages (like DMs) to the Customer Success department
    let risk = ActionRisk::DraftForReview;

    // Generate a draft reply
    let mut business_context = "A friendly bakery that sells vegan celebration cakes and classes.".to_string();
    let pool = get_pool();
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response(),
    };
    if let Err(_e) = crate::common::auth_utils::set_org_context(&mut *tx, &payload.tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response();
    }

    // Attempt to fetch real business context using tenant_id for data isolation
    if let Ok(row) = sqlx::query("SELECT name, type FROM businesses WHERE tenant_id = $1")
        .bind(&payload.tenant_id)
        .fetch_one(&mut *tx)
        .await
    {
        use sqlx::Row;
        let name: String = row.get("name");
        let biz_type: String = row.get("type");
        business_context = format!("A '{}' business named '{}'", biz_type, name);
    }
    let _ = tx.commit().await;

    let prompt = format!(
        "Write one concise, warm customer-service reply. Business context: {} Customer message: {}",
        business_context, payload.message
    );

    let mut draft_reply = "Thank you for reaching out! We will get back to you shortly.".to_string();

    let is_e2e = std::env::var("OHC_E2E_TEST").unwrap_or_default() == "1";
    let minimax_api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();

    if is_e2e && !minimax_api_key.is_empty() {
        let client = crate::minimax::MinimaxClient::new(minimax_api_key.clone());
        draft_reply = client.reason(&prompt).await.unwrap_or_else(|_| "Draft generation failed.".to_string());
    } else {

    if let Ok(gemini_api_key) = std::env::var("GEMINI_API_KEY") {
        if !gemini_api_key.is_empty() {
            let client = reqwest::Client::new();
            let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-pro:generateContent?key={}", gemini_api_key);
            let payload = serde_json::json!({
                "contents": [{
                    "parts": [{"text": prompt}]
                }]
            });

            if let Ok(response) = client.post(&url).json(&payload).send().await {
                if let Ok(json) = response.json::<serde_json::Value>().await {
                    if let Some(text) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                        draft_reply = text.to_string();
                    }
                }
            }
        }
    }

    if draft_reply == "Thank you for reaching out! We will get back to you shortly." || draft_reply == "Draft generation failed." {
        if let Ok(openai_api_key) = std::env::var("OPENAI_API_KEY") {
            if !openai_api_key.is_empty() {
                let client = reqwest::Client::new();
                let url = "https://api.openai.com/v1/chat/completions";
                let payload = serde_json::json!({
                    "model": "gpt-4o-mini",
                    "messages": [{"role": "user", "content": prompt}]
                });

                if let Ok(response) = client.post(url)
                    .header("Authorization", format!("Bearer {}", openai_api_key))
                    .json(&payload)
                    .send().await {
                    if let Ok(json) = response.json::<serde_json::Value>().await {
                        if let Some(text) = json["choices"][0]["message"]["content"].as_str() {
                            draft_reply = text.to_string();
                        }
                    }
                }
            }
        }
    }

    if draft_reply == "Thank you for reaching out! We will get back to you shortly." || draft_reply == "Draft generation failed." {
        if !minimax_api_key.is_empty() {
            let client = crate::minimax::MinimaxClient::new(minimax_api_key);
            draft_reply = client.reason(&prompt).await.unwrap_or_else(|_| "Draft generation failed.".to_string());
        }
    }
    }

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
        Err(e) => {
            if e.contains("AI Budget exhausted") {
                return (StatusCode::TOO_MANY_REQUESTS, Json(WebhookResponse { success: false, request_id: None })).into_response();
            } else {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response();
            }
        }
    }
}
