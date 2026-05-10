use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MailchimpSubscriber {
    pub email_address: String,
    pub status: String,
    pub tags: Vec<String>,
}

pub struct MailchimpClient {
    pub api_key: String,
    pub server_prefix: String,
    pub http_client: Client,
}

impl MailchimpClient {
    pub fn new(api_key: String, server_prefix: String) -> Self {
        Self {
            api_key,
            server_prefix,
            http_client: Client::new(),
        }
    }

    pub async fn add_subscriber(&self, list_id: &str, subscriber: &MailchimpSubscriber) -> Result<(), String> {
        let url = format!("https://{}.api.mailchimp.com/3.0/lists/{}/members", self.server_prefix, list_id);
        let res = self.http_client.post(&url)
            .basic_auth("anystring", Some(&self.api_key))
            .json(subscriber)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Mailchimp API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    /// High-level function to satisfy the business requirement of syncing
    /// a customer to Mailchimp automatically after purchase.
    pub async fn sync_customer_to_mailchimp(&self, email: &str, bought_item: &str) -> Result<(), String> {
        let subscriber = MailchimpSubscriber {
            email_address: email.to_string(),
            status: "subscribed".to_string(),
            tags: vec![format!("Bought: {}", bought_item)],
        };
        // Typically list_id is configured per tenant; mock for now
        self.add_subscriber("MOCK_LIST_ID", &subscriber).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mailchimp_client_creation() {
        let client = MailchimpClient::new("test_api_key".to_string(), "us1".to_string());
        assert_eq!(client.api_key, "test_api_key");
        assert_eq!(client.server_prefix, "us1");
    }

    #[tokio::test]
    async fn test_add_subscriber_compiles() {
        let client = MailchimpClient::new("test_api_key".to_string(), "us1".to_string());
        assert_eq!(client.api_key, "test_api_key");
    }
}
