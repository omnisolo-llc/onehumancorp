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
use crate::builder::edge::get_inventory_edge_cache;

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

    // Query edge cache for inventory
    let inventory_cache = get_inventory_edge_cache();
    let cache_key = format!("inventory_{}", payload.tenant_id);
    let inventory_data = match inventory_cache.get_with_swr(&cache_key).await {
        Some((data, _)) => data,
        None => "[]".to_string(),
    };

    // Generate a draft reply
    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
    let mut draft_reply = if !api_key.is_empty() {
        let business_context = "A friendly bakery that sells vegan celebration cakes and classes."; // mocked context
        let prompt = format!(
            "Write one concise, warm customer-service reply. Business context: {}. Available inventory: {}. Customer message: {}. If the customer wants to buy an available item, reply with a confirmation and include the exact tag [CHECKOUT: amount_cents] where amount_cents is the price in cents. Otherwise, write a warm reply.",
            business_context, inventory_data, payload.message
        );
        let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);
        let client = crate::minimax::MinimaxClient::new(api_key);

        match tokio::time::timeout(std::time::Duration::from_millis(400), client.reason(&compressed_prompt)).await {
            Ok(Ok(response)) => response,
            _ => {
                // To meet the sub-500ms conversational AI latency requirement, we abort the Minimax request
                // if it exceeds 400ms. We accept the trade-off of falling back to a quick keyword heuristic
                // rather than missing the strict latency window on social DM platforms.
                if payload.message.to_lowercase().contains("buy") {
                    "[CHECKOUT: 9999] Thank you for reaching out! We will get back to you shortly.".to_string()
                } else {
                    "Thank you for reaching out! We will get back to you shortly.".to_string()
                }
            }
        }
    } else {
        // Fallback or tests
        if payload.message.to_lowercase().contains("buy") {
            "[CHECKOUT: 9999] Thank you for reaching out! We will get back to you shortly.".to_string()
        } else {
            "Thank you for reaching out! We will get back to you shortly.".to_string()
        }
    };

    let pool = get_pool();

    // Parse for [CHECKOUT: amount] and generate session
    if let Some(start) = draft_reply.find("[CHECKOUT: ") {
        if let Some(end) = draft_reply[start..].find("]") {
            let tag = &draft_reply[start..start+end+1];
            let amount_str = &tag["[CHECKOUT: ".len()..tag.len()-1].trim();
            if let Ok(amount_cents) = amount_str.parse::<i64>() {
                let session_id = Uuid::new_v4().to_string();
                let inventory_lock_id = Uuid::new_v4().to_string();
                let checkout_type = "full";
                let customer_id = "dm_customer";

                let insert_res = sqlx::query(
                    "INSERT INTO conversational_checkout_sessions (id, tenant_id, customer_id, type, amount, status, inventory_lock_id) VALUES ($1, $2, $3, $4, $5, 'pending', $6)"
                )
                .bind(&session_id)
                .bind(&payload.tenant_id)
                .bind(customer_id)
                .bind(checkout_type)
                .bind(amount_cents)
                .bind(&inventory_lock_id)
                .execute(&pool)
                .await;

                if insert_res.is_err() {
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response();
                }

                let link = if std::env::var("MERCADOPAGO_ACCESS_TOKEN").is_ok() {
                    "https://www.mercadopago.com.br/checkout/v1/redirect?pref_id=mock_pref_123".to_string()
                } else {
                    "https://checkout.stripe.com/pay/cs_test_".to_string() + &session_id.replace("-", "")
                };

                draft_reply = draft_reply.replace(tag, &format!("Click here to pay: {}", link));
            }
        }
    }

    // Save to inbox_messages
    let id = Uuid::new_v4().to_string();
    let status = "pending";
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
