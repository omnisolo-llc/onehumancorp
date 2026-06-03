use reqwest::Client;
use std::future::Future;
use std::pin::Pin;

pub trait MetaClientWrapper: Send + Sync {
    fn send_message<'a>(&'a self, platform: &'a str, to: &'a str, body: &'a str) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>>;
}

pub struct RealMetaClient {
    access_token: String,
    http_client: Client,
}

impl RealMetaClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

impl MetaClientWrapper for RealMetaClient {
    fn send_message<'a>(&'a self, platform: &'a str, to: &'a str, body: &'a str) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        Box::pin(async move {
            let url = match platform {
                "whatsapp" => "https://graph.facebook.com/v19.0/me/messages".to_string(),
                _ => "https://graph.facebook.com/v19.0/me/messages".to_string(), // Simplified URL mapping
            };

            let payload = serde_json::json!({
                "recipient": {
                    "id": to
                },
                "message": {
                    "text": body
                },
                "messaging_type": "RESPONSE"
            });

            let res: Result<reqwest::Response, reqwest::Error> = self.http_client.post(&url)
                .bearer_auth(&self.access_token)
                .json(&payload)
                .send()
                .await;

            match res {
                Ok(resp) => {
                    if resp.status().is_success() {
                        // let _ = ::server_telemetry::record_api_call_cost(
                        //     &crate::db::get_pool(),
                        //     "unknown", // tenant context
                        //     &format!("{}_send_message", platform),
                        //     0.01 // nominal meta cost
                        // ).await;
                        Ok(())
                    } else {
                        Err(format!("Meta API error: {}", resp.status()))
                    }
                }
                Err(e) => Err(format!("Network error: {}", e)),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_client_creation() {
        let client = RealMetaClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }

    // Because send_message issues a real network request using reqwest,
    // we omit a full unit test calling it here to prevent external dependencies and network flakes in the test suite.
    // Provider tests cover the mock path.
}
