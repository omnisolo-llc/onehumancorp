use serde_json::json;

#[async_trait::async_trait]
pub trait SeoClient: Send + Sync {
    async fn generate_seo_metadata(&self, name: &str, description: &str, item_type: &str, price: f64) -> Result<(String, String, serde_json::Value), String>;
}

pub struct RuntimeSeoClient;

#[async_trait::async_trait]
impl SeoClient for RuntimeSeoClient {
    async fn generate_seo_metadata(&self, name: &str, description: &str, item_type: &str, price: f64) -> Result<(String, String, serde_json::Value), String> {
        // Mock SEO generation
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
