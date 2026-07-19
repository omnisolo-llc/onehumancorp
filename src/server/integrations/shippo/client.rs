use serde::{Deserialize, Serialize};
use serde_json::json;

const MAX_PARCEL_VALUE: f64 = 100_000.0;

fn parcel_dimensions(dimensions: &str) -> Result<(String, String, String), String> {
    let values = dimensions
        .split(['x', 'X'])
        .map(str::trim)
        .map(str::parse::<f64>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "parcel dimensions must contain three positive numbers".to_string())?;
    let [length, width, height] = values.as_slice() else {
        return Err("parcel dimensions must contain length, width, and height".to_string());
    };
    if !values
        .iter()
        .all(|value| value.is_finite() && *value > 0.0 && *value <= MAX_PARCEL_VALUE)
    {
        return Err("parcel dimensions must contain three positive numbers".to_string());
    }
    Ok((length.to_string(), width.to_string(), height.to_string()))
}

fn trusted_label_url(raw_url: &str) -> Option<String> {
    let url = reqwest::Url::parse(raw_url).ok()?;
    let host = url.host_str()?;
    let trusted_host = host == "goshippo.com"
        || host.ends_with(".goshippo.com")
        || host == "shippo-delivery.s3.amazonaws.com"
        || host == "shippo-delivery-east.s3.amazonaws.com"
        || host == "shippo-delivery-west.s3.amazonaws.com";
    (url.scheme() == "https"
        && trusted_host
        && url.username().is_empty()
        && url.password().is_none())
    .then(|| url.to_string())
}

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

    pub async fn fetch_rates(
        &self,
        weight: f64,
        dimensions: &str,
    ) -> Result<Vec<ShippoRate>, String> {
        self.validate_credentials()?;
        if weight <= 0.0 {
            return Err("shipment weight must be positive".to_string());
        }

        let (length, width, height) = parcel_dimensions(dimensions)?;

        let address_from = Self::configured_address("SHIPPO_ADDRESS_FROM_JSON")?;
        let address_to = Self::configured_address("SHIPPO_ADDRESS_TO_JSON")?;
        let parcel = json!({
            "length": length,
            "width": width,
            "height": height,
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

        let resp = self
            .http_client
            .post(format!("{}/shipments", Self::api_base()))
            .header(
                "Authorization",
                format!("ShippoToken {}", self.api_key.trim()),
            )
            .header("Content-Type", "application/json")
            .header("SHIPPO-API-VERSION", "2018-02-08")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Shippo shipment request failed: {e}"))?;

        let status = resp.status();
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Shippo rates response was not JSON: {e}"))?;
        if !status.is_success() {
            return Err(format!("Shippo rates API error {status}: {body}"));
        }

        let rates = body
            .get("rates")
            .and_then(|v| v.as_array())
            .ok_or_else(|| "Shippo rates response missing rates".to_string())?;

        Ok(rates
            .iter()
            .filter_map(|rate| {
                let id = rate
                    .get("object_id")
                    .or_else(|| rate.get("id"))
                    .and_then(|v| v.as_str())?;
                let carrier = rate
                    .get("provider")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Shippo");
                let service = rate
                    .get("servicelevel")
                    .and_then(|v| v.get("name"))
                    .and_then(|v| v.as_str())
                    .or_else(|| rate.get("service").and_then(|v| v.as_str()))
                    .unwrap_or("Service");
                let amount = rate
                    .get("amount")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let days = rate
                    .get("estimated_days")
                    .and_then(|v| v.as_u64())
                    .unwrap_or_default() as u32;
                Some(ShippoRate {
                    id: id.to_string(),
                    carrier: carrier.to_string(),
                    service: service.to_string(),
                    amount: amount.to_string(),
                    days,
                })
            })
            .collect())
    }

    pub async fn purchase_label(&self, rate_id: &str) -> Result<PurchaseLabelResponse, String> {
        self.validate_credentials()?;
        if rate_id.trim().is_empty() {
            return Err("Shippo rate id is required".to_string());
        }

        let resp = self
            .http_client
            .post(format!("{}/transactions", Self::api_base()))
            .header(
                "Authorization",
                format!("ShippoToken {}", self.api_key.trim()),
            )
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
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Shippo label response was not JSON: {e}"))?;
        if !status.is_success() {
            return Err(format!("Shippo label API error {status}: {body}"));
        }

        let label_url = body
            .get("label_url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Shippo label response missing label_url".to_string())?;
        let label_url = trusted_label_url(label_url)
            .ok_or_else(|| "Shippo label response returned an untrusted label URL".to_string())?;
        let tracking_number = body
            .get("tracking_number")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let carrier = body
            .get("tracking_carrier")
            .or_else(|| body.get("carrier"))
            .and_then(|v| v.as_str())
            .unwrap_or("Shippo");

        Ok(PurchaseLabelResponse {
            success: true,
            label_url,
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
        let err = client.fetch_rates(16.0, "10x8x4").await.unwrap_err();
        assert!(err.contains("Shippo API token is required"));
    }

    #[tokio::test]
    async fn purchase_label_requires_real_shippo_credentials() {
        let client = ShippoClient::new("".to_string());
        let err = client.purchase_label("rate_123").await.unwrap_err();
        assert!(err.contains("Shippo API token is required"));
    }

    #[test]
    fn parcel_dimensions_are_split_into_shippo_fields() {
        assert_eq!(
            parcel_dimensions("10x8x6"),
            Ok(("10".to_string(), "8".to_string(), "6".to_string()))
        );
        assert!(parcel_dimensions("10x8").is_err());
    }

    #[test]
    fn label_urls_are_limited_to_https_shippo_delivery_hosts_without_userinfo() {
        assert!(
            trusted_label_url(
                "https://shippo-delivery-east.s3.amazonaws.com/label.pdf?signature=needed"
            )
            .is_some()
        );
        assert!(trusted_label_url("https://app.goshippo.com/labels/label.pdf").is_some());
        assert!(trusted_label_url("https://attacker.example/label.pdf").is_none());
        assert!(trusted_label_url("https://shippo-delivery-attacker.s3.amazonaws.com/label.pdf").is_none());
        assert!(trusted_label_url("https://user:password@app.goshippo.com/label.pdf").is_none());
        assert!(trusted_label_url("http://app.goshippo.com/label.pdf").is_none());
    }
}
