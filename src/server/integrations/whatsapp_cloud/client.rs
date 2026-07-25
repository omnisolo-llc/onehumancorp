use async_trait::async_trait;
use serde_json::json;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateLanguage {
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateComponent {
    #[serde(rename = "type")]
    pub component_type: String,
    pub parameters: Vec<TemplateParameter>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TemplateParameter {
    Text { text: String },
    Currency { fallback_value: String, code: String, amount_1000: i64 },
    DateTime { fallback_value: String },
    Image { image: MediaIdOrUrl },
    Document { document: MediaIdOrUrl },
    Video { video: MediaIdOrUrl },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaIdOrUrl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Template {
    pub name: String,
    pub language: TemplateLanguage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<TemplateComponent>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Interactive {
    #[serde(rename = "type")]
    pub interactive_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<InteractiveHeader>,
    pub body: InteractiveBody,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<InteractiveFooter>,
    pub action: InteractiveAction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractiveHeader {
    #[serde(rename = "type")]
    pub header_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractiveBody {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractiveFooter {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractiveAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<InteractiveButton>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<InteractiveSection>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractiveButton {
    #[serde(rename = "type")]
    pub button_type: String,
    pub reply: InteractiveButtonReply,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractiveButtonReply {
    pub id: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractiveSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub rows: Vec<InteractiveRow>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractiveRow {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Media {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}


#[async_trait]
pub trait WhatsAppCloudClientWrapper: Send + Sync {
async fn send_message(&self, to: &str, body: &str) -> Result<(), String>;
    async fn send_template(&self, to: &str, template: Template) -> Result<(), String>;
    async fn send_interactive(&self, to: &str, interactive: Interactive) -> Result<(), String>;
    async fn send_media(&self, to: &str, media_type: &str, media: Media) -> Result<(), String>;
}

pub struct RealWhatsAppCloudClient {
    phone_number_id: String,
    access_token: String,
}

impl RealWhatsAppCloudClient {
    pub fn new(phone_number_id: String, access_token: String) -> Self {
        Self {
            phone_number_id,
            access_token,
        }
    }
}

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

#[async_trait]
impl WhatsAppCloudClientWrapper for RealWhatsAppCloudClient {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "text",
            "text": {
                "body": body
            }
        });

        self.send_payload(&url, &payload).await
            }

    async fn send_template(&self, to: &str, template: Template) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "template",
            "template": template
        });

        self.send_payload(&url, &payload).await
    }

    async fn send_interactive(&self, to: &str, interactive: Interactive) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "interactive",
            "interactive": interactive
        });

        self.send_payload(&url, &payload).await
    }

    async fn send_media(&self, to: &str, media_type: &str, media: Media) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let mut payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": media_type,
        });

        payload[media_type] = json!(media);

        self.send_payload(&url, &payload).await
    }
}

impl RealWhatsAppCloudClient {
    async fn send_payload(&self, url: &str, payload: &serde_json::Value) -> Result<(), String> {
        let client = HTTP_CLIENT.get_or_init(reqwest::Client::new);
        let res = client
            .post(url)
            .bearer_auth(&self.access_token)
            .json(payload)
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    let err_text = response.text().await.unwrap_or_default();
                    Err(format!("WhatsApp API error: {}", err_text))
                }
            }
            Err(e) => Err(format!("Reqwest error: {}", e)),
        }
    }
}
