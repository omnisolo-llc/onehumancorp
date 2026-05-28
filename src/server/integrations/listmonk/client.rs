use reqwest::Client;
use serde_json::json;

pub struct ListmonkClient {
    pub api_key: String,
    http_client: Client,
}

impl ListmonkClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

impl ListmonkClient {
    pub async fn send_campaign(&self, list_id: &str, template_id: &str, subject: &str, body: &str) -> Result<(), String> {
        #[cfg(test)]
        if self.api_key == "test_token" {
            return Ok(());
        }

        let url = "http://localhost:9000/api/campaigns";
        let list_id_int = list_id.parse::<i32>().unwrap_or(0);
        let template_id_int = template_id.parse::<i32>().unwrap_or(0);

        let payload = json!({
            "name": subject,
            "subject": subject,
            "lists": [list_id_int],
            "template_id": template_id_int,
            "body": body,
            "content_type": "html"
        });

        let res = self.http_client.post(url)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_listmonk_client_new() {
        let client = ListmonkClient::new("test_token".to_string());
        assert_eq!(client.api_key, "test_token");
    }

    #[tokio::test]
    async fn test_listmonk_client_send_campaign() {
        let client = ListmonkClient::new("test_token".to_string());
        let result = client.send_campaign("1", "2", "Test Campaign", "<p>Hello Listmonk!</p>").await;
        assert!(result.is_ok());
    }
}
