use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

#[async_trait]
pub trait CalcomClientWrapper: Send + Sync {
    async fn get_bookings(&self) -> Result<Value, String>;
    async fn create_booking(&self, booking_data: Value) -> Result<Value, String>;
}

pub struct RealCalcomClient {
    api_key: String,
    http_client: Client,
}

impl RealCalcomClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl CalcomClientWrapper for RealCalcomClient {
    async fn get_bookings(&self) -> Result<Value, String> {
        let url = format!("https://api.cal.com/v1/bookings?apiKey={}", self.api_key);
        let res = self.http_client.get(&url)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json = resp.json().await.map_err(|e| format!("Failed to parse JSON: {}", e))?;
                    Ok(json)
                } else {
                    Err(format!("Cal.com API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn create_booking(&self, booking_data: Value) -> Result<Value, String> {
        let url = format!("https://api.cal.com/v1/bookings?apiKey={}", self.api_key);
        let res = self.http_client.post(&url)
            .json(&booking_data)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json = resp.json().await.map_err(|e| format!("Failed to parse JSON: {}", e))?;
                    Ok(json)
                } else {
                    Err(format!("Cal.com API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
