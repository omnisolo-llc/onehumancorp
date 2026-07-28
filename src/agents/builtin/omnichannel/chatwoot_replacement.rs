use crate::omnichannel::models::*;
use crate::omnichannel::engine::OmnichannelEngine;

/// Replaces the Chatwoot bot/webhook protocol with a Native Rust implementation.
pub struct ChatwootReplacementHandler {
    pub engine: OmnichannelEngine,
}

impl ChatwootReplacementHandler {
    pub fn new(engine: OmnichannelEngine) -> Self {
        Self { engine }
    }

    /// Replicates the Chatwoot webhook handling behavior entirely locally in Rust.
    pub async fn handle_webhook(&self, payload: ChatwootWebhookPayload) -> Result<Option<ChatMessage>, String> {
        if payload.event != "message_created" {
            return Ok(None);
        }

        let mut conv = payload.conversation.ok_or("Missing conversation in webhook")?;

        if let Some(messages) = payload.messages {
            if let Some(latest_msg) = messages.last() {
                // Ignore if it's not from a Contact (e.g. ignore our own bot messages)
                if latest_msg.sender_type != "Contact" {
                    return Ok(None);
                }

                let response = self.engine.process_incoming_message(latest_msg, &mut conv).await?;
                return Ok(response);
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage};
    use crate::llm::LlmClient;
    use std::sync::Arc;

    struct MockOmniLlm {
        response_text: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockOmniLlm {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let content = if req.system.contains("Classify intent") {
                if req.messages.last().unwrap().content.contains("human") {
                    "human_handoff".to_string()
                } else {
                    "support".to_string()
                }
            } else {
                self.response_text.clone()
            };

            Ok(ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content,
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_chatwoot_replacement_human_handoff() {
        let client = Arc::new(MockOmniLlm { response_text: "I am a bot".to_string() });
        let engine = OmnichannelEngine::new(client);
        let handler = ChatwootReplacementHandler::new(engine);

        let conv = Conversation {
            id: 1, account_id: 1, inbox_id: 1, contact_id: 1, assignee_id: None,
            status: ConversationStatus::Bot, created_at: Utc::now(), custom_attributes: std::collections::HashMap::new(),
        };

        let msg = ChatMessage {
            id: 1, conversation_id: 1, account_id: 1, content: "I need a human".to_string(),
            message_type: MessageType::Incoming, created_at: Utc::now(), sender_type: "Contact".to_string(), sender_id: 1,
        };

        let payload = ChatwootWebhookPayload {
            event: "message_created".to_string(),
            conversation: Some(conv),
            messages: Some(vec![msg]),
        };

        let result = handler.handle_webhook(payload).await.unwrap().unwrap();
        assert_eq!(result.content, "I'll connect you with a human agent right away.");
        assert_eq!(result.sender_type, "Bot");
    }

    #[tokio::test]
    async fn test_chatwoot_replacement_auto_responder() {
        let client = Arc::new(MockOmniLlm { response_text: "I am a bot".to_string() });
        let engine = OmnichannelEngine::new(client);
        let handler = ChatwootReplacementHandler::new(engine);

        let conv = Conversation {
            id: 1, account_id: 1, inbox_id: 1, contact_id: 1, assignee_id: None,
            status: ConversationStatus::Bot, created_at: Utc::now(), custom_attributes: std::collections::HashMap::new(),
        };

        let msg = ChatMessage {
            id: 1, conversation_id: 1, account_id: 1, content: "Help me with my bill".to_string(),
            message_type: MessageType::Incoming, created_at: Utc::now(), sender_type: "Contact".to_string(), sender_id: 1,
        };

        let payload = ChatwootWebhookPayload {
            event: "message_created".to_string(),
            conversation: Some(conv),
            messages: Some(vec![msg]),
        };

        let result = handler.handle_webhook(payload).await.unwrap().unwrap();
        assert_eq!(result.content, "I am a bot");
        assert_eq!(result.sender_type, "Bot");
    }
}
