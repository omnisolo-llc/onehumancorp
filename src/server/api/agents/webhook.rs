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

#[derive(Clone)]
pub struct WebhookState {
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub queue: Arc<dyn crate::queue::TaskQueue>,
}

pub fn router<S>(state: WebhookState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_webhook))
        .with_state(state)
}

async fn handle_webhook(
    State(state): State<WebhookState>,
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

        let job = crate::queue::Job {
            id: Uuid::new_v4().to_string(),
            tenant_id: event.tenant_id.clone(),
            parent_task_id: "".to_string(),
            agent_role: "operations_agent".to_string(),
            payload: serde_json::to_string(&event).unwrap_or_default(),
            status: "QUEUED".to_string(),
            attempts: 0,
            max_attempts: 3,
            run_after: chrono::Utc::now(),
            locked_until: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let _ = state.queue.enqueue(job).await;
        return (StatusCode::OK, Json(WebhookResponse { success: true, request_id: None })).into_response();
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

            let job = crate::queue::Job {
                id: Uuid::new_v4().to_string(),
                tenant_id: event.tenant_id.clone(),
                parent_task_id: "".to_string(),
                agent_role: "operations_agent".to_string(),
                payload: serde_json::to_string(&event).unwrap_or_default(),
                status: "QUEUED".to_string(),
                attempts: 0,
                max_attempts: 3,
                run_after: chrono::Utc::now(),
                locked_until: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            let _ = state.queue.enqueue(job).await;
            return (StatusCode::OK, Json(WebhookResponse { success: true, request_id: None })).into_response();
        } else if payload.message == "pending" || payload.message == "rejected" {
            return (StatusCode::OK, Json(WebhookResponse { success: true, request_id: None })).into_response();
        }
    }

    let description = format!("Incoming message from {}: {}", payload.source, payload.message);

    // Save to inbox_messages to ack quickly
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
        "INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status) VALUES ($1, $2, $3, $4, '', $5)"
    )
    .bind(&id)
    .bind(&payload.tenant_id)
    .bind(&payload.source)
    .bind(&payload.message)
    .bind(&status)
    .execute(&mut *tx)
    .await;
    let _ = tx.commit().await;

    let payload_val = serde_json::json!({
        "type": "customer_message",
        "description": description,
        "payload": {
            "source": payload.source,
            "message": payload.message,
            "inbox_message_id": id,
        }
    });

    let job = crate::queue::Job {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        parent_task_id: "".to_string(),
        agent_role: "customer_success_agent".to_string(),
        payload: serde_json::to_string(&payload_val).unwrap_or_default(),
        status: "QUEUED".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: chrono::Utc::now(),
        locked_until: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let _ = state.queue.enqueue(job).await;

    (StatusCode::OK, Json(WebhookResponse { success: true, request_id: Some(id) })).into_response()
}
