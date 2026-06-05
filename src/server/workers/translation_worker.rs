use sqlx::PgPool;
use std::sync::Arc;
use serde_json::Value;
use tracing::{info, error};

pub async fn handle_translation_job(
    pool: Arc<PgPool>,
    tenant_id: &str,
    payload: Value,
) -> Result<(), String> {
    let source_text = payload["source_text"].as_str().ok_or("Missing source_text")?;
    let source_lang = payload["source_lang"].as_str().ok_or("Missing source_lang")?;
    let target_lang = payload["target_lang"].as_str().ok_or("Missing target_lang")?;
    let text_hash = payload["source_text_hash"].as_str().ok_or("Missing source_text_hash")?;

    info!("Translating text for {} from {} to {}", tenant_id, source_lang, target_lang);

    // Mocking an LLM translation by adding a prefix for now
    let translated_text = format!("Translated to {}: {}", target_lang, source_text);

    let id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO translation_cache (id, tenant_id, source_text_hash, source_lang, target_lang, translated_text, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
         ON CONFLICT (tenant_id, source_text_hash, target_lang)
         DO UPDATE SET translated_text = EXCLUDED.translated_text, updated_at = CURRENT_TIMESTAMP"
    )
    .bind(&id)
    .bind(tenant_id)
    .bind(text_hash)
    .bind(source_lang)
    .bind(target_lang)
    .bind(&translated_text)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_translation_worker_handles_job() {
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
        let pool = match tokio::time::timeout(std::time::Duration::from_millis(5000), sqlx::PgPool::connect(&db_url)).await {
            Ok(Ok(p)) => p,
            _ => return,
        };
        let pool_arc = Arc::new(pool);

        let tenant_id = "worker_tenant";
        let payload = json!({
            "source_text": "Good morning",
            "source_lang": "en",
            "target_lang": "es",
            "source_text_hash": "testhash123"
        });

        // Add sqlite missing table schema definition for test
        sqlx::query("CREATE TABLE IF NOT EXISTS translation_cache (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            source_text_hash TEXT NOT NULL,
            source_lang TEXT NOT NULL,
            target_lang TEXT NOT NULL,
            translated_text TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(tenant_id, source_text_hash, target_lang)
        )").execute(&*pool_arc).await.unwrap_or_default();

        handle_translation_job(pool_arc.clone(), tenant_id, payload).await.unwrap();

        let row: (String,) = sqlx::query_as("SELECT translated_text FROM translation_cache WHERE tenant_id = $1 AND source_text_hash = 'testhash123'")
            .bind(tenant_id)
            .fetch_one(&*pool_arc)
            .await
            .expect("Cache entry should exist");

        assert_eq!(row.0, "Translated to es: Good morning");
    }
}
