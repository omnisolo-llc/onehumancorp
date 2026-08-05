use regex::Regex;
use serde_json::{json, Value};
use std::sync::OnceLock;

static BSUID_REGEX: OnceLock<Regex> = OnceLock::new();

pub struct WhatsAppCloudService {
    pub phone_number_id: String,
    pub business_account_id: String,
    pub access_token: String,
}

impl WhatsAppCloudService {
    pub fn new(
        phone_number_id: String,
        business_account_id: String,
        access_token: String,
    ) -> Self {
        Self {
            phone_number_id,
            business_account_id,
            access_token,
        }
    }

    fn recipient_params(&self, identifier: &str) -> Value {
        let regex = BSUID_REGEX.get_or_init(|| {
            Regex::new(r"^[A-Z]{2}\.(?:ENT\.)?[A-Za-z0-9]{1,128}$").unwrap()
        });

        if regex.is_match(identifier) {
            json!({
                "recipient_type": "individual",
                "recipient": identifier
            })
        } else {
            json!({
                "to": identifier
            })
        }
    }

    pub fn send_message(&self, phone_number: &str, message_type: &str, content: Value) -> Value {
        let mut base = json!({
            "messaging_product": "whatsapp",
            "type": message_type,
        });

        if let Value::Object(ref mut map) = base {
            if let Value::Object(ref rec_params) = self.recipient_params(phone_number) {
                for (k, v) in rec_params {
                    map.insert(k.clone(), v.clone());
                }
            }
            map.insert(message_type.to_string(), content);
        }

        base
    }

    pub fn send_text_message(&self, phone_number: &str, text: &str) -> Value {
        self.send_message(phone_number, "text", json!({ "body": text }))
    }

    pub fn send_attachment_message(&self, phone_number: &str, file_type: &str, link: &str, caption: Option<&str>, filename: Option<&str>) -> Value {
        let mut content = json!({ "link": link });
        if let Value::Object(ref mut map) = content {
            if let Some(c) = caption {
                if file_type != "audio" && file_type != "sticker" {
                    map.insert("caption".to_string(), json!(c));
                }
            }
            if file_type == "document" {
                if let Some(f) = filename {
                    map.insert("filename".to_string(), json!(f));
                }
            }
        }
        self.send_message(phone_number, file_type, content)
    }

    pub fn send_interactive_message(&self, phone_number: &str, payload: Value) -> Value {
        self.send_message(phone_number, "interactive", payload)
    }

    pub fn send_template(&self, phone_number: &str, template_name: &str, lang_code: &str, components: Value) -> Value {
        let mut base = json!({
            "messaging_product": "whatsapp",
            "type": "template",
            "template": {
                "name": template_name,
                "language": {
                    "policy": "deterministic",
                    "code": lang_code
                },
                "components": components
            }
        });

        if let Value::Object(ref mut map) = base {
            if let Value::Object(ref rec_params) = self.recipient_params(phone_number) {
                for (k, v) in rec_params {
                    map.insert(k.clone(), v.clone());
                }
            }
        }

        // For templates, Chatwoot sets recipient_type: 'individual' manually in send_template
        if let Value::Object(ref mut map) = base {
           map.insert("recipient_type".to_string(), json!("individual"));
        }

        base
    }

    pub fn sync_templates(&self) {
        // Placeholder logic for syncing templates
    }
}
