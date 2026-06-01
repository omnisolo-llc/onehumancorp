use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use crate::integrations::registry::IntegrationsRegistry;

pub fn router(pool: PgPool, hub: Arc<IntegrationsRegistry>) -> Router {
    Router::new()
        .route("/:id/shipping-rates", post(fetch_shipping_rates))
        .route("/:id/purchase-label", post(purchase_shipping_label))
        .with_state(AppState { pool, hub })
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    hub: Arc<IntegrationsRegistry>,
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
    State(state): State<AppState>,
    Path(_id): Path<String>,
    Json(payload): Json<FetchRatesRequest>,
) -> impl IntoResponse {
    match state.hub.fetch_rates("shippo", payload.weight, &payload.dimensions).await {
        Ok(rates) => axum::response::Response::builder()
            .status(axum::http::StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(serde_json::to_string(&FetchRatesResponse { rates }).unwrap()))
            .unwrap(),
        Err(e) => axum::response::Response::builder()
            .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from(format!("Failed to fetch rates: {}", e)))
            .unwrap(),
    }
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
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<PurchaseLabelRequest>,
) -> impl IntoResponse {
    match state.hub.purchase_label("shippo", &payload.rate_id).await {
        Ok(label_url) => {
            // Generate a mock tracking number
            let tracking_number = format!("TRACK_{}", uuid::Uuid::new_v4().to_string()[..8].to_string());

            // Update order status in DB
            let update_res = sqlx::query(
                "UPDATE orders SET status = 'Shipped', tracking_number = $1, shipping_label_url = $2 WHERE id = $3"
            )
            .bind(&tracking_number)
            .bind(&label_url)
            .bind(&id)
            .execute(&state.pool)
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
        Err(e) => axum::response::Response::builder()
            .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::from(format!("Failed to purchase label: {}", e)))
            .unwrap(),
    }
}
