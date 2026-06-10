use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use ::server_common::Claims;

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
    axum::extract::Extension(claims): axum::extract::Extension<Claims>,
    State(pool): State<Arc<PgPool>>,
    Path(locale): Path<String>,
) -> Result<Json<Vec<I18nString>>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    let org_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    ::server_common::auth_utils::set_org_context(&mut *tx, &org_id).await.map_err(|e| e.to_string())?;

    let rows = sqlx::query(
        "SELECT key, value FROM ohc_i18n_strings
         WHERE (tenant_id = $1 OR tenant_id = 'SYSTEM') AND locale = $2"
    )
    .bind(&org_id)
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

pub async fn get_product_translations(
    axum::extract::Extension(claims): axum::extract::Extension<Claims>,
    State(pool): State<Arc<PgPool>>,
    Path(product_id): Path<String>,
) -> Result<Json<Vec<I18nString>>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    let org_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    ::server_common::auth_utils::set_org_context(&mut *tx, &org_id).await.map_err(|e| e.to_string())?;

    let prefix = format!("product:{}:", product_id);
    let rows = sqlx::query(
        "SELECT locale, key, value FROM ohc_i18n_strings
         WHERE tenant_id = $1 AND key LIKE $2"
    )
    .bind(&org_id)
    .bind(format!("{}%", prefix))
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let translations = rows.into_iter().map(|r| {
        use sqlx::Row;
        let locale: String = r.get("locale");
        let key: String = r.get("key");
        // format is: <locale>:<key> to let client know
        I18nString {
            key: format!("{}:{}", locale, key),
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

pub fn router<S: Clone + Send + Sync + 'static>(pool: Arc<PgPool>) -> axum::Router<S> {
    axum::Router::new()
        .route("/fx-rates", axum::routing::get(get_fx_rates))
        .route("/currency-config", axum::routing::get(get_currency_config))
        .route("/translations/:locale", axum::routing::get(get_translations))
        .route("/product/:product_id/translations", axum::routing::get(get_product_translations))
        .with_state(pool)
}

pub async fn get_currency_config(
    axum::extract::Extension(claims): axum::extract::Extension<Claims>,
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<serde_json::Value>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    let org_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    ::server_common::auth_utils::set_org_context(&mut *tx, &org_id).await.map_err(|e| e.to_string())?;

    let row = sqlx::query(
        "SELECT base_currency, supported_currencies FROM tenant_currency_configs WHERE tenant_id = $1"
    )
    .bind(&org_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let config = if let Some(r) = row {
        use sqlx::Row;
        serde_json::json!({
            "base_currency": r.get::<String, _>("base_currency"),
            "supported_currencies": r.get::<serde_json::Value, _>("supported_currencies")
        })
    } else {
        serde_json::json!({
            "base_currency": "USD",
            "supported_currencies": ["USD"]
        })
    };

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(Json(config))
}
