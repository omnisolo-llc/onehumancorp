use std::sync::Arc;
use crate::hub::Hub;
use crate::ohc::orchestration::Message;
use chrono::Utc;

pub struct MetaProvider {
    pub hub: Arc<Hub>,
}

impl MetaProvider {
    pub fn new(hub: Arc<Hub>) -> Self {
        MetaProvider { hub }
    }

    pub async fn handle_incoming_message(&self, payload: serde_json::Value) -> Result<(), String> {
        tracing::info!("MetaProvider handling incoming message");

        // Extract basic information. A typical Messenger/Instagram DM webhook payload contains `entry[0].messaging[0]`
        let mut message_text = String::new();
        let mut sender_id = String::new();

        if let Some(entries) = payload.get("entry").and_then(|e| e.as_array()) {
            if let Some(entry) = entries.first() {
                if let Some(messaging) = entry.get("messaging").and_then(|m| m.as_array()) {
                    if let Some(msg_event) = messaging.first() {
                        if let Some(msg) = msg_event.get("message") {
                            if let Some(text) = msg.get("text").and_then(|t| t.as_str()) {
                                message_text = text.to_string();
                            }
                        }
                        if let Some(sender) = msg_event.get("sender") {
                            if let Some(id) = sender.get("id").and_then(|i| i.as_str()) {
                                sender_id = id.to_string();
                            }
                        }
                    }
                }
            }
        }

        if !message_text.is_empty() {
            let chat_msg = Message {
                id: uuid::Uuid::new_v4().to_string(),
                from_agent: sender_id.clone(),
                to_agent: "agent:customer_success".to_string(),
                r#type: "incoming_dm".to_string(),
                content: message_text,
                meeting_id: "".to_string(),
                occurred_at_unix: Utc::now().timestamp_millis(),
            };

            if let Err(e) = self.hub.clone().publish(chat_msg) {
                tracing::error!("Failed to route Meta DM to Customer Success Agent: {:?}", e);
                return Err(format!("Failed to publish message to Hub: {:?}", e));
            } else {
                tracing::info!("Routed Meta DM from {} to Customer Success Agent", sender_id);
            }
        }

        Ok(())
    }
}
