use axum::{
    Extension, Json, Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;

use crate::db::{DB, DbStore};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalCommerceSettings {
    pub base_currency: String,
    pub enabled_currencies: Vec<String>,
}

fn tenant_id(auth_info: &::server_auth::orchestration::AuthInfo) -> Result<&str, Response> {
    let tenant_id = auth_info.org_id.trim();
    if tenant_id.is_empty() || tenant_id.eq_ignore_ascii_case("system") {
        return Err((StatusCode::UNAUTHORIZED, "Missing tenant ID").into_response());
    }
    Ok(tenant_id)
}

fn normalize_settings(
    mut settings: GlobalCommerceSettings,
) -> Result<GlobalCommerceSettings, Response> {
    settings.base_currency = settings.base_currency.trim().to_ascii_uppercase();
    if settings.base_currency.len() != 3
        || !settings
            .base_currency
            .chars()
            .all(|character| character.is_ascii_uppercase())
    {
        return Err((
            StatusCode::BAD_REQUEST,
            "base_currency must be a three-letter ISO currency code",
        )
            .into_response());
    }

    let mut currencies = settings
        .enabled_currencies
        .into_iter()
        .map(|currency| currency.trim().to_ascii_uppercase())
        .filter(|currency| {
            currency.len() == 3
                && currency
                    .chars()
                    .all(|character| character.is_ascii_uppercase())
        })
        .collect::<Vec<_>>();
    if !currencies
        .iter()
        .any(|currency| currency == &settings.base_currency)
    {
        currencies.push(settings.base_currency.clone());
    }
    currencies.sort();
    currencies.dedup();
    if currencies.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "enabled_currencies must contain at least one currency",
        )
            .into_response());
    }
    settings.enabled_currencies = currencies;
    Ok(settings)
}

fn decode_currencies(value: Option<serde_json::Value>) -> Vec<String> {
    value
        .and_then(|value| serde_json::from_value::<Vec<String>>(value).ok())
        .filter(|values| !values.is_empty())
        .unwrap_or_else(|| vec!["USD".to_string()])
}

fn settings_response(base_currency: Option<String>, enabled_currencies: Vec<String>) -> Response {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "tenant": GlobalCommerceSettings {
                base_currency: base_currency.unwrap_or_else(|| "USD".to_string()),
                enabled_currencies,
            }
        })),
    )
        .into_response()
}

pub async fn get_settings(
    Extension(db): Extension<Arc<DB>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
) -> Response {
    let tenant_id = match tenant_id(&auth_info) {
        Ok(tenant_id) => tenant_id,
        Err(response) => return response,
    };

    if let Some(pool) = crate::db::get_mysql_pool_if_exists() {
        let row = sqlx::query(
            "SELECT base_currency, enabled_currencies FROM tenants WHERE id = ? LIMIT 1",
        )
        .bind(tenant_id)
        .fetch_optional(&pool)
        .await;
        return match row {
            Ok(Some(row)) => {
                let base_currency = row
                    .try_get::<Option<String>, _>("base_currency")
                    .ok()
                    .flatten()
                    .unwrap_or_else(|| "USD".to_string());
                let currencies = row
                    .try_get::<Option<serde_json::Value>, _>("enabled_currencies")
                    .ok()
                    .flatten()
                    .map(|value| decode_currencies(Some(value)))
                    .or_else(|| {
                        row.try_get::<Option<String>, _>("enabled_currencies")
                            .ok()
                            .flatten()
                            .and_then(|value| serde_json::from_str(&value).ok())
                    })
                    .unwrap_or_else(|| vec!["USD".to_string()]);
                settings_response(Some(base_currency), currencies)
            }
            Ok(None) => (StatusCode::NOT_FOUND, "Tenant not found").into_response(),
            Err(error) => {
                tracing::error!(%error, "failed to read MySQL global commerce settings");
                StatusCode::SERVICE_UNAVAILABLE.into_response()
            }
        };
    }

    match &db.store {
        DbStore::Postgres => {
            let row = sqlx::query(
                "SELECT base_currency, enabled_currencies FROM tenants WHERE id = $1 LIMIT 1",
            )
            .bind(tenant_id)
            .fetch_optional(&db.pool)
            .await;
            match row {
                Ok(Some(row)) => {
                    let base_currency = row
                        .try_get::<Option<String>, _>("base_currency")
                        .ok()
                        .flatten();
                    let currencies = row
                        .try_get::<Option<serde_json::Value>, _>("enabled_currencies")
                        .ok()
                        .flatten()
                        .map(|value| decode_currencies(Some(value)))
                        .unwrap_or_else(|| vec!["USD".to_string()]);
                    settings_response(base_currency, currencies)
                }
                Ok(None) => (StatusCode::NOT_FOUND, "Tenant not found").into_response(),
                Err(error) => {
                    tracing::error!(%error, "failed to read global commerce settings");
                    StatusCode::SERVICE_UNAVAILABLE.into_response()
                }
            }
        }
        DbStore::Sqlite(pool) => {
            let row = sqlx::query(
                "SELECT base_currency, enabled_currencies FROM tenants WHERE id = ? LIMIT 1",
            )
            .bind(tenant_id)
            .fetch_optional(pool)
            .await;
            match row {
                Ok(Some(row)) => {
                    let base_currency = row
                        .try_get::<Option<String>, _>("base_currency")
                        .ok()
                        .flatten();
                    let currencies = row
                        .try_get::<Option<String>, _>("enabled_currencies")
                        .ok()
                        .flatten()
                        .and_then(|value| serde_json::from_str(&value).ok())
                        .unwrap_or_else(|| vec!["USD".to_string()]);
                    settings_response(base_currency, currencies)
                }
                Ok(None) => (StatusCode::NOT_FOUND, "Tenant not found").into_response(),
                Err(error) => {
                    tracing::error!(%error, "failed to read global commerce settings");
                    StatusCode::SERVICE_UNAVAILABLE.into_response()
                }
            }
        }
    }
}

pub async fn update_settings(
    Extension(db): Extension<Arc<DB>>,
    Extension(auth_info): Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<GlobalCommerceSettings>,
) -> Response {
    let tenant_id = match tenant_id(&auth_info) {
        Ok(tenant_id) => tenant_id,
        Err(response) => return response,
    };
    let payload = match normalize_settings(payload) {
        Ok(payload) => payload,
        Err(response) => return response,
    };
    let enabled_json = match serde_json::to_string(&payload.enabled_currencies) {
        Ok(value) => value,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let result: Result<u64, sqlx::Error> = if let Some(pool) = crate::db::get_mysql_pool_if_exists()
    {
        sqlx::query("UPDATE tenants SET base_currency = ?, enabled_currencies = ? WHERE id = ?")
            .bind(&payload.base_currency)
            .bind(&enabled_json)
            .bind(tenant_id)
            .execute(&pool)
            .await
            .map(|result| result.rows_affected())
    } else {
        match &db.store {
            DbStore::Postgres => sqlx::query(
                "UPDATE tenants SET base_currency = $1, enabled_currencies = $2::jsonb WHERE id = $3",
            )
            .bind(&payload.base_currency)
            .bind(&enabled_json)
            .bind(tenant_id)
            .execute(&db.pool)
            .await
            .map(|result| result.rows_affected()),
            DbStore::Sqlite(pool) => sqlx::query(
                "UPDATE tenants SET base_currency = ?, enabled_currencies = ? WHERE id = ?",
            )
            .bind(&payload.base_currency)
            .bind(&enabled_json)
            .bind(tenant_id)
            .execute(pool)
            .await
            .map(|result| result.rows_affected()),
        }
    };

    match result {
        Ok(rows_affected) if rows_affected > 0 => (
            StatusCode::OK,
            Json(serde_json::json!({ "success": true, "tenant": payload })),
        )
            .into_response(),
        Ok(_) => (StatusCode::NOT_FOUND, "Tenant not found").into_response(),
        Err(error) => {
            tracing::error!(%error, "failed to update global commerce settings");
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
    }
}

pub fn router() -> Router {
    Router::new().route("/", get(get_settings).put(update_settings))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_are_normalized_and_always_include_the_base_currency() {
        let normalized = normalize_settings(GlobalCommerceSettings {
            base_currency: " eur ".to_string(),
            enabled_currencies: vec!["usd".to_string(), "EUR".to_string(), "usd".to_string()],
        })
        .expect("valid currencies normalize");
        assert_eq!(normalized.base_currency, "EUR");
        assert_eq!(normalized.enabled_currencies, vec!["EUR", "USD"]);
    }

    #[test]
    fn invalid_currency_codes_are_rejected() {
        assert!(
            normalize_settings(GlobalCommerceSettings {
                base_currency: "US".to_string(),
                enabled_currencies: vec!["USD".to_string()],
            })
            .is_err()
        );
    }
}
