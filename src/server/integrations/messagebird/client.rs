use reqwest::Client;

#[async_trait::async_trait]
pub trait MessagebirdClientWrapper: Send + Sync {
    async fn send_sms(&self, to: &str, from: &str, body: &str) -> Result<(), String>;
}

pub struct RealMessagebirdClient {
    pub api_key: String,
    http_client: Client,
}

impl RealMessagebirdClient {
    pub fn new(api_key: String) -> Self {
        RealMessagebirdClient {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl MessagebirdClientWrapper for RealMessagebirdClient {
    async fn send_sms(&self, to: &str, from: &str, body: &str) -> Result<(), String> {
        let url = "https://rest.messagebird.com/messages";
        let res = self.http_client.post(url)
            .header("Authorization", format!("AccessKey {}", self.api_key))
            .form(&[
                ("recipients", to),
                ("originator", from),
                ("body", body),
            ])
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("MessageBird API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_real_client_instantiation() {
        let client = RealMessagebirdClient::new("test".to_string());
        assert_eq!(client.api_key, "test");
    }
}
