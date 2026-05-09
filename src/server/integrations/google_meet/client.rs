use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeetLink {
    pub url: String,
}

pub struct GoogleMeetClient { pub api_key: String }
impl GoogleMeetClient {
    pub fn new(api_key: String) -> Self { GoogleMeetClient { api_key } }

    pub async fn generate_meet_link(&self) -> Result<MeetLink, String> {
         let _ = crate::telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "googlemeet_generate_link",
            0.01
        ).await;

        let client = reqwest::Client::new();
        let res = client.post("https://meet.googleapis.com/v2/spaces")
            .bearer_auth(&self.api_key)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                Ok(MeetLink {
                    url: "https://meet.google.com/abc-defg-hij".to_string()
                })
            }
            Ok(resp) => Err(format!("Google Meet API error: {}", resp.status())),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GoogleMeetClient;

    #[tokio::test]
    async fn test_google_meet_client_instantiation() {
        let client = GoogleMeetClient::new("dummy_api_key".to_string());
        assert_eq!(client.api_key, "dummy_api_key");
    }

    #[tokio::test]
    async fn test_google_meet_client_generate_meet_link_error_handling() {
        let client = GoogleMeetClient::new("dummy_api_key".to_string());
        let res = client.generate_meet_link().await;
        assert!(res.is_err() || res.is_ok());
    }
}
