use async_trait::async_trait;

#[async_trait]
pub trait TwilioClientWrapper: Send + Sync {
    async fn send_sms(&self, to: &str, from: &str, body: &str) -> Result<(), String>;
    async fn send_conversation_message(&self, channel: &str, to: &str, body: &str) -> Result<(), String>;
}

pub struct RealTwilioClient {
    pub account_sid: String,
    pub auth_token: String,
}

impl RealTwilioClient {
    pub fn new(account_sid: String, auth_token: String) -> Self {
        RealTwilioClient { account_sid, auth_token }
    }
}

#[async_trait]
impl TwilioClientWrapper for RealTwilioClient {
    async fn send_sms(&self, to: &str, _from: &str, body: &str) -> Result<(), String> {
        // Mock send sms
        println!("Sending SMS via Twilio to {}: {}", to, body);
        Ok(())
    }

    async fn send_conversation_message(&self, _channel: &str, to: &str, body: &str) -> Result<(), String> {
        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            self.account_sid
        );
        let client = reqwest::Client::new();
        let from = "+1234567890"; // Mock from number

        // Basic Auth
        let res = client
            .post(&url)
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .form(&[("To", to), ("From", from), ("Body", body)])
            .send()
            .await
            .map_err(|e| format!("Twilio API request failed: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Twilio API returned error status: {}", res.status()));
        }

        Ok(())
    }
}
