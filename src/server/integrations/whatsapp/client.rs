
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct WhatsAppClient {
    client: Client,
    access_token: String,
    phone_number_id: String,
}

#[derive(Serialize)]
struct MessagePayload {
    messaging_product: String,
    to: String,
    #[serde(rename = "type")]
    msg_type: String,
    text: TextPayload,
}

#[derive(Serialize)]
struct TextPayload {
    body: String,
}

#[derive(Deserialize)]
struct MessageResponse {
    messages: Vec<MessageStatus>,
}

#[derive(Deserialize)]
struct MessageStatus {
    id: String,
}

impl WhatsAppClient {
    pub fn new(access_token: String, phone_number_id: String) -> Self {
        Self {
            client: Client::new(),
            access_token,
            phone_number_id,
        }
    }

    pub async fn send_message(&self, to: &str, body: &str) -> Result<String, String> {
        let url = format!(
            "https://graph.facebook.com/v17.0/{}/messages",
            self.phone_number_id
        );

        let payload = MessagePayload {
            messaging_product: "whatsapp".to_string(),
            to: to.to_string(),
            msg_type: "text".to_string(),
            text: TextPayload {
                body: body.to_string(),
            },
        };

        let res = self
            .client
            .post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await.map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Failed to send WhatsApp message: {} - {}", status, text));
        }

        let resp: MessageResponse = res.json().await.map_err(|e| e.to_string())?;
        Ok(resp
            .messages
            .first()
            .map(|m| m.id.clone())
            .unwrap_or_default())
    }
}
