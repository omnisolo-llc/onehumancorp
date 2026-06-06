use ::server_common::Claims;
use axum::{
    extract::{Extension, Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

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
    Extension(claims): Extension<Claims>,
    Path(locale): Path<String>,
    Extension(pool): Extension<Arc<PgPool>>,
) -> impl axum::response::IntoResponse {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    ::server_common::auth_utils::set_org_context(
        &mut *tx,
        claims.organization_id.as_deref().unwrap_or("SYSTEM"),
    )
    .await
    .map_err(|e| e.to_string())?;

    let rows = sqlx::query(
        "SELECT key, value FROM ohc_i18n_strings
         WHERE (tenant_id = $1 OR tenant_id = 'SYSTEM') AND locale = $2",
    )
    .bind(&claims.organization_id)
    .bind(locale)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let translations: Vec<I18nString> = rows
        .into_iter()
        .map(|r| {
            use sqlx::Row;
            I18nString {
                key: r.get("key"),
                value: r.get("value"),
            }
        })
        .collect();

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok::<_, String>(Json(translations))
}

pub async fn get_fx_rates(
    Extension(pool): Extension<Arc<PgPool>>,
) -> impl axum::response::IntoResponse {
    let rows = sqlx::query("SELECT from_currency, to_currency, rate FROM ohc_fx_rates")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    let rates: Vec<FxRateResponse> = rows
        .into_iter()
        .map(|r| {
            use sqlx::Row;
            FxRateResponse {
                from: r.get("from_currency"),
                to: r.get("to_currency"),
                rate: r.get("rate"),
            }
        })
        .collect();

    Ok::<_, String>(Json(rates))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalizedPricingRule {
    pub currency: String,
    pub strategy: String,
    pub charm_point: f64,
}

pub async fn get_pricing_rules(
    Extension(pool): Extension<Arc<PgPool>>,
    Extension(claims): Extension<Claims>,
) -> impl axum::response::IntoResponse {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    ::server_common::auth_utils::set_org_context(
        &mut *tx,
        claims.organization_id.as_deref().unwrap_or("SYSTEM"),
    )
    .await
    .map_err(|e| e.to_string())?;

    // Retrieve base currency and pricing strategies for the tenant
    // Since we don't have a specific `localized_pricing_rules` table in the migration,
    // we'll return a default set that honors the tenant's base currency from the `tenants` table.
    // In a real implementation this might fetch from a separate configuration table.
    let row = sqlx::query("SELECT tenant_base_currency FROM tenants WHERE id = $1")
        .bind(&claims.organization_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    let base_currency: String = if let Some(r) = row {
        use sqlx::Row;
        r.get("tenant_base_currency")
    } else {
        "USD".to_string()
    };

    let mut rules = Vec::new();
    rules.push(LocalizedPricingRule {
        currency: base_currency.clone(),
        strategy: "charming".to_string(),
        charm_point: 0.99,
    });
    if base_currency != "EUR" {
        rules.push(LocalizedPricingRule {
            currency: "EUR".to_string(),
            strategy: "charming".to_string(),
            charm_point: 0.95,
        });
    }

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok::<_, String>(Json(rules))
}

pub fn calculate_charming_price(amount_cents: i64, fx_rate: f64, charm_point: f64) -> i64 {
    let converted = amount_cents as f64 * fx_rate;
    let dollars = (converted / 100.0).floor();
    let _cents = converted % 100.0;

    let charm_cents = (charm_point * 100.0).round();

    ((dollars * 100.0) + charm_cents) as i64
}

pub fn router<S: Clone + Send + Sync + 'static>(pool: Arc<PgPool>) -> axum::Router<S> {
    axum::Router::new()
        .route(
            "/translations/:locale",
            axum::routing::get(get_translations),
        )
        .route("/fx-rates", axum::routing::get(get_fx_rates))
        .route("/pricing-rules", axum::routing::get(get_pricing_rules))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_charming_price() {
        assert_eq!(calculate_charming_price(1000, 1.482, 0.99), 1499);
        assert_eq!(calculate_charming_price(500, 1.2, 0.95), 695);
        assert_eq!(calculate_charming_price(1250, 1.0, 0.99), 1299);
    }
}
