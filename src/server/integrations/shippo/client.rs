use serde_json::json;

pub struct ShippoClient {
    pub api_key: String,
    http_client: reqwest::Client,
}

impl ShippoClient {
    pub fn new(api_key: String) -> Self {
        ShippoClient {
            api_key,
            http_client: reqwest::Client::new(),
        }
    }

    fn api_base() -> String {
        std::env::var("SHIPPO_API_BASE")
            .unwrap_or_else(|_| "https://api.goshippo.com".to_string())
            .trim_end_matches('/')
            .to_string()
    }

    fn validate_credentials(&self) -> Result<(), String> {
        let token = self.api_key.trim();
        if token.is_empty()
            || token.contains("dummy")
            || token.contains("mock")
            || token.contains("fake")
        {
            return Err("Shippo API token is required".to_string());
        }
        Ok(())
    }

    fn configured_address(var_name: &str) -> Result<serde_json::Value, String> {
        let raw = std::env::var(var_name)
            .map_err(|_| format!("{var_name} is required to request live Shippo rates"))?;
        serde_json::from_str(&raw)
            .map_err(|e| format!("{var_name} must be valid Shippo address JSON: {e}"))
    }

    pub async fn fetch_rates(&self, weight: f64, dimensions: &str) -> Result<Vec<String>, String> {
        self.validate_credentials()?;
        if weight <= 0.0 {
            return Err("shipment weight must be positive".to_string());
        }

        let address_from = Self::configured_address("SHIPPO_ADDRESS_FROM_JSON")?;
        let address_to = Self::configured_address("SHIPPO_ADDRESS_TO_JSON")?;
        let parcel = json!({
            "length": dimensions,
            "width": "1",
            "height": "1",
            "distance_unit": "in",
            "weight": weight.to_string(),
            "mass_unit": "oz",
        });
        let payload = json!({
            "address_from": address_from,
            "address_to": address_to,
            "parcels": [parcel],
            "async": false,
        });

        let resp = self.http_client
            .post(format!("{}/shipments", Self::api_base()))
            .header("Authorization", format!("ShippoToken {}", self.api_key.trim()))
            .header("Content-Type", "application/json")
            .header("SHIPPO-API-VERSION", "2018-02-08")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Shippo shipment request failed: {e}"))?;

        let status = resp.status();
        let body: serde_json::Value = resp.json().await
            .map_err(|e| format!("Shippo rates response was not JSON: {e}"))?;
        if !status.is_success() {
            return Err(format!("Shippo rates API error {status}: {body}"));
        }

        let rates = body.get("rates")
            .and_then(|v| v.as_array())
            .ok_or_else(|| "Shippo rates response missing rates".to_string())?;

        Ok(rates.iter().filter_map(|rate| {
            let provider = rate.get("provider").and_then(|v| v.as_str())?;
            let service = rate.get("servicelevel")
                .and_then(|v| v.get("name"))
                .and_then(|v| v.as_str())
                .or_else(|| rate.get("service").and_then(|v| v.as_str()))
                .unwrap_or("Service");
            let amount = rate.get("amount").and_then(|v| v.as_str())?;
            let currency = rate.get("currency").and_then(|v| v.as_str()).unwrap_or("USD");
            Some(format!("{provider} {service} - {amount} {currency}"))
        }).collect())
    }

    pub async fn purchase_label(&self, rate_id: &str) -> Result<String, String> {
        self.validate_credentials()?;
        if rate_id.trim().is_empty() {
            return Err("Shippo rate id is required".to_string());
        }

        let resp = self.http_client
            .post(format!("{}/transactions", Self::api_base()))
            .header("Authorization", format!("ShippoToken {}", self.api_key.trim()))
            .header("Content-Type", "application/json")
            .header("SHIPPO-API-VERSION", "2018-02-08")
            .json(&json!({
                "rate": rate_id,
                "async": false,
                "label_file_type": "PDF",
            }))
            .send()
            .await
            .map_err(|e| format!("Shippo label request failed: {e}"))?;

        let status = resp.status();
        let body: serde_json::Value = resp.json().await
            .map_err(|e| format!("Shippo label response was not JSON: {e}"))?;
        if !status.is_success() {
            return Err(format!("Shippo label API error {status}: {body}"));
        }

        body.get("label_url")
            .and_then(|v| v.as_str())
            .map(|url| url.to_string())
            .ok_or_else(|| "Shippo label response missing label_url".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_rates_requires_real_shippo_credentials() {
        let client = ShippoClient::new("dummy_token".to_string());
        let err = client.fetch_rates(16.0, "10x8x4").await.unwrap_err();
        assert!(err.contains("Shippo API token is required"));
    }

    #[tokio::test]
    async fn purchase_label_requires_real_shippo_credentials() {
        let client = ShippoClient::new("".to_string());
        let err = client.purchase_label("rate_123").await.unwrap_err();
        assert!(err.contains("Shippo API token is required"));
    }
}
