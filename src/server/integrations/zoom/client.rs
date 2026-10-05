use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

const MAX_RECEIPT_BYTES: usize = 64 * 1024;

#[derive(Deserialize)]
struct MeetingReceipt {
    join_url: String,
}

pub struct ZoomClient {
    pub api_key: String,
    http_client: Client,
}

impl ZoomClient {
    pub fn new(api_key: String) -> Self {
        ZoomClient {
            api_key,
            http_client: Self::http_client_builder(Duration::from_secs(30))
                .build()
                .expect("static meeting client configuration is valid"),
        }
    }

    fn http_client_builder(deadline: Duration) -> reqwest::ClientBuilder {
        Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .read_timeout(Duration::from_secs(10))
            .timeout(deadline)
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
    }

    pub async fn create_meeting(&self, topic: &str) -> Result<String, String> {
        self.create_meeting_at(topic, "https://api.zoom.us/v2/users/me/meetings")
            .await
    }

    async fn create_meeting_at(&self, topic: &str, url: &str) -> Result<String, String> {
        let payload = serde_json::json!({
            "topic": topic,
            "type": 2
        });

        let mut response = self
            .http_client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|_| unknown_outcome("request failed"))?;

        let status = response.status();
        if status.is_client_error() && status != reqwest::StatusCode::REQUEST_TIMEOUT {
            return Err(format!("Zoom API error: {status}"));
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
        // A successful HTTP status is not a usable meeting receipt. Keep the
        // provider's actual URL and reject missing, malformed or duplicate fields.
        let receipt: MeetingReceipt =
            serde_json::from_slice(&body).map_err(|_| unknown_outcome("invalid receipt"))?;
        let join_url = receipt.join_url;
        let valid_url = reqwest::Url::parse(&join_url).is_ok_and(|url| {
            url.scheme() == "https"
                && url.host_str().is_some()
                && url.username().is_empty()
                && url.password().is_none()
        });
        if !valid_url
            || join_url
                .chars()
                .any(|ch| ch.is_whitespace() || ch.is_control())
        {
            return Err(unknown_outcome("invalid meeting URL"));
        }
        Ok(join_url)
    }
}

fn unknown_outcome(reason: &'static str) -> String {
    format!("Zoom meeting creation outcome unknown ({reason}); reconcile with Zoom before retrying")
}

#[cfg(test)]
mod tests {
    type TestedClient = super::ZoomClient;
    const RECEIPT_FIELD: &str = "join_url";
    include!("../meeting_receipt_tests.rs");
}
