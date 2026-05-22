use reqwest::Client;

pub struct ListmonkClient {
    api_key: String,
    #[allow(dead_code)]
    http_client: Client,
}

impl ListmonkClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn send_campaign(&self, list_id: &str, template_id: &str, subject: &str, body: &str) -> Result<(), String> {
        let url = "http://localhost:9000/api/campaigns";

        let payload = serde_json::json!({
            "name": subject,
            "subject": subject,
            "lists": [list_id.parse::<i32>().unwrap_or(1)],
            "template_id": template_id.parse::<i32>().unwrap_or(1),
            "content_type": "html",
            "body": body
        });

        let res = self.http_client.post(url)
            .basic_auth("admin", Some(&self.api_key))
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
