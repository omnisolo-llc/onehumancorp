use reqwest::Client;
use serde_json::Value;

pub struct ShopifyClient {
    pub shop_name: String,
    pub access_token: String,
    pub http_client: Client,
}

impl ShopifyClient {
    pub fn new(shop_name: String, access_token: String) -> Self {
        Self {
            shop_name,
            access_token,
            http_client: Client::new(),
        }
    }

    fn base_url(&self) -> String {
        format!("https://{}.myshopify.com/admin/api/2024-01", self.shop_name)
    }

    pub async fn get_inventory_levels(&self, location_id: &str) -> Result<Value, String> {
        let url = format!("{}/locations/{}/inventory_levels.json", self.base_url(), location_id);
        let res = self.http_client.get(&url)
            .header("X-Shopify-Access-Token", &self.access_token)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json: Value = resp.json().await.map_err(|e| e.to_string())?;
                    Ok(json)
                } else {
                    Err(format!("Shopify API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn set_inventory_level(&self, inventory_item_id: i64, location_id: i64, available: i32) -> Result<Value, String> {
        let url = format!("{}/inventory_levels/set.json", self.base_url());
        let payload = serde_json::json!({
            "inventory_item_id": inventory_item_id,
            "location_id": location_id,
            "available": available,
        });

        let res = self.http_client.post(&url)
            .header("X-Shopify-Access-Token", &self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json: Value = resp.json().await.map_err(|e| e.to_string())?;
                    Ok(json)
                } else {
                    Err(format!("Shopify API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
