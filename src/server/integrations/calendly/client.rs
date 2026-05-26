use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CalendlyUserResponse {
    resource: CalendlyUser,
}

#[derive(Deserialize, Debug)]
struct CalendlyUser {
    uri: String,
}

#[derive(Deserialize, Debug)]
struct CalendlyEventTypesResponse {
    collection: Vec<CalendlyEventType>,
}

#[derive(Deserialize, Debug)]
struct CalendlyEventType {
    name: String,
}

#[async_trait]
pub trait CalendlyClientWrapper: Send + Sync {
    async fn fetch_event_types(&self) -> Result<Vec<String>, String>;
}

pub struct RealCalendlyClient {
    pub api_key: String,
    http_client: Client,
}

impl RealCalendlyClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl CalendlyClientWrapper for RealCalendlyClient {
    async fn fetch_event_types(&self) -> Result<Vec<String>, String> {
        let user_url = "https://api.calendly.com/users/me";

        let user_res = self.http_client.get(user_url)
            .bearer_auth(&self.api_key)
            .send()
            .await;

        let user_uri = match user_res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let user_data: CalendlyUserResponse = resp.json().await.map_err(|e| format!("Failed to parse user JSON: {}", e))?;
                    user_data.resource.uri
                } else {
                    return Err(format!("Calendly API error on /users/me: {}", resp.status()));
                }
            }
            Err(e) => return Err(format!("Network error: {}", e)),
        };

        let types_url = format!("https://api.calendly.com/event_types?user={}", user_uri);
        let types_res = self.http_client.get(&types_url)
            .bearer_auth(&self.api_key)
            .send()
            .await;

        match types_res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let types_data: CalendlyEventTypesResponse = resp.json().await.map_err(|e| format!("Failed to parse event types JSON: {}", e))?;
                    let type_names: Vec<String> = types_data.collection.into_iter().map(|t| t.name).collect();
                    Ok(type_names)
                } else {
                    Err(format!("Calendly API error on /event_types: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

pub struct MockCalendlyClient;

impl MockCalendlyClient {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CalendlyClientWrapper for MockCalendlyClient {
    async fn fetch_event_types(&self) -> Result<Vec<String>, String> {
        Ok(vec!["30-min Consultation".to_string()])
    }
}
