use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ManychatConversation {
    id: String,
    last_message: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ManychatConversationsResponse {
    data: Vec<ManychatConversation>,
}

pub struct ManychatClient {
    pub api_key: String,
    http_client: Client,
}

impl ManychatClient {
    pub fn new(api_key: String) -> Self {
        ManychatClient {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn fetch_conversations(&self) -> Result<Vec<String>, String> {
        let url = "https://api.manychat.com/fb/page/getConversations";

        let res = self.http_client.get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let data: ManychatConversationsResponse = resp.json().await.map_err(|e| e.to_string())?;
                    let convos = data.data.into_iter()
                        .map(|c| format!("Conversation ID: {} - Last Message: {}", c.id, c.last_message.unwrap_or_default()))
                        .collect();
                    Ok(convos)
                } else {
                    Err(format!("Manychat API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
