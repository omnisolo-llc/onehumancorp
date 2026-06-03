use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use crate::common::auth::Claims;

#[derive(Debug, Serialize, Deserialize)]
pub struct I18nString {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FxRateResponse {
    pub from: String,
    pub to: String,
    pub rate: f64,
}

pub async fn get_translations(
    claims: Claims,
    State(pool): State<Arc<PgPool>>,
    Path(locale): Path<String>,
) -> Result<Json<Vec<I18nString>>, (axum::http::StatusCode, String)> {
    let mut tx = pool.begin().await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &claims.organization_id).await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let rows = sqlx::query(
        "SELECT key, value FROM ohc_i18n_strings
         WHERE (tenant_id = $1 OR tenant_id = 'SYSTEM') AND locale = $2"
    )
    .bind(&claims.organization_id)
    .bind(locale)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let translations = rows.into_iter().map(|r| {
        use sqlx::Row;
        I18nString {
            key: r.get("key"),
            value: r.get("value"),
        }
    }).collect();

    tx.commit().await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(translations))
}

pub async fn get_fx_rates(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<Vec<FxRateResponse>>, (axum::http::StatusCode, String)> {
    let rows = sqlx::query("SELECT from_currency, to_currency, rate FROM ohc_fx_rates")
        .fetch_all(&*pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let rates = rows.into_iter().map(|r| {
        use sqlx::Row;
        FxRateResponse {
            from: r.get("from_currency"),
            to: r.get("to_currency"),
            rate: r.get("rate"),
        }
    }).collect();

    Ok(Json(rates))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceRequest {
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceResponse {
    pub original_amount: f64,
    pub original_currency: String,
    pub converted_amount: f64,
    pub target_currency: String,
    pub rounded_amount: f64,
}

pub fn cosmetic_rounding(amount: f64) -> f64 {
    let int_part = amount.floor();
    let frac_part = amount - int_part;

    if frac_part < 0.25 {
        int_part
    } else if frac_part < 0.75 {
        int_part + 0.49
    } else {
        int_part + 0.99
    }
}

pub async fn localize_price(
    State(pool): State<Arc<PgPool>>,
    Path(target_currency): Path<String>,
    Json(payload): Json<PriceRequest>,
) -> Result<Json<PriceResponse>, (axum::http::StatusCode, String)> {
    if payload.currency == target_currency {
        return Ok(Json(PriceResponse {
            original_amount: payload.amount,
            original_currency: payload.currency.clone(),
            converted_amount: payload.amount,
            target_currency: target_currency.clone(),
            rounded_amount: payload.amount,
        }));
    }

    let row = sqlx::query("SELECT rate FROM ohc_fx_rates WHERE from_currency = $1 AND to_currency = $2")
        .bind(&payload.currency)
        .bind(&target_currency)
        .fetch_optional(&*pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let rate: f64 = match row {
        Some(r) => {
            use sqlx::Row;
            r.get("rate")
        }
        None => return Err((axum::http::StatusCode::NOT_FOUND, format!("No exchange rate found from {} to {}", payload.currency, target_currency))),
    };

    let converted = payload.amount * rate;
    let rounded = cosmetic_rounding(converted);

    Ok(Json(PriceResponse {
        original_amount: payload.amount,
        original_currency: payload.currency,
        converted_amount: converted,
        target_currency,
        rounded_amount: rounded,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosmetic_rounding() {
        assert_eq!(cosmetic_rounding(14.10), 14.00);
        assert_eq!(cosmetic_rounding(14.50), 14.49);
        assert_eq!(cosmetic_rounding(14.82), 14.99);
    }
}
/*
 * Manual Verification Steps (due to restricted test environment):
 * 1. Start the stack with `docker compose up --build`.
 * 2. Send a POST request to `/api/v1/pricing/localize/EUR` with JSON payload `{"amount": 54.00, "currency": "USD"}`.
 * 3. Verify that the response returns `{"original_amount": 54.0, "original_currency": "USD", "converted_amount": <converted_val>, "target_currency": "EUR", "rounded_amount": <rounded_val>}`.
 * 4. Verify that `rounded_amount` ends in `.00`, `.49`, or `.99` correctly via the cosmetic rounding strategy.
 */
