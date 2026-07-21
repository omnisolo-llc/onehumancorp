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

pub fn router(orchestrator: Arc<DepartmentOrchestrator>) -> Router {
    Router::new()
        .route("/shopify", post(shopify_handler))
        .route("/square", post(square_handler))
        .route("/woocommerce", post(woo_handler))
        .with_state(orchestrator)
}

async fn shopify_handler(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    headers: axum::http::header::HeaderMap,
    Json(payload): Json<ShopifyInventoryWebhook>,
) -> impl IntoResponse {
    let tenant_id = extract_tenant_from_headers(&headers).unwrap_or_default();
    if tenant_id.is_empty() {
        return (StatusCode::BAD_REQUEST, "Missing X-Tenant-Id header");
    }
    let product_id = payload.inventory_item_id.to_string();
    let new_stock = payload.available;

    let _ = orchestrator.reconcile_inventory_conflict(&tenant_id, &product_id, new_stock, "Shopify").await;
    (StatusCode::OK, "OK")
}

async fn square_handler(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    headers: axum::http::header::HeaderMap,
    Json(payload): Json<SquareInventoryWebhook>,
) -> impl IntoResponse {
    let tenant_id = extract_tenant_from_headers(&headers).unwrap_or_default();
    if tenant_id.is_empty() {
        return (StatusCode::BAD_REQUEST, "Missing X-Tenant-Id header");
    }
    let product_id = "square-prod".to_string();
    let new_stock = 0;

    let _ = orchestrator.reconcile_inventory_conflict(&tenant_id, &product_id, new_stock, "Square").await;
    (StatusCode::OK, "OK")
}

async fn woo_handler(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    headers: axum::http::header::HeaderMap,
    Json(payload): Json<WooInventoryWebhook>,
) -> impl IntoResponse {
    let tenant_id = extract_tenant_from_headers(&headers).unwrap_or_default();
    if tenant_id.is_empty() {
        return (StatusCode::BAD_REQUEST, "Missing X-Tenant-Id header");
    }
    let product_id = payload.id.to_string();
    let new_stock = payload.stock_quantity;

    let _ = orchestrator.reconcile_inventory_conflict(&tenant_id, &product_id, new_stock, "WooCommerce").await;
    (StatusCode::OK, "OK")
}

fn extract_tenant_from_headers(headers: &axum::http::header::HeaderMap) -> Option<String> {
    headers
        .get("X-Tenant-Id")
        .and_then(|v| v.to_str().ok())
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::header::HeaderMap;

    #[test]
    fn test_extract_tenant_from_valid_header() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Tenant-Id", "tenant-123".parse().unwrap());
        assert_eq!(extract_tenant_from_headers(&headers), Some("tenant-123".to_string()));
    }

    #[test]
    fn test_extract_tenant_missing_header() {
        let headers = HeaderMap::new();
        assert_eq!(extract_tenant_from_headers(&headers), None);
    }

    #[test]
    fn test_extract_tenant_empty_header() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Tenant-Id", "".parse().unwrap());
        assert_eq!(extract_tenant_from_headers(&headers), None);
    }

    #[test]
    fn test_extract_tenant_whitespace_header() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Tenant-Id", "  ".parse().unwrap());
        // Whitespace-only values should still be extracted (not filtered)
        assert_eq!(extract_tenant_from_headers(&headers), Some("  ".to_string()));
    }

    #[test]
    fn test_no_hardcoded_tenant() {
        // Verify the source code does not contain the old hardcoded tenant
        let source = include_str!("ecommerce_webhook.rs");
        assert!(!source.contains("tenant-priya"), "Hardcoded tenant 'tenant-priya' should have been removed");
    }
}
