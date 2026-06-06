use axum::{
    extract::{Path, State},
    Json,
    Router,
    routing::get,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalizationSyncResponse {
    pub locale: String,
    pub translations: std::collections::HashMap<String, String>,
    pub fx_rates: Vec<FxRateResponse>,
}

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/sync", get(sync_localization))
        .route("/translations/:locale", get(get_translations))
        .route("/fx_rates", get(get_fx_rates))
        .with_state(pool)
}

pub async fn sync_localization(
    claims: Claims,
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<LocalizationSyncResponse>, (axum::http::StatusCode, String)> {
    // Default locale for sync, could be passed as query param, assuming 'en' or user's default here
    // For this example, let's just fetch english as default or we could fetch from user settings.
    // Let's assume the locale is provided via a query param or default to 'en'
    // To keep it simple, we will return 'en'
    let locale = "en".to_string();

    let mut tx = pool.begin().await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &claims.organization_id).await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let i18n_rows = sqlx::query(
        "SELECT key, value FROM ohc_i18n_strings
         WHERE (tenant_id = $1 OR tenant_id = 'SYSTEM') AND locale = $2"
    )
    .bind(&claims.organization_id)
    .bind(&locale)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut translations = std::collections::HashMap::new();
    for r in i18n_rows {
        use sqlx::Row;
        translations.insert(r.get("key"), r.get("value"));
    }

    let fx_rows = sqlx::query("SELECT from_currency, to_currency, rate FROM ohc_fx_rates")
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let fx_rates = fx_rows.into_iter().map(|r| {
        use sqlx::Row;
        FxRateResponse {
            from: r.get("from_currency"),
            to: r.get("to_currency"),
            rate: r.get("rate"),
        }
    }).collect();

    tx.commit().await.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(LocalizationSyncResponse {
        locale,
        translations,
        fx_rates,
    }))
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
