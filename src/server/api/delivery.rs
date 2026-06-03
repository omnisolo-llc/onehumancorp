use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use crate::server::api::error::ApiError;
use crate::server::db::Pool;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliverySettings {
    pub enabled: bool,
    pub radius_miles: f64,
    pub flat_fee_cents: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteRequest {
    pub address: String,
    pub city: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub available: bool,
    pub fee_cents: i32,
    pub estimated_minutes: i32,
}

pub async fn get_settings(
    State(pool): State<Pool>,
    Path(org_id): Path<String>,
) -> Result<Json<DeliverySettings>, ApiError> {
    let row = sqlx::query!(
        "SELECT enabled, radius_miles, flat_fee_cents FROM doordash_delivery_settings WHERE organization_id = $1",
        org_id
    )
    .fetch_optional(&pool)
    .await?;

    match row {
        Some(r) => Ok(Json(DeliverySettings {
            enabled: r.enabled.unwrap_or(false),
            radius_miles: r.radius_miles.unwrap_or(5.0),
            flat_fee_cents: r.flat_fee_cents.unwrap_or(850),
        })),
        None => Ok(Json(DeliverySettings {
            enabled: false,
            radius_miles: 5.0,
            flat_fee_cents: 850,
        })),
    }
}

pub async fn update_settings(
    State(pool): State<Pool>,
    Path(org_id): Path<String>,
    Json(payload): Json<DeliverySettings>,
) -> Result<Json<DeliverySettings>, ApiError> {
    sqlx::query!(
        r#"
        INSERT INTO doordash_delivery_settings (organization_id, enabled, radius_miles, flat_fee_cents)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (organization_id) DO UPDATE
        SET enabled = $2, radius_miles = $3, flat_fee_cents = $4, updated_at = CURRENT_TIMESTAMP
        "#,
        org_id,
        payload.enabled,
        payload.radius_miles,
        payload.flat_fee_cents
    )
    .execute(&pool)
    .await?;

    Ok(Json(payload))
}

pub async fn get_quote(
    State(_pool): State<Pool>,
    Json(_req): Json<QuoteRequest>,
) -> Result<Json<QuoteResponse>, ApiError> {
    // In a real implementation, this would call the DoorDash Drive API
    // For now, return a successful mock quote
    Ok(Json(QuoteResponse {
        available: true,
        fee_cents: 850,
        estimated_minutes: 35,
    }))
}
