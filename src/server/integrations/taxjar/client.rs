use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRate {
    #[serde(rename = "amount_to_collect", with = "super::money")]
    pub amount_to_collect_cents: i64,
    #[serde(default)]
    pub rate: f64,
}

#[derive(Serialize)]
pub struct TaxJarParams<'a> {
    #[serde(rename = "amount", serialize_with = "super::money::serialize")]
    pub amount_cents: i64,
    #[serde(rename = "shipping", serialize_with = "super::money::serialize")]
    pub shipping_cents: i64,
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

        let resp = self
            .http_client
            .post(format!("{}/taxes", Self::api_base()))
            .header("Authorization", format!("Bearer {}", self.api_key.trim()))
            .header("Content-Type", "application/json")
            .json(&params)
            .send()
            .await
            .map_err(|e| format!("TaxJar request failed: {e}"))?;

        let status = resp.status();
        let body = resp
            .bytes()
            .await
            .map_err(|e| format!("TaxJar response could not be read: {e}"))?;
        if !status.is_success() {
            return Err(format!(
                "TaxJar API error {status}: {}",
                String::from_utf8_lossy(&body)
            ));
        }

        #[derive(Deserialize)]
        struct TaxResponse {
            tax: TaxRate,
        }
        // Decode directly from the response bytes. An intermediate JSON Value
        // would round large decimal numbers before our money boundary sees them.
        serde_json::from_slice::<TaxResponse>(&body)
            .map(|response| response.tax)
            .map_err(|e| format!("TaxJar response contains invalid tax: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn calculate_tax_requires_real_taxjar_credentials() {
        let client = TaxJarClient::new("dummy_token".to_string());
        let params = TaxJarParams {
            amount_cents: 10_000,
            shipping_cents: 1_000,
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
