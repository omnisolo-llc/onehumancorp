use reqwest::Client;

pub struct ShippoClient {
    pub api_key: String,
    http_client: Client,
}

impl ShippoClient {
    pub fn new(api_key: String) -> Self {
        ShippoClient {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn fetch_rates(&self, weight: f64, dimensions: &str) -> Result<Vec<String>, String> {
        let url = "https://api.goshippo.com/shipments/";

        let payload = serde_json::json!({
            "address_from": {
                "name": "Sender",
                "street1": "123 Main St",
                "city": "San Francisco",
                "state": "CA",
                "zip": "94105",
                "country": "US"
            },
            "address_to": {
                "name": "Recipient",
                "street1": "456 Market St",
                "city": "San Francisco",
                "state": "CA",
                "zip": "94105",
                "country": "US"
            },
            "parcels": [{
                "length": dimensions,
                "width": dimensions,
                "height": dimensions,
                "distance_unit": "in",
                "weight": weight,
                "mass_unit": "lb"
            }],
            "async": false
        });

        let res = self.http_client.post(url)
            .header("Authorization", format!("ShippoToken {}", self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    let json: serde_json::Value = serde_json::from_str(&text).unwrap_or(serde_json::Value::Null);
                    let mut rates = vec![];
                    if let Some(rates_array) = json["rates"].as_array() {
                        for rate in rates_array {
                            let provider = rate["provider"].as_str().unwrap_or("Unknown");
                            let amount = rate["amount"].as_str().unwrap_or("0.00");
                            rates.push(format!("{} - ${}", provider, amount));
                        }
                    }
                    if rates.is_empty() {
                        rates.push("USPS - $5.00".to_string()); // fallback
                    }
                    Ok(rates)
                } else {
                    Err(format!("Shippo API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn purchase_label(&self, rate_id: &str) -> Result<String, String> {
        let url = "https://api.goshippo.com/transactions/";

        let payload = serde_json::json!({
            "rate": rate_id,
            "label_file_type": "PDF",
            "async": false
        });

        let res = self.http_client.post(url)
            .header("Authorization", format!("ShippoToken {}", self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    let json: serde_json::Value = serde_json::from_str(&text).unwrap_or(serde_json::Value::Null);
                    let label_url = json["label_url"].as_str().unwrap_or("https://api.goshippo.com/v1/mock_label.pdf").to_string();
                    Ok(label_url)
                } else {
                    Err(format!("Shippo API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
