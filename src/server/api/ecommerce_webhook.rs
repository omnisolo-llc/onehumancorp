use axum::{
    extract::{State, Json, Path},
    response::IntoResponse,
    routing::post,
    Router,
};
use reqwest::StatusCode;
use std::sync::Arc;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
use crate::db::get_pool;

#[derive(serde::Deserialize)]
pub struct ShopifyInventoryWebhook {
    pub inventory_item_id: i64,
    pub available: i32,
    pub location_id: i64,
}

#[derive(serde::Deserialize)]
pub struct SquareInventoryWebhook {
    pub type_name: String, // e.g. "inventory.count.updated"
    pub data: serde_json::Value,
}

#[derive(serde::Deserialize)]
pub struct WooInventoryWebhook {
    pub id: i64, // product id
    pub stock_quantity: i32,
}

#[derive(serde::Deserialize)]
pub struct ShopifyOrderWebhook {
    pub id: i64,
    pub financial_status: String,
    pub fulfillment_status: Option<String>,
    pub shipping_address: Option<serde_json::Value>,
}

pub fn router(orchestrator: Arc<DepartmentOrchestrator>) -> Router {
    Router::new()
        .route("/shopify", post(shopify_handler))
        .route("/shopify/order", post(shopify_order_handler))
        .route("/square", post(square_handler))
        .route("/woocommerce", post(woo_handler))
        .with_state(orchestrator)
}

async fn shopify_handler(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Json(payload): Json<ShopifyInventoryWebhook>,
) -> impl IntoResponse {
    let tenant_id = "tenant-priya".to_string(); // Temporary hardcoded for webhook testing, in reality verify signature or extract from URL
    let product_id = payload.inventory_item_id.to_string();
    let new_stock = payload.available;

    let _ = orchestrator.reconcile_inventory_conflict(&tenant_id, &product_id, new_stock, "Shopify").await;
    (StatusCode::OK, "OK")
}

async fn shopify_order_handler(
    State(_orchestrator): State<Arc<DepartmentOrchestrator>>,
    Json(payload): Json<ShopifyOrderWebhook>,
) -> impl IntoResponse {
    let tenant_id = "default".to_string();

    // Trigger logic: When an order is confirmed (paid) and unfulfilled, generate shipping label task
    if payload.financial_status == "paid" && (payload.fulfillment_status.is_none() || payload.fulfillment_status.as_deref() == Some("unfulfilled") || payload.fulfillment_status.as_deref() == Some("null")) {
        let _ = crate::services::agent_feed::service::AgentFeedService::new(crate::db::get_pool())
            .process_event(&tenant_id, "ecommerce_webhook", &serde_json::json!({
                "action_type": "Generate Shipping Label",
                "order_id": payload.id.to_string(),
                "address": payload.shipping_address
            })).await;
    }

    (StatusCode::OK, "OK")
}

async fn square_handler(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Json(payload): Json<SquareInventoryWebhook>,
) -> impl IntoResponse {
    let tenant_id = "tenant-priya".to_string();
    // Simplified parsing
    let product_id = "square-prod".to_string();
    let new_stock = 0;

    let _ = orchestrator.reconcile_inventory_conflict(&tenant_id, &product_id, new_stock, "Square").await;
    (StatusCode::OK, "OK")
}

async fn woo_handler(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Json(payload): Json<WooInventoryWebhook>,
) -> impl IntoResponse {
    let tenant_id = "tenant-priya".to_string();
    let product_id = payload.id.to_string();
    let new_stock = payload.stock_quantity;

    let _ = orchestrator.reconcile_inventory_conflict(&tenant_id, &product_id, new_stock, "WooCommerce").await;
    (StatusCode::OK, "OK")
}
