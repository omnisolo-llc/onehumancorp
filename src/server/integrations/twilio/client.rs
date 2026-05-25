use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait TwilioClientWrapper: Send + Sync {
    async fn send_sms(&self, to: &str, body: &str) -> Result<(), String>;
    async fn register_opt_in(&self, to: &str) -> Result<(), String>;
}

pub struct TwilioClient {
    pub api_key: String,
    http_client: Client,
}

impl TwilioClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl TwilioClientWrapper for TwilioClient {
    async fn send_sms(&self, to: &str, body: &str) -> Result<(), String> {
        let account_sid = std::env::var("TWILIO_ACCOUNT_SID").unwrap_or_else(|_| "".to_string());
        let from_number = std::env::var("TWILIO_FROM_NUMBER").unwrap_or_else(|_| "".to_string());

        let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", account_sid);
        let params = [
            ("To", to),
            ("From", &from_number),
            ("Body", body)
        ];

        let res = self.http_client.post(&url)
            .basic_auth(&account_sid, Some(&self.api_key))
            .form(&params)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => Ok(()),
            _ => Err("Failed to send SMS".to_string())
        }
    }

    async fn register_opt_in(&self, _to: &str) -> Result<(), String> {
        Ok(())
    }
}
