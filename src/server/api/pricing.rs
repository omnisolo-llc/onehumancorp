use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

use crate::pricing::dynamic::engine::{AutonomousPricingEngine, DynamicPrice, LocalContext};
use crate::hub::Hub;

#[derive(Deserialize)]
pub struct EvaluatePriceQuery {
    pub item_id: String,
    pub item_type: String, // "product" or "booking"
    // Mocks for local context
    pub inventory_velocity: Option<String>,
    pub time_to_close_hours: Option<f64>,
    pub weather_demand_multiplier: Option<f64>,
}

pub async fn evaluate_dynamic_price_handler(
    State(pool): State<PgPool>,
    Query(query): Query<EvaluatePriceQuery>,
) -> Json<Option<DynamicPrice>> {
    let context = LocalContext {
        inventory_velocity: query.inventory_velocity.unwrap_or_else(|| "normal".to_string()),
        time_to_close_hours: query.time_to_close_hours,
        weather_demand_multiplier: query.weather_demand_multiplier,
    };

    let bounds_query = if query.item_type == "product" {
        sqlx::query_as::<_, (i64, Option<bool>, Option<i64>, Option<i64>)>(
            "SELECT price_cents, dynamic_pricing_enabled, min_price_cents, max_price_cents FROM products WHERE id = $1",
        )
        .bind(&query.item_id)
        .fetch_optional(&pool)
        .await
    } else {
        sqlx::query_as::<_, (i64, Option<bool>, Option<i64>, Option<i64>)>(
            "SELECT CAST(COALESCE(s.price * 100, 1000) AS INTEGER) as price_cents, b.dynamic_pricing_enabled, b.min_price_cents, b.max_price_cents FROM bookings b LEFT JOIN services s ON b.service_id = s.id WHERE b.id = $1",
        )
        .bind(&query.item_id)
        .fetch_optional(&pool)
        .await
    };

    match bounds_query {
        Ok(Some((base_price_cents, dynamic_pricing_enabled, min_price_cents, max_price_cents))) => {
            if dynamic_pricing_enabled.unwrap_or(false) {
                let min = min_price_cents.unwrap_or(base_price_cents);
                let max = max_price_cents.unwrap_or(base_price_cents);
                let result = AutonomousPricingEngine::evaluate_price(
                    base_price_cents,
                    min,
                    max,
                    context,
                );
                Json(Some(result))
            } else {
                Json(Some(DynamicPrice {
                    original_price_cents: base_price_cents,
                    adjusted_price_cents: base_price_cents,
                    reason: "Dynamic Pricing Disabled".to_string(),
                }))
            }
        }
        _ => Json(None),
    }
}

#[derive(Deserialize)]
pub struct ConfigurePricingRequest {
    pub item_id: String,
    pub item_type: String, // "product" or "booking"
    pub enabled: bool,
    pub min_price_cents: Option<i64>,
    pub max_price_cents: Option<i64>,
}

#[derive(Serialize)]
pub struct ConfigurePricingResponse {
    pub success: bool,
}

use axum::extract::Extension;
use server_common::Claims;

pub async fn configure_dynamic_pricing_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ConfigurePricingRequest>,
) -> Json<ConfigurePricingResponse> {
    // Assuming organization_id is the tenant boundary here. Fallback to sub if none (depends on the repo's auth model).
    // In this repo, tenant_id is often mapped to organization_id.
    let tenant_id = claims.organization_id.unwrap_or_else(|| claims.sub.clone());
    let result = if payload.item_type == "product" {
        sqlx::query(
            r#"
            UPDATE products
            SET dynamic_pricing_enabled = $1, min_price_cents = $2, max_price_cents = $3
            WHERE id = $4 AND tenant_id = $5
            "#,
        )
        .bind(payload.enabled)
        .bind(payload.min_price_cents)
        .bind(payload.max_price_cents)
        .bind(&payload.item_id)
        .bind(&tenant_id)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            r#"
            UPDATE bookings
            SET dynamic_pricing_enabled = $1, min_price_cents = $2, max_price_cents = $3
            WHERE id = $4 AND tenant_id = $5
            "#,
        )
        .bind(payload.enabled)
        .bind(payload.min_price_cents)
        .bind(payload.max_price_cents)
        .bind(&payload.item_id)
        .bind(&tenant_id)
        .execute(&pool)
        .await
    };

    match result {
        Ok(_) => Json(ConfigurePricingResponse { success: true }),
        Err(_) => Json(ConfigurePricingResponse { success: false }),
    }
}

pub fn router<S>(pool: PgPool, _hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route(
            "/dynamic/evaluate",
            get(evaluate_dynamic_price_handler).with_state(pool.clone()),
        )
        .route(
            "/dynamic/configure",
            post(configure_dynamic_pricing_handler).with_state(pool),
        )
}
