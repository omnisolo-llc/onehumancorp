use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait EasyPostClientWrapper: Send + Sync {
    async fn purchase_label(&self, shipment_id: &str, rate_id: &str) -> Result<String, String>;
}

pub struct RealEasyPostClient {
    api_key: String,
    http_client: Client,
}

impl RealEasyPostClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[derive(Serialize)]
struct PurchaseRequest<'a> {
    rate: Rate<'a>,
}

#[derive(Serialize)]
struct Rate<'a> {
    id: &'a str,
}

#[derive(Deserialize, Debug)]
struct PurchaseResponse {
    postage_label: PostageLabel,
}

#[derive(Deserialize, Debug)]
struct PostageLabel {
    label_url: String,
}

#[async_trait]
impl EasyPostClientWrapper for RealEasyPostClient {
    async fn purchase_label(&self, shipment_id: &str, rate_id: &str) -> Result<String, String> {
        let url = format!("https://api.easypost.com/v2/shipments/{}/buy", shipment_id);
        let req = PurchaseRequest {
            rate: Rate { id: rate_id },
        };

        let res = self.http_client.post(&url)
            .basic_auth(&self.api_key, Some(""))
            .json(&req)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.map_err(|e| format!("Failed to read response text: {}", e))?;
                    let parsed: PurchaseResponse = serde_json::from_str(&text).map_err(|e| format!("Failed to parse response: {}", e))?;
                    Ok(parsed.postage_label.label_url)
                } else {
                    Err(format!("EasyPost API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
