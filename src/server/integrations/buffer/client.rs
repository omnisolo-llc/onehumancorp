use reqwest::Client;

pub struct BufferClient {
    pub access_token: String,
    http_client: Client,
}

impl BufferClient {
    pub fn new(access_token: String) -> Self {
        BufferClient {
            access_token,
            http_client: Client::new(),
        }
    }

    pub async fn get_profiles(&self) -> Result<String, String> {
        let url = "https://api.bufferapp.com/1/profiles.json".to_string();
        let res = self.http_client.get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(resp.text().await.unwrap_or_default())
                } else {
                    Err(format!("Buffer API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
