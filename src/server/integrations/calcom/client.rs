use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait CalComClientWrapper: Send + Sync {
    async fn create_booking_link(&self, user_id: &str, hours: &str) -> Result<String, String>;
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
    async fn create_booking_link(&self, user_id: &str, _hours: &str) -> Result<String, String> {
        let url = "https://api.cal.com/v1/event-types";
        let res = self.http_client.post(url)
            .query(&[("apiKey", &self.api_key)])
            .json(&serde_json::json!({
                "title": "Booking",
                "slug": format!("booking-{}", user_id),
                "length": 30,
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(format!("https://cal.com/user/booking-{}", user_id))
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
        let client = RealCalComClient::new("token".to_string());
        assert_eq!(client.api_key, "token");
    }

    #[tokio::test]
    async fn test_create_booking_link_error_handling() {
        let client = RealCalComClient::new("token".to_string());
        let _ = client.create_booking_link("user1", "9-5").await;
    }
}
