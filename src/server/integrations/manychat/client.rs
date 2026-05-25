use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait ManychatClientWrapper: Send + Sync {
    async fn fetch_inbox(&self) -> Result<Vec<String>, String>;
    async fn send_reply(&self, platform: &str, to: &str, body: &str) -> Result<(), String>;
    async fn get_oauth_url(&self, redirect_uri: &str) -> String;
    async fn exchange_token(&self, code: &str, redirect_uri: &str) -> Result<String, String>;
}

pub struct RealManychatClient {
    pub api_key: String,
    http_client: Client,
}

impl RealManychatClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[derive(Serialize)]
struct ManychatTokenRequest {
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
    grant_type: String,
}

#[derive(Deserialize)]
struct ManychatTokenResponse {
    access_token: String,
}

#[async_trait]
impl ManychatClientWrapper for RealManychatClient {
    async fn fetch_inbox(&self) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    async fn send_reply(&self, _platform: &str, to: &str, body: &str) -> Result<(), String> {
        let url = "https://api.manychat.com/fb/sending/sendContent".to_string();

        let payload = serde_json::json!({
            "subscriber_id": to,
            "data": {
                "version": "v2",
                "content": {
                    "messages": [
                        {
                            "type": "text",
                            "text": body
                        }
                    ]
                }
            },
            "message_tag": "ACCOUNT_UPDATE"
        });

        let res = self.http_client.post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown", // tenant context
                        "manychat_send_reply",
                        0.01
                    ).await;
                    Ok(())
                } else {
                    Err(format!("Manychat API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn get_oauth_url(&self, redirect_uri: &str) -> String {
        let client_id = std::env::var("MANYCHAT_CLIENT_ID").unwrap_or_else(|_| "".to_string());
        format!("https://manychat.com/oauth?client_id={}&redirect_uri={}&response_type=code", client_id, redirect_uri)
    }

    async fn exchange_token(&self, code: &str, redirect_uri: &str) -> Result<String, String> {
        let url = "https://api.manychat.com/oauth/access_token".to_string();
        let client_id = std::env::var("MANYCHAT_CLIENT_ID").unwrap_or_else(|_| "".to_string());
        let client_secret = std::env::var("MANYCHAT_CLIENT_SECRET").unwrap_or_else(|_| "".to_string());

        let payload = ManychatTokenRequest {
            client_id,
            client_secret,
            code: code.to_string(),
            redirect_uri: redirect_uri.to_string(),
            grant_type: "authorization_code".to_string(),
        };

        let res = self.http_client.post(&url)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let token_data: ManychatTokenResponse = resp.json().await.map_err(|e| e.to_string())?;
                    Ok(token_data.access_token)
                } else {
                    Err(format!("Manychat OAuth error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
