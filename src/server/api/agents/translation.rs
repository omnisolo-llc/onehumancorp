#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboxTranslation {
    pub original_content: String,
    pub translated_content: String,
    pub source_language: Option<String>,
    pub target_language: String,
}

#[async_trait::async_trait]
pub trait InboxTranslationClient: Send + Sync {
    async fn translate_for_inbox(
        &self,
        tenant_id: &str,
        source: &str,
        message: &str,
        target_language: &str,
    ) -> Result<InboxTranslation, String>;
}

pub struct LlmInboxTranslationClient;

#[async_trait::async_trait]
impl InboxTranslationClient for LlmInboxTranslationClient {
    async fn translate_for_inbox(
        &self,
        tenant_id: &str,
        source: &str,
        message: &str,
        target_language: &str,
    ) -> Result<InboxTranslation, String> {
        translate_inbox_message_with_llm(tenant_id, source, message, target_language).await
    }
}

pub async fn translate_inbox_message_with_llm(
    tenant_id: &str,
    source: &str,
    message: &str,
    target_language: &str,
) -> Result<InboxTranslation, String> {
    let prompt = format!(
        "Return strict JSON with keys source_language and translated_content. Detect the customer's message language and translate it to {target_language} for an omnichannel SMB inbox. Preserve names, prices, dates, and order details. Tenant: {tenant_id}. Source: {source}. Message: {message}"
    );

    let raw = match std::env::var("OHC_TRANSLATION_LLM_PROVIDER")
        .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
        .as_deref()
    {
        Ok("minimax") => {
            let api_key = std::env::var("MINIMAX_API_KEY")
                .map_err(|_| "MINIMAX_API_KEY is required for minimax inbox translation".to_string())?;
            crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await
        }
        _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
    }?;

    parse_inbox_translation(message, target_language, &raw)
}

pub fn parse_inbox_translation(
    original_content: &str,
    target_language: &str,
    raw: &str,
) -> Result<InboxTranslation, String> {
    let value: serde_json::Value = match serde_json::from_str(raw) {
        Ok(value) => value,
        Err(_) => {
            let start = raw.find('{').ok_or_else(|| "translation response missing JSON object".to_string())?;
            let end = raw.rfind('}').ok_or_else(|| "translation response missing JSON object".to_string())?;
            serde_json::from_str(&raw[start..=end])
                .map_err(|e| format!("failed to parse translation JSON: {e}"))?
        }
    };

    let translated_content = value
        .get("translated_content")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .ok_or_else(|| "translation response missing translated_content".to_string())?
        .to_string();

    let source_language = value
        .get("source_language")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(ToString::to_string);

    Ok(InboxTranslation {
        original_content: original_content.to_string(),
        translated_content,
        source_language,
        target_language: target_language.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_translation_json_with_original_content() {
        let parsed = parse_inbox_translation(
            "¿Tienes pastel vegano mañana?",
            "en",
            r#"{"source_language":"es","translated_content":"Do you have vegan cake tomorrow?"}"#,
        )
        .unwrap();

        assert_eq!(parsed.original_content, "¿Tienes pastel vegano mañana?");
        assert_eq!(parsed.source_language.as_deref(), Some("es"));
        assert_eq!(parsed.translated_content, "Do you have vegan cake tomorrow?");
        assert_eq!(parsed.target_language, "en");
    }
}

pub async fn generate_inbox_draft_reply(
    tenant_id: &str,
    source: &str,
    translation: &InboxTranslation,
    customer_context: Option<&str>,
) -> Result<String, String> {
    let mut prompt = format!(
        "Write one concise, warm customer-service reply in {} for an omnichannel SMB inbox. Do not invent policies, availability, prices, or order state. Tenant: {tenant_id}. Source: {source}. Customer message: {}",
        translation.target_language,
        translation.translated_content
    );
    if let Some(ctx) = customer_context {
        prompt.push_str(&format!("\n\nCustomer Context:\n{}", ctx));
    }
    let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

    match std::env::var("OHC_INBOX_DRAFT_LLM_PROVIDER")
        .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
        .as_deref()
    {
        Ok("minimax") => {
            let api_key = std::env::var("MINIMAX_API_KEY")
                .map_err(|_| "MINIMAX_API_KEY is required for minimax inbox draft generation".to_string())?;
            crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt).await
        }
        _ => crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await,
    }
}
