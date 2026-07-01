use axum::{
    extract::{Path, State},
    Extension,
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
    Extension(claims): Extension<Claims>,
    State(pool): State<Arc<PgPool>>,
    Path(locale): Path<String>,
) -> Result<Json<Vec<I18nString>>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    let org_id = claims.organization_id.unwrap_or_default();
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
    Extension(claims): Extension<Claims>,
    State(pool): State<Arc<PgPool>>,
    Path(product_id): Path<String>,
) -> Result<Json<Vec<I18nString>>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    let org_id = claims.organization_id.unwrap_or_default();
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
