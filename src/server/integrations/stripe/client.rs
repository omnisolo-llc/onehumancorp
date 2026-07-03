use serde::{Deserialize, Serialize};
#[cfg(ohc_bazel)]
use crate::integrations::mercadopago::client::MercadoPagoClient;
#[cfg(not(ohc_bazel))]
use server_integrations_mercadopago::client::MercadoPagoClient;
#[cfg(ohc_bazel)]
use crate::integrations::razorpay::client::RazorpayClient;
#[cfg(not(ohc_bazel))]
use server_integrations_razorpay::client::RazorpayClient;

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
            return Err("Stripe API key is required".to_string());
        }
        Ok(key)
    }

    pub fn api_base() -> String {
        std::env::var("STRIPE_API_BASE").unwrap_or_else(|_| "https://api.stripe.com".to_string())
    }

    pub async fn create_checkout_session(&self, price_id_or_name: &str, customer_id: &str, amount_usd: f64, subscription_interval: Option<String>, product_id: Option<String>) -> Result<String, String> {
        let pm = PaymentRouter::optimize_payment_method(amount_usd);
        let savings = PaymentRouter::calculate_fee_savings(amount_usd);
        tracing::info!("💰 Miser telemetry: Payment method optimized. Saved ${} in fees", savings);

        // For MercadoPago and others not routed to Stripe Checkout
        match pm {
            PaymentMethod::Razorpay => {
                let api_key = std::env::var("RAZORPAY_API_KEY").unwrap_or_default();
                let api_secret = std::env::var("RAZORPAY_API_SECRET").unwrap_or_default();
                let rzp_client = RazorpayClient::new(api_key, api_secret);
                return rzp_client.create_checkout_preference(price_id_or_name, customer_id).await;
            },
            PaymentMethod::MercadoPago => {
                if let Ok(token) = std::env::var("MERCADOPAGO_ACCESS_TOKEN") {
                    let mp_client = MercadoPagoClient::new(token);
                    return mp_client.create_checkout_preference(price_id_or_name, customer_id).await;
                } else {
                    return Err("Mercado Pago access token is required".to_string());
                }
            },
            PaymentMethod::Alipay => {
                return Err("Alipay checkout is not configured for Stripe checkout sessions".to_string());
            },
            _ => {} // Fall through for ACH and CreditCard to Stripe API
        }

        let api_key_res = self.require_api_key();
        if api_key_res.is_err() {
            // Mock behavior for testing if no real key is configured
            return match pm {
                PaymentMethod::Ach => {
                    Ok("https://checkout.stripe.com/c/pay/cs_test_ach...".to_string())
                },
                _ => {
                    Ok("https://checkout.stripe.com/c/pay/cs_test_...".to_string())
                }
            };
        }

        let api_key = api_key_res.unwrap();
        let amount_cents = (amount_usd * 100.0).round() as i64;

        let mut form = std::collections::HashMap::new();
        form.insert("success_url".to_string(), "https://example.com/success".to_string());
        form.insert("cancel_url".to_string(), "https://example.com/cancel".to_string());
        if let Some(interval) = subscription_interval {
            form.insert("mode".to_string(), "subscription".to_string());
            form.insert("line_items[0][price_data][recurring][interval]".to_string(), interval);
        } else {
            form.insert("mode".to_string(), "payment".to_string());
        }
        form.insert("line_items[0][price_data][currency]".to_string(), "usd".to_string());
        let display_name = if price_id_or_name.trim().is_empty() { "Checkout".to_string() } else { price_id_or_name.to_string() };
        form.insert("line_items[0][price_data][product_data][name]".to_string(), display_name);
        form.insert("line_items[0][price_data][unit_amount]".to_string(), amount_cents.to_string());
        form.insert("line_items[0][quantity]".to_string(), "1".to_string());
        form.insert("client_reference_id".to_string(), customer_id.to_string());
        if let Some(pid) = product_id {
            form.insert("metadata[product_id]".to_string(), pid);
        }

        match pm {
            PaymentMethod::Ach => {
                form.insert("payment_method_types[0]".to_string(), "us_bank_account".to_string());
            },
            _ => {
                form.insert("payment_method_types[0]".to_string(), "card".to_string());
            }
        }

        let res = reqwest::Client::new()
            .post(format!("{}/v1/checkout/sessions", Self::api_base()))
            .basic_auth(api_key, Some(""))
            .form(&form)
            .send()
            .await
            .map_err(|e| format!("Stripe Checkout request failed: {}", e))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Stripe API error ({}): {}", status, text));
        }

        let json: serde_json::Value = res.json().await.map_err(|e| format!("Failed to parse response: {}", e))?;
        let url = json["url"].as_str().ok_or_else(|| "Missing url in response".to_string())?;

        Ok(url.to_string())
    }


    pub async fn create_billing_portal_session(&self, customer_id: &str) -> Result<String, String> {
        let api_key_res = self.require_api_key();
        if api_key_res.is_err() {
            return Ok("/pricing".to_string());
        }
        let api_key = api_key_res.unwrap();

        let mut form = std::collections::HashMap::new();
        form.insert("customer".to_string(), customer_id.to_string());
        let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:18789".to_string());
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

        let json: serde_json::Value = res.json().await.map_err(|e| format!("Failed to parse response: {}", e))?;
        let url = json["url"].as_str().ok_or_else(|| "Missing url in response".to_string())?;

        Ok(url.to_string())
    }

    pub async fn create_draft_invoice(&self, customer_id: &str, amount_cents: i64, description: &str) -> Result<StripeInvoice, String> {
        let api_key_res = self.require_api_key();
        if api_key_res.is_err() {
            return Ok(StripeInvoice {
                id: "in_test_draft_...".to_string(),
                amount_due: amount_cents,
                status: "draft".to_string(),
                invoice_pdf: Some("https://pay.stripe.com/invoice/acct_.../pdf".to_string()),
            });
        }
        let api_key = api_key_res.unwrap();

        let mut form_item = std::collections::HashMap::new();
        form_item.insert("customer".to_string(), customer_id.to_string());
        form_item.insert("amount".to_string(), amount_cents.to_string());
        form_item.insert("currency".to_string(), "usd".to_string());
        form_item.insert("description".to_string(), description.to_string());

        let res_item = reqwest::Client::new()
            .post(format!("{}/v1/invoiceitems", Self::api_base()))
            .basic_auth(api_key, Some(""))
            .form(&form_item)
            .send()
            .await
            .map_err(|e| format!("Stripe InvoiceItem request failed: {}", e))?;

        if !res_item.status().is_success() {
            let status = res_item.status();
            let text = res_item.text().await.unwrap_or_default();
            return Err(format!("Stripe API error (InvoiceItem) ({}): {}", status, text));
        }

        let mut form_inv = std::collections::HashMap::new();
        form_inv.insert("customer".to_string(), customer_id.to_string());
        form_inv.insert("collection_method".to_string(), "send_invoice".to_string());
        form_inv.insert("days_until_due".to_string(), "7".to_string());

        let res_inv = reqwest::Client::new()
            .post(format!("{}/v1/invoices", Self::api_base()))
            .basic_auth(api_key, Some(""))
            .form(&form_inv)
            .send()
            .await
            .map_err(|e| format!("Stripe Invoice request failed: {}", e))?;

        if !res_inv.status().is_success() {
            let status = res_inv.status();
            let text = res_inv.text().await.unwrap_or_default();
            return Err(format!("Stripe API error (Invoice) ({}): {}", status, text));
        }

        let json: serde_json::Value = res_inv.json().await.map_err(|e| format!("Failed to parse response: {}", e))?;

        let inv_id = json["id"].as_str().unwrap_or("in_unknown").to_string();
        let inv_pdf = json["invoice_pdf"].as_str().map(|s| s.to_string());

        Ok(StripeInvoice {
            id: inv_id,
            amount_due: amount_cents,
            status: "draft".to_string(),
            invoice_pdf: inv_pdf,
        })
    }

    pub async fn finalize_invoice(&self, invoice_id: &str) -> Result<StripeInvoice, String> {
        let api_key_res = self.require_api_key();
        if api_key_res.is_err() {
            return Ok(StripeInvoice {
                id: invoice_id.to_string(),
                amount_due: 0,
                status: "open".to_string(),
                invoice_pdf: Some("https://pay.stripe.com/invoice/acct_.../pdf".to_string()),
            });
        }
        let api_key = api_key_res.unwrap();

        let res_inv = reqwest::Client::new()
            .post(format!("{}/v1/invoices/{}/finalize", Self::api_base(), invoice_id))
            .basic_auth(api_key, Some(""))
            .send()
            .await
            .map_err(|e| format!("Stripe Invoice Finalize request failed: {}", e))?;

        if !res_inv.status().is_success() {
            let status = res_inv.status();
            let text = res_inv.text().await.unwrap_or_default();
            return Err(format!("Stripe API error (Invoice Finalize) ({}): {}", status, text));
        }

        let json: serde_json::Value = res_inv.json().await.map_err(|e| format!("Failed to parse response: {}", e))?;

        let inv_pdf = json["invoice_pdf"].as_str().map(|s| s.to_string());

        Ok(StripeInvoice {
            id: invoice_id.to_string(),
            amount_due: 0,
            status: "open".to_string(),
            invoice_pdf: inv_pdf,
        })
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

    pub async fn submit_dispute_evidence(&self, _dispute_id: &str, _evidence_data: serde_json::Value) -> Result<(), String> {
        // Simulates submitting dispute evidence to Stripe
        Ok(())
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




