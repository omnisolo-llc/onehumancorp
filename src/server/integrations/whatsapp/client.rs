
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct WhatsAppClient {
    client: Client,
    access_token: String,
    phone_number_id: String,
}

#[derive(Serialize)]
pub struct MessagePayload {
    pub messaging_product: String,
    pub to: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextPayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<TemplatePayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<InteractivePayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<MediaPayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<MediaPayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<MediaPayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<MediaPayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationPayload>,
}

#[derive(Serialize)]
pub struct TextPayload {
    pub body: String,
}

#[derive(Serialize)]
pub struct TemplatePayload {
    pub name: String,
    pub language: TemplateLanguage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<TemplateComponent>>,
}

#[derive(Serialize)]
pub struct TemplateLanguage {
    pub code: String,
}

#[derive(Serialize)]
pub struct TemplateComponent {
    #[serde(rename = "type")]
    pub component_type: String,
    pub parameters: Vec<TemplateParameter>,
}

#[derive(Serialize)]
pub struct TemplateParameter {
    #[serde(rename = "type")]
    pub param_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Serialize)]
pub struct InteractivePayload {
    #[serde(rename = "type")]
    pub interactive_type: String,
    pub action: InteractiveAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<TextPayload>,
}

#[derive(Serialize)]
pub struct InteractiveAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<InteractiveButton>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<InteractiveSection>>,
}

#[derive(Serialize)]
pub struct InteractiveButton {
    #[serde(rename = "type")]
    pub button_type: String,
    pub reply: InteractiveButtonReply,
}

#[derive(Serialize)]
pub struct InteractiveButtonReply {
    pub id: String,
    pub title: String,
}

#[derive(Serialize)]
pub struct InteractiveSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub rows: Vec<InteractiveRow>,
}

#[derive(Serialize)]
pub struct InteractiveRow {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct MediaPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}

#[derive(Serialize)]
pub struct LocationPayload {
    pub longitude: f64,
    pub latitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
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
        let payload = MessagePayload {
            messaging_product: "whatsapp".to_string(),
            to: to.to_string(),
            msg_type: "text".to_string(),
            text: Some(TextPayload {
                body: body.to_string(),
            }),
            template: None,
            interactive: None,
            image: None,
            audio: None,
            video: None,
            document: None,
            location: None,
        };

        self.send_custom_message(&payload).await
    }

    pub async fn send_custom_message(&self, payload: &MessagePayload) -> Result<String, String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let res = self
            .client
            .post(&url)
            .bearer_auth(&self.access_token)
            .json(payload)
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

    pub async fn register_phone_number(&self, pin: &str) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/register",
            self.phone_number_id
        );

        let payload = serde_json::json!({
            "messaging_product": "whatsapp",
            "pin": pin
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
            return Err(format!("Failed to register WhatsApp phone number: {} - {}", status, text));
        }

        Ok(())
    }
}
