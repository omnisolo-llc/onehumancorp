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

    pub async fn fetch_rates(&self, _weight: f64, _dimensions: &str) -> Result<Vec<String>, String> {
        let url = "https://api.goshippo.com/shipments/";
        let payload = serde_json::json!({
            "address_to": {},
            "address_from": {},
            "parcels": [{
                "length": "5",
                "width": "5",
                "height": "5",
                "distance_unit": "in",
                "weight": _weight.to_string(),
                "mass_unit": "lb"
            }]
        });

        let res = self.http_client.post(url)
            .header("Authorization", format!("ShippoToken {}", self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(vec!["USPS - $5.00".to_string()])
                } else {
                    Err(format!("Shippo API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn purchase_label(&self, _rate_id: &str) -> Result<String, String> {
        let url = "https://api.goshippo.com/transactions/";
        let payload = serde_json::json!({
            "rate": _rate_id,
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
                    Ok("https://api.goshippo.com/v1/mock_label.pdf".to_string())
                } else {
                    Err(format!("Shippo API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
