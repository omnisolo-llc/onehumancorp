use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait MetaClientWrapper: Send + Sync {
    async fn send_reply(&self, recipient_id: &str, message: &str) -> Result<(), String>;
}

pub struct RealMetaClient {
    page_access_token: String,
    http_client: Client,
}

impl RealMetaClient {
    pub fn new(page_access_token: String) -> Self {
        Self {
            page_access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl MetaClientWrapper for RealMetaClient {
    async fn send_reply(&self, recipient_id: &str, message: &str) -> Result<(), String> {
        let url = "https://graph.facebook.com/v17.0/me/messages";
        let res = self.http_client.post(url)
            .query(&[("access_token", &self.page_access_token)])
            .json(&serde_json::json!({
                "recipient": { "id": recipient_id },
                "message": { "text": message }
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Meta Graph API error: {}", resp.status()))
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
        assert_eq!(client.page_access_token, "token");
    }

    #[tokio::test]
    async fn test_send_reply_error_handling() {
        let client = RealMetaClient::new("token".to_string());
        let _ = client.send_reply("123", "Hello").await;
    }
}
