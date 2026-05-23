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
}

impl EasyPostClient {
    pub async fn create_shipment(&self, to_address: &str, from_address: &str, parcel_details: &str) -> Result<String, String> {
        let url = "https://api.easypost.com/v2/shipments";

        // This is a simplified payload. A real integration would properly construct
        // to_address, from_address, and parcel as objects.
        let payload = serde_json::json!({
            "shipment": {
                "to_address": { "id": to_address },
                "from_address": { "id": from_address },
                "parcel": { "id": parcel_details }
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
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "easypost_create_shipment",
                        0.05 // Mock cost for label generation
                    ).await;
                    // Mock returning a shipping label url, as a true parsing requires a full JSON structure mapping
                    Ok("https://easypost.com/labels/mock_label_123.pdf".to_string())
                } else {
                    Err(format!("EasyPost API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
