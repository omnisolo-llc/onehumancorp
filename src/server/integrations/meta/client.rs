use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait MetaClientWrapper: Send + Sync {
    async fn send_message(&self, platform: &str, to: &str, body: &str) -> Result<(), String>;
    async fn create_paid_ad_campaign(
        &self,
        ad_account_id: &str,
        name: &str,
        daily_budget_cents: i64,
        objective: &str,
    ) -> Result<String, String>;
}

pub struct RealMetaClient {
    access_token: String,
    http_client: Client,
}

impl RealMetaClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl MetaClientWrapper for RealMetaClient {
    async fn send_message(&self, platform: &str, to: &str, body: &str) -> Result<(), String> {
        let url = match platform {
            "whatsapp" => "https://graph.facebook.com/v19.0/me/messages".to_string(),
            _ => "https://graph.facebook.com/v19.0/me/messages".to_string(), // Simplified URL mapping
        };

        let payload = serde_json::json!({
            "recipient": {
                "id": to
            },
            "message": {
                "text": body
            },
            "messaging_type": "RESPONSE"
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown", // tenant context
                        &format!("{}_send_message", platform),
                        0.01 // nominal meta cost
                    ).await;
                    Ok(())
                } else {
                    Err(format!("Meta API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn create_paid_ad_campaign(
        &self,
        ad_account_id: &str,
        name: &str,
        daily_budget_cents: i64,
        objective: &str,
    ) -> Result<String, String> {
        if ad_account_id.trim().is_empty() {
            return Err("Meta ad account id is required".to_string());
        }
        if daily_budget_cents <= 0 {
            return Err("daily budget must be positive".to_string());
        }

        let url = format!(
            "https://graph.facebook.com/v19.0/{}/campaigns",
            ad_account_id.trim()
        );
        let payload = serde_json::json!({
            "name": name,
            "objective": objective,
            "status": "PAUSED",
            "special_ad_categories": [],
            "daily_budget": daily_budget_cents,
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    let body: serde_json::Value = resp.json().await.unwrap_or_else(|_| serde_json::json!({}));
                    Ok(body
                        .get("id")
                        .and_then(|id| id.as_str())
                        .unwrap_or("meta-campaign-created")
                        .to_string())
                } else {
                    Err(format!("Meta Ads API error: {}", status))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_client_creation() {
        let client = RealMetaClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }

    // Because send_message issues a real network request using reqwest,
    // we omit a full unit test calling it here to prevent external dependencies and network flakes in the test suite.
    // Provider tests cover the mock path.
}
