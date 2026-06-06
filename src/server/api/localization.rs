use axum::{
    extract::{Path, State, Extension},
    Json,
    routing::get,
    Router,
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
pub struct PricingRuleResponse {
    pub tenant_id: String,
    pub tenant_base_currency: String,
    pub rounding_strategy: String,
}

pub fn router<S: Clone + Send + Sync + 'static>(pool: Arc<PgPool>) -> Router<S> {
    let internal_router = Router::new()
        .route("/pricing-rules", get(get_pricing_rules))
        .route("/fx-rates", get(get_fx_rates))
        .route("/translations/:locale", get(get_translations))
        .with_state(pool);

    Router::new().merge(internal_router)
}

pub async fn get_pricing_rules(
    Extension(claims): Extension<::server_common::Claims>,
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<PricingRuleResponse>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    ::server_common::auth_utils::set_org_context(&mut *tx, claims.organization_id.as_deref().unwrap_or("SYSTEM")).await.map_err(|e| e.to_string())?;

    let row = sqlx::query(
        "SELECT tenant_id, tenant_base_currency, rounding_strategy FROM ohc_localized_pricing_rules WHERE tenant_id = $1"
    )
    .bind(claims.organization_id.as_deref().unwrap_or("SYSTEM"))
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;

    match row {
        Some(r) => {
            use sqlx::Row;
            Ok(Json(PricingRuleResponse {
                tenant_id: r.get("tenant_id"),
                tenant_base_currency: r.get("tenant_base_currency"),
                rounding_strategy: r.get("rounding_strategy"),
            }))
        }
        None => {
            // Default response if no rule is set
            Ok(Json(PricingRuleResponse {
                tenant_id: claims.organization_id.unwrap_or_else(|| "SYSTEM".to_string()),
                tenant_base_currency: "USD".to_string(),
                rounding_strategy: "nearest_99".to_string(),
            }))
        }
    }
}

pub async fn get_translations(
    Extension(claims): Extension<::server_common::Claims>,
    State(pool): State<Arc<PgPool>>,
    Path(locale): Path<String>,
) -> Result<Json<Vec<I18nString>>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    ::server_common::auth_utils::set_org_context(&mut *tx, claims.organization_id.as_deref().unwrap_or("SYSTEM")).await.map_err(|e| e.to_string())?;

    let rows = sqlx::query(
        "SELECT key, value FROM ohc_i18n_strings
         WHERE (tenant_id = $1 OR tenant_id = 'SYSTEM') AND locale = $2"
    )
    .bind(claims.organization_id.as_deref().unwrap_or("SYSTEM"))
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

/// Applies charming pricing rounding strategy to a base amount and an FX rate.
/// `amount_cents` is the base price in cents.
/// Returns the converted amount in cents, rounded according to the strategy.
pub fn apply_charming_price(amount_cents: i64, rate: f64, strategy: &str) -> i64 {
    let converted = (amount_cents as f64 * rate).round() as i64;

    match strategy {
        "nearest_99" => {
            let cents = converted % 100;
            if cents == 99 {
                converted
            } else if cents < 50 {
                // Round down to the previous whole dollar and add 99 cents
                (converted / 100 - 1) * 100 + 99
            } else {
                // Round up to the current whole dollar and add 99 cents
                (converted / 100) * 100 + 99
            }
        }
        "nearest_00" => {
            let cents = converted % 100;
            if cents < 50 {
                // Round down to the nearest whole dollar
                (converted / 100) * 100
            } else {
                // Round up to the nearest whole dollar
                (converted / 100 + 1) * 100
            }
        }
        "nearest_95" => {
            let cents = converted % 100;
            if cents == 95 {
                converted
            } else if cents < 50 {
                // Round down to the previous whole dollar and add 95 cents
                (converted / 100 - 1) * 100 + 95
            } else {
                // Round up to the current whole dollar and add 95 cents
                (converted / 100) * 100 + 95
            }
        }
        _ => converted, // Exact, no charm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_charming_price_nearest_99() {
        // Exact converted: $14.82 (1482 cents) -> $14.99 (1499 cents)
        let res = apply_charming_price(1000, 1.482, "nearest_99");
        assert_eq!(res, 1499);

        let res2 = apply_charming_price(1499, 1.0, "nearest_99");
        assert_eq!(res2, 1499);

        // Exact converted: $14.01 (1401 cents) -> nearest .99 is $13.99
        let res3 = apply_charming_price(1000, 1.401, "nearest_99");
        assert_eq!(res3, 1399);

        // Exact converted: $14.50 (1450 cents) -> nearest .99 is $14.99
        let res4 = apply_charming_price(1000, 1.450, "nearest_99");
        assert_eq!(res4, 1499);
    }

    #[test]
    fn test_apply_charming_price_nearest_00() {
        // Exact converted: $14.82 (1482 cents) -> $15.00 (1500 cents)
        let res = apply_charming_price(1000, 1.482, "nearest_00");
        assert_eq!(res, 1500);

        // Exact converted: $14.20 (1420 cents) -> $14.00 (1400 cents)
        let res2 = apply_charming_price(1000, 1.420, "nearest_00");
        assert_eq!(res2, 1400);

        // Exact converted: $14.50 (1450 cents) -> $15.00 (1500 cents)
        let res3 = apply_charming_price(1000, 1.450, "nearest_00");
        assert_eq!(res3, 1500);
    }

    #[test]
    fn test_apply_charming_price_nearest_95() {
        // Exact converted: $14.82 (1482 cents) -> $14.95 (1495 cents)
        let res = apply_charming_price(1000, 1.482, "nearest_95");
        assert_eq!(res, 1495);

        // Exact converted: $14.01 (1401 cents) -> $13.95 (1395 cents)
        let res2 = apply_charming_price(1000, 1.401, "nearest_95");
        assert_eq!(res2, 1395);
    }

    #[test]
    fn test_apply_charming_price_exact() {
        // Fallback or exact
        let res = apply_charming_price(1000, 1.482, "exact");
        assert_eq!(res, 1482);
    }
}
