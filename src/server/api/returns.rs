use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::PgPool;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;
use serde_json::Value;

#[derive(Clone)]
pub struct ReturnsState {
    pub db_pool: PgPool,
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

#[derive(Deserialize)]
pub struct InitiateReturnRequest {
    pub tenant_id: String,
    pub order_id: String,
    pub reason: String,
}

#[derive(Serialize)]
pub struct InitiateReturnResponse {
    pub success: bool,
    pub return_id: Option<String>,
}

#[derive(Deserialize)]
pub struct ApproveReturnRequest {
    pub return_id: String,
    pub tenant_id: String,
    pub approved: bool,
}

#[derive(Deserialize)]
pub struct CarrierWebhookPayload {
    pub tracking_number: String,
    pub return_id: String,
    pub status: String,
}

pub fn router<S>(state: ReturnsState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/initiate", post(initiate_return))
        .route("/approve", post(approve_return))
        .route("/webhook/carrier_scan", post(carrier_scan_webhook))
        .with_state(state)
}

pub async fn initiate_return(
    State(state): State<ReturnsState>,
    Json(payload): Json<InitiateReturnRequest>,
) -> impl IntoResponse {
    let return_id = Uuid::new_v4().to_string();

    let res = sqlx::query(
        "INSERT INTO return_requests (id, tenant_id, order_id, reason, status, created_at) VALUES ($1, $2, $3, $4, 'pending', CURRENT_TIMESTAMP)"
    )
    .bind(&return_id)
    .bind(&payload.tenant_id)
    .bind(&payload.order_id)
    .bind(&payload.reason)
    .execute(&state.db_pool)
    .await;

    if res.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(InitiateReturnResponse { success: false, return_id: None })).into_response();
    }

    let event = DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        event_type: "tenant.return.requested".to_string(),
        payload: serde_json::json!({
            "return_id": return_id,
            "order_id": payload.order_id,
            "reason": payload.reason,
            "feature_type": "return_request"
        }),
    };

    let _ = state.orchestrator.dispatch_event(event).await;

    (StatusCode::OK, Json(InitiateReturnResponse { success: true, return_id: Some(return_id) })).into_response()
}

pub async fn approve_return(
    State(state): State<ReturnsState>,
    Json(payload): Json<ApproveReturnRequest>,
) -> impl IntoResponse {
    let new_status = if payload.approved { "approved" } else { "rejected" };

    let res = sqlx::query("UPDATE return_requests SET status = $1 WHERE id = $2 AND tenant_id = $3")
        .bind(new_status)
        .bind(&payload.return_id)
        .bind(&payload.tenant_id)
        .execute(&state.db_pool)
        .await;

    if res.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update return request").into_response();
    }

    (StatusCode::OK, "OK").into_response()
}

pub async fn carrier_scan_webhook(
    State(state): State<ReturnsState>,
    Json(payload): Json<CarrierWebhookPayload>,
) -> impl IntoResponse {
    if payload.status == "in_transit" || payload.status == "delivered" {
        // Find the return request and associated order items
        // For simplicity in this orchestrated test, we will assume order items exist and we restock.
        // We will query the order to find products or just assume standard restocking logic.

        let row = sqlx::query("SELECT tenant_id, order_id FROM return_requests WHERE id = $1")
            .bind(&payload.return_id)
            .fetch_optional(&state.db_pool)
            .await;

        if let Ok(Some(r)) = row {
            use sqlx::Row;
            let tenant_id: String = r.get("tenant_id");
            let order_id: String = r.get("order_id");

            let items = sqlx::query("SELECT product_id, quantity FROM order_items WHERE order_id = $1 AND tenant_id = $2")
                .bind(&order_id)
                .bind(&tenant_id)
                .fetch_all(&state.db_pool)
                .await
                .unwrap_or(vec![]);

            for item_row in items {
                let pid: String = item_row.get("product_id");
                let qty: i32 = item_row.get("quantity");

                let _ = sqlx::query("UPDATE products SET inventory_count = inventory_count + $1 WHERE id = $2 AND tenant_id = $3")
                    .bind(qty)
                    .bind(&pid)
                    .bind(&tenant_id)
                    .execute(&state.db_pool)
                    .await;
            }

            let _ = sqlx::query("UPDATE return_requests SET status = $1 WHERE id = $2")
                .bind(&payload.status)
                .bind(&payload.return_id)
                .execute(&state.db_pool)
                .await;

            let event = DepartmentEvent {
                id: Uuid::new_v4().to_string(),
                tenant_id: tenant_id.clone(),
                event_type: "return.package.scanned".to_string(),
                payload: serde_json::json!({
                    "return_id": payload.return_id,
                    "order_id": order_id,
                    "tracking_number": payload.tracking_number,
                    "status": payload.status,
                    "payment_intent_id": format!("pi_generated_for_{}", order_id) // Placeholder for real DB lookup
                }),
            };

            let _ = state.orchestrator.dispatch_event(event).await;
        }
    }

    (StatusCode::OK, "Webhook Processed").into_response()
}
