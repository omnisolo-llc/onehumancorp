use reqwest::Client;

pub struct AyrshareClient {
    pub api_key: String,
    http_client: Client,
}

impl AyrshareClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

impl AyrshareClient {
    pub async fn post_message(&self, message: &str, platforms: Vec<&str>) -> Result<(), String> {
        let url = "https://app.ayrshare.com/api/post";
        let payload = serde_json::json!({
            "post": message,
            "platforms": platforms
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Ayrshare API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn get_messages(&self) -> Result<Vec<serde_json::Value>, String> {
        let url = "https://app.ayrshare.com/api/analytics/messages";

        let res = self.http_client.get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json_resp: serde_json::Value = resp.json().await.map_err(|e| format!("JSON parsing error: {}", e))?;
                    if let Some(arr) = json_resp.as_array() {
                        Ok(arr.clone())
                    } else {
                        Ok(vec![])
                    }
                } else {
                    Err(format!("Ayrshare API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
