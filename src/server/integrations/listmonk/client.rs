use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListmonkCampaign {
    pub id: i32,
    pub name: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
struct CampaignsResponse {
    data: Vec<ListmonkCampaign>,
}

#[derive(Debug, Deserialize)]
struct CreateCampaignResponse {
    data: ListmonkCampaign,
}

pub struct ListmonkClient {
    pub base_url: String,
    pub api_key: String,
    http_client: Client,
}

impl ListmonkClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn create_campaign(&self, name: &str, subject: &str, body: &str, organization_id: &str) -> Result<i32, String> {
        let url = format!("{}/api/campaigns", self.base_url);
        let res = self.http_client.post(&url)
            .basic_auth("admin", Some(&self.api_key)) // Listmonk often uses basic auth
            .json(&serde_json::json!({
                "name": name,
                "subject": subject,
                "body": body,
                "type": "regular",
                "content_type": "html",
            }))
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                let _ = ::server_telemetry::record_api_call_cost(
                    &crate::db::get_pool(),
                    organization_id,
                    "listmonk_create_campaign",
                    0.02
                ).await;
                let data: CreateCampaignResponse = resp.json().await.map_err(|e| e.to_string())?;
                Ok(data.data.id)
            }
            Ok(resp) => Err(format!("Listmonk API error: {}", resp.status())),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn list_campaigns(&self, organization_id: &str) -> Result<Vec<ListmonkCampaign>, String> {
        let url = format!("{}/api/campaigns", self.base_url);
        let res = self.http_client.get(&url)
            .basic_auth("admin", Some(&self.api_key))
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                let _ = ::server_telemetry::record_api_call_cost(
                    &crate::db::get_pool(),
                    organization_id,
                    "listmonk_list_campaigns",
                    0.01
                ).await;
                let data: CampaignsResponse = resp.json().await.map_err(|e| e.to_string())?;
                Ok(data.data)
            }
            Ok(resp) => Err(format!("Listmonk API error: {}", resp.status())),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_listmonk_creation() {
        let _client = ListmonkClient::new("http://localhost".to_string(), "key".to_string());
    }
}
