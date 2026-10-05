use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

const MAX_RECEIPT_BYTES: usize = 64 * 1024;

// Preserve only the existing string-ID contract. The current v1 provider schema
// has not been verified; accepting numeric IDs requires a separate migration.
#[derive(Deserialize)]
struct BookingReceipt {
    booking: BookingIdentity,
}

#[derive(Deserialize)]
struct BookingIdentity {
    id: String,
}

pub struct CalComClient {
    pub access_token: String,
    http_client: Client,
    api_base_url: String,
}

impl CalComClient {
    fn http_client_builder(deadline: Duration) -> reqwest::ClientBuilder {
        Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .read_timeout(Duration::from_secs(10))
            .timeout(deadline)
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
    }

    pub fn new(access_token: String) -> Self {
        CalComClient {
            access_token,
            http_client: Self::http_client_builder(Duration::from_secs(30))
                .build()
                .expect("static Cal.com client configuration is valid"),
            api_base_url: "https://api.cal.com/v1".into(),
        }
    }
}

impl CalComClient {
    pub async fn get_free_busy(&self, time_min: &str, time_max: &str) -> Result<String, String> {
        let url = format!("{}/availability", self.api_base_url);

        let res = self
            .http_client
            .get(&url)
            .query(&[
                ("apiKey", &self.access_token),
                ("dateFrom", &time_min.to_string()),
                ("dateTo", &time_max.to_string()),
            ])
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.text()
                        .await
                        .map_err(|_| "Cal.com availability response could not be read".to_string())
                } else {
                    Err(format!("Cal.com API error: {}", resp.status()))
                }
            }
            Err(_) => Err("Cal.com availability request failed".to_string()),
        }
    }

    pub async fn create_event(
        &self,
        summary: &str,
        start_time: &str,
        end_time: &str,
    ) -> Result<String, String> {
        let url = format!("{}/bookings", self.api_base_url);

        let payload = serde_json::json!({
            "title": summary,
            "start": start_time,
            "end": end_time
        });

        let mut response = self
            .http_client
            .post(&url)
            .query(&[("apiKey", &self.access_token)])
            .json(&payload)
            .send()
            .await
            .map_err(|_| unknown_outcome("request failed"))?;

        let status = response.status();
        if status.is_client_error() && status != reqwest::StatusCode::REQUEST_TIMEOUT {
            return Err(format!("Cal.com API error: {status}"));
        }
        if !status.is_success() {
            return Err(format!(
                "{}: HTTP {status}",
                unknown_outcome("provider response")
            ));
        }
        if response
            .content_length()
            .is_some_and(|size| size > MAX_RECEIPT_BYTES as u64)
        {
            return Err(unknown_outcome("receipt exceeds 64 KiB"));
        }
        let mut body = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| unknown_outcome("receipt read failed"))?
        {
            if chunk.len() > MAX_RECEIPT_BYTES.saturating_sub(body.len()) {
                return Err(unknown_outcome("receipt exceeds 64 KiB"));
            }
            body.extend_from_slice(&chunk);
        }
        let receipt: BookingReceipt = serde_json::from_slice(&body)
            .map_err(|_| unknown_outcome("invalid or unsupported booking receipt"))?;
        if receipt.booking.id.trim().is_empty() {
            return Err(unknown_outcome("empty booking ID"));
        }
        Ok(receipt.booking.id)
    }

    pub async fn get_booking_link(&self, _event_type: &str) -> Result<String, String> {
        // A list response alone does not verify the account/team owner and event
        // slug needed to resolve a real booking URL. Do not invent a tenant URL.
        Err("Cal.com booking link unavailable: verified account owner and event-type mapping required".to_string())
    }
}

fn unknown_outcome(reason: &'static str) -> String {
    format!(
        "Cal.com booking creation outcome unknown ({reason}); reconcile with Cal.com before retrying"
    )
}

#[cfg(test)]
#[path = "client_test.rs"]
mod tests;
