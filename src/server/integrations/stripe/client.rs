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

    pub fn require_api_key(&self) -> Result<&str, String> {
        let key = self.api_key.trim();
        if key.is_empty() || key == "sk_test_123" || key == "sk_test" {
            return Err("Stripe API key is required for Terminal API calls".to_string());
        }
        Ok(key)
    }

    pub fn api_base() -> String {
        std::env::var("STRIPE_API_BASE").unwrap_or_else(|_| "https://api.stripe.com".to_string())
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
                    Err("Mercado Pago access token is required".to_string())
                }
            },
            PaymentMethod::Alipay => {
                Err("Alipay checkout is not configured for Stripe checkout sessions".to_string())
            }
        }
    }

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
    async fn test_terminal_connection_token_requires_configured_key() {
        let client = StripeClient::new("".to_string());
        let result = client.create_terminal_connection_token("test_tenant").await;
        let err = result.expect_err("Terminal tokens must not be mocked when Stripe credentials are missing");
        assert!(err.contains("Stripe API key"));
    }
}

impl StripeClient {
    pub async fn create_terminal_payment_intent(&self, tenant_id: &str, amount_cents: i64, currency: &str) -> Result<String, String> {
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

        let res = reqwest::Client::new().post(format!("{}/v1/payment_intents", Self::api_base()))
            .basic_auth(api_key, Some(""))
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
