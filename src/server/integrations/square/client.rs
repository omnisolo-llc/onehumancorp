use reqwest::Client;
use serde_json::Value;

pub struct SquareClient {
    pub access_token: String,
    pub http_client: Client,
    pub base_url: String,
}

impl SquareClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
            base_url: "https://connect.squareup.com/v2".to_string(), // Adjust for sandbox/prod if needed
        }
    }

    pub async fn get_catalog(&self) -> Result<Value, String> {
        let url = format!("{}/catalog/list", self.base_url);
        let res = self.http_client.get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    let json: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
                    Ok(json)
                } else {
                    Err(format!("Square API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn get_inventory(&self) -> Result<Value, String> {
        let url = format!("{}/inventory/counts/batch-retrieve", self.base_url);
        let res = self.http_client.post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .json(&serde_json::json!({}))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    let json: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
                    Ok(json)
                } else {
                    Err(format!("Square API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn update_inventory_count(&self, catalog_object_id: &str, quantity: i32, location_id: &str, state: &str) -> Result<Value, String> {
        let url = format!("{}/inventory/changes/batch-create", self.base_url);

        let idempotency_key = uuid::Uuid::new_v4().to_string();

        let payload = serde_json::json!({
            "idempotency_key": idempotency_key,
            "changes": [
                {
                    "type": "ADJUSTMENT",
                    "adjustment": {
                        "catalog_object_id": catalog_object_id,
                        "location_id": location_id,
                        "state": state,
                        "quantity": quantity.to_string(),
                    }
                }
            ]
        });

        let res = self.http_client.post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    let json: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
                    Ok(json)
                } else {
                    Err(format!("Square API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_init() {
        let client = SquareClient::new("test_token".to_string());
        assert_eq!(client.access_token, "test_token");
        assert_eq!(client.base_url, "https://connect.squareup.com/v2");
    }
}
