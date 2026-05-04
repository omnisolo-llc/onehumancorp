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

    pub async fn create_checkout_session(&self, plan_id: &str, customer_id: &str) -> Result<String, String> {
        let client = reqwest::Client::new();

        // Map logical plans to Stripe Price IDs (in a real app, these come from DB/Config)
        let price_id = match plan_id {
            "Starter" => "price_starter_test_123",
            "Pro" => "price_pro_test_123",
            "Business" => "price_business_test_123",
            _ => return Err("Invalid plan specified".into()),
        };

        let params = [
            ("success_url", "https://app.onehumancorp.com/billing/success"),
            ("cancel_url", "https://app.onehumancorp.com/billing/cancel"),
            ("customer", customer_id),
            ("line_items[0][price]", price_id),
            ("line_items[0][quantity]", "1"),
            ("mode", "subscription"),
        ];

        let res = client
            .post("https://api.stripe.com/v1/checkout/sessions")
            .basic_auth(&self.api_key, Some(""))
            .form(&params)
            .send()
            .await
            .map_err(|e| format!("Stripe API connection error: {}", e))?;

        if !res.status().is_success() {
            let err_text = res.text().await.unwrap_or_default();
            return Err(format!("Stripe Error: {}", err_text));
        }

        let json: serde_json::Value = res.json().await.map_err(|e| format!("Invalid JSON: {}", e))?;

        if let Some(url) = json.get("url").and_then(|u| u.as_str()) {
            Ok(url.to_string())
        } else {
            Err("No checkout URL returned from Stripe".to_string())
        }
    }

    pub async fn get_subscription(&self, _subscription_id: &str) -> Result<StripeSubscription, String> {
        Ok(StripeSubscription {
            id: "sub_test_123".to_string(),
            status: "active".to_string(),
            current_period_end: 1714560000,
        })
    }

    pub async fn list_invoices(&self, _customer_id: &str) -> Result<Vec<StripeInvoice>, String> {
        Ok(vec![
            StripeInvoice {
                id: "in_test_123".to_string(),
                amount_due: 2900,
                status: "paid".to_string(),
                invoice_pdf: Some("https://pay.stripe.com/invoice/acct_123/pdf".to_string()),
            }
        ])
    }

    pub async fn cancel_subscription(&self, _subscription_id: &str) -> Result<StripeSubscription, String> {
        Ok(StripeSubscription {
            id: "sub_test_123".to_string(),
            status: "canceled".to_string(),
            current_period_end: 1714560000,
        })
    }
}
