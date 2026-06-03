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

    // Get tenant's preferred language
    let mut preferred_language = "en".to_string();
    {
        let pool = crate::db::get_pool();
        if let Ok(mut tx) = pool.begin().await {
            if let Ok(row) = sqlx::query("SELECT preferred_language FROM tenants WHERE tenant_id = $1")
                .bind(&payload.tenant_id)
                .fetch_optional(&mut *tx)
                .await
            {
                use sqlx::Row;
                if let Some(row) = row {
                    if let Ok(lang) = row.try_get("preferred_language") {
                        preferred_language = lang;
                    }
                }
            }
            let _ = tx.commit().await;
        }
    }

    let original_content = payload.message.clone();
    let mut content = payload.message.clone();

    // Generate a draft reply
    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
    let draft_reply = if !api_key.is_empty() {
        let client = crate::minimax::MinimaxClient::new(api_key.clone());
        let translation_prompt = format!(
            "Translate the following customer message to the language code or name '{}'. Message: {}. Only output the translated text.",
            preferred_language, payload.message
        );
        if let Ok(translated) = client.reason(&translation_prompt).await {
            if !translated.trim().is_empty() {
                content = translated;
            }
        }

        let business_context = "A friendly bakery that sells vegan celebration cakes and classes."; // mocked context
        let prompt = format!(
            "Write one concise, warm customer-service reply in the language code or name '{}'. Business context: {} Customer message: {}",
            preferred_language, business_context, content
        );
        let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);
        client.reason(&compressed_prompt).await.unwrap_or_else(|_| "Draft generation failed.".to_string())
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
        "INSERT INTO inbox_messages (id, tenant_id, source, content, original_content, draft_reply, status) VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(&id)
    .bind(&payload.tenant_id)
    .bind(&payload.source)
    .bind(&content)
    .bind(&original_content)
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
            "message": content,
            "original_content": original_content,
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
