use async_trait::async_trait;

#[async_trait]
pub trait TwilioClientWrapper: Send + Sync {
    async fn send_sms(&self, to: &str, from: &str, body: &str) -> Result<(), String>;
}

use reqwest::Client;

pub struct RealTwilioClient {
    pub account_sid: String,
    pub auth_token: String,
    http_client: Client,
}

impl RealTwilioClient {
    pub fn new(account_sid: String, auth_token: String) -> Self {
        RealTwilioClient {
            account_sid,
            auth_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl TwilioClientWrapper for RealTwilioClient {
    async fn send_sms(&self, to: &str, from: &str, body: &str) -> Result<(), String> {
        let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", self.account_sid);

        let params = [
            ("To", to),
            ("From", from),
            ("Body", body),
        ];

        let res = self.http_client.post(&url)
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .form(&params)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Twilio API error: {}", resp.status()))
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
    fn test_twilio_client_new() {
        let client = RealTwilioClient::new("dummy_sid".to_string(), "dummy_token".to_string());
        assert_eq!(client.account_sid, "dummy_sid");
        assert_eq!(client.auth_token, "dummy_token");
    }
}
