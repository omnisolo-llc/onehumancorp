use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use sha2::{Digest, Sha256};
use hex;

#[derive(Debug, Clone)]
pub struct TranslationMeshService {
    pool: Arc<PgPool>,
}

impl TranslationMeshService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn translate(
        &self,
        tenant_id: &str,
        source_text: &str,
        source_lang: &str,
        target_lang: &str,
    ) -> Result<(String, bool), String> {
        let mut hasher = Sha256::new();
        hasher.update(source_text);
        let text_hash = hex::encode(hasher.finalize());

        // Check cache first
        let row = sqlx::query(
            "SELECT translated_text FROM translation_cache WHERE tenant_id = $1 AND source_text_hash = $2 AND target_lang = $3"
        )
        .bind(tenant_id)
        .bind(&text_hash)
        .bind(target_lang)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            let translated_text: String = r.get("translated_text");
            return Ok((translated_text, true));
        }

        // If missing, push to queue and return pending
        let job_id = Uuid::new_v4().to_string();
        let payload = serde_json::json!({
            "type": "TRANSLATION",
            "source_text": source_text,
            "source_lang": source_lang,
            "target_lang": target_lang,
            "source_text_hash": text_hash
        });

        sqlx::query(
            "INSERT INTO sub_agent_queue (id, tenant_id, parent_task_id, payload, status, scheduled_at, created_at, updated_at)
             VALUES ($1, $2, NULL, $3, 'QUEUED', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
        )
        .bind(&job_id)
        .bind(tenant_id)
        .bind(payload.to_string())
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(("Pending translation...".to_string(), false))
    }
}
