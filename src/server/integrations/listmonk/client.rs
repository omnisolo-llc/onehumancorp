use reqwest::Client;

pub struct ListmonkClient {
    pub api_key: String,
    pub base_url: String,
    http_client: Client,
}

impl ListmonkClient {
    pub fn new(api_key: String) -> Self {
        let base_url = std::env::var("LISTMONK_BASE_URL").unwrap_or_else(|_| "http://localhost:9000/api".to_string());
        Self {
            api_key,
            base_url,
            http_client: Client::new(),
        }
    }
}

impl ListmonkClient {
    pub async fn send_campaign(&self, list_id: &str, template_id: &str, subject: &str, body: &str) -> Result<(), String> {
        let url = format!("{}/campaigns", self.base_url);

        let parsed_list_id = list_id.parse::<i32>().map_err(|_| "Invalid list_id format".to_string())?;
        let parsed_template_id = template_id.parse::<i32>().map_err(|_| "Invalid template_id format".to_string())?;

        let payload = serde_json::json!({
            "name": subject,
            "subject": subject,
            "lists": [parsed_list_id],
            "template_id": parsed_template_id,
            "body": body,
            "content_type": "html"
        });

        // Using Basic Auth or Token depending on the listmonk instance setup.
        // The api_key here will represent basic auth string or token.
        let res = self.http_client.post(&url)
            .header("Authorization", format!("token {}", self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Listmonk API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
