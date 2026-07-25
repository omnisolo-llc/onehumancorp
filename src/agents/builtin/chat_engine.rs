use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ohc_builtin_agent_llm::LlmClient;
use ohc_builtin_agent_core::types::{ChatRequest, Message, Role};

/// Chatwoot Retirement & Native Rust Omnichannel Chat Integration
/// Implements matching native AI auto-responder, copilot response drafting,
/// intent classification, and human agent handoff features in Rust.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
    Snoozed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: i64,
    pub account_id: i64,
    pub inbox_id: i64,
    pub status: ConversationStatus,
    pub assignee_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: i64,
    pub conversation_id: i64,
    pub content: String,
    pub message_type: i32, // e.g., 0 for incoming, 1 for outgoing
    pub sender_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub event: String,
    pub conversation: Option<Conversation>,
    pub message: Option<ChatMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Intent {
    Support,
    Sales,
    Billing,
    Unknown,
}

pub struct OmnichannelChatEngine {
    pub llm_client: Arc<dyn LlmClient>,
    pub model: String,
}

impl OmnichannelChatEngine {
    pub fn new(llm_client: Arc<dyn LlmClient>, model: String) -> Self {
        Self { llm_client, model }
    }

    /// Handles incoming webhook payloads mirroring Chatwoot bot/webhook protocols.
    pub async fn handle_webhook_payload(&self, payload: &WebhookPayload) -> Result<String, String> {
        match payload.event.as_str() {
            "message_created" => {
                if let Some(msg) = &payload.message {
                    if msg.message_type == 0 && msg.sender_type == "Contact" {
                        return Ok(format!("Processed new incoming message in conversation {}", msg.conversation_id));
                    }
                }
                Ok("Ignored non-contact message".to_string())
            }
            "conversation_created" => {
                Ok("Conversation created".to_string())
            }
            _ => Ok(format!("Unhandled event: {}", payload.event)),
        }
    }

    /// Generates an AI auto-response for a given message content using the LLM.
    pub async fn generate_auto_response(&self, message_content: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        if message_content.is_empty() {
            return Ok(String::new());
        }

        let sys_prompt = "You are a helpful AI assistant for OneHumanCorp. Please generate a concise, polite, and helpful auto-reply to the following customer message.".to_string();

        let req = ChatRequest {
            model: self.model.clone(),
            system: Some(sys_prompt),
            messages: vec![Message::user(message_content)],
            temperature: 0.7,
            max_tokens: 500,
            tools: vec![],
            json_schema: None,
        };

        let response = self.llm_client.chat(req).await?;
        Ok(response.message.content.clone())
    }

    /// Drafts a copilot response for an agent to review using the LLM.
    pub async fn draft_copilot_response(&self, message_content: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
         if message_content.is_empty() {
            return Ok(String::new());
        }

        let sys_prompt = "You are an AI Copilot assisting a human support agent. Please draft a professional and empathetic response to the following customer message. Prefix your response with '[Draft] '.".to_string();

        let req = ChatRequest {
            model: self.model.clone(),
            system: Some(sys_prompt),
            messages: vec![Message::user(message_content)],
            temperature: 0.7,
            max_tokens: 500,
            tools: vec![],
            json_schema: None,
        };

        let response = self.llm_client.chat(req).await?;
        Ok(response.message.content.clone())
    }

    /// Classifies the intent of the message using the LLM.
    pub async fn classify_intent(&self, message_content: &str) -> Result<Intent, Box<dyn std::error::Error + Send + Sync>> {
        if message_content.is_empty() {
            return Ok(Intent::Unknown);
        }

        let sys_prompt = "Classify the intent of the following customer message into exactly one of these categories: Support, Sales, Billing, Unknown. Output ONLY the category name.".to_string();

        let req = ChatRequest {
            model: self.model.clone(),
            system: Some(sys_prompt),
            messages: vec![Message::user(message_content)],
            temperature: 0.0,
            max_tokens: 10,
            tools: vec![],
            json_schema: None,
        };

        let response = self.llm_client.chat(req).await?;
        let result = response.message.content.trim().to_lowercase();

        match result.as_str() {
            "support" => Ok(Intent::Support),
            "sales" => Ok(Intent::Sales),
            "billing" => Ok(Intent::Billing),
            _ => Ok(Intent::Unknown),
        }
    }

    /// Hands off the conversation to a human agent.
    pub fn handoff_to_human(&self, conversation: &mut Conversation, human_agent_id: i64) {
        conversation.assignee_id = Some(human_agent_id);
        conversation.status = ConversationStatus::Open;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage};
    use tokio::sync::Mutex;

    struct MockChatEngineLlm {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockChatEngineLlm {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "Mock response".to_string()
            };

            Ok(ChatResponse {
                message: Message::assistant(content),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_handle_webhook_payload_message_created() {
        let client = Arc::new(MockChatEngineLlm { responses: Mutex::new(vec![]) });
        let engine = OmnichannelChatEngine::new(client, "test-model".to_string());

        let payload = WebhookPayload {
            event: "message_created".to_string(),
            conversation: None,
            message: Some(ChatMessage {
                id: 1,
                conversation_id: 100,
                content: "Hello".to_string(),
                message_type: 0,
                sender_type: "Contact".to_string(),
            }),
        };
        let res = engine.handle_webhook_payload(&payload).await.unwrap();
        assert_eq!(res, "Processed new incoming message in conversation 100");
    }

    #[tokio::test]
    async fn test_generate_auto_response() {
        let client = Arc::new(MockChatEngineLlm {
            responses: Mutex::new(vec!["Hello! How can I help you today?".to_string()])
        });
        let engine = OmnichannelChatEngine::new(client, "test-model".to_string());

        let res = engine.generate_auto_response("hi").await.unwrap();
        assert_eq!(res, "Hello! How can I help you today?");

        let res_empty = engine.generate_auto_response("").await.unwrap();
        assert_eq!(res_empty, "");
    }

    #[tokio::test]
    async fn test_draft_copilot_response() {
        let client = Arc::new(MockChatEngineLlm {
            responses: Mutex::new(vec!["[Draft] I'm sorry to hear about that issue.".to_string()])
        });
        let engine = OmnichannelChatEngine::new(client, "test-model".to_string());

        let res = engine.draft_copilot_response("My app crashed").await.unwrap();
        assert_eq!(res, "[Draft] I'm sorry to hear about that issue.");
    }

    #[tokio::test]
    async fn test_classify_intent() {
        let client = Arc::new(MockChatEngineLlm {
            responses: Mutex::new(vec!["support".to_string(), "billing".to_string()])
        });
        let engine = OmnichannelChatEngine::new(client, "test-model".to_string());

        let res1 = engine.classify_intent("Help me").await.unwrap();
        assert_eq!(res1, Intent::Support);

        let res2 = engine.classify_intent("Invoice please").await.unwrap();
        assert_eq!(res2, Intent::Billing);
    }

    #[test]
    fn test_handoff_to_human() {
        // Handoff doesn't require LLM so we can use a dummy
        let client = Arc::new(MockChatEngineLlm { responses: Mutex::new(vec![]) });
        let engine = OmnichannelChatEngine::new(client, "test-model".to_string());

        let mut conv = Conversation {
            id: 1,
            account_id: 1,
            inbox_id: 1,
            status: ConversationStatus::Pending,
            assignee_id: None,
        };
        engine.handoff_to_human(&mut conv, 42);
        assert_eq!(conv.assignee_id, Some(42));
        assert_eq!(conv.status, ConversationStatus::Open);
    }
}
