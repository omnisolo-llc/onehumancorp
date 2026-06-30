use super::client::StripeClient;


pub struct TerminalSessionManager {
    client: StripeClient,
}

impl TerminalSessionManager {
    pub fn new(client: StripeClient) -> Self {
        Self { client }
    }

    pub async fn create_terminal_connection_token(&self, tenant_id: &str) -> Result<String, String> {
        if tenant_id.is_empty() {
            return Err("Unauthenticated: Missing tenant ID".to_string());
        }
        self.client.create_terminal_connection_token(tenant_id).await
    }

    pub async fn create_terminal_payment_intent(
        &self,
        tenant_id: &str,
        amount_cents: i64,
        currency: &str,
        product_id: Option<&str>,
        quantity: Option<i32>,
        order_id: Option<&str>,
        idempotency_key: &str,
    ) -> Result<(String, String), String> {
        if tenant_id.is_empty() {
            return Err("Unauthenticated: Missing tenant ID".to_string());
        }
        self.client.create_terminal_payment_intent(
            tenant_id,
            amount_cents,
            currency,
            product_id,
            quantity,
            order_id,
            idempotency_key
        ).await
    }
}

impl StripeClient {

    pub async fn create_terminal_connection_token(&self, _tenant_id: &str) -> Result<String, String> {
        let api_key = self.require_api_key()?;
        let res = reqwest::Client::new()
            .post(format!("{}/v1/terminal/connection_tokens", Self::api_base()))
            .basic_auth(api_key, Some(""))
            .form(&std::collections::HashMap::<String, String>::new())
            .send()
            .await
            .map_err(|e| format!("Stripe Terminal connection token request failed: {}", e))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Stripe Terminal API error ({}): {}", status, text));
        }

        let json: serde_json::Value = res
            .json()
            .await
            .map_err(|e| format!("Failed to parse Stripe Terminal token response: {}", e))?;
        json["secret"]
            .as_str()
            .map(|secret| secret.to_string())
            .ok_or_else(|| "Missing secret in Stripe Terminal token response".to_string())
    }

    pub async fn create_terminal_payment_intent(
        &self,
        tenant_id: &str,
        amount_cents: i64,
        currency: &str,
        product_id: Option<&str>,
        quantity: Option<i32>,
        order_id: Option<&str>,
        idempotency_key: &str,
    ) -> Result<(String, String), String> {
        let api_key = self.require_api_key()?;
        if amount_cents <= 0 {
            return Err("amount_cents must be positive".to_string());
        }
        if currency.trim().is_empty() {
            return Err("currency is required".to_string());
        }

        let mut form = std::collections::HashMap::new();
        form.insert("amount".to_string(), amount_cents.to_string());
        form.insert("currency".to_string(), currency.to_string());
        form.insert("payment_method_types[]".to_string(), "card_present".to_string());
        form.insert("capture_method".to_string(), "manual".to_string());
        form.insert("metadata[tenant_id]".to_string(), tenant_id.to_string());
        form.insert("metadata[source]".to_string(), "in_person".to_string());
        form.insert("metadata[idempotency_key]".to_string(), idempotency_key.to_string());

        if let Some(pid) = product_id {
            form.insert("metadata[product_id]".to_string(), pid.to_string());
        }
        if let Some(qty) = quantity {
            form.insert("metadata[quantity]".to_string(), qty.to_string());
        }
        if let Some(oid) = order_id {
            form.insert("metadata[order_id]".to_string(), oid.to_string());
        }

        let res = reqwest::Client::new().post(format!("{}/v1/payment_intents", Self::api_base()))
            .basic_auth(api_key, Some(""))
            .header("Idempotency-Key", idempotency_key)
            .form(&form)
            .send()
            .await
            .map_err(|e| format!("Stripe API request failed: {}", e))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Stripe API error ({}): {}", status, text));
        }

        let json: serde_json::Value = res.json().await.map_err(|e| format!("Failed to parse response: {}", e))?;
        let secret = json["client_secret"].as_str().ok_or_else(|| "Missing client_secret in response".to_string())?;
        let id = json["id"].as_str().ok_or_else(|| "Missing id in response".to_string())?;

        Ok((secret.to_string(), id.to_string()))
    }

    pub async fn capture_terminal_payment_intent(
        &self,
        payment_intent_id: &str,
    ) -> Result<String, String> {
        let api_key = self.require_api_key()?;
        let res = reqwest::Client::new()
            .post(format!("{}/v1/payment_intents/{}/capture", Self::api_base(), payment_intent_id))
            .basic_auth(api_key, Some(""))
            .send()
            .await
            .map_err(|e| format!("Stripe API capture request failed: {}", e))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Stripe API error ({}): {}", status, text));
        }

        let json: serde_json::Value = res.json().await.map_err(|e| format!("Failed to parse response: {}", e))?;
        json["status"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "Missing status in capture response".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_terminal_connection_token_requires_configured_key() {
        let client = StripeClient::new("".to_string());
        let result = client.create_terminal_connection_token("test_tenant").await;
        let err = result.expect_err("Terminal tokens must not be mocked when Stripe credentials are missing");
        assert!(err.contains("Stripe API key"));
    }

    #[tokio::test]
    async fn test_capture_terminal_payment_intent_requires_configured_key() {
        let client = StripeClient::new("".to_string());
        let result = client.capture_terminal_payment_intent("pi_test_123").await;
        let err = result.expect_err("Capture intent must not be mocked when Stripe credentials are missing");
        assert!(err.contains("Stripe API key"));
    }
}
