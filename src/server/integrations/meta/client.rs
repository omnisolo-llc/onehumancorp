use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Client;

lazy_static! {
    static ref BSUID_REGEX: Regex =
        Regex::new(r"^[A-Z]{2}\.(?:ENT\.)?[A-Za-z0-9]{1,128}$").unwrap();
}

pub(crate) fn build_meta_payload(platform: &str, to: &str, body: &str) -> serde_json::Value {
    if platform == "instagram" || platform == "facebook" {
        serde_json::json!({
            "recipient": {
                "id": to
            },
            "message": {
                "text": body
            }
        })
    } else if platform == "whatsapp" {
        if BSUID_REGEX.is_match(to) {
            serde_json::json!({
                "messaging_product": "whatsapp",
                "recipient_type": "individual",
                "recipient": to,
                "type": "text",
                "text": {
                    "preview_url": false,
                    "body": body
                }
            })
        } else {
            serde_json::json!({
                "messaging_product": "whatsapp",
                "recipient_type": "individual",
                "to": to,
                "type": "text",
                "text": {
                    "preview_url": false,
                    "body": body
                }
            })
        }
    } else {
        serde_json::json!({
            "recipient": {
                "id": to
            },
            "message": {
                "text": body
            },
            "messaging_type": "RESPONSE"
        })
    }
}

#[async_trait]
pub trait MetaClientWrapper: Send + Sync {
    async fn send_message(
        &self,
        platform: &str,
        from: Option<&str>,
        to: &str,
        body: &str,
    ) -> Result<(), String>;
}

pub struct RealMetaClient {
    access_token: String,
    http_client: Client,
}

impl RealMetaClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl MetaClientWrapper for RealMetaClient {
    async fn send_message(
        &self,
        platform: &str,
        from: Option<&str>,
        to: &str,
        body: &str,
    ) -> Result<(), String> {
        let url = match platform {
            "whatsapp" => {
                if let Some(from_id) = from {
                    format!("https://graph.facebook.com/v19.0/{}/messages", from_id)
                } else {
                    "https://graph.facebook.com/v19.0/me/messages".to_string()
                }
            }
            _ => "https://graph.facebook.com/v19.0/me/messages".to_string(), // Simplified URL mapping
        };

        let payload = build_meta_payload(platform, to, body);

        let res = self
            .http_client
            .post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Meta API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_client_creation() {
        let client = RealMetaClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }

    #[test]
    fn test_build_meta_payload_whatsapp_standard() {
        let payload = build_meta_payload("whatsapp", "1234567890", "hello");
        assert_eq!(payload["to"], "1234567890");
        assert!(payload.get("recipient").is_none());
    }

    #[test]
    fn test_build_meta_payload_whatsapp_bsuid() {
        let payload = build_meta_payload("whatsapp", "BR.ENT.123456789", "hello");
        assert_eq!(payload["recipient"], "BR.ENT.123456789");
        assert!(payload.get("to").is_none());
    }

    #[test]
    fn test_build_meta_payload_other() {
        let payload = build_meta_payload("facebook", "1234567890", "hello");
        assert_eq!(payload["recipient"]["id"], "1234567890");
    }
}
