use reqwest::Client;

pub struct ShipengineClient {
    pub api_key: String,
    http_client: Client,
}

impl ShipengineClient {
    pub fn new(api_key: String) -> Self {
        ShipengineClient {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn create_label(&self, carrier_id: &str, service_code: &str) -> Result<String, String> {
        let url = "https://api.shipengine.com/v1/labels".to_string();
        let payload = serde_json::json!({
            "shipment": {
                "carrier_id": carrier_id,
                "service_code": service_code,
                "ship_to": {
                    "name": "Customer",
                    "address_line1": "Address Line 1",
                    "city_locality": "City",
                    "state_province": "State",
                    "postal_code": "Zip",
                    "country_code": "US"
                },
                "ship_from": {
                    "name": "John Doe",
                    "company_name": "Example Corp.",
                    "address_line1": "4009 Marathon Blvd",
                    "city_locality": "Austin",
                    "state_province": "TX",
                    "postal_code": "78756",
                    "country_code": "US",
                    "phone": "512-555-5555"
                },
                "packages": [
                    {
                        "weight": {
                            "value": 1.0,
                            "unit": "ounce"
                        }
                    }
                ]
            }
        });

        let res = self.http_client.post(&url)
            .header("API-Key", &self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(resp.text().await.unwrap_or_default())
                } else {
                    Err(format!("ShipEngine API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
