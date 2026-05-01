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

    pub async fn create_checkout_session(&self, _price_id: &str, _customer_id: &str) -> Result<String, String> {
        Ok("https://checkout.stripe.com/c/pay/cs_test_...".to_string())
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
