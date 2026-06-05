use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::queue::Job;
use crate::queue::TaskJobHandler as JobHandler;
use async_trait::async_trait;
use serde_json::Value;
use sha2::Digest;

pub struct TranslationWorker {
    pub db: Arc<DB>,
}

impl TranslationWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl JobHandler for TranslationWorker {
    async fn handle(&self, job: Job) -> Result<(), String> {
        let db = self.db.clone();
            tracing::info!("Processing translation job: {}", job.id);
            let payload: Value = serde_json::from_str(&job.payload).map_err(|e| e.to_string())?;

            let source_text = payload.get("source_text").and_then(|v| v.as_str()).unwrap_or("");
            let source_lang = payload.get("source_lang").and_then(|v| v.as_str()).unwrap_or("");
            let target_lang = payload.get("target_lang").and_then(|v| v.as_str()).unwrap_or("");

            if source_text.is_empty() || source_lang.is_empty() || target_lang.is_empty() {
                return Err("Missing required fields in translation job payload".to_string());
            }

            let text_hash = format!("{:x}", sha2::Sha256::digest(source_text.as_bytes()));

            // Check if it's already translated
            let translated_text_opt: Option<String> = match &db.store {
                DbStore::Postgres => {
                    let row = sqlx::query("SELECT translated_text FROM translation_cache WHERE tenant_id = $1 AND text_hash = $2 AND target_lang = $3")
                        .bind(&job.tenant_id)
                        .bind(&text_hash)
                        .bind(&target_lang)
                        .fetch_optional(&db.pool)
                        .await
                        .map_err(|e| e.to_string())?;
                    row.map(|r| {
                        use sqlx::Row;
                        r.get("translated_text")
                    })
                }
                DbStore::Sqlite(pool) => {
                    let row = sqlx::query("SELECT translated_text FROM translation_cache WHERE tenant_id = ? AND text_hash = ? AND target_lang = ?")
                        .bind(&job.tenant_id)
                        .bind(&text_hash)
                        .bind(&target_lang)
                        .fetch_optional(pool)
                        .await
                        .map_err(|e| e.to_string())?;
                    row.map(|r| {
                        use sqlx::Row;
                        r.get("translated_text")
                    })
                }
            };

            if translated_text_opt.is_some() {
                return Ok(());
            }

            // Dummy translation logic (as we simulate LLM)
            let translated_text = format!("[{target_lang}] {source_text}");
            let id = uuid::Uuid::new_v4().to_string();

            // Insert
            match &db.store {
                DbStore::Postgres => {
                    sqlx::query("INSERT INTO translation_cache (id, tenant_id, text_hash, source_lang, target_lang, translated_text) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (tenant_id, text_hash, target_lang) DO NOTHING")
                        .bind(&id)
                        .bind(&job.tenant_id)
                        .bind(&text_hash)
                        .bind(&source_lang)
                        .bind(&target_lang)
                        .bind(&translated_text)
                        .execute(&db.pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
                DbStore::Sqlite(pool) => {
                    sqlx::query("INSERT OR IGNORE INTO translation_cache (id, tenant_id, text_hash, source_lang, target_lang, translated_text) VALUES (?, ?, ?, ?, ?, ?)")
                        .bind(&id)
                        .bind(&job.tenant_id)
                        .bind(&text_hash)
                        .bind(&source_lang)
                        .bind(&target_lang)
                        .bind(&translated_text)
                        .execute(pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }

            Ok(())

    }
}
