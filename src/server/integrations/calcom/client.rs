use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait CalComClientWrapper: Send + Sync {
    async fn get_bookings(&self) -> Result<Vec<Booking>, String>;
    async fn generate_booking_link(&self, hours: &str) -> Result<String, String>;
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

#[derive(Serialize)]
struct CreateLinkRequest<'a> {
    hours: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Booking {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
}

#[derive(Deserialize, Debug)]
struct BookingsResponse {
    bookings: Vec<Booking>,
}

#[derive(Deserialize, Debug)]
struct CreateLinkResponse {
    link: String,
}

#[async_trait]
impl CalComClientWrapper for RealCalComClient {
    async fn get_bookings(&self) -> Result<Vec<Booking>, String> {
        let url = "https://api.cal.com/v1/bookings";

        let res = self.http_client.get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.map_err(|e| format!("Failed to read response text: {}", e))?;
                    if text.is_empty() { return Ok(vec![]); }
                    let parsed: BookingsResponse = serde_json::from_str(&text).map_err(|e| format!("Failed to parse response: {}", e))?;
                    Ok(parsed.bookings)
                } else {
                    Err(format!("Cal.com API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn generate_booking_link(&self, hours: &str) -> Result<String, String> {
        let url = "https://api.cal.com/v1/links";
        let req = CreateLinkRequest { hours };

        let res = self.http_client.post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&req)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.map_err(|e| format!("Failed to read response text: {}", e))?;
                    let parsed: CreateLinkResponse = serde_json::from_str(&text).map_err(|e| format!("Failed to parse response: {}", e))?;
                    Ok(parsed.link)
                } else {
                    Err(format!("Cal.com API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
