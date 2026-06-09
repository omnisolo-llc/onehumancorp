use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippoRate {
    pub id: String,
    pub carrier: String,
    pub service: String,
    pub amount: String,
    pub days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseLabelResponse {
    pub success: bool,
    #[serde(rename = "labelUrl")]
    pub label_url: String,
    #[serde(rename = "trackingNumber")]
    pub tracking_number: String,
    pub carrier: String,
}

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

    pub async fn fetch_rates(&self, weight: f64, dimensions: &str, address_to: Option<serde_json::Value>) -> Result<Vec<ShippoRate>, String> {
        self.validate_credentials()?;
        if weight <= 0.0 {
            return Err("shipment weight must be positive".to_string());
        }

        let address_from = Self::configured_address("SHIPPO_ADDRESS_FROM_JSON")?;
        let mut addr_to = address_to.unwrap_or_else(|| Self::configured_address("SHIPPO_ADDRESS_TO_JSON").unwrap_or_default());
        if addr_to.is_object() {
            addr_to.as_object_mut().unwrap().insert("validate".to_string(), json!(true));
        }

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
            "address_to": addr_to,
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
            let id = rate.get("object_id")
                .or_else(|| rate.get("id"))
                .and_then(|v| v.as_str())?;
            let carrier = rate.get("provider")
                .and_then(|v| v.as_str())
                .unwrap_or("Shippo");
            let service = rate.get("servicelevel")
                .and_then(|v| v.get("name"))
                .and_then(|v| v.as_str())
                .or_else(|| rate.get("service").and_then(|v| v.as_str()))
                .unwrap_or("Service");
            let amount = rate.get("amount").and_then(|v| v.as_str()).unwrap_or_default();
            let days = rate.get("estimated_days")
                .and_then(|v| v.as_u64())
                .unwrap_or_default() as u32;
            Some(ShippoRate {
                id: id.to_string(),
                carrier: carrier.to_string(),
                service: service.to_string(),
                amount: amount.to_string(),
                days,
            })
        }).collect())
    }

    pub async fn validate_address(&self, address: &serde_json::Value) -> Result<bool, String> {
        self.validate_credentials()?;

        let mut payload = address.clone();
        if payload.is_object() {
            payload.as_object_mut().unwrap().insert("validate".to_string(), json!(true));
        }

        let resp = self.http_client
            .post(format!("{}/addresses/", Self::api_base()))
            .header("Authorization", format!("ShippoToken {}", self.api_key.trim()))
            .header("Content-Type", "application/json")
            .header("SHIPPO-API-VERSION", "2018-02-08")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Shippo address request failed: {e}"))?;

        let status = resp.status();
        let body: serde_json::Value = resp.json().await
            .map_err(|e| format!("Shippo address response was not JSON: {e}"))?;

        if !status.is_success() {
            return Err(format!("Shippo address API error {status}: {body}"));
        }

        let is_valid = body.get("validation_results")
            .and_then(|v| v.get("is_valid"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        Ok(is_valid)
    }

    pub async fn create_sub_account(&self, email: &str, company_name: &str) -> Result<String, String> {
        self.validate_credentials()?;

        let resp = self.http_client
            .post(format!("{}/shippo-accounts/", Self::api_base()))
            .header("Authorization", format!("ShippoToken {}", self.api_key.trim()))
            .header("Content-Type", "application/json")
            .header("SHIPPO-API-VERSION", "2018-02-08")
            .json(&json!({
                "email": email,
                "first_name": "Tenant",
                "last_name": "Owner",
                "company_name": company_name,
                "platform": "OneHumanCorp"
            }))
            .send()
            .await
            .map_err(|e| format!("Shippo sub-account request failed: {e}"))?;

        let status = resp.status();
        let body: serde_json::Value = resp.json().await
            .map_err(|e| format!("Shippo sub-account response was not JSON: {e}"))?;

        if !status.is_success() {
            return Err(format!("Shippo sub-account API error {status}: {body}"));
        }

        let object_id = body.get("object_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Shippo sub-account response missing object_id".to_string())?;

        Ok(object_id.to_string())
    }

    pub async fn purchase_label(&self, rate_id: &str) -> Result<PurchaseLabelResponse, String> {
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

        let label_url = body.get("label_url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Shippo label response missing label_url".to_string())?;
        let tracking_number = body.get("tracking_number")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let carrier = body.get("tracking_carrier")
            .or_else(|| body.get("carrier"))
            .and_then(|v| v.as_str())
            .unwrap_or("Shippo");

        Ok(PurchaseLabelResponse {
            success: true,
            label_url: label_url.to_string(),
            tracking_number: tracking_number.to_string(),
            carrier: carrier.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_rates_requires_real_shippo_credentials() {
        let client = ShippoClient::new("dummy_token".to_string());
        let err = client.fetch_rates(16.0, "10x8x4", None).await.unwrap_err();
        assert!(err.contains("Shippo API token is required"));
    }

    #[tokio::test]
    async fn purchase_label_requires_real_shippo_credentials() {
        let client = ShippoClient::new("".to_string());
        let err = client.purchase_label("rate_123").await.unwrap_err();
        assert!(err.contains("Shippo API token is required"));
    }
}
