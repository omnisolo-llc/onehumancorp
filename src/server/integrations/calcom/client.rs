use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CalBooking {
    pub id: i32,
    pub start: String,
    pub end: String,
    pub attendee_email: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ListBookingsResponse {
    bookings: Vec<CalBooking>,
}

pub struct CalComClient {
    pub api_key: String,
    http_client: Client,
}

impl CalComClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn create_booking_link(&self, tenant_id: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "calcom_create_link",
            0.01
        ).await;

        Ok(format!("https://cal.com/{}/booking", tenant_id))
    }

    pub async fn list_bookings(&self, tenant_id: &str) -> Result<Vec<CalBooking>, String> {
        let url = format!("https://api.cal.com/v1/bookings?apiKey={}", self.api_key);
        let res = self.http_client.get(&url)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                let _ = ::server_telemetry::record_api_call_cost(
                    &crate::db::get_pool(),
                    tenant_id,
                    "calcom_list_bookings",
                    0.01
                ).await;
                let data: ListBookingsResponse = resp.json().await.map_err(|e| e.to_string())?;
                Ok(data.bookings)
            }
            Ok(resp) => Err(format!("Cal.com API error: {}", resp.status())),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calcom_creation() {
        let _client = CalComClient::new("key".to_string());
    }
}
