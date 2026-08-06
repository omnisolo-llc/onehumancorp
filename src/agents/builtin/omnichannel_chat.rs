use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::{ChatRequest, Message as AgentMessage};

/// Chatwoot Retirement & Native Rust Omnichannel Chat Integration
/// Replicates Chatwoot's core entities and features natively in Rust.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelType {
    WebWidget,
    Api,
    Email,
    WhatsApp,
    Sms,
    Instagram,
    Twitter,
    Telegram,
    Line,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub id: i32,
    pub account_id: i32,
    pub name: String,
    pub channel_type: ChannelType,
    pub auto_assignment_config: Option<serde_json::Value>,
    pub enable_auto_assignment: bool,
    pub greeting_enabled: bool,
    pub greeting_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContactType {
    Visitor,
    Lead,
    Customer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: i32,
    pub account_id: i32,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub identifier: Option<String>,
    pub contact_type: ContactType,
    pub custom_attributes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
    Snoozed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: i32,
    pub account_id: i32,
    pub inbox_id: i32,
    pub contact_id: i32,
    pub assignee_id: Option<i32>,
    pub assignee_agent_bot_id: Option<i32>,
    pub status: ConversationStatus,
    pub custom_attributes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Incoming = 0,
    Outgoing = 1,
    Activity = 2,
    Template = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SenderType {
    Contact,
    User, // Human Agent
    AgentBot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub account_id: i32,
    pub inbox_id: i32,
    pub conversation_id: i32,
    pub message_type: MessageType,
    pub content: String,
    pub sender_type: SenderType,
    pub sender_id: Option<i32>,
    pub private: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBot {
    pub id: i32,
    pub account_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub outgoing_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntentClassification {
    Support,
    Sales,
    Billing,
    Feedback,
    Other(String),
}

/// The OmnichannelEngine manages routing and native AI features.
pub struct OmnichannelEngine {
    pub inboxes: HashMap<i32, Inbox>,
    pub conversations: HashMap<i32, Conversation>,
    pub messages: Vec<Message>,
    pub agent_bots: HashMap<i32, AgentBot>,
    pub ai_agent: Option<Arc<Agent>>,
}

impl OmnichannelEngine {
    pub fn new(ai_agent: Option<Arc<Agent>>) -> Self {
        Self {
            inboxes: HashMap::new(),
            conversations: HashMap::new(),
            messages: Vec::new(),
            agent_bots: HashMap::new(),
            ai_agent,
        }
    }

    /// Replicates Chatwoot's bot auto-responder behavior via Webhooks/IPC.
    pub async fn process_incoming_message(&mut self, msg: Message) -> Result<Option<Message>, String> {
        self.messages.push(msg.clone());

        if let Some(conv) = self.conversations.get(&msg.conversation_id) {
            // Check if assigned to a bot
            if let Some(bot_id) = conv.assignee_agent_bot_id {
                if let Some(bot) = self.agent_bots.get(&bot_id) {
                    let mut auto_response = format!("Bot {} auto-reply to: {}", bot.name, msg.content);
                    if let Some(agent) = &self.ai_agent {
                        let config = AgentRunConfig {
                            user_instructions: format!("Respond to the customer message: {}", msg.content),
                            ..Default::default()
                        };
                        let mut on_event = |_e| {};
                        if let Ok(resp) = agent.run(&config, &msg.content, &mut on_event).await {
                            auto_response = resp;
                        }
                    }

                    let reply = Message {
                        id: self.messages.len() as i32 + 1,
                        account_id: msg.account_id,
                        inbox_id: msg.inbox_id,
                        conversation_id: msg.conversation_id,
                        message_type: MessageType::Outgoing,
                        content: auto_response,
                        sender_type: SenderType::AgentBot,
                        sender_id: Some(bot_id),
                        private: false,
                    };
                    self.messages.push(reply.clone());
                    return Ok(Some(reply));
                }
            }
        }
        Ok(None)
    }

    /// Feature: Human Agent Handoff
    pub fn handoff_to_human(&mut self, conversation_id: i32, human_agent_id: Option<i32>) -> Result<(), String> {
        if let Some(conv) = self.conversations.get_mut(&conversation_id) {
            conv.assignee_agent_bot_id = None;
            conv.assignee_id = human_agent_id; // Assign to specific agent or queue

            // Add activity message
            let activity = Message {
                id: self.messages.len() as i32 + 1,
                account_id: conv.account_id,
                inbox_id: conv.inbox_id,
                conversation_id,
                message_type: MessageType::Activity,
                content: "Conversation handed off to human agent.".to_string(),
                sender_type: SenderType::User,
                sender_id: None,
                private: true,
            };
            self.messages.push(activity);

            Ok(())
        } else {
            Err("Conversation not found".to_string())
        }
    }

    /// Feature: Copilot response drafting
    pub async fn draft_copilot_response(&self, conversation_id: i32, context: &str) -> Result<String, String> {
        if !self.conversations.contains_key(&conversation_id) {
            return Err("Conversation not found".to_string());
        }

        let mut draft = format!("[Draft] Based on {}, here is a suggested reply...", context);
        if let Some(agent) = &self.ai_agent {
            let config = AgentRunConfig {
                user_instructions: format!("Draft a polite copilot response for a customer based on this context: {}", context),
                ..Default::default()
            };
            let mut on_event = |_e| {};
            if let Ok(resp) = agent.run(&config, context, &mut on_event).await {
                draft = format!("[Draft] {}", resp);
            }
        }

        Ok(draft)
    }

    /// Feature: Intent classification
    pub async fn classify_intent(&self, text: &str) -> Result<IntentClassification, String> {
        if let Some(agent) = &self.ai_agent {
            let req = ChatRequest {
                model: "mock-model".to_string(),
                system: "Classify intent as Support, Sales, Billing, Feedback, or Other".to_string(),
                messages: vec![AgentMessage::user(text)],
                tools: vec![],
                max_tokens: 50,
                temperature: 0.0,
            };
            if let Ok(resp) = agent.llm.chat(req).await {
                let ans = resp.message.content.to_lowercase();
                if ans.contains("support") {
                    return Ok(IntentClassification::Support);
                } else if ans.contains("sales") {
                    return Ok(IntentClassification::Sales);
                } else if ans.contains("billing") {
                    return Ok(IntentClassification::Billing);
                } else if ans.contains("feedback") {
                    return Ok(IntentClassification::Feedback);
                } else {
                    return Ok(IntentClassification::Other(resp.message.content));
                }
            }
        }

        let text_lower = text.to_lowercase();
        if text_lower.contains("pricing") || text_lower.contains("cost") || text_lower.contains("pay") {
            Ok(IntentClassification::Billing)
        } else if text_lower.contains("buy") || text_lower.contains("purchase") {
            Ok(IntentClassification::Sales)
        } else if text_lower.contains("help") || text_lower.contains("broken") || text_lower.contains("issue") {
            Ok(IntentClassification::Support)
        } else if text_lower.contains("sucks") || text_lower.contains("great") {
            Ok(IntentClassification::Feedback)
        } else {
            Ok(IntentClassification::Other("General".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use ohc_builtin_agent_core::types::ChatResponse;
    use std::sync::Arc;

    struct MockLlmClient;

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, req: ChatRequest) -> Result<ohc_builtin_agent_core::types::ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let text = req.messages.last().unwrap().content.to_lowercase();
            let mut result = "other";
            if text.contains("cost") {
                result = "billing";
            } else if text.contains("buy") {
                result = "sales";
            } else if text.contains("broken") {
                result = "support";
            } else if text.contains("sucks") {
                result = "feedback";
            }
            Ok(ChatResponse {
                message: AgentMessage::assistant(result),
                usage: ohc_builtin_agent_core::types::Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id1".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_auto_responder() {
        let mut engine = OmnichannelEngine::new(None);

        let bot = AgentBot { id: 1, account_id: Some(1), name: "SupportBot".to_string(), description: None, outgoing_url: None };
        engine.agent_bots.insert(1, bot);

        let conv = Conversation { id: 1, account_id: 1, inbox_id: 1, contact_id: 1, assignee_id: None, assignee_agent_bot_id: Some(1), status: ConversationStatus::Open, custom_attributes: None };
        engine.conversations.insert(1, conv);

        let incoming = Message { id: 1, account_id: 1, inbox_id: 1, conversation_id: 1, message_type: MessageType::Incoming, content: "Hello".to_string(), sender_type: SenderType::Contact, sender_id: Some(1), private: false };

        let reply = engine.process_incoming_message(incoming).await.unwrap().unwrap();

        assert_eq!(reply.content, "Bot SupportBot auto-reply to: Hello");
        assert_eq!(engine.messages.len(), 2);
    }

    #[test]
    fn test_human_handoff() {
        let mut engine = OmnichannelEngine::new(None);
        let conv = Conversation { id: 1, account_id: 1, inbox_id: 1, contact_id: 1, assignee_id: None, assignee_agent_bot_id: Some(1), status: ConversationStatus::Open, custom_attributes: None };
        engine.conversations.insert(1, conv);

        engine.handoff_to_human(1, Some(99)).unwrap();

        let updated_conv = engine.conversations.get(&1).unwrap();
        assert_eq!(updated_conv.assignee_agent_bot_id, None);
        assert_eq!(updated_conv.assignee_id, Some(99));

        assert_eq!(engine.messages.len(), 1);
        assert_eq!(engine.messages[0].message_type.clone() as i32, MessageType::Activity as i32);
        assert!(engine.messages[0].content.contains("handed off"));
    }

    #[tokio::test]
    async fn test_copilot_drafting() {
        let mut engine = OmnichannelEngine::new(None);
        engine.conversations.insert(1, Conversation { id: 1, account_id: 1, inbox_id: 1, contact_id: 1, assignee_id: None, assignee_agent_bot_id: None, status: ConversationStatus::Open, custom_attributes: None });

        let draft = engine.draft_copilot_response(1, "customer asked for refund").await.unwrap();
        assert!(draft.contains("[Draft]"));
        assert!(draft.contains("refund"));
    }

    #[tokio::test]
    async fn test_intent_classification() {
        let mut engine = OmnichannelEngine::new(None);

        let intent1 = engine.classify_intent("How much does this cost?").await.unwrap();
        assert!(matches!(intent1, IntentClassification::Billing));

        let intent2 = engine.classify_intent("My app is broken").await.unwrap();
        assert!(matches!(intent2, IntentClassification::Support));

        // Test with mock AI agent
        let llm = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(llm, vec![]));
        engine.ai_agent = Some(agent);

        let intent_sales = engine.classify_intent("I want to buy").await.unwrap();
        assert!(matches!(intent_sales, IntentClassification::Sales));

        let intent_feedback = engine.classify_intent("It sucks").await.unwrap();
        assert!(matches!(intent_feedback, IntentClassification::Feedback));
    }
}
