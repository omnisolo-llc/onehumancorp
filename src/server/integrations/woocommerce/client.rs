use reqwest::Client;
use serde_json::Value;

pub struct WooCommerceClient {
    pub base_url: String,
    pub consumer_key: String,
    pub consumer_secret: String,
    pub http_client: Client,
}

impl WooCommerceClient {
    pub fn new(base_url: String, consumer_key: String, consumer_secret: String) -> Self {
        Self {
            base_url,
            consumer_key,
            consumer_secret,
            http_client: Client::new(),
        }
    }

    pub async fn get_product(&self, product_id: &str) -> Result<Value, String> {
        let url = format!("{}/wp-json/wc/v3/products/{}", self.base_url, product_id);
        let res = self.http_client.get(&url)
            .basic_auth(&self.consumer_key, Some(&self.consumer_secret))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json: Value = resp.json().await.map_err(|e| e.to_string())?;
                    Ok(json)
                } else {
                    Err(format!("WooCommerce API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn update_inventory(&self, product_id: &str, stock_quantity: i32) -> Result<Value, String> {
        let url = format!("{}/wp-json/wc/v3/products/{}", self.base_url, product_id);
        let payload = serde_json::json!({
            "stock_quantity": stock_quantity,
            "manage_stock": true,
        });

        let res = self.http_client.put(&url)
            .basic_auth(&self.consumer_key, Some(&self.consumer_secret))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json: Value = resp.json().await.map_err(|e| e.to_string())?;
                    Ok(json)
                } else {
                    Err(format!("WooCommerce API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
