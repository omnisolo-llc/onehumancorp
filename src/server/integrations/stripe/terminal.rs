use super::client::StripeClient;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TerminalIntentReceipt {
    pub id: String,
    pub amount: i64,
    pub amount_received: i64,
    pub amount_capturable: i64,
    pub currency: String,
    pub status: String,
    pub metadata: std::collections::HashMap<String,String>,
    #[serde(default, skip_serializing)]
    pub client_secret: Option<String>,
}


/// Authenticated tenant, amount and replay identity for one terminal operation.
#[derive(Clone, Copy)]
pub struct TerminalPaymentRequest<'a> {
    pub tenant_id: &'a str,
    pub amount_cents: i64,
    pub currency: &'a str,
    pub product_id: Option<&'a str>,
    pub quantity: Option<i32>,
    pub order_id: Option<&'a str>,
    pub idempotency_key: &'a str,
}

pub struct TerminalSessionManager {
    client: StripeClient,
}

impl TerminalSessionManager {
    pub fn new(client: StripeClient) -> Self {
        Self { client }
    }

    pub async fn create_terminal_connection_token(
        &self,
        tenant_id: &str,
    ) -> Result<String, String> {
        if tenant_id.is_empty() {
            return Err("Unauthenticated: Missing tenant ID".to_string());
        }
        self.client
            .create_terminal_connection_token(tenant_id)
            .await
    }

    pub async fn create_terminal_payment_intent(
        &self,
        request: TerminalPaymentRequest<'_>,
    ) -> Result<(String, String), String> {
        if request.tenant_id.is_empty() {
            return Err("Unauthenticated: Missing tenant ID".to_string());
        }
        self.client.create_terminal_payment_intent(request).await
    }
}

impl StripeClient {
    pub async fn create_terminal_connection_token(
        &self,
        _tenant_id: &str,
    ) -> Result<String, String> {
        let api_key = self.require_api_key()?;
        let res = reqwest::Client::new()
            .post(format!(
                "{}/v1/terminal/connection_tokens",
                Self::api_base()
            ))
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

    pub async fn create_terminal_payment_intent_receipt(
        &self,
        request: TerminalPaymentRequest<'_>,
    ) -> Result<TerminalIntentReceipt, String> {
        let TerminalPaymentRequest {
            tenant_id,
            amount_cents,
            currency,
            product_id,
            quantity,
            order_id,
            idempotency_key,
        } = request;
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
        form.insert(
            "payment_method_types[]".to_string(),
            "card_present".to_string(),
        );
        form.insert("capture_method".to_string(), "manual".to_string());
        form.insert("metadata[tenant_id]".to_string(), tenant_id.to_string());
        form.insert("metadata[source]".to_string(), "in_person".to_string());
        form.insert(
            "metadata[idempotency_key]".to_string(),
            idempotency_key.to_string(),
        );

        if let Some(pid) = product_id {
            form.insert("metadata[product_id]".to_string(), pid.to_string());
        }
        if let Some(qty) = quantity {
            form.insert("metadata[quantity]".to_string(), qty.to_string());
        }
        if let Some(oid) = order_id {
            form.insert("metadata[order_id]".to_string(), oid.to_string());
        }

        let res = reqwest::Client::new()
            .post(format!("{}/v1/payment_intents", Self::api_base()))
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

        res.json::<TerminalIntentReceipt>().await
            .map_err(|_| "Invalid Stripe terminal intent receipt".to_string())
    }

    pub async fn create_terminal_payment_intent(
        &self,
        request: TerminalPaymentRequest<'_>,
    ) -> Result<(String, String), String> {
        let receipt = self.create_terminal_payment_intent_receipt(request).await?;
        let secret = receipt.client_secret.ok_or("Missing client secret")?;
        Ok((receipt.id, secret))
    }

    async fn terminal_intent_receipt_request(&self, id: &str, capture_amount: Option<i64>) -> Result<TerminalIntentReceipt, String> {
        if !id.starts_with("pi_") || id.len() <= 3 || id.len() > 128
            || !id.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'_') {
            return Err("Invalid payment intent ID".into());
        }
        if capture_amount.is_some_and(|amount| !(1..=99_999_999).contains(&amount)) {
            return Err("A positive bounded capture amount is required".into());
        }
        let key = self.require_api_key()?;
        let http = reqwest::Client::builder().timeout(std::time::Duration::from_secs(20))
            .redirect(reqwest::redirect::Policy::none()).build().map_err(|_| "Provider transport unavailable")?;
        let url = format!("{}/v1/payment_intents/{}{}", Self::api_base(), id, if capture_amount.is_some() {"/capture"} else {""});
        let request = if let Some(amount) = capture_amount {
            http.post(url)
                .header("Idempotency-Key", format!("ohc_terminal_capture:{id}:{amount}"))
                .form(&[("amount_to_capture", amount.to_string())])
        } else { http.get(url) };
        let response = request.basic_auth(key, Some("")).send().await.map_err(|_| "Terminal provider outcome is unconfirmed")?;
        if !response.status().is_success() { return Err("Terminal provider did not confirm the requested operation".into()); }
        response.json().await.map_err(|_| "Invalid terminal provider receipt".into())
    }

    pub async fn retrieve_terminal_payment_intent(&self, id: &str) -> Result<TerminalIntentReceipt, String> {
        self.terminal_intent_receipt_request(id, None).await
    }

    pub async fn capture_terminal_payment_intent_receipt(&self, id: &str, amount_cents: i64) -> Result<TerminalIntentReceipt, String> {
        self.terminal_intent_receipt_request(id, Some(amount_cents)).await
    }

    /// Compatibility entry point: no caller may capture by ID without a saved
    /// authorized amount. Legacy offline callers require reconciliation first.
    pub async fn capture_terminal_payment_intent(
        &self,
        _payment_intent_id: &str,
    ) -> Result<String, String> {
        self.require_api_key()?;
        Err("A persisted authorized capture amount is required; reconcile the original operation.".into())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_terminal_connection_token_requires_configured_key() {
        let client = StripeClient::new("".to_string());
        let result = client.create_terminal_connection_token("test_tenant").await;
        let err = result
            .expect_err("Terminal tokens must not be mocked when Stripe credentials are missing");
        assert!(err.contains("Stripe API key"));
    }

    #[tokio::test]
    async fn test_create_terminal_payment_intent_requires_configured_key() {
        let client = StripeClient::new("".to_string());
        let result = client
            .create_terminal_payment_intent(TerminalPaymentRequest {
                tenant_id: "test_tenant",
                amount_cents: 1000,
                currency: "usd",
                product_id: None,
                quantity: None,
                order_id: None,
                idempotency_key: "idempotency_key",
            })
            .await;
        let err = result
            .expect_err("Create intent must not be mocked when Stripe credentials are missing");
        assert!(err.contains("Stripe API key"));
    }

    #[tokio::test]
    async fn test_capture_terminal_payment_intent_requires_configured_key() {
        let client = StripeClient::new("".to_string());
        let result = client.capture_terminal_payment_intent("pi_test_123").await;
        let err = result
            .expect_err("Capture intent must not be mocked when Stripe credentials are missing");
        assert!(err.contains("Stripe API key"));
    }
}
