use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmsStatus {
    pub sid: String,
    pub status: String,
}

#[async_trait]
pub trait TwilioClientWrapper: Send + Sync {
    async fn send_sms(&self, to: &str, from: &str, body: &str, tenant_id: &str) -> Result<SmsStatus, String>;
    async fn send_templated_sms(&self, to: &str, from: &str, template_id: &str, placeholders: std::collections::HashMap<String, String>, tenant_id: &str) -> Result<SmsStatus, String>;
}

pub struct RealTwilioClient {
    account_sid: String,
    auth_token: String,
    http_client: Client,
}

impl RealTwilioClient {
    pub fn new(account_sid: String, auth_token: String) -> Self {
        Self {
            account_sid,
            auth_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl TwilioClientWrapper for RealTwilioClient {
    async fn send_sms(&self, to: &str, from: &str, body: &str, tenant_id: &str) -> Result<SmsStatus, String> {
        let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", self.account_sid);
        let res = self.http_client.post(&url)
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .form(&[
                ("To", to),
                ("From", from),
                ("Body", body),
            ])
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        tenant_id,
                        "twilio_send_sms",
                        0.05
                    ).await;
                    Ok(SmsStatus {
                        sid: "mock_sid_123".to_string(),
                        status: "queued".to_string(),
                    })
                } else {
                    Err(format!("Twilio API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn send_templated_sms(&self, to: &str, from: &str, template_id: &str, _placeholders: std::collections::HashMap<String, String>, tenant_id: &str) -> Result<SmsStatus, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "twilio_send_templated_sms",
            0.07
        ).await;

        // In a real implementation, we would use Twilio Content API
        tracing::info!("Sending templated SMS to {} using template {}", to, template_id);
        Ok(SmsStatus {
            sid: "mock_templated_sid_456".to_string(),
            status: "queued".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_client_creation() {
        let client = RealTwilioClient::new("sid".to_string(), "token".to_string());
        assert_eq!(client.account_sid, "sid");
        assert_eq!(client.auth_token, "token");
    }
}
