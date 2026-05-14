use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait TwilioClientWrapper: Send + Sync {
    async fn send_sms(&self, to: &str, from: &str, body: &str) -> Result<(), String>;
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
    async fn send_sms(&self, to: &str, from: &str, body: &str) -> Result<(), String> {
        // Validate phone number format (simple check for E.164-ish)
        if !to.starts_with('+') || to.len() < 8 {
            return Err("Invalid recipient phone number format. Must be E.164 (+1234567890)".to_string());
        }

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
                        "unknown",
                        "twilio_send_sms",
                        0.05
                    ).await;
                    Ok(())
                } else {
                    let status = resp.status();
                    let error_body = resp.text().await.unwrap_or_default();
                    Err(format!("Twilio API error ({}): {}", status, error_body))
                }
            }
            Err(e) => Err(format!("Network error while contacting Twilio: {}", e)),
        }
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

    #[tokio::test]
    async fn test_send_sms_invalid_number() {
        let client = RealTwilioClient::new("sid".to_string(), "token".to_string());
        let res = client.send_sms("123", "+1234", "test").await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Invalid recipient phone number format"));
    }
}
