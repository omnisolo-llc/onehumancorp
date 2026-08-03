use serde_json::json;

#[async_trait::async_trait]
pub trait SeoClient: Send + Sync {
    async fn generate_seo_metadata(&self, name: &str, description: &str, item_type: &str, price: f64) -> Result<(String, String, serde_json::Value), String>;
}

pub struct RuntimeSeoClient;

#[async_trait::async_trait]
impl SeoClient for RuntimeSeoClient {
    async fn generate_seo_metadata(&self, name: &str, description: &str, item_type: &str, price: f64) -> Result<(String, String, serde_json::Value), String> {
        let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
        if !api_key.is_empty() {
            let minimax = crate::minimax::MinimaxClient::new(api_key);
            let prompt = format!("You are an expert SEO AI. Generate SEO metadata for a {item_type} named '{name}' with description '{description}' priced at {price}. Return a JSON object with 'title', 'description', and a 'schema' object containing valid JSON-LD for schema.org/{item_type}. Only return the JSON object without markdown formatting blocks.");

            if let Ok(res) = minimax.reason(&prompt).await {
                let cleaned = res.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(cleaned) {
                    let title = json.get("title").and_then(|v| v.as_str()).unwrap_or(name).to_string();
                    let desc = json.get("description").and_then(|v| v.as_str()).unwrap_or(description).to_string();
                    let schema = json.get("schema").cloned().unwrap_or_else(|| json!({}));
                    return Ok((title, desc, schema));
                }
            }
        }

        // Fallback to basic SEO generation
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
