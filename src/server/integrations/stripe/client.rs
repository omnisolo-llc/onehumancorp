use serde::{Deserialize, Serialize};

use super::payout_batcher::PayoutBatcher;

#[cfg(test)]
mod missing_configuration_tests {
    use super::*;
    #[tokio::test]
    async fn absent_provider_never_fabricates_external_success() {
        for key in ["", "sk_test_123", "sk_test_mock", "placeholder"] {
            let client = StripeClient::new(key.into());
            assert!(client.create_payment_link("service", 100).await.is_err());
            assert!(
                client
                    .create_checkout_session("service", "client", 1.0, None, None, None)
                    .await
                    .is_err()
            );
            assert!(
                client
                    .create_billing_portal_session("cus_fixture", None)
                    .await
                    .is_err()
            );
            assert!(client.cancel_subscription("sub_fixture").await.is_err());
            assert!(
                client
                    .create_draft_invoice("cus_fixture", 100, "service")
                    .await
                    .is_err()
            );
            assert!(
                client
                    .finalize_and_send_invoice("in_fixture")
                    .await
                    .is_err()
            );
        }
    }
}

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
        if key.is_empty()
            || key == "sk_test_123"
            || key == "sk_test"
            || key.contains("mock")
            || key.contains("placeholder")
            || key.len() > 4096
            || key.chars().any(char::is_control)
        {
            return Err("Stripe API key is required".to_string());
        }
        Ok(key)
    }

    pub fn api_base() -> String {
        std::env::var("STRIPE_API_BASE").unwrap_or_else(|_| "https://api.stripe.com".to_string())
    }

    pub async fn create_payment_link(
        &self,
        name: &str,
        amount_cents: i64,
    ) -> Result<String, String> {
        let api_key = self.require_api_key()?;
        if name.trim().is_empty() || !(1..=99_999_999).contains(&amount_cents) {
            return Err("A description and valid amount are required".into());
        }
        let client = reqwest::Client::new();

        // 1. Create a Product
        let mut product_form = std::collections::HashMap::new();
        product_form.insert("name".to_string(), name.to_string());

        let product_res = client
            .post(format!("{}/v1/products", Self::api_base()))
            .basic_auth(api_key, Some(""))
            .form(&product_form)
            .send()
            .await
            .map_err(|e| format!("Stripe API error creating product: {}", e))?;

        if !product_res.status().is_success() {
            return Err(format!(
                "Stripe API error creating product: {}",
                product_res.text().await.unwrap_or_default()
            ));
        }

        let product_json: serde_json::Value =
            product_res.json().await.map_err(|e| e.to_string())?;
        let product_id = product_json["id"].as_str().unwrap_or_default().to_string();

        // 2. Create a Price
        let mut price_form = std::collections::HashMap::new();
        price_form.insert("product".to_string(), product_id);
        price_form.insert("currency".to_string(), "usd".to_string());
        price_form.insert("unit_amount".to_string(), amount_cents.to_string());

        let price_res = client
            .post(format!("{}/v1/prices", Self::api_base()))
            .basic_auth(api_key, Some(""))
            .form(&price_form)
            .send()
            .await
            .map_err(|e| format!("Stripe API error creating price: {}", e))?;

        if !price_res.status().is_success() {
            return Err(format!(
                "Stripe API error creating price: {}",
                price_res.text().await.unwrap_or_default()
            ));
        }
        let price_json: serde_json::Value = price_res.json().await.map_err(|e| e.to_string())?;
        let price_id = price_json["id"].as_str().unwrap_or_default().to_string();

        // 3. Create Payment Link
        let mut link_form = std::collections::HashMap::new();
        link_form.insert("line_items[0][price]".to_string(), price_id);
        link_form.insert("line_items[0][quantity]".to_string(), "1".to_string());

        let link_res = client
            .post(format!("{}/v1/payment_links", Self::api_base()))
            .basic_auth(api_key, Some(""))
            .form(&link_form)
            .send()
            .await
            .map_err(|e| format!("Stripe API error creating payment link: {}", e))?;

        if !link_res.status().is_success() {
            return Err(format!(
                "Stripe API error creating payment link: {}",
                link_res.text().await.unwrap_or_default()
            ));
        }

        let link_json: serde_json::Value = link_res.json().await.map_err(|e| e.to_string())?;
        let url = link_json["url"].as_str().unwrap_or_default().to_string();

        Ok(url)
    }

    pub async fn create_checkout_session(
        &self,
        price_id_or_name: &str,
        customer_id: &str,
        amount_usd: f64,
        subscription_interval: Option<String>,
        product_id: Option<String>,
        target_currency: Option<String>,
    ) -> Result<String, String> {
        // Legacy callers represent a new interactive checkout. Durable workflows
        // must use create_checkout_session_idempotent with their persisted ID.
        let amount_cents = super::safe_checkout::money_minor_units(amount_usd)?;
        let operation_id = format!("checkout:{}", uuid::Uuid::new_v4());
        let receipt = self
            .create_checkout_session_idempotent(super::safe_checkout::CheckoutRequest {
                name: price_id_or_name,
                reference: customer_id,
                amount_cents,
                interval: subscription_interval.as_deref(),
                product: product_id.as_deref(),
                currency: target_currency.as_deref().unwrap_or("usd"),
                operation_id: &operation_id,
                metadata: None,
            })
            .await?;
        Ok(receipt.url)
    }

    pub async fn create_billing_portal_session(
        &self,
        customer_id: &str,
        return_url_base: Option<&str>,
    ) -> Result<String, String> {
        let api_key = self.require_api_key()?;

        let mut form = std::collections::HashMap::new();
        form.insert("customer".to_string(), customer_id.to_string());
        let base_url = return_url_base.map(|s| s.to_string()).unwrap_or_else(|| {
            std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:18789".to_string())
        });
        form.insert("return_url".to_string(), format!("{}/pricing", base_url));

        let res = reqwest::Client::new()
            .post(format!("{}/v1/billing_portal/sessions", Self::api_base()))
            .basic_auth(api_key, Some(""))
            .form(&form)
            .send()
            .await
            .map_err(|e| format!("Stripe Billing Portal request failed: {}", e))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Stripe API error ({}): {}", status, text));
        }

        let json: serde_json::Value = res
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        let url = json["url"]
            .as_str()
            .ok_or_else(|| "Missing url in response".to_string())?;

        Ok(url.to_string())
    }

    pub async fn get_subscription(
        &self,
        subscription_id: &str,
    ) -> Result<StripeSubscription, String> {
        let api_key = self.require_api_key()?;
        let client = reqwest::Client::new();

        let res = client
            .get(format!(
                "{}/v1/subscriptions/{}",
                Self::api_base(),
                subscription_id
            ))
            .basic_auth(api_key, Some(""))
            .send()
            .await
            .map_err(|e| format!("Stripe get subscription request failed: {}", e))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Stripe API error ({}): {}", status, text));
        }

        let json: serde_json::Value = res
            .json()
            .await
            .map_err(|e| format!("Failed to parse Stripe subscription response: {}", e))?;

        Ok(StripeSubscription {
            id: json["id"].as_str().unwrap_or_default().to_string(),
            status: json["status"].as_str().unwrap_or_default().to_string(),
            current_period_end: json["current_period_end"].as_i64().unwrap_or(0),
        })
    }

    pub async fn list_invoices(&self, customer_id: &str) -> Result<Vec<StripeInvoice>, String> {
        let api_key = self.require_api_key()?;
        let client = reqwest::Client::new();

        let res = client
            .get(format!(
                "{}/v1/invoices?customer={}&limit=10",
                Self::api_base(),
                customer_id
            ))
            .basic_auth(api_key, Some(""))
            .send()
            .await
            .map_err(|e| format!("Stripe list invoices request failed: {}", e))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Stripe API error ({}): {}", status, text));
        }

        let json: serde_json::Value = res
            .json()
            .await
            .map_err(|e| format!("Failed to parse Stripe invoices response: {}", e))?;

        let invoices = json["data"]
            .as_array()
            .ok_or_else(|| "Missing data array in Stripe invoices response".to_string())?
            .iter()
            .map(|inv| StripeInvoice {
                id: inv["id"].as_str().unwrap_or_default().to_string(),
                amount_due: inv["amount_due"].as_i64().unwrap_or(0),
                status: inv["status"].as_str().unwrap_or_default().to_string(),
                invoice_pdf: inv["invoice_pdf"].as_str().map(|s| s.to_string()),
            })
            .collect();

        Ok(invoices)
    }

    pub async fn cancel_subscription(
        &self,
        subscription_id: &str,
    ) -> Result<StripeSubscription, String> {
        let api_key = self.require_api_key()?;
        if !super::safe_checkout::valid_provider_id(subscription_id, "sub_") {
            return Err("Invalid subscription identity".into());
        }

        let res = reqwest::Client::new()
            .delete(format!(
                "{}/v1/subscriptions/{}",
                Self::api_base(),
                subscription_id
            ))
            .basic_auth(api_key, Some(""))
            .send()
            .await
            .map_err(|e| format!("Stripe cancel subscription request failed: {}", e))?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(format!(
                "Stripe API Error cancelling subscription: {}",
                error_text
            ));
        }

        let stripe_sub: StripeSubscription = res
            .json()
            .await
            .map_err(|e| format!("Failed to parse Stripe cancel subscription response: {}", e))?;

        Ok(stripe_sub)
    }

    pub async fn submit_dispute_evidence(
        &self,
        dispute_id: &str,
        evidence_data: serde_json::Value,
    ) -> Result<(), String> {
        let api_key = self.require_api_key()?;
        let client = reqwest::Client::new();

        let mut form = std::collections::HashMap::new();
        if let Some(evidence_str) = evidence_data["evidence"].as_str() {
            form.insert("evidence".to_string(), evidence_str.to_string());
        }
        if let Some(customer_email) = evidence_data["customer_email_address"].as_str() {
            form.insert(
                "customer_email_address".to_string(),
                customer_email.to_string(),
            );
        }
        if let Some(uncategorized) = evidence_data["uncategorized_text"].as_str() {
            form.insert("uncategorized_text".to_string(), uncategorized.to_string());
        }

        let res = client
            .post(format!("{}/v1/disputes/{}", Self::api_base(), dispute_id))
            .basic_auth(api_key, Some(""))
            .form(&form)
            .send()
            .await
            .map_err(|e| format!("Stripe submit dispute evidence request failed: {}", e))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Stripe API error ({}): {}", status, text));
        }

        Ok(())
    }

    pub async fn create_draft_invoice(
        &self,
        customer_id: &str,
        amount_cents: i64,
        description: &str,
    ) -> Result<StripeInvoice, String> {
        let api_key = self.require_api_key()?;
        if !super::safe_checkout::valid_provider_id(customer_id, "cus_")
            || !(1..=99_999_999).contains(&amount_cents)
        {
            return Err("Invalid invoice customer or amount".into());
        }

        let client = reqwest::Client::new();
        // 1. Create an invoice item
        let mut form_item = std::collections::HashMap::new();
        form_item.insert("customer".to_string(), customer_id.to_string());
        form_item.insert("amount".to_string(), amount_cents.to_string());
        form_item.insert("currency".to_string(), "usd".to_string());
        form_item.insert("description".to_string(), description.to_string());

        let res_item = client
            .post(format!("{}/v1/invoiceitems", Self::api_base()))
            .basic_auth(api_key, Some(""))
            .form(&form_item)
            .send()
            .await
            .map_err(|e| format!("Stripe InvoiceItem request failed: {}", e))?;

        if !res_item.status().is_success() {
            let status = res_item.status();
            let text = res_item.text().await.unwrap_or_default();
            return Err(format!(
                "Stripe API error creating invoice item ({}): {}",
                status, text
            ));
        }

        // 2. Create the draft invoice
        let mut form_inv = std::collections::HashMap::new();
        form_inv.insert("customer".to_string(), customer_id.to_string());
        form_inv.insert("collection_method".to_string(), "send_invoice".to_string());
        form_inv.insert("days_until_due".to_string(), "30".to_string());

        let res_inv = client
            .post(format!("{}/v1/invoices", Self::api_base()))
            .basic_auth(api_key, Some(""))
            .form(&form_inv)
            .send()
            .await
            .map_err(|e| format!("Stripe Invoice request failed: {}", e))?;

        if !res_inv.status().is_success() {
            let status = res_inv.status();
            let text = res_inv.text().await.unwrap_or_default();
            return Err(format!(
                "Stripe API error creating invoice ({}): {}",
                status, text
            ));
        }

        let json: serde_json::Value = res_inv
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(StripeInvoice {
            id: json["id"].as_str().unwrap_or_default().to_string(),
            amount_due: amount_cents,
            status: json["status"].as_str().unwrap_or("draft").to_string(),
            invoice_pdf: json["invoice_pdf"].as_str().map(|s| s.to_string()),
        })
    }

    pub async fn finalize_and_send_invoice(
        &self,
        invoice_id: &str,
    ) -> Result<StripeInvoice, String> {
        let api_key = self.require_api_key()?;
        if !super::safe_checkout::valid_provider_id(invoice_id, "in_") {
            return Err("Invalid invoice identity".into());
        }
        let client = reqwest::Client::new();

        let res_inv = client
            .post(format!(
                "{}/v1/invoices/{}/send",
                Self::api_base(),
                invoice_id
            ))
            .basic_auth(api_key, Some(""))
            .send()
            .await
            .map_err(|e| format!("Stripe Invoice Send request failed: {}", e))?;

        if !res_inv.status().is_success() {
            let status = res_inv.status();
            let text = res_inv.text().await.unwrap_or_default();
            return Err(format!(
                "Stripe API error sending invoice ({}): {}",
                status, text
            ));
        }

        let json: serde_json::Value = res_inv
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(StripeInvoice {
            id: json["id"].as_str().unwrap_or_default().to_string(),
            amount_due: json["amount_due"].as_i64().unwrap_or(0),
            status: json["status"].as_str().unwrap_or("open").to_string(),
            invoice_pdf: json["invoice_pdf"].as_str().map(|s| s.to_string()),
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
        // Do not remove queued amounts or invent a payout receipt. Real payout
        // execution requires an approved destination and durable reconciliation.
        let _ = (account_id, amount_cents, batcher);
        Err("Automated payouts are unavailable; use the verified provider dashboard".into())
    }
}
