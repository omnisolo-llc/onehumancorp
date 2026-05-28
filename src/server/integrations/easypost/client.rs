use reqwest::Client;

pub struct EasyPostClient {
    api_key: String,
    http_client: Client,
}

impl EasyPostClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn create_shipment(&self, to_address: &str, from_address: &str, parcel_details: &str) -> Result<String, String> {
        let url = "https://api.easypost.com/v2/shipments";
        let payload = serde_json::json!({
            "shipment": {
                "to_address": to_address,
                "from_address": from_address,
                "parcel": parcel_details
            }
        });

        let res = self.http_client.post(url)
            .basic_auth(&self.api_key, Some(""))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json_resp = resp.json::<serde_json::Value>().await.map_err(|e| format!("Failed to parse response: {}", e))?;
                    if let Some(label_url) = json_resp.get("postage_label").and_then(|p| p.get("label_url")).and_then(|u| u.as_str()) {
                        Ok(label_url.to_string())
                    } else {
                        Err("Failed to extract label_url from response".to_string())
                    }
                } else {
                    Err(format!("EasyPost API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
