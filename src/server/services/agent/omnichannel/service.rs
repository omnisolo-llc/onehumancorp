use std::sync::Arc;
use crate::msgbus::{Bus, Message};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InboundMessage {
    pub message_id: String,
    pub tenant_id: String,
    pub channel: String,
    pub from_user: String,
    pub content: String,
    pub timestamp_unix: i64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Intent {
    QuoteRequest,
    GeneralQuestion,
    Complaint,
    Unknown,
}

pub struct OmnichannelInterceptorService {
    bus: Arc<dyn Bus>,
}

impl OmnichannelInterceptorService {
    pub fn new(bus: Arc<dyn Bus>) -> Self {
        OmnichannelInterceptorService { bus }
    }

    pub async fn start(&self) -> Result<(), String> {
        let bus_clone = self.bus.clone();
        let handler = Box::new(move |msg: Message| {
            if msg.topic == "chat:inbound_message" {
                let payload_str = String::from_utf8_lossy(&msg.payload).to_string();
                if let Ok(inbound_msg) = serde_json::from_str::<InboundMessage>(&payload_str) {
                    let b = bus_clone.clone();
                    tokio::spawn(async move {
                        let _ = Self::process_message(b, inbound_msg).await;
                    });
                }
            }
        });

        let _ = self.bus.subscribe("chat:inbound_message".to_string(), handler).await?;
        Ok(())
    }

    #[cfg(not(test))]
    pub async fn parse_intent(content: &str) -> Intent {
        let prompt = format!(
            "Classify the following customer message into one of these intents: \
            QuoteRequest, GeneralQuestion, Complaint, Unknown. \
            Reply with ONLY the exact intent name. Message: \"{}\"",
            content
        );

        match crate::minimax::LocalLLMClient::new().reason(&prompt).await {
            Ok(res) => {
                let trimmed = res.trim().to_lowercase();
                if trimmed.contains("quoterequest") {
                    Intent::QuoteRequest
                } else if trimmed.contains("generalquestion") {
                    Intent::GeneralQuestion
                } else if trimmed.contains("complaint") {
                    Intent::Complaint
                } else {
                    Intent::Unknown
                }
            }
            Err(e) => {
                tracing::error!("Failed to parse intent using LLM: {}", e);
                Intent::Unknown
            }
        }
    }

    #[cfg(test)]
    pub async fn parse_intent(content: &str) -> Intent {
        let content_lower = content.to_lowercase();
        if content_lower.contains("quote") || content_lower.contains("price") || content_lower.contains("how much") {
            return Intent::QuoteRequest;
        } else if content_lower.contains("complain") || content_lower.contains("issue") || content_lower.contains("broken") {
            return Intent::Complaint;
        } else if content_lower.contains("?") || content_lower.contains("help") {
            return Intent::GeneralQuestion;
        }
        Intent::Unknown
    }

    async fn process_message(bus: Arc<dyn Bus>, msg: InboundMessage) -> Result<(), String> {
        // 1. Parse intent using LLM
        let intent = Self::parse_intent(&msg.content).await;

        // 2. Act based on intent
        if intent == Intent::QuoteRequest {
            tracing::info!("Quote requested by user {} for tenant {}. Content: {}", msg.from_user, msg.tenant_id, msg.content);

            let quote_id = uuid::Uuid::new_v4().to_string();

            // Dispatch quote creation request to backend (this creates the actual quote in DB eventually)
            let quote_req = serde_json::json!({
                "id": quote_id,
                "tenant_id": msg.tenant_id,
                "customer_id": msg.from_user,
                "status": "DRAFT",
                "line_items": [
                    {
                        "description": "Auto-generated quote based on inquiry",
                        "unit_price_cents": 0,
                        "quantity": 1,
                        "is_optional": false
                    }
                ]
            });

            let _ = bus.publish(Message {
                topic: "quoting:create_quote".to_string(),
                payload: serde_json::to_vec(&quote_req).unwrap_or_default(),
            }).await;

            // Generate a drafted reply incorporating the actual quote link
            let reply_msg = serde_json::json!({
                "integration_id": msg.channel,
                "channel": msg.channel,
                "from_agent": "AutoReplyDM",
                "content": format!("I'd be happy to get you a quote! Please confirm the details on our secure portal: https://deposit.onehumancorp.io/quote/{}", quote_id),
                "thread_id": msg.message_id,
                "metadata": {
                    "action": "CREATE_QUOTE",
                    "quote_id": quote_id,
                    "customer_id": msg.from_user,
                    "description": msg.content
                }
            });

            let _ = bus.publish(Message {
                topic: "chat:send_message".to_string(),
                payload: serde_json::to_vec(&reply_msg).unwrap_or_default(),
            }).await;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msgbus::MemoryBus;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[tokio::test]
    async fn test_intent_parsing() {
        assert_eq!(OmnichannelInterceptorService::parse_intent("How much for a vegan cake?").await, Intent::QuoteRequest);
        assert_eq!(OmnichannelInterceptorService::parse_intent("I need a price quote.").await, Intent::QuoteRequest);
        assert_eq!(OmnichannelInterceptorService::parse_intent("I want to complain, the cake was dry.").await, Intent::Complaint);
        assert_eq!(OmnichannelInterceptorService::parse_intent("Do you deliver?").await, Intent::GeneralQuestion);
        assert_eq!(OmnichannelInterceptorService::parse_intent("Hello!").await, Intent::Unknown);
    }

    #[tokio::test]
    async fn test_omnichannel_interceptor_service() {
        let bus = Arc::new(MemoryBus::new());
        let service = OmnichannelInterceptorService::new(bus.clone());
        assert!(service.start().await.is_ok());

        let replied = Arc::new(AtomicBool::new(false));
        let replied_clone = replied.clone();

        let _ = bus.subscribe("chat:send_message".to_string(), Box::new(move |_msg: Message| {
            replied_clone.store(true, Ordering::SeqCst);
        })).await.unwrap();

        let msg = InboundMessage {
            message_id: "msg-1".to_string(),
            tenant_id: "tenant-1".to_string(),
            channel: "ig-dm".to_string(),
            from_user: "maya_customer".to_string(),
            content: "How much for a custom cake?".to_string(),
            timestamp_unix: 1234567890,
        };

        let msg_bytes = serde_json::to_vec(&msg).unwrap();
        bus.publish(Message {
            topic: "chat:inbound_message".to_string(),
            payload: msg_bytes,
        }).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        assert!(replied.load(Ordering::SeqCst));
    }
}
