use reqwest::Client;
use serde_json::Value;

pub struct ListmonkClient {
    base_url: String,
    api_key: String,
    http_client: Client,
}

impl ListmonkClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
            http_client: Client::new()
        }
    }
}

impl ListmonkClient {
    pub async fn send_campaign(&self, list_id: &str, template_id: &str, subject: &str, body: &str) -> Result<(), String> {
        let lists_id = list_id.parse::<u64>().unwrap_or(1);
        let t_id = template_id.parse::<u64>().unwrap_or(1);

        let url = format!("{}/api/campaigns", self.base_url);
        let payload = serde_json::json!({
            "name": format!("Campaign for {}", subject),
            "subject": subject,
            "lists": [lists_id],
            "type": "regular",
            "content_type": "richtext",
            "body": body,
            "template_id": t_id
        });

        // Basic auth is often username:password, here we assume api_key is configured as token or username:token
        let mut auth_parts = self.api_key.splitn(2, ':');
        let username = auth_parts.next().unwrap_or("listmonk");
        let password = auth_parts.next();

        let res = self.http_client.post(&url)
            .basic_auth(username, password)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let resp_json = resp.json::<Value>().await.map_err(|e| format!("Failed to parse response: {}", e))?;
                    if let Some(campaign_id) = resp_json.get("data").and_then(|d| d.get("id")).and_then(|i| i.as_u64()) {

                        let status_url = format!("{}/api/campaigns/{}/status", self.base_url, campaign_id);
                        let status_payload = serde_json::json!({"status": "running"});
                        let status_res = self.http_client.put(&status_url)
                            .basic_auth(username, password)
                            .json(&status_payload)
                            .send()
                            .await;

                        match status_res {
                            Ok(sr) if sr.status().is_success() => return Ok(()),
                            Ok(sr) => return Err(format!("Failed to start campaign: {}", sr.status())),
                            Err(e) => return Err(format!("Network error starting campaign: {}", e)),
                        }

                    } else {
                        return Err("Could not extract campaign id".to_string());
                    }
                } else {
                    Err(format!("Listmonk API error creating campaign: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
