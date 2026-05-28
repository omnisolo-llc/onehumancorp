use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintJobRequest {
    #[serde(rename = "printerId")]
    pub printer_id: i32,
    pub title: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub content: String,
    pub source: String,
}

#[derive(Clone)]
pub struct PrintNodeProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl PrintNodeProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.printnode.com".to_string(),
        }
    }

    pub fn to_integration_provider(&self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: ProviderMetadata {
                id: "printnode".to_string(),
                name: "PrintNode".to_string(),
                category: "printing".to_string(),
                base_url: self.base_url.clone(),
            },
        }
    }

    pub async fn print_job(&self, req: PrintJobRequest) -> Result<String, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/printjobs", self.base_url);
        let res = self
            .client
            .post(&url)
            .basic_auth(self.api_key.clone(), Some(""))
            .json(&req)
            .send()
            .await?;

        if res.status().is_success() {
            let id: String = res.text().await?;
            Ok(id)
        } else {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            Err(format!("PrintNode API error: {} {}", status, text).into())
        }
    }
}
