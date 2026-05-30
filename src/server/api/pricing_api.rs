use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use serde_json::json;

use ::server_pricing::engine::{
    PricingEngine, DynamicPricingConfig, PricingContext, AdjustedPrice, PricingStrategy
};
use crate::db::DB;

pub fn router(db: DB) -> Router {
    Router::new()
        .route("/:product_id/price", get(get_dynamic_price))
        .route("/:product_id/config", post(update_pricing_config))
        .with_state(db)
}

async fn get_dynamic_price(
    State(db): State<DB>,
    Path(product_id): Path<String>,
) -> Json<serde_json::Value> {
    // 1. Fetch base price and config from DB
    // 2. Fetch context (mocked for now)
    // 3. Calculate adjusted price
    // 4. Return it

    // Mock data for demonstration
    let base_price_cents = 1000;

    let config = DynamicPricingConfig {
        enabled: true,
        min_price_cents: 800,
        max_price_cents: 1200,
        strategies: vec![PricingStrategy::ClearInventory, PricingStrategy::WeatherDemand],
    };

    let context = PricingContext {
        temperature_f: Some(95.0),
        is_raining: Some(false),
        inventory_velocity: Some(5.0),
        current_hour: Some(14),
        inventory_remaining: Some(10),
        closing_in_hours: Some(1),
    };

    let adjusted_price = PricingEngine::calculate_price(base_price_cents, &config, &context);

    Json(json!(adjusted_price))
}

async fn update_pricing_config(
    State(db): State<DB>,
    Path(product_id): Path<String>,
    Json(config): Json<DynamicPricingConfig>,
) -> Json<serde_json::Value> {
    // Update config in DB for the product
    Json(json!({"status": "success"}))
}
