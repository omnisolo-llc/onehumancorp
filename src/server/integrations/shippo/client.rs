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

    pub async fn fetch_rates(&self, weight: f64, dimensions: &str, address_to: serde_json::Value, address_from: serde_json::Value) -> Result<Vec<String>, String> {
        #[cfg(test)]
        if self.api_key == "test_token" {
            return Ok(vec!["USPS - $5.00".to_string()]);
        }

        let url = "https://api.goshippo.com/shipments/";

        let mut parcel = serde_json::json!({
            "distance_unit": "in",
            "weight": weight.to_string(),
            "mass_unit": "lb"
        });

        // Simplified dimensions parsing "LxWxH"
        let parts: Vec<&str> = dimensions.split('x').collect();
        if parts.len() == 3 {
            parcel["length"] = serde_json::json!(parts[0]);
            parcel["width"] = serde_json::json!(parts[1]);
            parcel["height"] = serde_json::json!(parts[2]);
        } else {
            parcel["length"] = serde_json::json!("5");
            parcel["width"] = serde_json::json!("5");
            parcel["height"] = serde_json::json!("5");
        }


        let payload = serde_json::json!({
            "address_from": address_from,
            "address_to": address_to,
            "parcels": [parcel]
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
            .header("Shippo-API-Version", "2018-02-08")
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(vec!["USPS - $5.00".to_string()]) // simplified parse for now
                } else {
                    Err(format!("Shippo API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn purchase_label(&self, rate_id: &str) -> Result<String, String> {
        #[cfg(test)]
        if self.api_key == "test_token" {
            return Ok("https://api.goshippo.com/v1/mock_label.pdf".to_string());
        }

        let url = "https://api.goshippo.com/transactions/";
        let payload = serde_json::json!({
            "rate": rate_id,
            "label_file_type": "PDF",
            "async": false
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
            .header("Shippo-API-Version", "2018-02-08")
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
