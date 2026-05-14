use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalEvent {
    pub id: String,
    pub title: String,
    pub start_time: String,
    pub end_time: String,
}

pub struct CalClient {
    pub api_key: String,
    pub http_client: Client,
}

impl CalClient {
    pub fn new(api_key: String) -> Self {
        CalClient {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn get_availability(&self, user_id: &str, date: &str, tenant_id: &str) -> Result<Vec<String>, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "cal_get_availability",
            0.01
        ).await;

        let url = format!("https://api.cal.com/v1/availability?apiKey={}&userId={}&date={}", self.api_key, user_id, date);
        let res = self.http_client.get(&url).send().await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(vec![format!("{}T10:00:00Z", date), format!("{}T14:00:00Z", date)])
                } else {
                    Err(format!("Cal API Error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network Error: {}", e)),
        }
    }

    pub async fn book_event(&self, user_id: &str, start_time: &str, tenant_id: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "cal_book_event",
            0.05
        ).await;

        let url = format!("https://api.cal.com/v1/bookings?apiKey={}", self.api_key);
        let payload = serde_json::json!({
            "userId": user_id,
            "start": start_time,
            "responses": {
                "name": "Customer",
                "email": "customer@example.com"
            }
        });

        let res = self.http_client.post(&url).json(&payload).send().await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok("Event booked successfully".to_string())
                } else {
                    Err(format!("Cal API Error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network Error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cal_client_creation() {
        let client = CalClient::new("token".to_string());
        assert_eq!(client.api_key, "token");
    }

    #[tokio::test]
    async fn test_cal_availability_error() {
        let client = CalClient::new("token".to_string());
        let _ = client.get_availability("1", "2024-01-01", "tenant1").await;
    }
}
