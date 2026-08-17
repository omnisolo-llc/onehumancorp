use axum::{
    Json, Router,
    extract::{Extension, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};

const SUPPORTED_CURRENCIES: [&str; 6] = ["USD", "EUR", "GBP", "CAD", "AUD", "JPY"];

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalCommerceSettings {
    pub base_currency: String,
    pub enabled_currencies: Vec<String>,
}

fn authenticated_tenant(claims: &::server_common::Claims) -> Result<&str, StatusCode> {
    claims
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|tenant_id| !tenant_id.is_empty())
        .ok_or(StatusCode::UNAUTHORIZED)
}

fn valid_settings(settings: &GlobalCommerceSettings) -> bool {
    let supported = |currency: &str| SUPPORTED_CURRENCIES.contains(&currency);
    !settings.enabled_currencies.is_empty()
        && settings.enabled_currencies.len() <= SUPPORTED_CURRENCIES.len()
        && supported(&settings.base_currency)
        && settings
            .enabled_currencies
            .iter()
            .all(|currency| supported(currency))
        && settings
            .enabled_currencies
            .iter()
            .filter(|currency| *currency == &settings.base_currency)
            .count()
            == 1
        && settings
            .enabled_currencies
            .iter()
            .enumerate()
            .all(|(index, currency)| !settings.enabled_currencies[..index].contains(currency))
}

async fn get_settings(
    State(pool): State<PgPool>,
    Extension(claims): Extension<::server_common::Claims>,
) -> Response {
    let tenant_id = match authenticated_tenant(&claims) {
        Ok(tenant_id) => tenant_id,
        Err(status) => return status.into_response(),
    };
    let row = sqlx::query("SELECT base_currency, enabled_currencies FROM tenants WHERE id = $1")
        .bind(tenant_id)
        .fetch_optional(&pool)
        .await;

    match row {
        Ok(Some(record)) => {
            let base_currency = record
                .try_get::<Option<String>, _>("base_currency")
                .ok()
                .flatten()
                .unwrap_or_else(|| "USD".to_string());
            let enabled_currencies = record
                .try_get::<Option<serde_json::Value>, _>("enabled_currencies")
                .ok()
                .flatten()
                .and_then(|value| serde_json::from_value(value).ok())
                .unwrap_or_else(|| vec![base_currency.clone()]);
            Json(serde_json::json!({
                "tenant": GlobalCommerceSettings { base_currency, enabled_currencies }
            }))
            .into_response()
        }
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            tracing::error!(error = %error, "global commerce settings lookup failed");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn update_settings(
    State(pool): State<PgPool>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(settings): Json<GlobalCommerceSettings>,
) -> Response {
    let tenant_id = match authenticated_tenant(&claims) {
        Ok(tenant_id) => tenant_id,
        Err(status) => return status.into_response(),
    };
    if !valid_settings(&settings) {
        return (StatusCode::BAD_REQUEST, "invalid currency settings").into_response();
    }
    let enabled = match serde_json::to_value(&settings.enabled_currencies) {
        Ok(enabled) => enabled,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };
    let result = sqlx::query(
        "UPDATE tenants SET base_currency = $1, enabled_currencies = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3",
    )
    .bind(settings.base_currency)
    .bind(enabled)
    .bind(tenant_id)
    .execute(&pool)
    .await;

    match result {
        Ok(result) if result.rows_affected() == 1 => {
            Json(serde_json::json!({ "success": true })).into_response()
        }
        Ok(_) => StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            tracing::error!(error = %error, "global commerce settings update failed");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    Router::new().route("/", get(get_settings).put(update_settings))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn currency_settings_are_bounded_and_include_the_base_once() {
        assert!(valid_settings(&GlobalCommerceSettings {
            base_currency: "EUR".to_string(),
            enabled_currencies: vec!["USD".to_string(), "EUR".to_string()],
        }));
        assert!(!valid_settings(&GlobalCommerceSettings {
            base_currency: "EUR".to_string(),
            enabled_currencies: vec!["USD".to_string()],
        }));
        assert!(!valid_settings(&GlobalCommerceSettings {
            base_currency: "USD".to_string(),
            enabled_currencies: vec!["USD".to_string(), "USD".to_string()],
        }));
        assert!(!valid_settings(&GlobalCommerceSettings {
            base_currency: "BTC".to_string(),
            enabled_currencies: vec!["BTC".to_string()],
        }));
    }
}
