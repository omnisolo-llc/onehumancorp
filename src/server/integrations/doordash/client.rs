use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeliveryQuote {
    pub fee: f64,
    pub dropoff_eta: String,
    pub pickup_eta: String,
}

pub struct DoorDashClient {
    pub api_key: String,
    http_client: reqwest::Client,
}

impl DoorDashClient {
    pub fn new(api_key: String) -> Self {
        DoorDashClient {
            api_key,
            http_client: reqwest::Client::new(),
        }
    }

    fn api_base() -> String {
        std::env::var("DOORDASH_API_BASE")
            .unwrap_or_else(|_| "https://openapi.doordash.com".to_string())
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
            return Err("DoorDash API key is required".to_string());
        }
        Ok(())
    }

    fn external_delivery_id(order_id: &str) -> String {
        let sanitized: String = order_id
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '~'))
            .collect();
        if sanitized.is_empty() {
            "ohc-delivery".to_string()
        } else {
            sanitized
        }
    }

    pub async fn get_delivery_quote(&self, pickup_address: &str, dropoff_address: &str) -> Result<DeliveryQuote, String> {
        self.validate_credentials()?;
        if pickup_address.trim().is_empty() || dropoff_address.trim().is_empty() {
            return Err("pickup and dropoff addresses are required".to_string());
        }

        let external_delivery_id = format!(
            "quote-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| e.to_string())?
                .as_millis()
        );
        let payload = json!({
            "external_delivery_id": external_delivery_id,
            "pickup_address": pickup_address,
            "dropoff_address": dropoff_address,
        });

        let resp = self.http_client
            .post(format!("{}/drive/v2/quotes", Self::api_base()))
            .bearer_auth(self.api_key.trim())
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("DoorDash quote request failed: {e}"))?;

        let status = resp.status();
        let body: serde_json::Value = resp.json().await
            .map_err(|e| format!("DoorDash quote response was not JSON: {e}"))?;
        if !status.is_success() {
            return Err(format!("DoorDash quote API error {status}: {body}"));
        }

        let fee_cents = body.get("fee").and_then(|v| v.as_f64())
            .ok_or_else(|| "DoorDash quote response missing fee".to_string())?;
        let pickup_eta = body.get("pickup_time_estimated")
            .or_else(|| body.get("pickup_eta"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| "DoorDash quote response missing pickup ETA".to_string())?;
        let dropoff_eta = body.get("dropoff_time_estimated")
            .or_else(|| body.get("dropoff_eta"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| "DoorDash quote response missing dropoff ETA".to_string())?;

        Ok(DeliveryQuote {
            fee: fee_cents / 100.0,
            dropoff_eta: dropoff_eta.to_string(),
            pickup_eta: pickup_eta.to_string(),
        })
    }

    pub async fn dispatch_delivery(&self, pickup_address: &str, dropoff_address: &str, order_id: &str) -> Result<String, String> {
        self.validate_credentials()?;
        if pickup_address.trim().is_empty() || dropoff_address.trim().is_empty() || order_id.trim().is_empty() {
            return Err("pickup address, dropoff address, and order id are required".to_string());
        }

        let external_delivery_id = Self::external_delivery_id(order_id);
        let payload = json!({
            "external_delivery_id": external_delivery_id,
            "pickup_address": pickup_address,
            "dropoff_address": dropoff_address,
        });

        let resp = self.http_client
            .post(format!("{}/drive/v2/deliveries", Self::api_base()))
            .bearer_auth(self.api_key.trim())
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("DoorDash delivery request failed: {e}"))?;

        let status = resp.status();
        let body: serde_json::Value = resp.json().await
            .map_err(|e| format!("DoorDash delivery response was not JSON: {e}"))?;
        if !status.is_success() {
            return Err(format!("DoorDash delivery API error {status}: {body}"));
        }

        Ok(body.get("external_delivery_id")
            .and_then(|v| v.as_str())
            .unwrap_or(&external_delivery_id)
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_delivery_quote_requires_real_credentials() {
        let client = DoorDashClient::new("dummy_key".to_string());
        let err = client.get_delivery_quote("123 Pickup St", "456 Dropoff Ave").await.unwrap_err();
        assert!(err.contains("DoorDash API key is required"));
    }

    #[tokio::test]
    async fn test_dispatch_delivery_requires_real_credentials() {
        let client = DoorDashClient::new("".to_string());
        let err = client.dispatch_delivery("123 Pickup St", "456 Dropoff Ave", "ord_123").await.unwrap_err();
        assert!(err.contains("DoorDash API key is required"));
    }
}
