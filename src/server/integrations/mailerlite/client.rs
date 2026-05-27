use reqwest::Client;

pub struct MailerLiteClient {
    pub api_key: String,
    http_client: Client,
}

impl MailerLiteClient {
    pub fn new(api_key: String) -> Self {
        MailerLiteClient {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn sync_customer(&self, email: &str, name: &str) -> Result<(), String> {
        let url = "https://connect.mailerlite.com/api/subscribers";
        let payload = serde_json::json!({
            "email": email,
            "fields": {
                "name": name
            }
        });

        let res = self.http_client.post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("MailerLite API error: {}", resp.status()))
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
    fn test_mailerlite_client_new() {
        let client = MailerLiteClient::new("dummy_token".to_string());
        assert_eq!(client.api_key, "dummy_token");
    }
}
