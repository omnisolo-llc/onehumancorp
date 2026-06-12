use async_trait::async_trait;

#[async_trait]
pub trait TwilioClientWrapper: Send + Sync {
    async fn send_sms(&self, to: &str, from: &str, body: &str) -> Result<(), String>;
    async fn send_whatsapp(&self, to: &str, from: &str, body: &str) -> Result<(), String>;
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

    async fn send_whatsapp(&self, to: &str, from: &str, body: &str) -> Result<(), String> {
        let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", self.account_sid);

        let formatted_to = if to.starts_with("whatsapp:") { to.to_string() } else { format!("whatsapp:{}", to) };
        let formatted_from = if from.starts_with("whatsapp:") { from.to_string() } else { format!("whatsapp:{}", from) };

        let params = [
            ("To", formatted_to.as_str()),
            ("From", formatted_from.as_str()),
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
