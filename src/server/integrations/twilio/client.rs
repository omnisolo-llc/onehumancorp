use async_trait::async_trait;

#[async_trait]
pub trait TwilioClientWrapper: Send + Sync {
    async fn send_sms(&self, to: &str, from: &str, body: &str) -> Result<(), String>;
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
}
