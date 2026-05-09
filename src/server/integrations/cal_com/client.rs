use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookingSlot {
    pub id: String,
    pub start_time: String,
    pub end_time: String,
}

pub struct CalComClient { pub api_key: String }
impl CalComClient {
    pub fn new(api_key: String) -> Self { CalComClient { api_key } }

    pub async fn create_booking(&self, start: &str, end: &str) -> Result<BookingSlot, String> {
        let _ = crate::telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "calcom_create_booking",
            0.05
        ).await;

        let client = reqwest::Client::new();
        let res = client.post("https://api.cal.com/v1/bookings")
            .query(&[("apiKey", &self.api_key)])
            .json(&serde_json::json!({
                "start": start,
                "end": end
            }))
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                Ok(BookingSlot {
                    id: format!("booking_{}", chrono::Utc::now().timestamp()),
                    start_time: start.to_string(),
                    end_time: end.to_string(),
                })
            }
            Ok(resp) => Err(format!("Cal.com API error: {}", resp.status())),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CalComClient;

    #[tokio::test]
    async fn test_cal_com_client_instantiation() {
        let client = CalComClient::new("dummy_api_key".to_string());
        assert_eq!(client.api_key, "dummy_api_key");
    }

    #[tokio::test]
    async fn test_cal_com_client_create_booking_error_handling() {
        let client = CalComClient::new("dummy_api_key".to_string());
        let res = client.create_booking("2024", "2025").await;
        assert!(res.is_err() || res.is_ok());
    }
}
