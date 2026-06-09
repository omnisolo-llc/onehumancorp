use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct OrderEvent {
    pub type_name: String, // e.g. "UPDATE_ORDER_STATUS"
    pub payload: serde_json::Value,
    pub timestamp: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct InventoryEvent {
    pub type_name: String, // e.g. "TOGGLE_SOLD_OUT"
    pub payload: serde_json::Value,
    pub timestamp: String,
}

pub async fn get_orders_handler(
    State(_db): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let _tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");

    // In a real app, we would query the database for orders here
    // But for this KDS example, let's just return a mock order
    let mock_orders = serde_json::json!([
        {
            "id": "1",
            "customer_name": "Ahmed",
            "status": "Received",
            "items": ["Chicken Over Rice"]
        }
    ]);

    (StatusCode::OK, Json(mock_orders)).into_response()
}

pub async fn post_orders_handler(
    State(_db): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    Json(_events): Json<Vec<serde_json::Value>>,
) -> impl IntoResponse {
    let _tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");

    // Process events (in a real app, update the DB)
    // For now, just return success

    (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
}

pub async fn delete_orders_handler(
    State(_db): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let _tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");

    (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
}

pub async fn get_inventory_handler(
    State(_db): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let _tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");

    let mock_inventory = serde_json::json!([
        {
            "id": "inv_1",
            "name_en": "Chicken Over Rice",
            "name_ar": "دجاج فوق الرز",
            "is_sold_out": false
        }
    ]);

    (StatusCode::OK, Json(mock_inventory)).into_response()
}

pub async fn post_inventory_handler(
    State(_db): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    Json(_events): Json<Vec<serde_json::Value>>,
) -> impl IntoResponse {
    let _tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");

    // Process events

    (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
}

pub async fn delete_inventory_handler(
    State(_db): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let _tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");

    (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
}
