use axum::{
    extract::{State, Json},
    response::IntoResponse,
    routing::post,
    Router,
};
use reqwest::StatusCode;
use std::sync::Arc;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;

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

pub fn router(orchestrator: Arc<DepartmentOrchestrator>) -> Router {
    Router::new()
        .route("/shopify", post(shopify_handler))
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

async fn square_handler(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Json(_payload): Json<SquareInventoryWebhook>,
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
