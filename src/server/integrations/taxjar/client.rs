use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxJarTaxResponse {
    pub tax: TaxJarTax,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxJarTax {
    pub amount_to_collect: f64,
    pub rate: f64,
    pub taxable_amount: f64,
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

    pub async fn calculate_tax(&self, amount: f64, shipping: f64, to_country: &str, to_zip: &str, to_state: &str) -> Result<TaxJarTaxResponse, String> {
        self.validate_credentials()?;

        let payload = json!({
            "from_country": "US",
            "from_zip": "92093",
            "from_state": "CA",
            "to_country": to_country,
            "to_zip": to_zip,
            "to_state": to_state,
            "amount": amount,
            "shipping": shipping,
        });

        let resp = self.http_client
            .post(format!("{}/taxes", Self::api_base()))
            .header("Authorization", format!("Bearer {}", self.api_key.trim()))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("TaxJar calculate tax request failed: {e}"))?;

        let status = resp.status();
        let body: serde_json::Value = resp.json().await
            .map_err(|e| format!("TaxJar response was not JSON: {e}"))?;

        if !status.is_success() {
            return Err(format!("TaxJar API error {status}: {body}"));
        }

        let tax = body.get("tax")
            .ok_or_else(|| "TaxJar response missing tax object".to_string())?;

        let amount_to_collect = tax.get("amount_to_collect").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let rate = tax.get("rate").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let taxable_amount = tax.get("taxable_amount").and_then(|v| v.as_f64()).unwrap_or(0.0);

        Ok(TaxJarTaxResponse {
            tax: TaxJarTax {
                amount_to_collect,
                rate,
                taxable_amount,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn calculate_tax_requires_real_taxjar_credentials() {
        let client = TaxJarClient::new("dummy_token".to_string());
        let err = client.calculate_tax(15.0, 1.5, "US", "90002", "CA").await.unwrap_err();
        assert!(err.contains("TaxJar API token is required"));
    }
}
