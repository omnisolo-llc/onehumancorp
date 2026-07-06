use async_trait::async_trait;

#[async_trait]
pub trait TwilioClientWrapper: Send + Sync {
    async fn send_sms(&self, to: &str, from: &str, body: &str) -> Result<(), String>;
    async fn send_whatsapp(&self, to: &str, from: &str, body: &str, media_url: Option<&str>) -> Result<(), String>;
    async fn provision_number(&self, area_code: &str) -> Result<String, String>;
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

        let mut retries = 3;
        while retries > 0 {
            let res = self.http_client.post(&url)
                .basic_auth(&self.account_sid, Some(&self.auth_token))
                .form(&params)
                .send()
                .await;

            match res {
                Ok(resp) => {
                    if resp.status().is_success() {
                        return Ok(());
                    } else if resp.status().is_server_error() {
                        retries -= 1;
                        if retries == 0 {
                            return Err(format!("Twilio API error: {}", resp.status()));
                        }
                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    } else {
                        return Err(format!("Twilio API error: {}", resp.status()));
                    }
                }
                Err(e) => {
                    retries -= 1;
                    if retries == 0 {
                        return Err(format!("Network error: {}", e));
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
            }
        }
        Err("Failed to send SMS after retries".to_string())
    }

    async fn provision_number(&self, area_code: &str) -> Result<String, String> {
        if self.account_sid.is_empty() || self.account_sid == "test" || self.account_sid == "dummy" {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let last_four: u32 = rng.gen_range(1000..9999);
            return Ok(format!("+1555123{}", last_four));
        }

        let search_url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/AvailablePhoneNumbers/US/Local.json", self.account_sid);
        let search_res = self.http_client.get(&search_url)
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .query(&[("AreaCode", area_code)])
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if !search_res.status().is_success() {
            return Err(format!("Twilio search API error: {}", search_res.status()));
        }

        let search_data: serde_json::Value = search_res.json().await.map_err(|e| format!("Failed to parse response: {}", e))?;
        let phone_number = search_data.get("available_phone_numbers")
            .and_then(|arr| arr.as_array())
            .and_then(|arr| arr.first())
            .and_then(|obj| obj.get("phone_number"))
            .and_then(|s| s.as_str())
            .ok_or("No available numbers found")?;

        let provision_url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/IncomingPhoneNumbers.json", self.account_sid);
        let params = [("PhoneNumber", phone_number)];
        let provision_res = self.http_client.post(&provision_url)
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .form(&params)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if !provision_res.status().is_success() {
             return Err(format!("Twilio provision API error: {}", provision_res.status()));
        }

        Ok(phone_number.to_string())
    }

    async fn send_whatsapp(&self, to: &str, from: &str, body: &str, media_url: Option<&str>) -> Result<(), String> {
        let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", self.account_sid);

        let formatted_to = if to.starts_with("whatsapp:") { to.to_string() } else { format!("whatsapp:{}", to) };
        let formatted_from = if from.starts_with("whatsapp:") { from.to_string() } else { format!("whatsapp:{}", from) };

        let mut params = vec![
            ("To", formatted_to.as_str()),
            ("From", formatted_from.as_str()),
            ("Body", body),
        ];

        if let Some(m_url) = media_url {
            params.push(("MediaUrl", m_url));
        }

        let mut retries = 3;
        while retries > 0 {
            let res = self.http_client.post(&url)
                .basic_auth(&self.account_sid, Some(&self.auth_token))
                .form(&params)
                .send()
                .await;

            match res {
                Ok(resp) => {
                    if resp.status().is_success() {
                        return Ok(());
                    } else if resp.status().is_server_error() {
                        retries -= 1;
                        if retries == 0 {
                            return Err(format!("Twilio API error: {}", resp.status()));
                        }
                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    } else {
                        return Err(format!("Twilio API error: {}", resp.status()));
                    }
                }
                Err(e) => {
                    retries -= 1;
                    if retries == 0 {
                        return Err(format!("Network error: {}", e));
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
            }
        }
        Err("Failed to send WhatsApp message after retries".to_string())
    }
}
