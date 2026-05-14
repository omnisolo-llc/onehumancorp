use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use super::{Tool, ToolExecutor};
use tracing::info;

pub struct QrGenerateExecutor;

#[async_trait::async_trait]
impl ToolExecutor for QrGenerateExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let content = args["content"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("qr_generate: content is required".to_string()))?;

        let label = args["label"].as_str().unwrap_or("QR Code");

        info!("Generating QR code for content: {} with label: {}", content, label);

        use qrcode::QrCode;

        let code = QrCode::new(content.as_bytes())
            .map_err(|e| format!("failed to generate QR code: {}", e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        let image_str = code.render::<char>()
            .quiet_zone(false)
            .module_dimensions(1, 1)
            .build();

        Ok(json!({
            "status": "success",
            "message": format!("QR code for '{}' has been generated.", content),
            "label": label,
            "ascii_art": image_str
        }).to_string())
    }
}

pub fn qr_generate_tool() -> Tool {
    Tool {
        name: "qr_generate".to_string(),
        description: "Generate a QR code for a given URL or text content.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "content": {
                    "type": "string",
                    "description": "The URL or text to encode in the QR code."
                },
                "label": {
                    "type": "string",
                    "description": "Optional label for the QR code."
                }
            },
            "required": ["content"]
        }),
        execute: Arc::new(QrGenerateExecutor),
    }
}

pub struct MetaGraphApiExecutor;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MetaWebhookPayload {
    pub object: String,
    pub entry: Vec<MetaEntry>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MetaEntry {
    pub id: String,
    pub time: i64,
    pub messaging: Vec<MetaMessaging>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MetaMessaging {
    pub sender: MetaParticipant,
    pub recipient: MetaParticipant,
    pub timestamp: i64,
    pub message: MetaMessageData,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MetaParticipant {
    pub id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MetaMessageData {
    pub mid: String,
    pub text: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MetaSendResponse {
    pub recipient_id: String,
    pub message_id: String,
    pub status: String,
}

impl MetaGraphApiExecutor {
    pub async fn simulate_send_message(recipient_id: &str, text: &str) -> Result<MetaSendResponse, ToolError> {
        info!("Sending Meta message to {}: {}", recipient_id, text);

        if recipient_id.is_empty() {
            return Err(ToolError::LlmRecoverable("recipient_id cannot be empty".to_string()));
        }

        if text.len() > 2000 {
            return Err(ToolError::LlmRecoverable("message text exceeds Meta limits".to_string()));
        }

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        Ok(MetaSendResponse {
            recipient_id: recipient_id.to_string(),
            message_id: format!("m_{}", chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)),
            status: "sent".to_string(),
        })
    }

    pub fn parse_webhook(payload: &Value) -> Result<MetaWebhookPayload, ToolError> {
        serde_json::from_value(payload.clone()).map_err(|e| ToolError::LlmRecoverable(format!("Failed to parse webhook: {}", e)))
    }
}

#[async_trait::async_trait]
impl ToolExecutor for MetaGraphApiExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let action = args["action"].as_str().unwrap_or("send");

        match action {
            "send" => {
                let recipient = args["recipient_id"].as_str().unwrap_or("");
                let text = args["text"].as_str().unwrap_or("");

                let result = Self::simulate_send_message(recipient, text).await?;
                Ok(serde_json::to_string(&result).unwrap())
            },
            "parse_webhook" => {
                if let Some(payload) = args.get("payload") {
                    let parsed = Self::parse_webhook(payload)?;
                    let messages: Vec<_> = parsed.entry.into_iter()
                        .flat_map(|e| e.messaging)
                        .map(|m| json!({
                            "from": m.sender.id,
                            "text": m.message.text
                        }))
                        .collect();

                    Ok(json!({
                        "status": "success",
                        "extracted_messages": messages
                    }).to_string())
                } else {
                    Err(ToolError::LlmRecoverable("Missing 'payload' for webhook parsing".to_string()))
                }
            },
            _ => {
                Err(ToolError::LlmRecoverable(format!("Unknown marketing action: {}", action)))
            }
        }
    }
}

pub fn marketing_meta_tool() -> Tool {
    Tool {
        name: "meta_business_suite".to_string(),
        description: "Integrates with Meta Graph API for unified inbox handling (Instagram/WhatsApp/Messenger).".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["send", "parse_webhook"],
                    "description": "Action to perform on the Meta API."
                },
                "recipient_id": {
                    "type": "string",
                    "description": "Recipient PSID for sending messages."
                },
                "text": {
                    "type": "string",
                    "description": "Message text to send."
                },
                "payload": {
                    "type": "object",
                    "description": "Raw Meta webhook payload for parsing."
                }
            }
        }),
        execute: Arc::new(MetaGraphApiExecutor),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_meta_send_message_success() {
        let executor = MetaGraphApiExecutor;
        let args = json!({
            "action": "send",
            "recipient_id": "12345",
            "text": "Hello from OHC"
        });
        let result = executor.execute(args).await.unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["status"], "sent");
        assert_eq!(parsed["recipient_id"], "12345");
        assert!(parsed["message_id"].as_str().unwrap().starts_with("m_"));
    }

    #[tokio::test]
    async fn test_meta_send_empty_recipient() {
        let executor = MetaGraphApiExecutor;
        let args = json!({
            "action": "send",
            "recipient_id": "",
            "text": "Hello"
        });
        let result = executor.execute(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_meta_send_too_long() {
        let executor = MetaGraphApiExecutor;
        let args = json!({
            "action": "send",
            "recipient_id": "123",
            "text": "A".repeat(3000)
        });
        let result = executor.execute(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_meta_parse_webhook_valid() {
        let executor = MetaGraphApiExecutor;
        let payload = json!({
            "object": "page",
            "entry": [
                {
                    "id": "page_id",
                    "time": 12345678,
                    "messaging": [
                        {
                            "sender": { "id": "user_id" },
                            "recipient": { "id": "page_id" },
                            "timestamp": 12345678,
                            "message": { "mid": "m_1", "text": "Do you have vegan cakes?" }
                        }
                    ]
                }
            ]
        });
        let args = json!({
            "action": "parse_webhook",
            "payload": payload
        });
        let result = executor.execute(args).await.unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["status"], "success");
    }

    #[tokio::test]
    async fn test_meta_parse_webhook_invalid() {
        let executor = MetaGraphApiExecutor;
        let args = json!({
            "action": "parse_webhook",
            "payload": { "invalid": "structure" }
        });
        let result = executor.execute(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_qr_generate_success() {
        let executor = QrGenerateExecutor;
        let args = json!({
            "content": "https://example.com"
        });
        let result = executor.execute(args).await.unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["status"], "success");
    }

    #[tokio::test]
    async fn test_qr_generate_missing_content() {
        let executor = QrGenerateExecutor;
        let args = json!({});
        let result = executor.execute(args).await;
        assert!(result.is_err());
    }
}
