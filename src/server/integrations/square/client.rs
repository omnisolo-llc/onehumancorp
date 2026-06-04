use reqwest::Client;
use serde_json::Value;

pub struct SquareClient {
    access_token: String,
    http_client: Client,
    base_url: String,
}

impl SquareClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
            base_url: "https://connect.squareup.com/v2".to_string(), // Use sandbox for tests, connect.squareup.com for prod
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    pub async fn list_catalog(&self) -> Result<Value, String> {
        let url = format!("{}/catalog/list", self.base_url);
        let res = self.http_client.get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json: Value = resp.json().await.map_err(|e| e.to_string())?;
                    Ok(json)
                } else {
                    Err(format!("Square API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn batch_retrieve_inventory_counts(&self, catalog_object_ids: Vec<String>) -> Result<Value, String> {
        let url = format!("{}/inventory/counts/batch-retrieve", self.base_url);
        let payload = serde_json::json!({
            "catalog_object_ids": catalog_object_ids
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json: Value = resp.json().await.map_err(|e| e.to_string())?;
                    Ok(json)
                } else {
                    Err(format!("Square API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn batch_change_inventory(&self, idempotency_key: String, physical_count: i32, catalog_object_id: String, location_id: String, state: String) -> Result<Value, String> {
        let url = format!("{}/inventory/changes/batch-create", self.base_url);
        let payload = serde_json::json!({
            "idempotency_key": idempotency_key,
            "changes": [
                {
                    "type": "PHYSICAL_COUNT",
                    "physical_count": {
                        "catalog_object_id": catalog_object_id,
                        "state": state,
                        "location_id": location_id,
                        "quantity": physical_count.to_string(),
                    }
                }
            ]
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json: Value = resp.json().await.map_err(|e| e.to_string())?;
                    Ok(json)
                } else {
                    Err(format!("Square API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
