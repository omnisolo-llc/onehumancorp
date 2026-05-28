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


    // Fetch products and inventory for context
    let mut products_info = String::new();
    let pool = crate::db::get_pool();
    let rows = sqlx::query("SELECT id, name, price, inventory_count FROM products WHERE tenant_id = $1 OR organization_id = $1")
        .bind(&payload.tenant_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    if rows.is_empty() {
        products_info = "No products found in catalog.".to_string();
    } else {
        for row in rows {
            use sqlx::Row;
            let p_id: String = row.try_get("id").unwrap_or_default();
            let p_name: String = row.try_get("name").unwrap_or_default();
            let p_price: f64 = row.try_get("price").unwrap_or(0.0);
            let p_inv: i32 = row.try_get("inventory_count").unwrap_or(0);
            products_info.push_str(&format!("- {} (ID: {}): ${:.2} ({} in stock)\n", p_name, p_id, p_price, p_inv));
        }
    }

    // Generate a draft reply
    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
    let draft_reply = if !api_key.is_empty() {
        let business_context = "A friendly bakery that sells vegan celebration cakes and classes."; // mocked context
        let prompt = format!(
            "You are an autonomous sales agent for a business. Business context: {}. \nHere is the current catalog and inventory:\n{}\n\nThe customer has sent the following message: '{}'\n\nUnderstand the customer's request, negotiate if appropriate, and check if the requested items are in stock. If the customer wants to buy an available product, provide a secure payment link in the format: https://ohc.app/checkout?product_id=[ID]&tenant={} \n\nWrite a concise, warm reply in the customer's language. If they request human help, add [HUMAN_NEEDED] to your response.",
            business_context, products_info, payload.message, payload.tenant_id
        );
        let client = crate::minimax::MinimaxClient::new(api_key);
        client.reason(&prompt).await.unwrap_or_else(|_| "Draft generation failed.".to_string())
    } else {
        "Thank you for reaching out! We will get back to you shortly.".to_string()
    };
    let mut risk = ActionRisk::DraftForReview;
    let mut status = "pending".to_string();

    if draft_reply.contains("[HUMAN_NEEDED]") || draft_reply.to_lowercase().contains("human") || draft_reply.to_lowercase().contains("someone will get back to you") {
        status = "pending_human".to_string();
        // Route it strictly to human queue, perhaps risk is draft
        risk = ActionRisk::DraftForReview;
    }


    // Save to inbox_messages
    let id = Uuid::new_v4().to_string();

    let pool = crate::db::get_pool();
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
