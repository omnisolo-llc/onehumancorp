use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use server_common::auth::Claims;

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
) -> Result<Json<Vec<I18nString>>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    server_common::auth_utils::set_org_context(&mut *tx, &claims.organization_id).await.map_err(|e| e.to_string())?;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateRequestPayload {
    pub source_text: String,
    pub source_lang: String,
    pub target_lang: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateResponsePayload {
    pub translated_text: String,
    pub cached: bool,
}

pub async fn translate_text(
    claims: Claims,
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<TranslateRequestPayload>,
) -> Result<Json<TranslateResponsePayload>, String> {
    let service = server_services::translation::TranslationMeshService::new(pool);
    let (translated_text, cached) = service
        .translate(
            &claims.organization_id,
            &payload.source_text,
            &payload.source_lang,
            &payload.target_lang,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(Json(TranslateResponsePayload {
        translated_text,
        cached,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::State;
    use axum::Json;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_translate_text_api() {
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(5000), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return,
        };

        let claims = Claims {
            sub: "test_user".to_string(),
            email: "test@example.com".to_string(),
            organization_id: "test_tenant".to_string(),
            exp: 0,
            iat: 0,
        };

        let payload = TranslateRequestPayload {
            source_text: "Hello API".to_string(),
            source_lang: "en".to_string(),
            target_lang: "fr".to_string(),
        };

        let result = translate_text(claims, State(Arc::new(pool)), Json(payload)).await;
        assert!(result.is_ok());
        let res_payload = result.unwrap().0;
        assert_eq!(res_payload.translated_text, "Pending translation...");
        assert!(!res_payload.cached);
    }
}
