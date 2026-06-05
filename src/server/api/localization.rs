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
) -> Result<Json<Vec<I18nString>>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &claims.organization_id).await.map_err(|e| e.to_string())?;

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
pub struct TranslationJobPayload {
    pub text_hash: String,
    pub original_text: String,
    pub target_locale: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateTextRequest {
    pub text_hash: String,
    pub original_text: String,
    pub target_locale: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateTextResponse {
    pub translated_text: Option<String>,
    pub status: String,
}

pub async fn translate_text(
    claims: Claims,
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<TranslateTextRequest>,
) -> Result<Json<TranslateTextResponse>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    crate::common::auth_utils::set_org_context(&mut *tx, &claims.organization_id).await.map_err(|e| e.to_string())?;

    // Check if the translation is in the cache
    let row = sqlx::query(
        "SELECT translated_text FROM ohc_translation_cache
         WHERE tenant_id = $1 AND text_hash = $2 AND target_locale = $3"
    )
    .bind(&claims.organization_id)
    .bind(&payload.text_hash)
    .bind(&payload.target_locale)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(r) = row {
        use sqlx::Row;
        let translated_text: String = r.get("translated_text");
        tx.commit().await.map_err(|e| e.to_string())?;
        return Ok(Json(TranslateTextResponse {
            translated_text: Some(translated_text),
            status: "COMPLETED".to_string(),
        }));
    }

    // Not found, enqueue a translation job
    let job_payload = TranslationJobPayload {
        text_hash: payload.text_hash.clone(),
        original_text: payload.original_text.clone(),
        target_locale: payload.target_locale.clone(),
    };
    let payload_json = serde_json::to_string(&job_payload).map_err(|e| e.to_string())?;
    let job_id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO sub_agent_queue (id, tenant_id, payload, status, scheduled_at, created_at, updated_at)
         VALUES ($1, $2, $3, 'QUEUED', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
    )
    .bind(&job_id)
    .bind(&claims.organization_id)
    .bind(&payload_json)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(Json(TranslateTextResponse {
        translated_text: None,
        status: "PROCESSING".to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_payload() {
        let payload = TranslationJobPayload {
            text_hash: "hash".to_string(),
            original_text: "text".to_string(),
            target_locale: "fr".to_string(),
        };
        assert_eq!(payload.text_hash, "hash");
    }

    #[test]
    fn test_translate_text_request() {
        let request = TranslateTextRequest {
            text_hash: "hash".to_string(),
            original_text: "text".to_string(),
            target_locale: "fr".to_string(),
        };
        assert_eq!(request.text_hash, "hash");
    }

    #[test]
    fn test_translate_text_response() {
        let response = TranslateTextResponse {
            translated_text: Some("texte".to_string()),
            status: "COMPLETED".to_string(),
        };
        assert_eq!(response.status, "COMPLETED");
    }
}
