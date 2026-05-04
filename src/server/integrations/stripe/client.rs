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



    pub async fn create_payment_intent(&self, amount_cents: i64, _customer_id: &str) -> Result<serde_json::Value, String> {
        if amount_cents >= 100000 {
            // Route high-value transactions (>= $1000) through ACH to minimize percentage-based Stripe fees.
            Ok(serde_json::json!({
                "client_secret": "pi_ach_test_secret_12345",
                "status": "requires_payment_method",
                "payment_method_types": ["us_bank_account"]
            }))
        } else {
            // Standard Card routing for smaller payments.
            Ok(serde_json::json!({
                "client_secret": "pi_card_test_secret_67890",
                "status": "requires_payment_method",
                "payment_method_types": ["card"]
            }))
        }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_payment_intent_routing() {
        let client = StripeClient::new("test_key".to_string());

        // High value -> ACH
        let ach_intent = client.create_payment_intent(100000, "cus_123").await.unwrap();
        assert_eq!(ach_intent["client_secret"], "pi_ach_test_secret_12345");

        let ach_intent_larger = client.create_payment_intent(500000, "cus_123").await.unwrap();
        assert_eq!(ach_intent_larger["client_secret"], "pi_ach_test_secret_12345");

        // Low value -> Card
        let card_intent = client.create_payment_intent(99999, "cus_123").await.unwrap();
        assert_eq!(card_intent["client_secret"], "pi_card_test_secret_67890");

        let card_intent_smaller = client.create_payment_intent(5000, "cus_123").await.unwrap();
        assert_eq!(card_intent_smaller["client_secret"], "pi_card_test_secret_67890");
    }
}
