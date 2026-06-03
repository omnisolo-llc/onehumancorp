use reqwest::Client;
use serde_json::Value;

pub struct PrintfulClient {
    api_key: String,
    http_client: Client,
}

impl PrintfulClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn get_catalog(&self) -> Result<Value, String> {
        let url = "https://api.printful.com/products";
        let res = self.http_client.get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    let json: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
                    Ok(json)
                } else {
                    Err(format!("Printful API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn create_mockup_task(&self, product_id: i32, variant_id: i32, image_url: &str) -> Result<String, String> {
        let url = format!("https://api.printful.com/mockup-generator/create-task/{}", product_id);
        let payload = serde_json::json!({
            "variant_ids": [variant_id],
            "format": "png",
            "files": [
                {
                    "placement": "front",
                    "image_url": image_url,
                    "position": {
                        "area_width": 1800,
                        "area_height": 2400,
                        "width": 1800,
                        "height": 1800,
                        "top": 300,
                        "left": 0
                    }
                }
            ]
        });

        let res = self.http_client.post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    let json: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
                    let task_key = json["result"]["task_key"].as_str().unwrap_or_default().to_string();
                    Ok(task_key)
                } else {
                    Err(format!("Printful API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn get_mockup_task(&self, task_key: &str) -> Result<Value, String> {
        let url = format!("https://api.printful.com/mockup-generator/task?task_key={}", task_key);
        let res = self.http_client.get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    let json: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
                    Ok(json)
                } else {
                    Err(format!("Printful API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn create_order(&self, order_details: &Value) -> Result<Value, String> {
        let url = "https://api.printful.com/orders";
        let res = self.http_client.post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(order_details)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    let json: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
                    Ok(json)
                } else {
                    Err(format!("Printful API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
