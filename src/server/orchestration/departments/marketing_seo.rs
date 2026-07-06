use serde_json::json;

#[async_trait::async_trait]
pub trait SeoClient: Send + Sync {
    async fn generate_seo_metadata(&self, name: &str, description: &str, item_type: &str, price: f64) -> Result<(String, String, serde_json::Value), String>;
}

pub struct RuntimeSeoClient;

#[async_trait::async_trait]
impl SeoClient for RuntimeSeoClient {
    async fn generate_seo_metadata(&self, name: &str, description: &str, item_type: &str, price: f64) -> Result<(String, String, serde_json::Value), String> {
        let prompt = format!(
            "Generate JSON for SEO metadata for a {item_type} named '{name}'. Description: '{description}'. Price: {price}.
"
        );
        let prompt = format!(
            "{prompt} Output ONLY valid JSON in this exact format, with no markdown formatting or extra text:
"
        );
        let prompt = format!(
            "{} {{\"title\": \"SEO Title (max 60 chars)\", \"description\": \"SEO Description (max 160 chars)\", \"schema\": {{\"@context\": \"https://schema.org/\", \"@type\": \"{}\", \"name\": \"{}\", \"description\": \"...\", \"offers\": {{\"@type\": \"Offer\", \"price\": {}, \"priceCurrency\": \"USD\"}}}}}}",
            prompt, item_type, name, price
        );

        let raw_response = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
            Ok("minimax") => {
                let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                if api_key.is_empty() {
                    crate::minimax::LocalLLMClient::new().reason(&prompt).await.unwrap_or_default()
                } else {
                    let client = crate::minimax::MinimaxClient::new(api_key);
                    client.reason(&prompt).await.unwrap_or_default()
                }
            },
            _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await.unwrap_or_default(),
        };

        if !raw_response.is_empty() {
            // Strip any markdown fences
            let cleaned = raw_response.replace("```json", "").replace("```", "").trim().to_string();
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&cleaned) {
                if let (Some(title), Some(desc), Some(schema)) = (
                    parsed.get("title").and_then(|v| v.as_str()),
                    parsed.get("description").and_then(|v| v.as_str()),
                    parsed.get("schema"),
                ) {
                    return Ok((title.to_string(), desc.to_string(), schema.clone()));
                }
            }
        }

        // Fallback Mock SEO generation
        let title = format!("{} | Buy Online", name);
        let desc = format!("Purchase {} online. {}", name, description);
        let schema = json!({
            "@context": "https://schema.org/",
            "@type": item_type,
            "name": name,
            "description": desc,
            "offers": {
                "@type": "Offer",
                "price": price,
                "priceCurrency": "USD"
            }
        });
        Ok((title, desc, schema))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_runtime_seo_client_fallback() {
        let client = RuntimeSeoClient;
        let (title, desc, schema) = client.generate_seo_metadata("Test Product", "Test Description", "Product", 9.99).await.unwrap();
        assert_eq!(title, "Test Product | Buy Online");
        assert_eq!(desc, "Purchase Test Product online. Test Description");
        assert_eq!(schema.get("name").unwrap().as_str().unwrap(), "Test Product");
    }
}
