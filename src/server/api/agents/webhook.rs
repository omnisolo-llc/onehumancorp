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
    pub customer_identifier: Option<String>,
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

    let _description = format!("Incoming message from {}: {}", payload.source, payload.message);

    let customer_id = payload.customer_identifier.unwrap_or_else(|| Uuid::new_v4().to_string());

    // Upsert Customer360 if they don't exist
    let customer_record = crate::orchestration::departments::types::Customer360 {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        customer_id: customer_id.clone(),
        email: None,
        phone: None,
        mood: None,
        preferences: None,
        created_at: None,
        updated_at: None,
    };
    let _ = orchestrator.upsert_customer360(&customer_record).await;

    // Save to inbox_messages
    let id = Uuid::new_v4().to_string();
    let status = "received";
    let pool = get_pool();
    if let Ok(mut tx) = pool.begin().await {
        if crate::common::auth_utils::set_org_context(&mut *tx, &payload.tenant_id).await.is_ok() {
            let _ = sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status) VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(&id)
            .bind(&payload.tenant_id)
            .bind(&payload.source)
            .bind(&payload.message)
            .bind("")
            .bind(&status)
            .execute(&mut *tx)
            .await;
            let _ = tx.commit().await;
        }
    }

    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        event_type: "tenant.message.received".to_string(),
        payload: serde_json::json!({
            "source": payload.source,
            "message": payload.message,
            "customer_id": customer_id,
            "inbox_message_id": id,
        }),
    };

    match orchestrator.dispatch_event(event).await {
        Ok(_) => (StatusCode::OK, Json(WebhookResponse { success: true, request_id: None })).into_response(),
        Err(e) => {
            if e.contains("AI Budget exhausted") {
                return (StatusCode::TOO_MANY_REQUESTS, Json(WebhookResponse { success: false, request_id: None })).into_response();
            } else {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response();
            }
        }
    }
}
