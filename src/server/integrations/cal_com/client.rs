use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

#[async_trait]
pub trait CalComClientWrapper: Send + Sync {
    async fn get_bookings(&self) -> Result<(), String>;
}

pub struct RealCalComClient {
    api_key: String,
    http_client: Client,
}

impl RealCalComClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl CalComClientWrapper for RealCalComClient {
    async fn get_bookings(&self) -> Result<(), String> {
        let url = format!("https://api.cal.com/v1/bookings?apiKey={}", self.api_key);
        let res = self.http_client.get(&url).send().await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Cal.com API error: {}", resp.status()))
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
        let client = RealCalComClient::new("api_key".to_string());
        assert_eq!(client.api_key, "api_key");
    }

    #[tokio::test]
    async fn test_get_bookings_error_handling() {
        let client = RealCalComClient::new("api_key".to_string());
        let _ = client.get_bookings().await;
    }
}
