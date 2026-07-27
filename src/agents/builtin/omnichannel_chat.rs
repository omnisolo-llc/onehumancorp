use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::llm::LlmClient;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
    Snoozed,
    Bot, // Being handled by an AI AgentBot
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    Incoming, // From customer
    Outgoing, // From agent/bot
    Activity, // System event
    Template, // Interactive templates (cards, options)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub content: String,
    pub message_type: MessageType,
    pub sender_id: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub account_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub assignee_id: Option<String>,
    pub status: ConversationStatus,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentClassification {
    pub intent: String,
    pub confidence: f32,
    pub requires_handoff: bool,
}

pub struct OmnichannelEngine {
    pub llm: Arc<dyn LlmClient>,
    pub conversations: Mutex<HashMap<String, Conversation>>,
}

impl OmnichannelEngine {
    pub fn new(llm: Arc<dyn LlmClient>) -> Self {
        Self {
            llm,
            conversations: Mutex::new(HashMap::new()),
        }
    }

    /// Chatwoot bot/webhook protocol replicate: AI auto-responder
    pub async fn process_incoming_message(&self, mut conv: Conversation, msg: Message) -> Result<Conversation, String> {
        conv.messages.push(msg.clone());

        if conv.status == ConversationStatus::Bot {
            // Process with LLM
            let classification = self.classify_intent(&msg.content).await?;

            if classification.requires_handoff {
                // Human agent handoff
                conv.status = ConversationStatus::Open;
                let handoff_msg = Message {
                    id: uuid::Uuid::new_v4().to_string(),
                    conversation_id: conv.id.clone(),
                    content: "Transferring you to a human agent...".to_string(),
                    message_type: MessageType::Outgoing,
                    sender_id: Some("bot".to_string()),
                    created_at: chrono::Utc::now(),
                };
                conv.messages.push(handoff_msg);
            } else {
                // Auto-reply
                let draft = self.draft_response(&conv).await?;
                let reply_msg = Message {
                    id: uuid::Uuid::new_v4().to_string(),
                    conversation_id: conv.id.clone(),
                    content: draft,
                    message_type: MessageType::Outgoing,
                    sender_id: Some("bot".to_string()),
                    created_at: chrono::Utc::now(),
                };
                conv.messages.push(reply_msg);
            }
        }

        let mut map = self.conversations.lock().await;
        map.insert(conv.id.clone(), conv.clone());

        Ok(conv)
    }

    /// Intent classification based on user message
    pub async fn classify_intent(&self, content: &str) -> Result<IntentClassification, String> {
        let req = crate::types::ChatRequest {
            model: "default".to_string(),
            system: "Classify the intent of the following customer message. Respond in strict JSON format: {\"intent\": \"string\", \"confidence\": float, \"requires_handoff\": boolean}".to_string(),
            messages: vec![crate::types::Message::user(content.to_string())],
            tools: vec![],
            max_tokens: 200,
            temperature: 0.0,
        };

        let res = self.llm.chat(req).await.map_err(|e| e.to_string())?;

        let raw_json = res.message.content.replace("```json", "").replace("```", "").trim().to_string();
        let parsed: IntentClassification = serde_json::from_str(&raw_json).unwrap_or_else(|_| {
            IntentClassification {
                intent: "unknown".to_string(),
                confidence: 0.0,
                requires_handoff: true,
            }
        });

        Ok(parsed)
    }

    /// Copilot response drafting based on conversation history
    pub async fn draft_response(&self, conv: &Conversation) -> Result<String, String> {
        let mut history = String::new();
        for m in &conv.messages {
            let sender = if m.message_type == MessageType::Incoming { "User" } else { "Agent" };
            history.push_str(&format!("{}: {}\n", sender, m.content));
        }

        let req = crate::types::ChatRequest {
            model: "default".to_string(),
            system: "You are an omnichannel customer support bot. Draft a helpful, concise response to the user based on the conversation history.".to_string(),
            messages: vec![crate::types::Message::user(history)],
            tools: vec![],
            max_tokens: 500,
            temperature: 0.7,
        };

        let res = self.llm.chat(req).await.map_err(|e| e.to_string())?;
        Ok(res.message.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatRequest, ChatResponse, Usage, Role};

    struct MockLlm;

    #[async_trait::async_trait]
    impl LlmClient for MockLlm {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let is_classify = req.system.contains("Classify");

            let content = if is_classify {
                if req.messages[0].content.contains("human") || req.messages[0].content.contains("manager") {
                    r#"{"intent": "escalation", "confidence": 0.95, "requires_handoff": true}"#.to_string()
                } else {
                    r#"{"intent": "inquiry", "confidence": 0.85, "requires_handoff": false}"#.to_string()
                }
            } else {
                "Hello, I can help with that!".to_string()
            };

            Ok(ChatResponse {
                message: crate::types::Message {
                    role: Role::Assistant,
                    content,
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: Some("id".to_string()),
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_omnichannel_auto_responder() {
        let llm = Arc::new(MockLlm);
        let engine = OmnichannelEngine::new(llm);

        let conv = Conversation {
            id: "conv-1".to_string(),
            account_id: "acc-1".to_string(),
            inbox_id: "inb-1".to_string(),
            contact_id: "cont-1".to_string(),
            assignee_id: None,
            status: ConversationStatus::Bot,
            messages: vec![],
        };

        let msg = Message {
            id: "msg-1".to_string(),
            conversation_id: "conv-1".to_string(),
            content: "What are your hours?".to_string(),
            message_type: MessageType::Incoming,
            sender_id: Some("cont-1".to_string()),
            created_at: chrono::Utc::now(),
        };

        let updated_conv = engine.process_incoming_message(conv, msg).await.unwrap();

        assert_eq!(updated_conv.status, ConversationStatus::Bot);
        assert_eq!(updated_conv.messages.len(), 2);
        assert_eq!(updated_conv.messages[1].message_type, MessageType::Outgoing);
        assert_eq!(updated_conv.messages[1].content, "Hello, I can help with that!");
    }

    #[tokio::test]
    async fn test_omnichannel_human_handoff() {
        let llm = Arc::new(MockLlm);
        let engine = OmnichannelEngine::new(llm);

        let conv = Conversation {
            id: "conv-2".to_string(),
            account_id: "acc-1".to_string(),
            inbox_id: "inb-1".to_string(),
            contact_id: "cont-1".to_string(),
            assignee_id: None,
            status: ConversationStatus::Bot,
            messages: vec![],
        };

        let msg = Message {
            id: "msg-2".to_string(),
            conversation_id: "conv-2".to_string(),
            content: "I need to talk to a manager".to_string(),
            message_type: MessageType::Incoming,
            sender_id: Some("cont-1".to_string()),
            created_at: chrono::Utc::now(),
        };

        let updated_conv = engine.process_incoming_message(conv, msg).await.unwrap();

        // Handoff to human agent
        assert_eq!(updated_conv.status, ConversationStatus::Open);
        assert_eq!(updated_conv.messages.len(), 2);
        assert_eq!(updated_conv.messages[1].content, "Transferring you to a human agent...");
    }

    #[tokio::test]
    async fn test_omnichannel_intent_classification() {
        let llm = Arc::new(MockLlm);
        let engine = OmnichannelEngine::new(llm);

        let classification = engine.classify_intent("I need to talk to a human").await.unwrap();
        assert_eq!(classification.requires_handoff, true);
        assert_eq!(classification.intent, "escalation");

        let classification2 = engine.classify_intent("Where is my order?").await.unwrap();
        assert_eq!(classification2.requires_handoff, false);
        assert_eq!(classification2.intent, "inquiry");
    }
}
