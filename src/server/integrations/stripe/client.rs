use serde::{Deserialize, Serialize};
#[cfg(ohc_bazel)]
use crate::integrations::mercadopago::client::MercadoPagoClient;
#[cfg(not(ohc_bazel))]
use server_integrations_mercadopago::client::MercadoPagoClient;

use super::payout_batcher::PayoutBatcher;
use super::routing::{PaymentMethod, PaymentRouter};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StripeSubscription {
    pub id: String,
    pub status: String,
    pub current_period_end: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StripeInvoice {
    pub id: String,
    pub amount_due: i64,
    pub status: String,
    pub invoice_pdf: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StripeCustomer {
    pub id: String,
    pub email: Option<String>,
}

pub struct StripeClient {
    pub api_key: String,
}

impl StripeClient {
    pub fn new(api_key: String) -> Self {
        StripeClient { api_key }
    }

    pub async fn create_checkout_session(&self, _price_id: &str, customer_id: &str, amount_usd: f64) -> Result<String, String> {

        // Use PaymentRouter to optimize method
        let pm = PaymentRouter::optimize_payment_method(amount_usd);

        match pm {
            PaymentMethod::Ach => {
                Ok("https://checkout.stripe.com/c/pay/cs_test_ach...".to_string())
            },
            PaymentMethod::CreditCard => {
                Ok("https://checkout.stripe.com/c/pay/cs_test_...".to_string())
            },
            PaymentMethod::Razorpay => {
                // Return razorpay checkout dummy link here since routing was updated
                Ok("https://checkout.razorpay.com/pay/cs_test_...".to_string())
            },
            PaymentMethod::MercadoPago => {
                if let Ok(token) = std::env::var("MERCADOPAGO_ACCESS_TOKEN") {
                    let mp_client = MercadoPagoClient::new(token);
                    mp_client.create_checkout_preference(_price_id, customer_id).await
                } else {
                    Ok("https://www.mercadopago.com.br/checkout/v1/redirect?pref_id=mock_pref_123".to_string())
                }
            }
        }
    }

    pub async fn create_terminal_connection_token(&self, tenant_id: &str) -> Result<String, String> {

        // In a real implementation, this would make an HTTP POST to Stripe's /v1/terminal/connection_tokens
        // endpoint. Since we're mocking external APIs, we return a mock token string here.
        // We simulate the token being tightly scoped to the tenant for multi-tenant isolation.
        let mock_token = format!("tss_mock_token_for_{}", tenant_id);

        Ok(mock_token)
    }

    pub async fn get_subscription(&self, _subscription_id: &str) -> Result<StripeSubscription, String> {
        Ok(StripeSubscription {
            id: "sub_test_...".to_string(),
            status: "active".to_string(),
            current_period_end: 1714560000,
        })
    }

    pub async fn list_invoices(&self, _customer_id: &str) -> Result<Vec<StripeInvoice>, String> {
        Ok(vec![
            StripeInvoice {
                id: "in_test_...".to_string(),
                amount_due: 2900,
                status: "paid".to_string(),
                invoice_pdf: Some("https://pay.stripe.com/invoice/acct_.../pdf".to_string()),
            }
        ])
    }

    pub async fn cancel_subscription(&self, _subscription_id: &str) -> Result<StripeSubscription, String> {
        Ok(StripeSubscription {
            id: "sub_test_...".to_string(),
            status: "canceled".to_string(),
            current_period_end: 1714560000,
        })
    }
}

impl StripeClient {
    /// Dispatches a batch payout check. If batched amount > threshold, actually performs payout.
    pub async fn process_payout_with_batching(
        &self,
        account_id: &str,
        amount_cents: i64,
        batcher: &PayoutBatcher,
    ) -> Result<Option<String>, String> {
        let payout_amount = batcher.record_payout(account_id, amount_cents).await?;
        if let Some(total_cents) = payout_amount {

            // Execute real payout call here...
            Ok(Some(format!("po_test_{}", total_cents)))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_terminal_connection_token() {
        let client = StripeClient::new("sk_test_123".to_string());
        let result = client.create_terminal_connection_token("test_tenant").await;
        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token, "tss_mock_token_for_test_tenant");
    }
}

impl StripeClient {
    pub async fn create_terminal_payment_intent(&self, tenant_id: &str, amount_cents: i64, currency: &str) -> Result<String, String> {
        let mut form = std::collections::HashMap::new();
        form.insert("amount".to_string(), amount_cents.to_string());
        form.insert("currency".to_string(), currency.to_string());
        form.insert("payment_method_types[]".to_string(), "card_present".to_string());
        form.insert("capture_method".to_string(), "manual".to_string());
        form.insert("metadata[tenant_id]".to_string(), tenant_id.to_string());

        let res = reqwest::Client::new().post("https://api.stripe.com/v1/payment_intents")
            .basic_auth(&self.api_key, Some(""))
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

        Ok(secret.to_string())
    }
}
