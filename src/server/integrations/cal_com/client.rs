use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait CalComClientWrapper: Send + Sync {
    async fn get_bookings(&self) -> Result<(), String>;
}

pub struct RealCalComClient {
    access_token: String,
    http_client: Client,
}

impl RealCalComClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl CalComClientWrapper for RealCalComClient {
    async fn get_bookings(&self) -> Result<(), String> {
        let url = "https://api.cal.com/v1/bookings".to_string();
        let res = self.http_client.get(&url)
            .query(&[("apiKey", &self.access_token)])
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "cal_com_get_bookings",
                        0.01
                    ).await;
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
        let client = RealCalComClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }

    #[tokio::test]
    async fn test_get_bookings_error_handling() {
        let client = RealCalComClient::new("token".to_string());
        // Simple structural test
        let _ = client.get_bookings().await;
    }
}
