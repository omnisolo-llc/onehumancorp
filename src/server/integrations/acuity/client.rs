use reqwest::Client;

pub struct AcuityClient {
    pub user_id: String,
    pub api_key: String,
    http_client: Client,
}

impl AcuityClient {
    pub fn new(user_id: String, api_key: String) -> Self {
        AcuityClient {
            user_id,
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn get_appointment_types(&self) -> Result<String, String> {
        let url = "https://acuityscheduling.com/api/v1/appointment-types".to_string();
        let res = self.http_client.get(&url)
            .basic_auth(&self.user_id, Some(&self.api_key))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(resp.text().await.unwrap_or_default())
                } else {
                    Err(format!("Acuity API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
