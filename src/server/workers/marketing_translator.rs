use crate::builder::localizable_content::upsert_localizable_content;
use crate::minimax::LocalLLMClient;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use tracing::info;

pub async fn translate_content_job(
    pool: Arc<PgPool>,
    llm: Arc<LocalLLMClient>,
    tenant_id: Uuid,
    resource_id: Uuid,
    resource_type: String,
    field_name: String,
    content: String,
) -> Result<(), String> {
    // We want to translate into Arabic ("ar-SA") and English ("en-US").
    // We assume the input might be either, and we'll ask LLM to translate appropriately.

    info!("Starting translation job for {} {} field {}", resource_type, resource_id, field_name);

    let target_languages = vec!["ar-SA", "en-US"];

    for lang in target_languages {
        let prompt = format!(
            "Translate the following text into {}, adapting it culturally for a small business storefront if necessary. Provide ONLY the translated text, no other comments.\n\nText: {}",
            lang, content
        );

        let translated = llm.reason(&prompt).await?;
        let translated = translated.trim().to_string();

        if !translated.is_empty() {
            upsert_localizable_content(
                &pool,
                tenant_id,
                resource_id,
                &resource_type,
                &field_name,
                lang,
                &translated,
            ).await.map_err(|e| e.to_string())?;
            info!("Successfully translated into {}", lang);
        }
    }

    Ok(())
}
