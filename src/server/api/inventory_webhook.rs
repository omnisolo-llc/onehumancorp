use axum::{
    extract::{State},
    response::IntoResponse,
    http::{StatusCode, HeaderMap},
};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;
use crate::orchestration::departments::types::{DepartmentEvent};

#[derive(Clone)]
pub struct InventoryWebhookState {
    pub db: Arc<crate::db::DB>,
    pub orchestrator: Arc<crate::orchestration::departments::orchestrator::DepartmentOrchestrator>,
}

pub async fn shopify_inventory_webhook_handler(
    headers: HeaderMap,
    State(state): State<InventoryWebhookState>,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    let payload: Value = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let sku = payload.get("sku").and_then(|v| v.as_str()).unwrap_or("unknown");
    let tenant_id = headers.get("x-ohc-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");

    trigger_reconciliation(&state, tenant_id.to_string(), sku.to_string(), "Shopify").await;

    StatusCode::OK.into_response()
}

pub async fn square_inventory_webhook_handler(
    headers: HeaderMap,
    State(state): State<InventoryWebhookState>,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    let payload: Value = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let sku = payload.get("data").and_then(|d| d.get("object")).and_then(|o| o.get("inventory_adjustment")).and_then(|a| a.get("sku")).and_then(|v| v.as_str()).unwrap_or("unknown");
    let tenant_id = headers.get("x-ohc-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");

    trigger_reconciliation(&state, tenant_id.to_string(), sku.to_string(), "Square").await;

    StatusCode::OK.into_response()
}

pub async fn woocommerce_inventory_webhook_handler(
    headers: HeaderMap,
    State(state): State<InventoryWebhookState>,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    let payload: Value = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let sku = payload.get("sku").and_then(|v| v.as_str()).unwrap_or("unknown");
    let tenant_id = headers.get("x-ohc-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");

    trigger_reconciliation(&state, tenant_id.to_string(), sku.to_string(), "WooCommerce").await;

    StatusCode::OK.into_response()
}

async fn trigger_reconciliation(state: &InventoryWebhookState, tenant_id: String, sku: String, _source: &str) {
    let event = DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: tenant_id.clone(),
        event_type: "tenant.inventory.reconcile".to_string(),
        payload: serde_json::json!({
            "sku": sku,
            "trigger": "webhook_update"
        }),
    };

    let _ = state.orchestrator.dispatch_event(event).await;
}
