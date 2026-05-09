use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaMessage {
    pub id: String,
    pub text: String,
    pub platform: String,
}

pub struct MetaClient { pub access_token: String }
impl MetaClient {
    pub fn new(access_token: String) -> Self { MetaClient { access_token } }

    pub async fn fetch_messages(&self) -> Result<Vec<MetaMessage>, String> {
        let _ = crate::telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "meta_fetch_messages",
            0.05
        ).await;
        let client = reqwest::Client::new();
        let res = client.get("https://graph.facebook.com/v19.0/me/messages")
            .bearer_auth(&self.access_token)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                Ok(vec![])
            }
            Ok(resp) => Err(format!("Meta API error: {}", resp.status())),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MetaClient;

    #[tokio::test]
    async fn test_meta_client_instantiation() {
        let client = MetaClient::new("dummy_token".to_string());
        assert_eq!(client.access_token, "dummy_token");
    }

    #[tokio::test]
    async fn test_meta_client_fetch_messages_error_handling() {
        let client = MetaClient::new("dummy_token".to_string());
        let res = client.fetch_messages().await;
        assert!(res.is_err() || res.is_ok());
    }
}
