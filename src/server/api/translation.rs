use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use crate::common::Claims;
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationRequest {
    pub text: String,
    pub locale: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationResponse {
    pub translated_text: String,
    pub status: String,
}

pub async fn get_translation(
    claims: Claims,
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<TranslationRequest>,
) -> Result<Json<TranslationResponse>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    ::server_common::auth_utils::set_org_context(&mut *tx, claims.organization_id.as_deref().unwrap_or("")).await.map_err(|e| e.to_string())?;

    let mut hasher = Sha256::new();
    hasher.update(payload.text.as_bytes());
    let text_hash = hex::encode(hasher.finalize());

    let row = sqlx::query(
        "SELECT translated_text FROM translation_cache
         WHERE tenant_id = $1 AND text_hash = $2 AND locale = $3"
    )
    .bind(&claims.organization_id)
    .bind(&text_hash)
    .bind(&payload.locale)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(r) = row {
        use sqlx::Row;
        let translated_text: String = r.get("translated_text");
        tx.commit().await.map_err(|e| e.to_string())?;
        return Ok(Json(TranslationResponse {
            translated_text,
            status: "cached".to_string(),
        }));
    }

    // Queue for translation
    let job_id = uuid::Uuid::new_v4().to_string();
    let job_payload = serde_json::json!({
        "text": payload.text,
        "text_hash": text_hash,
        "locale": payload.locale,
    });

    sqlx::query(
        "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload)
         VALUES ($1, $2, 'translation_batch', $3)"
    )
    .bind(&job_id)
    .bind(&claims.organization_id)
    .bind(job_payload)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(Json(TranslationResponse {
        translated_text: "".to_string(),
        status: "queued".to_string(),
    }))
}
