use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub name: String,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub address_city: String,
    pub address_state: String,
    pub address_zip: String,
    pub address_country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostcardRequest {
    pub description: String,
    pub to: Address,
    pub front: String,
    pub back: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostcardResponse {
    pub id: String,
    pub expected_delivery_date: String,
}

pub struct LobClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl LobClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.lob.com/v1".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn create_postcard(
        &self,
        request: &PostcardRequest,
    ) -> Result<PostcardResponse, String> {
        let url = format!("{}/postcards", self.base_url);
        let res = self
            .client
            .post(&url)
            .basic_auth(&self.api_key, Some(""))
            .json(request)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            res.json().await.map_err(|e| e.to_string())
        } else {
            Err(format!("Lob API error: {}", res.status()))
        }
    }
}
