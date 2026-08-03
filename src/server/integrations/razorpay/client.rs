pub struct RazorpayClient {
    pub api_key: String,
    pub api_secret: String,
    http_client: reqwest::Client,
}

impl RazorpayClient {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_key,
            api_secret,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn create_checkout_preference(&self, price_id: &str, tenant_id: &str) -> Result<String, String> {
        self.create_payment(0.0, &format!("Checkout {price_id}"), tenant_id).await
    }

    pub async fn create_payment(&self, amount: f64, description: &str, receipt: &str) -> Result<String, String> {
        if self.api_key.trim().is_empty()
            || self.api_secret.trim().is_empty()
            || self.api_key.contains("test")
            || self.api_key.contains("dummy")
            || self.api_secret.contains("dummy")
        {
            return Err("Razorpay API credentials are required".to_string());
        }
        if amount < 0.0 {
            return Err("Razorpay amount must be non-negative".to_string());
        }

        let base_url = std::env::var("RAZORPAY_API_BASE")
            .unwrap_or_else(|_| "https://api.razorpay.com/v1".to_string())
            .trim_end_matches('/')
            .to_string();
        let payload = serde_json::json!({
            "amount": (amount * 100.0).round() as i64,
            "currency": "INR",
            "receipt": receipt,
            "notes": {
                "description": description
            }
        });

        let resp = self.http_client
            .post(format!("{base_url}/orders"))
            .basic_auth(self.api_key.trim(), Some(self.api_secret.trim()))
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Razorpay order request failed: {e}"))?;

        let status = resp.status();
        let body: serde_json::Value = resp.json().await
            .map_err(|e| format!("Razorpay order response was not JSON: {e}"))?;
        if !status.is_success() {
            return Err(format!("Razorpay API error {status}: {body}"));
        }

        body.get("id")
            .and_then(|value| value.as_str())
            .map(|id| id.to_string())
            .ok_or_else(|| "Razorpay order response missing id".to_string())
    }

    pub async fn handle_webhook(&self, payload: &str) -> Result<(), String> {
        let value: serde_json::Value = serde_json::from_str(payload)
            .map_err(|e| format!("Invalid Razorpay webhook JSON: {e}"))?;
        if value.get("event").and_then(|event| event.as_str()).is_none() {
            return Err("Razorpay webhook missing event".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_payment_requires_real_credentials() {
        let client = RazorpayClient::new("dummy_key".to_string(), "dummy_secret".to_string());
        let err = client.create_payment(100.0, "Order", "receipt-1").await.unwrap_err();
        assert!(err.contains("Razorpay API credentials are required"));
    }

    #[tokio::test]
    async fn handle_webhook_requires_event() {
        let client = RazorpayClient::new("rzp_live_key".to_string(), "secret".to_string());
        let err = client.handle_webhook("{}").await.unwrap_err();
        assert_eq!(err, "Razorpay webhook missing event");
    }
}
