use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::sync::RwLock;
use std::sync::Arc;
use ::server_common::Claims;
use crate::domain::repository::models::Order;

#[derive(Deserialize, Debug)]
pub struct CreateFoodOrderRequest {
    pub items: Vec<String>,
    pub total_amount: f64,
    pub pickup_time: Option<DateTime<Utc>>,
    pub customer_notes: Option<String>,
}

#[derive(Serialize)]
pub struct FoodOrderResponse {
    pub success: bool,
    pub order: Order,
}

#[derive(Serialize)]
pub struct ListFoodOrdersResponse {
    pub orders: Vec<Order>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateFoodOrderStatusRequest {
    pub order_id: String,
    pub status: String,
}

use sqlx::{Pool, Postgres};

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(create_food_order).get(list_food_orders))
        .route("/status", post(update_order_status))
}

async fn create_food_order(
    Extension(db): Extension<sqlx::Pool<sqlx::Postgres>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateFoodOrderRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "fatima_food_cart".to_string());
    let customer_id = "customer_1".to_string(); // Mock customer

    let translated_notes = payload.customer_notes.as_ref().map(|n| {
        if n.is_empty() {
            "".to_string()
        } else {
            // Simulated Operations Agent translation to Arabic
            format!("[AR] {}", n)
        }
    });

    let order_id = uuid::Uuid::new_v4().to_string();

    let query = r#"
        INSERT INTO orders (id, tenant_id, customer_id, total_amount, status, pickup_time, customer_notes, translated_notes, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())
        RETURNING id, tenant_id, customer_id, total_amount, status, payment_source, pickup_time, customer_notes, translated_notes, created_at, updated_at
    "#;

    match sqlx::query_as::<_, Order>(query)
        .bind(&order_id)
        .bind(&tenant_id)
        .bind(&customer_id)
        .bind(payload.total_amount)
        .bind("Received")
        .bind(payload.pickup_time)
        .bind(&payload.customer_notes)
        .bind(&translated_notes)
        .fetch_one(&db)
        .await
    {
        Ok(order) => {
            // Low stock detection
            for item in payload.items {
                // Mock: if item is "Falafel", pretend we update stock
                if item.to_lowercase().contains("falafel") {
                    let _ = sqlx::query("UPDATE products SET inventory_count = inventory_count - 1 WHERE tenant_id = $1 AND name = 'Falafel' AND inventory_count > 0")
                        .bind(&tenant_id)
                        .execute(&db).await;
                }
            }
            (StatusCode::CREATED, Json(FoodOrderResponse { success: true, order })).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to insert food order: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to create order"}))).into_response()
        }
    }
}

async fn list_food_orders(
    Extension(db): Extension<sqlx::Pool<sqlx::Postgres>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "fatima_food_cart".to_string());

    let query = "SELECT id, tenant_id, customer_id, total_amount, status, payment_source, pickup_time, customer_notes, translated_notes, created_at, updated_at FROM orders WHERE tenant_id = $1 ORDER BY created_at DESC";

    match sqlx::query_as::<_, Order>(query)
        .bind(&tenant_id)
        .fetch_all(&db)
        .await
    {
        Ok(orders) => (StatusCode::OK, Json(ListFoodOrdersResponse { orders })).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch food orders: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to fetch orders"}))).into_response()
        }
    }
}

async fn update_order_status(
    Extension(db): Extension<sqlx::Pool<sqlx::Postgres>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateFoodOrderStatusRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "fatima_food_cart".to_string());

    let query = "UPDATE orders SET status = $1, updated_at = NOW() WHERE id = $2 AND tenant_id = $3 RETURNING id, tenant_id, customer_id, total_amount, status, payment_source, pickup_time, customer_notes, translated_notes, created_at, updated_at";

    match sqlx::query_as::<_, Order>(query)
        .bind(&payload.status)
        .bind(&payload.order_id)
        .bind(&tenant_id)
        .fetch_one(&db)
        .await
    {
        Ok(order) => (StatusCode::OK, Json(FoodOrderResponse { success: true, order })).into_response(),
        Err(e) => {
            tracing::error!("Failed to update order status: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to update status"}))).into_response()
        }
    }
}
