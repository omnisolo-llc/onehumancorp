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
            "weight": weight,
            "dimensions": dimensions
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
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

    pub async fn purchase_label(&self, rate_id: &str) -> Result<String, String> {
        let url = "https://api.goshippo.com/transactions/";

        let payload = serde_json::json!({
            "rate": rate_id
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
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
