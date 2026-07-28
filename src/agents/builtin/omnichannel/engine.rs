use crate::omnichannel::models::*;
use ohc_builtin_agent_core::types::{ChatRequest, Message};
use crate::llm::LlmClient;
use std::sync::Arc;
use chrono::Utc;

pub struct OmnichannelEngine {
    llm: Arc<dyn LlmClient>,
}

impl OmnichannelEngine {
    pub fn new(llm: Arc<dyn LlmClient>) -> Self {
        Self { llm }
    }

    pub async fn process_incoming_message(&self, msg: &ChatMessage, conv: &mut Conversation) -> Result<Option<ChatMessage>, String> {
        if msg.message_type != MessageType::Incoming || msg.sender_type != "Contact" {
            return Ok(None); // Ignore non-incoming
        }

        // Feature 1: Intent Classification & Copilot Drafting
        let intent = self.classify_intent(&msg.content).await?;

        match intent.as_str() {
            "human_handoff" => {
                conv.status = ConversationStatus::Open;
                Ok(Some(ChatMessage {
                    id: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0),
                    conversation_id: conv.id,
                    account_id: conv.account_id,
                    content: "I'll connect you with a human agent right away.".to_string(),
                    message_type: MessageType::Outgoing,
                    created_at: Utc::now(),
                    sender_type: "Bot".to_string(),
                    sender_id: 0,
                }))
            },
            _ => {
                // Auto-Responder
                let response = self.draft_auto_response(&msg.content).await?;
                Ok(Some(ChatMessage {
                    id: chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0),
                    conversation_id: conv.id,
                    account_id: conv.account_id,
                    content: response,
                    message_type: MessageType::Outgoing,
                    created_at: Utc::now(),
                    sender_type: "Bot".to_string(),
                    sender_id: 0,
                }))
            }
        }
    }

    async fn classify_intent(&self, text: &str) -> Result<String, String> {
        let req = ChatRequest {
            model: "default".to_string(),
            system: "Classify intent as 'support', 'sales', 'human_handoff', or 'general'. Respond only with the classification string.".to_string(),
            messages: vec![Message::user(text.to_string())],
            tools: vec![],
            max_tokens: 10,
            temperature: 0.1,
        };

        let resp = self.llm.chat(req).await.map_err(|e| e.to_string())?;
        Ok(resp.message.content.trim().to_lowercase())
    }

    async fn draft_auto_response(&self, text: &str) -> Result<String, String> {
        let req = ChatRequest {
            model: "default".to_string(),
            system: "You are an AI support bot. Draft a helpful, concise response to the user's message.".to_string(),
            messages: vec![Message::user(text.to_string())],
            tools: vec![],
            max_tokens: 200,
            temperature: 0.7,
        };

        let resp = self.llm.chat(req).await.map_err(|e| e.to_string())?;
        Ok(resp.message.content)
    }
}
