use serde_json::json;

#[async_trait::async_trait]
pub trait SeoClient: Send + Sync {
    async fn generate_seo_metadata(&self, name: &str, description: &str, item_type: &str, price: f64) -> Result<(String, String, serde_json::Value), String>;
}

pub struct RuntimeSeoClient;

#[async_trait::async_trait]
impl SeoClient for RuntimeSeoClient {
    async fn generate_seo_metadata(&self, name: &str, description: &str, item_type: &str, price: f64) -> Result<(String, String, serde_json::Value), String> {
        let prompt = format!("You are an expert SEO AI. Based on the following product details, generate a JSON object with SEO metadata for Generative Engine Optimization (GEO). The JSON must include 'title', and a rich 'description' acting as a natural language summary optimized for AI search engines like ChatGPT and Gemini. Only return the JSON object. Product name: {}. Description: {}. Type: {}.", name, description, item_type);

        let mut attempts = 0;
        let mut ai_res = String::new();
        let mut ai_call_succeeded = false;

        let provider = std::env::var("OHC_LLM_PROVIDER").unwrap_or_else(|_| "minimax".to_string());

        while attempts < 3 {
            let reason_future = async {
                if provider == "minimax" {
                    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                    if api_key.is_empty() {
                        return Err("No API key".to_string());
                    }
                    crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await
                } else {
                    crate::minimax::LocalLLMClient::new().reason(&prompt).await
                }
            };

            match tokio::time::timeout(std::time::Duration::from_secs(60), reason_future).await {
                Ok(Ok(res)) => {
                    ai_res = res;
                    ai_call_succeeded = true;
                    break;
                },
                _ => {
                    attempts += 1;
                    tokio::time::sleep(std::time::Duration::from_secs(2u64.pow(attempts))).await;
                }
            }
        }

        if ai_call_succeeded {
            let cleaned = ai_res.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
            if let Ok(seo_json) = serde_json::from_str::<serde_json::Value>(cleaned) {
                let title = seo_json.get("title").and_then(|v| v.as_str()).unwrap_or(&format!("{} | Buy Online", name)).to_string();
                let desc = seo_json.get("description").and_then(|v| v.as_str()).unwrap_or(&format!("Purchase {} online.", name)).to_string();
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
                return Ok((title, desc, schema));
            }
        }

        // Fallback to basic if LLM call failed or parsing failed
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
