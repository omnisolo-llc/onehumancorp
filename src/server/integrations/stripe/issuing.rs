use super::client::StripeClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualCard {
    pub id: String,
    pub last4: String,
    pub exp_month: u8,
    pub exp_year: u16,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    pub id: String,
    pub amount: i64,
    pub currency: String,
    pub merchant_data: MerchantData,
    pub card: VirtualCard,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MerchantData {
    pub name: String,
    pub category: String,
}

impl StripeClient {
    pub async fn create_issuing_card(
        &self,
        spending_limit_cents: i64,
    ) -> Result<VirtualCard, String> {
        let auth_header = format!("Bearer {}", self.api_key);
        let form_data = [
            ("type", "virtual"),
            ("currency", "usd"),
            (
                "spending_controls[spending_limits][0][amount]",
                &spending_limit_cents.to_string(),
            ),
            (
                "spending_controls[spending_limits][0][interval]",
                "per_authorization",
            ),
        ];

        let req = reqwest::Client::new()
            .post("https://api.stripe.com/v1/issuing/cards")
            .header("Authorization", auth_header)
            .form(&form_data);

        let res = req.send().await.map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            let err = res.text().await.unwrap_or_default();
            return Err(format!("Stripe error: {}", err));
        }

        let card: VirtualCard = res.json().await.map_err(|e| e.to_string())?;
        Ok(card)
    }

    pub async fn handle_issuing_webhook(
        &self,
        payload: &str,
        _sig: &str,
        _secret: &str,
        budget_cents: i64,
    ) -> Result<bool, String> {
        // In a real app we'd verify the signature here.
        let event: serde_json::Value = serde_json::from_str(payload).map_err(|e| e.to_string())?;

        if event["type"] == "issuing_authorization.request" {
            let auth: AuthorizationRequest =
                serde_json::from_value(event["data"]["object"].clone())
                    .map_err(|e| e.to_string())?;

            // Check budget policy
            if auth.amount <= budget_cents {
                // Approve
                self.approve_issuing_authorization(&auth.id).await?;
                return Ok(true);
            } else {
                // Decline
                self.decline_issuing_authorization(&auth.id).await?;
                return Ok(false);
            }
        }

        Ok(false) // Not an auth request or unhandled
    }

    pub async fn approve_issuing_authorization(&self, auth_id: &str) -> Result<(), String> {
        let auth_header = format!("Bearer {}", self.api_key);
        let req = reqwest::Client::new()
            .post(&format!(
                "https://api.stripe.com/v1/issuing/authorizations/{}/approve",
                auth_id
            ))
            .header("Authorization", auth_header);

        let res = req.send().await.map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            let err = res.text().await.unwrap_or_default();
            return Err(format!("Stripe error: {}", err));
        }

        Ok(())
    }

    pub async fn decline_issuing_authorization(&self, auth_id: &str) -> Result<(), String> {
        let auth_header = format!("Bearer {}", self.api_key);
        let req = reqwest::Client::new()
            .post(&format!(
                "https://api.stripe.com/v1/issuing/authorizations/{}/decline",
                auth_id
            ))
            .header("Authorization", auth_header);

        let res = req.send().await.map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            let err = res.text().await.unwrap_or_default();
            return Err(format!("Stripe error: {}", err));
        }

        Ok(())
    }
}
