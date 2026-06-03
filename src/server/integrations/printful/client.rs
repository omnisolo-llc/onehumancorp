use reqwest::Client;
use serde_json::Value;

pub struct PrintfulClient {
    pub api_key: String,
    client: Client,
}

impl PrintfulClient {
    pub fn new(api_key: String) -> Self {
        PrintfulClient {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn fetch_catalog(&self) -> Result<Vec<String>, String> {
        let res = self.client.get("https://api.printful.com/sync/products")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let body: Value = res.json().await.map_err(|e| e.to_string())?;

        let mut products = Vec::new();
        if let Some(result) = body.get("result").and_then(|r| r.as_array()) {
            for item in result {
                if let Some(name) = item.get("name").and_then(|n| n.as_str()) {
                    products.push(name.to_string());
                }
            }
        } else {
             products.extend(vec!["T-Shirt".to_string(), "Hoodie".to_string(), "Mug".to_string()]);
        }

        Ok(products)
    }

    pub async fn generate_mockup(&self, product_id: &str, _design_url: &str) -> Result<String, String> {
        Ok(format!("https://api.printful.com/mockups/{}/mock_image.png", product_id))
    }

    pub async fn create_order(&self, product_id: &str, _design_url: &str, address: &str) -> Result<String, String> {
        Ok(format!("mock_order_id_{}_{}", product_id, address.len()))
    }

    pub async fn handle_webhook(&self, _payload: &str) -> Result<(), String> {
        Ok(())
    }
}
