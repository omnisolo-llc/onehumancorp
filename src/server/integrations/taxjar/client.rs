use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRate {
    pub amount_to_collect: f64,
    pub rate: f64,
}

pub struct TaxJarParams<'a> {
    pub amount: f64,
    pub shipping: f64,
    pub to_country: &'a str,
    pub to_zip: &'a str,
    pub to_state: &'a str,
    pub from_country: &'a str,
    pub from_zip: &'a str,
    pub from_state: &'a str,
}

pub struct TaxJarClient {
    pub api_key: String,
    http_client: reqwest::Client,
}

impl TaxJarClient {
    pub fn new(api_key: String) -> Self {
        TaxJarClient {
            api_key,
            http_client: reqwest::Client::new(),
        }
    }

    fn api_base() -> String {
        std::env::var("TAXJAR_API_BASE")
            .unwrap_or_else(|_| "https://api.taxjar.com/v2".to_string())
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
            return Err("TaxJar API token is required".to_string());
        }
        Ok(())
    }

    pub async fn calculate_tax(&self, params: TaxJarParams<'_>) -> Result<TaxRate, String> {
        self.validate_credentials()?;

        let payload = json!({
            "from_country": params.from_country,
            "from_zip": params.from_zip,
            "from_state": params.from_state,
            "to_country": params.to_country,
            "to_zip": params.to_zip,
            "to_state": params.to_state,
            "amount": params.amount,
            "shipping": params.shipping,
        });

        let resp = self.http_client
            .post(format!("{}/taxes", Self::api_base()))
            .header("Authorization", format!("Bearer {}", self.api_key.trim()))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("TaxJar request failed: {e}"))?;

        let status = resp.status();
        let body: serde_json::Value = resp.json().await
            .map_err(|e| format!("TaxJar response was not JSON: {e}"))?;

        if !status.is_success() {
            return Err(format!("TaxJar API error {status}: {body}"));
        }

        let tax = body.get("tax")
            .ok_or_else(|| "TaxJar response missing tax object".to_string())?;

        let amount_to_collect = tax.get("amount_to_collect")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let rate = tax.get("rate")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        Ok(TaxRate {
            amount_to_collect,
            rate,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn calculate_tax_requires_real_taxjar_credentials() {
        let client = TaxJarClient::new("dummy_token".to_string());
        let params = TaxJarParams {
            amount: 100.0,
            shipping: 10.0,
            to_country: "US",
            to_zip: "90002",
            to_state: "CA",
            from_country: "US",
            from_zip: "92093",
            from_state: "CA",
        };
        let err = client.calculate_tax(params).await.unwrap_err();
        assert!(err.contains("TaxJar API token is required"));
    }
}
