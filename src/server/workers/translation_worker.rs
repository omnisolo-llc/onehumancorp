use crate::queue::SubAgentJob;
use crate::db::DB;
use std::sync::Arc;
use tracing::{info};

pub async fn handle_translation_job(db: Arc<DB>, job: SubAgentJob) -> Result<(), String> {
    info!("Processing translation job: {}", job.id);

    let source_text = job.payload.get("source_text").and_then(|v| v.as_str()).unwrap_or_default();
    let target_locale = job.payload.get("target_locale").and_then(|v| v.as_str()).unwrap_or_default();
    let source_hash = job.payload.get("source_hash").and_then(|v| v.as_str()).unwrap_or_default();

    if source_text.is_empty() || target_locale.is_empty() {
        return Err("Missing source_text or target_locale".to_string());
    }

    let prompt = format!(
        "Translate the following text to {locale}.\nReturn ONLY the translated text, with no markdown, quotes, or additional commentary.\n\nText:\n{text}",
        locale = target_locale,
        text = source_text
    );

    let provider = std::env::var("OHC_LLM_PROVIDER").unwrap_or_else(|_| "minimax".to_string());

    let translated_text = if provider == "minimax" || provider == "openai" {
        let minimax_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
        // Fix: Use MinimaxClient, not MinimaxClient
        let minimax = crate::minimax::MinimaxClient::new(minimax_key);
        minimax.reason(&prompt).await.map_err(|e| format!("LLM error: {}", e))?
    } else {
        // Fallback
        format!("[TR: {}] {}", target_locale, source_text)
    };

    let id = uuid::Uuid::new_v4().to_string();

    db.save_translation_to_cache(
        &id,
        &job.tenant_id,
        source_hash,
        source_text,
        target_locale,
        &translated_text
    ).await.map_err(|e| e.to_string())?;

    info!("Translation job {} completed successfully.", job.id);

    Ok(())
}
