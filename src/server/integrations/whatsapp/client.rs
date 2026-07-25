
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

    pub async fn send_template(&self, to: &str, template_name: &str, language_code: &str) -> Result<String, String> {
        let url = format!("https://graph.facebook.com/v17.0/{}/messages", self.phone_number_id);

        let payload = serde_json::json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "template",
            "template": {
                "name": template_name,
                "language": {
                    "code": language_code
                }
            }
        });

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
            return Err(format!("Failed to send WhatsApp template message: {} - {}", status, text));
        }

        let resp: MessageResponse = res.json().await.map_err(|e| e.to_string())?;
        Ok(resp
            .messages
            .first()
            .map(|m| m.id.clone())
            .unwrap_or_default())
    }

    pub async fn send_interactive(&self, to: &str, interactive_payload: serde_json::Value) -> Result<String, String> {
        let url = format!("https://graph.facebook.com/v17.0/{}/messages", self.phone_number_id);

        let payload = serde_json::json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "interactive",
            "interactive": interactive_payload
        });

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
            return Err(format!("Failed to send WhatsApp interactive message: {} - {}", status, text));
        }

        let resp: MessageResponse = res.json().await.map_err(|e| e.to_string())?;
        Ok(resp
            .messages
            .first()
            .map(|m| m.id.clone())
            .unwrap_or_default())
    }

    pub async fn send_media(&self, to: &str, media_type: &str, media_id: Option<&str>, media_link: Option<&str>, caption: Option<&str>) -> Result<String, String> {
        let url = format!("https://graph.facebook.com/v17.0/{}/messages", self.phone_number_id);

        let mut media_payload = serde_json::json!({});
        if let Some(id) = media_id {
            media_payload["id"] = serde_json::json!(id);
        } else if let Some(link) = media_link {
            media_payload["link"] = serde_json::json!(link);
        } else {
            return Err("Must provide either media_id or media_link".to_string());
        }

        if let Some(cap) = caption {
            media_payload["caption"] = serde_json::json!(cap);
        }

        let mut payload = serde_json::json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": media_type,
        });

        payload[media_type] = media_payload;

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
            return Err(format!("Failed to send WhatsApp media message: {} - {}", status, text));
        }

        let resp: MessageResponse = res.json().await.map_err(|e| e.to_string())?;
        Ok(resp
            .messages
            .first()
            .map(|m| m.id.clone())
            .unwrap_or_default())
    }
}
