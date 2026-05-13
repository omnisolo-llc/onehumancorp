use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait DailyCoClientWrapper: Send + Sync {
    async fn create_room(&self, booking_id: &str) -> Result<String, String>;
}

pub struct RealDailyCoClient {
    api_key: String,
    http_client: Client,
}

impl RealDailyCoClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl DailyCoClientWrapper for RealDailyCoClient {
    async fn create_room(&self, booking_id: &str) -> Result<String, String> {
        let url = "https://api.daily.co/v1/rooms";
        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
            .json(&serde_json::json!({
                "name": format!("booking-{}", booking_id),
                "privacy": "public"
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(format!("https://ohc.daily.co/booking-{}", booking_id))
                } else {
                    Err(format!("Daily.co API error: {}", resp.status()))
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
        let client = RealDailyCoClient::new("token".to_string());
        assert_eq!(client.api_key, "token");
    }

    #[tokio::test]
    async fn test_create_room_error_handling() {
        let client = RealDailyCoClient::new("token".to_string());
        let _ = client.create_room("1234").await;
    }
}
