use reqwest::Client;

pub struct ListmonkClient {
    pub api_key: String,
    pub base_url: String,
    http_client: Client,
}

impl ListmonkClient {
    pub fn new(api_key: String, base_url: String) -> Self {
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

        let parsed_list_id = list_id.parse::<u32>().map_err(|_| "Invalid List ID provided".to_string())?;
        let parsed_template_id = template_id.parse::<u32>().map_err(|_| "Invalid Template ID provided".to_string())?;

        let payload = serde_json::json!({
            "name": subject,
            "subject": subject,
            "lists": [parsed_list_id],
            "template_id": parsed_template_id,
            "type": "regular",
            "content_type": "html",
            "body": body
        });

        // First create the campaign
        let res = self.http_client.post(&url)
            .basic_auth("listmonk", Some(&self.api_key))
            .json(&payload)
            .send()
            .await;

        let campaign_id = match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json: serde_json::Value = resp.json().await.unwrap_or_default();
                    if let Some(id) = json.get("data").and_then(|d| d.get("id")).and_then(|i| i.as_u64()) {
                        id
                    } else {
                        return Err("Failed to parse campaign ID".to_string());
                    }
                } else {
                    return Err(format!("Listmonk API error (create): {}", resp.status()));
                }
            }
            Err(e) => return Err(format!("Network error: {}", e)),
        };

        // Then change the status to 'running' to send it
        let status_url = format!("{}/{}/status", url, campaign_id);
        let status_payload = serde_json::json!({
            "status": "running"
        });

        let status_res = self.http_client.put(&status_url)
            .basic_auth("listmonk", Some(&self.api_key))
            .json(&status_payload)
            .send()
            .await;

        match status_res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Listmonk API error (send): {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_listmonk_client_new() {
        let client = ListmonkClient::new("test_token".to_string(), "http://localhost:9000/api".to_string());
        assert_eq!(client.api_key, "test_token");
        assert_eq!(client.base_url, "http://localhost:9000/api");
    }
}
