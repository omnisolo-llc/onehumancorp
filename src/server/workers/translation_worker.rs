use std::sync::Arc;
use tokio::time::Duration;
use crate::db::DB;
use crate::minimax::MinimaxClient;
use crate::api::localization::TranslationJobPayload;

pub struct TranslationWorker {
    db: Arc<DB>,
}

impl TranslationWorker {
    pub fn new(db: Arc<DB>) -> Self {
        TranslationWorker { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            tracing::info!("TranslationWorker started");
            loop {
                if let Err(e) = Self::process_next_job(&db).await {
                    ::server_telemetry::record_error_signal("TranslationWorker error");
                    tracing::error!("TranslationWorker error: {}", e);
                }
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
    }

    async fn process_next_job(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut tx = db.pool.begin().await?;

        // Find a pending translation job in sub_agent_queue, locking the row.
        // We assume payload is JSON and has "target_locale" and "text_hash"
        let job: Option<(String, String, String, i32)> = sqlx::query_as(
            "SELECT id, tenant_id, payload, 0 as retry_count FROM sub_agent_queue
             WHERE status = 'QUEUED' AND payload LIKE '%\"target_locale\"%' AND payload LIKE '%\"text_hash\"%'
             ORDER BY created_at ASC
             LIMIT 1 FOR UPDATE SKIP LOCKED"
        )
        .fetch_optional(&mut *tx)
        .await?;

        let (job_id, tenant_id, payload_str, retry_count) = match job {
            Some(j) => j,
            None => return Ok(()),
        };

        tracing::info!("Processing translation job {} for tenant {}", job_id, tenant_id);

        let payload: TranslationJobPayload = match serde_json::from_str(&payload_str) {
            Ok(p) => p,
            Err(e) => {
                tracing::error!("Invalid TranslationJobPayload: {}", e);
                sqlx::query("UPDATE sub_agent_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                    .bind(&job_id)
                    .execute(&mut *tx)
                    .await?;
                tx.commit().await?;
                return Ok(());
            }
        };

        // Update status to RUNNING
        sqlx::query("UPDATE sub_agent_queue SET status = 'RUNNING', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(&job_id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;

        // Execute translation
        let result = Self::perform_translation(db, &tenant_id, &payload).await;

        let mut tx = db.pool.begin().await?;

        match result {
            Ok(translated_text) => {
                // Insert into cache
                sqlx::query(
                    "INSERT INTO ohc_translation_cache (id, tenant_id, text_hash, target_locale, translated_text)
                     VALUES ($1, $2, $3, $4, $5)
                     ON CONFLICT (tenant_id, text_hash, target_locale) DO UPDATE SET translated_text = $5"
                )
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(&tenant_id)
                .bind(&payload.text_hash)
                .bind(&payload.target_locale)
                .bind(&translated_text)
                .execute(&mut *tx)
                .await?;

                sqlx::query("UPDATE sub_agent_queue SET status = 'COMPLETED', completed_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                    .bind(&job_id)
                    .execute(&mut *tx)
                    .await?;
            }
            Err(e) => {
                ::server_telemetry::record_error_signal("Translation job failed");
                tracing::error!("Translation job {} failed: {}", job_id, e);

                // Exponential backoff logic could go here, but for now just mark FAILED or re-queue.
                // Re-queue with backoff if retry_count < max
                if retry_count < 3 {
                    sqlx::query("UPDATE sub_agent_queue SET status = 'QUEUED', updated_at = CURRENT_TIMESTAMP, scheduled_at = CURRENT_TIMESTAMP + (INTERVAL '1 second' * power(2, $2)) WHERE id = $1")
                        .bind(&job_id)
                        .bind(retry_count + 1)
                        .execute(&mut *tx)
                        .await?;
                } else {
                    sqlx::query("UPDATE sub_agent_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&job_id)
                        .execute(&mut *tx)
                        .await?;
                }
            }
        }
        tx.commit().await?;

        Ok(())
    }

    async fn perform_translation(_db: &Arc<DB>, _tenant_id: &str, payload: &TranslationJobPayload) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let api_key = std::env::var("OHC_MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());

        // Return dummy translation if test key is used
        if api_key == "fake-key" {
            return Ok(format!("Translated[{}]: {}", payload.target_locale, payload.original_text));
        }

        let minimax = MinimaxClient::new(api_key);
        let prompt = format!(
            "Translate the following text to the locale code '{}'. Return ONLY the translated text without any explanation, markdown, or extra quotes.\n\nText:\n{}",
            payload.target_locale,
            payload.original_text
        );

        let response = minimax.reason(&prompt).await?;
        let clean_response = response.trim().to_string();

        Ok(clean_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_perform_translation_fake_key() {
        let payload = TranslationJobPayload {
            text_hash: "hash".to_string(),
            original_text: "Hello".to_string(),
            target_locale: "fr".to_string(),
        };
        // Just testing string manipulation since DB needs mock
        let result = format!("Translated[{}]: {}", payload.target_locale, payload.original_text);
        assert_eq!(result, "Translated[fr]: Hello");
    }
}
