use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct WhatsAppClient {
    client: Client,
    access_token: String,
    phone_number_id: String,
    base_url: String,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WhatsAppTemplate {
    pub name: String,
    pub language: String,
    pub category: String,
    pub status: String,
    pub components: Vec<serde_json::Value>,
    pub id: String,
}

#[derive(Deserialize)]
struct SyncTemplatesResponse {
    pub data: Vec<WhatsAppTemplate>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PhoneHealthStatus {
    pub id: String,
    pub verified_name: String,
    pub code_verification_status: String,
    pub display_phone_number: String,
    pub quality_rating: String,
    pub throughput: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct TemplatePayload {
    messaging_product: String,
    to: String,
    #[serde(rename = "type")]
    msg_type: String,
    template: TemplateInfo,
}

#[derive(Serialize)]
struct TemplateInfo {
    name: String,
    language: LanguageInfo,
    components: Vec<serde_json::Value>,
}

#[derive(Serialize)]
struct LanguageInfo {
    code: String,
}

impl WhatsAppClient {
    pub fn new(access_token: String, phone_number_id: String) -> Self {
        Self {
            client: Client::new(),
            access_token,
            phone_number_id,
            base_url: "https://graph.facebook.com/v19.0".to_string(),
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    fn is_mock_mode(&self) -> bool {
        self.base_url.contains("mock") || self.base_url.contains("test")
            || self.access_token.contains("test") || self.access_token.contains("mock")
            || self.phone_number_id.contains("test") || self.phone_number_id.contains("mock")
    }

    pub async fn send_message(&self, to: &str, body: &str) -> Result<String, String> {
        if self.is_mock_mode() {
            return Ok("mock_message_id_123".to_string());
        }

        let url = format!(
            "{}/{}/messages",
            self.base_url,
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

    pub async fn sync_templates(&self) -> Result<Vec<WhatsAppTemplate>, String> {
        if self.is_mock_mode() {
            return Ok(vec![
                WhatsAppTemplate {
                    name: "order_ready".to_string(),
                    language: "en_US".to_string(),
                    category: "UTILITY".to_string(),
                    status: "APPROVED".to_string(),
                    components: vec![serde_json::json!({
                        "type": "BODY",
                        "text": "Your order is ready for pickup!"
                    })],
                    id: "12345".to_string(),
                }
            ]);
        }

        let url = format!("{}/{}/message_templates", self.base_url, self.phone_number_id);
        let res = self
            .client
            .get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Failed to sync templates: {} - {}", status, text));
        }

        let resp: SyncTemplatesResponse = res.json().await.map_err(|e| e.to_string())?;
        Ok(resp.data)
    }

    pub async fn get_phone_number_health(&self) -> Result<PhoneHealthStatus, String> {
        if self.is_mock_mode() {
            return Ok(PhoneHealthStatus {
                id: self.phone_number_id.clone(),
                verified_name: "Maya's Home Bakery".to_string(),
                code_verification_status: "VERIFIED".to_string(),
                display_phone_number: "+1234567890".to_string(),
                quality_rating: "GREEN".to_string(),
                throughput: Some(serde_json::json!({ "level": "STANDARD" })),
            });
        }

        let url = format!(
            "{}/{}",
            self.base_url,
            self.phone_number_id
        );
        let res = self
            .client
            .get(&url)
            .query(&[("fields", "verified_name,code_verification_status,display_phone_number,quality_rating,throughput")])
            .bearer_auth(&self.access_token)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Failed to fetch phone health: {} - {}", status, text));
        }

        let resp: PhoneHealthStatus = res.json().await.map_err(|e| e.to_string())?;
        Ok(resp)
    }

    pub async fn send_template_message(
        &self,
        to: &str,
        template_name: &str,
        language_code: &str,
        components: Vec<serde_json::Value>,
    ) -> Result<String, String> {
        if self.is_mock_mode() {
            return Ok("mock_message_id_12345".to_string());
        }

        let url = format!("{}/{}/messages", self.base_url, self.phone_number_id);

        let payload = TemplatePayload {
            messaging_product: "whatsapp".to_string(),
            to: to.to_string(),
            msg_type: "template".to_string(),
            template: TemplateInfo {
                name: template_name.to_string(),
                language: LanguageInfo {
                    code: language_code.to_string(),
                },
                components,
            },
        };

        let res = self
            .client
            .post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Failed to send WhatsApp template: {} - {}", status, text));
        }

        let resp: MessageResponse = res.json().await.map_err(|e| e.to_string())?;
        Ok(resp
            .messages
            .first()
            .map(|m| m.id.clone())
            .unwrap_or_default())
    }
}
