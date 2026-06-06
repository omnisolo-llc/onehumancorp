use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalizedPricingRuleResponse {
    pub id: String,
    pub tenant_id: String,
    pub rule_type: String,
    pub charm_point: String,
    pub target_currency: String,
}

pub async fn get_translations(
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    State(pool): State<Arc<PgPool>>,
    Path(locale): Path<String>,
) -> Result<Json<Vec<I18nString>>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &claims.organization_id.clone().unwrap_or_default()).await.map_err(|e| e.to_string())?;

    let rows = sqlx::query(
        "SELECT key, value FROM ohc_i18n_strings
         WHERE (tenant_id = $1 OR tenant_id = 'SYSTEM') AND locale = $2"
    )
    .bind(&claims.organization_id)
    .bind(locale)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let translations = rows.into_iter().map(|r| {
        use sqlx::Row;
        I18nString {
            key: r.get("key"),
            value: r.get("value"),
        }
    }).collect();

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(Json(translations))
}

pub async fn get_fx_rates(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<Vec<FxRateResponse>>, String> {
    let rows = sqlx::query("SELECT from_currency, to_currency, rate FROM ohc_fx_rates")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

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

pub async fn get_pricing_rules(
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<Vec<LocalizedPricingRuleResponse>>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &claims.organization_id.clone().unwrap_or_default()).await.map_err(|e| e.to_string())?;

    let rows = sqlx::query(
        "SELECT id, tenant_id, rule_type, charm_point, target_currency
         FROM ohc_localized_pricing_rules
         WHERE tenant_id = $1"
    )
    .bind(&claims.organization_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let rules = rows.into_iter().map(|r| {
        use sqlx::Row;
        LocalizedPricingRuleResponse {
            id: r.get("id"),
            tenant_id: r.get("tenant_id"),
            rule_type: r.get("rule_type"),
            charm_point: r.get("charm_point"),
            target_currency: r.get("target_currency"),
        }
    }).collect();

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(Json(rules))
}

pub fn apply_charming_price(amount: f64, charm_point: &str) -> f64 {
    let whole_part = amount.floor();
    match charm_point {
        ".99" => whole_part + 0.99,
        ".00" => amount.round(),
        ".95" => whole_part + 0.95,
        ".50" => whole_part + 0.50,
        _ => amount, // Fallback if no valid charm point
    }
}

pub fn convert_and_apply_charming_price(amount: f64, fx_rate: f64, charm_point: &str) -> f64 {
    let converted = amount * fx_rate;
    apply_charming_price(converted, charm_point)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_charming_price() {
        assert_eq!(apply_charming_price(10.12, ".99"), 10.99);
        assert_eq!(apply_charming_price(14.82, ".99"), 14.99);

        assert_eq!(apply_charming_price(10.12, ".00"), 10.00);
        assert_eq!(apply_charming_price(14.82, ".00"), 15.00);

        assert_eq!(apply_charming_price(10.12, ".95"), 10.95);
        assert_eq!(apply_charming_price(10.12, ".50"), 10.50);

        // Fallback case
        assert_eq!(apply_charming_price(10.12, "invalid"), 10.12);
    }

    #[test]
    fn test_convert_and_apply_charming_price() {
        // Convert $10 to EUR at 0.85 rate -> 8.5
        assert_eq!(convert_and_apply_charming_price(10.0, 0.85, ".99"), 8.99);
        assert_eq!(convert_and_apply_charming_price(10.0, 0.85, ".00"), 9.00);
    }
}
