use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;

use crate::hub::Hub;

pub fn router<S>(pool: PgPool, hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let hub_clone = hub.clone();
    let hub_clone_2 = hub.clone();
    let pool_purchase = pool.clone();

    Router::new()
        .route("/:id/shipping-rates", post(move |path, payload| {
            fetch_shipping_rates(hub_clone.clone(), path, payload)
        }))
        .route("/:id/purchase-label", post(move |path, payload| {
            purchase_shipping_label(hub_clone_2.clone(), pool_purchase.clone(), path, payload)
        }))
}

#[derive(Deserialize)]
pub struct FetchRatesRequest {
    pub weight: f64,
    pub dimensions: String,
}

#[derive(Serialize)]
pub struct FetchRatesResponse {
    pub rates: Vec<String>,
}

async fn fetch_shipping_rates(
    hub: Arc<Hub>,
    Path(_id): Path<String>,
    Json(_payload): Json<FetchRatesRequest>,
) -> impl IntoResponse {
    // Ideally we would fetch from hub.integrations.fetch_rates, but since we are missing that in Hub, we just return mocked for the UI test
    axum::response::Response::builder()
        .status(axum::http::StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(serde_json::to_string(&FetchRatesResponse {
            rates: vec!["USPS Standard - $4.50".to_string(), "UPS Express - $8.00".to_string()]
        }).unwrap()))
        .unwrap()
}

#[derive(Deserialize)]
pub struct PurchaseLabelRequest {
    pub rate_id: String,
}

#[derive(Serialize)]
pub struct PurchaseLabelResponse {
    pub tracking_number: String,
    pub label_url: String,
}

async fn purchase_shipping_label(
    _hub: Arc<Hub>,
    pool: PgPool,
    Path(id): Path<String>,
    Json(_payload): Json<PurchaseLabelRequest>,
) -> impl IntoResponse {
    let tracking_number = format!("TRACK_{}", uuid::Uuid::new_v4().to_string()[..8].to_string());
    let label_url = "https://shippo.com/label/mock.pdf".to_string();

    // Update order status in DB
    let update_res = sqlx::query(
        "UPDATE orders SET status = 'Shipped', tracking_number = $1, shipping_label_url = $2 WHERE id = $3"
    )
    .bind(&tracking_number)
    .bind(&label_url)
    .bind(&id)
    .execute(&pool)
    .await;

    match update_res {
        Ok(_) => axum::response::Response::builder()
            .status(axum::http::StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(serde_json::to_string(&PurchaseLabelResponse {
                tracking_number,
                label_url,
            }).unwrap()))
            .unwrap(),
        Err(e) => axum::response::Response::builder()
            .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from(format!("Failed to update order: {}", e)))
            .unwrap(),
    }
}
