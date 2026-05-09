use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait ListmonkClientWrapper: Send + Sync {
    async fn send_campaign(&self, list_ids: Vec<i32>, subject: &str, body: &str) -> Result<(), String>;
}

pub struct RealListmonkClient {
    base_url: String,
    username: String,
    password: Option<String>,
    http_client: Client,
}

impl RealListmonkClient {
    pub fn new(base_url: String, username: String, password: Option<String>) -> Self {
        Self {
            base_url,
            username,
            password,
            http_client: Client::new(),
        }
    }
}

#[derive(Serialize)]
struct CreateCampaignRequest<'a> {
    name: &'a str,
    subject: &'a str,
    lists: Vec<i32>,
    r#type: &'a str,
    content_type: &'a str,
    body: &'a str,
}

#[async_trait]
impl ListmonkClientWrapper for RealListmonkClient {
    async fn send_campaign(&self, list_ids: Vec<i32>, subject: &str, body: &str) -> Result<(), String> {
        let url = format!("{}/api/campaigns", self.base_url);
        let req = CreateCampaignRequest {
            name: subject,
            subject,
            lists: list_ids,
            r#type: "regular",
            content_type: "html",
            body,
        };

        let mut builder = self.http_client.post(&url).json(&req);
        if let Some(pass) = &self.password {
            builder = builder.basic_auth(&self.username, Some(pass));
        } else {
            builder = builder.bearer_auth(&self.username); // username doubles as api key in some setups
        }

        let res = builder.send().await;

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
