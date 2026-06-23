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
            let prompt = format!(
                r#"You are an expert SEO Agent. Generate optimized SEO metadata for a product.
Name: {}
Description: {}
Type: {}
Price: {}

Return a JSON object with:
{{
  "title": "Optimized SEO title",
  "description": "Compelling SEO description (max 160 chars)",
  "schema": {{ JSON-LD schema.org object }}
}}
Only return the JSON."#,
                name, description, item_type, price
            );

            if let Ok(res) = minimax.reason(&prompt).await {
                let cleaned = res.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(cleaned) {
                    let title = json.get("title").and_then(|v| v.as_str()).unwrap_or(name).to_string();
                    let desc = json.get("description").and_then(|v| v.as_str()).unwrap_or(description).to_string();
                    let schema = json.get("schema").cloned().unwrap_or(json!({}));
                    return Ok((title, desc, schema));
                }
            }
        }

        // Fallback to mock SEO generation
        let title = format!("{} | Buy Online", name);
        let desc = if description.len() > 157 {
            format!("{}...", &description[..157])
        } else {
            description.to_string()
        };
        let schema = json!({
            "@context": "https://schema.org/",
            "@type": item_type,
            "name": name,
            "description": desc,
            "offers": {
                "@type": "Offer",
                "price": price,
                "priceCurrency": "USD",
                "availability": "https://schema.org/InStock"
            }
        });
        Ok((title, desc, schema))
    }
}
