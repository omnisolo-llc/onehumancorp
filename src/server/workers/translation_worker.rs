use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use sqlx::Row;
use serde_json::json;
use tokio::time::sleep;

pub struct TranslationWorker {
    db: Arc<DB>,
}

impl TranslationWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            loop {
                match Self::process_next_translation(&db).await {
                    Ok(true) => {
                        // Found and processed, tight loop
                        tokio::task::yield_now().await;
                    }
                    Ok(false) => {
                        // No items, sleep
                        sleep(Duration::from_secs(5)).await;
                    }
                    Err(e) => {
                        tracing::error!("TranslationWorker error: {}", e);
                        sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        });
    }

    async fn process_next_translation(db: &Arc<DB>) -> Result<bool, String> {
        let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;

        // Simplified query to find a product that needs translation for a specific language
        let row = sqlx::query(
            r#"
            SELECT
                p.id as product_id,
                p.tenant_id,
                p.title,
                p.description,
                u.lang as target_language
            FROM products p
            JOIN merchant_translation_preferences mtp ON p.tenant_id = mtp.tenant_id
            CROSS JOIN LATERAL unnest(mtp.enabled_languages) AS u(lang)
            WHERE mtp.auto_translate = true
              AND NOT EXISTS (
                  SELECT 1 FROM localization_registry lr
                  WHERE lr.tenant_id = p.tenant_id
                    AND lr.resource_id = p.id
                    AND lr.language_code = u.lang
              )
            LIMIT 1 FOR UPDATE SKIP LOCKED
            "#
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let row = match row {
            Some(r) => r,
            None => return Ok(false),
        };

        let product_id: String = row.get("product_id");
        let tenant_id: String = row.get("tenant_id");
        let title: String = row.get("title");
        let description: Option<String> = row.get("description");
        let target_language: String = row.get("target_language");

        let api_key = match std::env::var("MINIMAX_API_KEY") {
            Ok(k) => k,
            Err(_) => {
                let localized = format!("(Translated to {}) {}", target_language, title);
                let translated_text = json!({
                    "title": localized,
                    "description": description.clone().unwrap_or_default()
                }).to_string();

                sqlx::query(
                    "INSERT INTO localization_registry (id, tenant_id, resource_id, resource_type, language_code, translated_text) VALUES ($1, $2, $3, $4, $5, $6)"
                )
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(&tenant_id)
                .bind(&product_id)
                .bind("product")
                .bind(&target_language)
                .bind(&translated_text)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
                return Ok(true);
            }
        };

        let client = crate::minimax::MinimaxClient::new(api_key);
        let prompt = format!(
            "Translate the following product information into language code '{}'.\nTitle: {}\nDescription: {}\nReturn ONLY JSON with 'title' and 'description' keys.",
            target_language,
            title,
            description.clone().unwrap_or_default()
        );

        let result = match client.reason(&prompt).await {
            Ok(res) => res,
            Err(e) => {
                tx.rollback().await.map_err(|e| e.to_string())?;
                return Err(e);
            }
        };

        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap_or_else(|_| json!({
            "title": format!("(Translated) {}", title),
            "description": "Translation parsing failed."
        }));

        let translated_title = parsed.get("title").and_then(|v| v.as_str()).unwrap_or(&title).to_string();
        let translated_desc = parsed.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();

        let translated_text = json!({
            "title": translated_title,
            "description": translated_desc
        }).to_string();

        sqlx::query(
            "INSERT INTO localization_registry (id, tenant_id, resource_id, resource_type, language_code, translated_text) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(&tenant_id)
        .bind(&product_id)
        .bind("product")
        .bind(&target_language)
        .bind(&translated_text)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(true)
    }
}
