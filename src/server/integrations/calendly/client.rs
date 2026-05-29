use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CalendlyEventType {
    name: String,
}

#[derive(Deserialize, Debug)]
struct CalendlyEventTypesResponse {
    collection: Vec<CalendlyEventType>,
}

pub struct CalendlyClient {
    pub api_key: String,
    http_client: Client,
}

impl CalendlyClient {
    pub fn new(api_key: String) -> Self {
        CalendlyClient {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn fetch_event_types(&self) -> Result<Vec<String>, String> {
        // Must fetch the user's URI first in a real scenario, but assuming we have a known user scope or generic endpoint:
        // Actually Calendly requires `user` parameter. We'll use the 'me' endpoint to get the user URI, then fetch event types.
        let me_url = "https://api.calendly.com/users/me";

        #[derive(Deserialize)]
        struct MeResource { uri: String }
        #[derive(Deserialize)]
        struct MeResp { resource: MeResource }

        let me_res = self.http_client.get(me_url)
            .bearer_auth(&self.api_key)
            .send()
            .await;

        let user_uri = match me_res {
            Ok(resp) if resp.status().is_success() => {
                let data: MeResp = resp.json().await.map_err(|e| e.to_string())?;
                data.resource.uri
            }
            Ok(resp) => return Err(format!("Calendly API error (me): {}", resp.status())),
            Err(e) => return Err(format!("Network error: {}", e)),
        };

        let events_url = format!("https://api.calendly.com/event_types?user={}", user_uri);
        let res = self.http_client.get(&events_url)
            .bearer_auth(&self.api_key)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let data: CalendlyEventTypesResponse = resp.json().await.map_err(|e| e.to_string())?;
                    let types = data.collection.into_iter().map(|e| e.name).collect();
                    Ok(types)
                } else {
                    Err(format!("Calendly API error (events): {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
