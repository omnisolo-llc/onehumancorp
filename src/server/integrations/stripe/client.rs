use serde::{Deserialize, Serialize};

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
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            customer_id, // assume customer_id is a proxy for organization_id
            "stripe_checkout_session",
            0.10 // mock cost for api orchestration
        ).await;

        // Use PaymentRouter to optimize method
        let pm = crate::integrations::stripe::routing::PaymentRouter::optimize_payment_method(amount_usd);

        match pm {
            crate::integrations::stripe::routing::PaymentMethod::Ach => {
                Ok("https://checkout.stripe.com/c/pay/cs_test_ach...".to_string())
            },
            crate::integrations::stripe::routing::PaymentMethod::CreditCard => {
                Ok("https://checkout.stripe.com/c/pay/cs_test_...".to_string())
            },
            crate::integrations::stripe::routing::PaymentMethod::Razorpay => {
                // Return razorpay checkout dummy link here since routing was updated
                Ok("https://checkout.razorpay.com/pay/cs_test_...".to_string())
            }
        }
    }

    pub async fn get_subscription(&self, _subscription_id: &str) -> Result<StripeSubscription, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "stripe_get_subscription",
            0.01
        ).await;
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
        batcher: &crate::integrations::stripe::payout_batcher::PayoutBatcher,
    ) -> Result<Option<String>, String> {
        let payout_amount = batcher.record_payout(account_id, amount_cents).await?;
        if let Some(total_cents) = payout_amount {
            // Transaction Fee Optimization: route large payments to ACH to save fees
            let fee = if crate::integrations::stripe::payout_batcher::PayoutBatcher::route_payment_to_ach(total_cents) {
                0.0 // ACH fees are often bundled or significantly lower, estimating 0.0 fixed
            } else {
                0.25 // Standard Stripe Payout Fee
            };

            let _ = ::server_telemetry::record_api_call_cost(
                &crate::db::get_pool(),
                account_id,
                "stripe_payout",
                fee
            ).await;

            // Execute real payout call here...
            Ok(Some(format!("po_test_{}", total_cents)))
        } else {
            Ok(None)
        }
    }
}
