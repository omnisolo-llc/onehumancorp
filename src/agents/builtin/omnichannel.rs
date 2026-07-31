use std::sync::Arc;
use crate::llm::LlmClient;
use crate::types::{ChatRequest, Message};

/// Custom Rust Omnichannel Chat System Standard
/// Replaces external Chatwoot dependencies with a native Rust implementation.
/// Includes Intent Classification, Auto-Responder, Copilot Response Drafting, and Human Handoff.

#[derive(Debug, Clone, PartialEq)]
pub enum ConversationIntent {
    Support,
    Sales,
    Billing,
    GeneralInquiry,
    HumanEscalation,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub sender_id: String,
    pub content: String,
    pub is_from_customer: bool,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct Conversation {
    pub id: String,
    pub customer_id: String,
    pub messages: Vec<ChatMessage>,
    pub intent: Option<ConversationIntent>,
    pub requires_human: bool,
}

pub struct OmnichannelEngine {
    llm: Arc<dyn LlmClient>,
}

impl OmnichannelEngine {
    pub fn new(llm: Arc<dyn LlmClient>) -> Self {
        Self { llm }
    }

    /// Replicates Chatwoot's intent classification (often handled by external NLP or Dialogflow).
    /// Uses the LLM to classify the intent based on the conversation history.
    pub async fn classify_intent(&self, conversation: &Conversation) -> Result<ConversationIntent, String> {
        let mut prompt = "Classify the intent of the following customer conversation into one of the following categories: Support, Sales, Billing, GeneralInquiry, HumanEscalation. Reply ONLY with the category name.\n\nConversation:\n".to_string();

        for msg in &conversation.messages {
            let sender = if msg.is_from_customer { "Customer" } else { "Agent" };
            prompt.push_str(&format!("{}: {}\n", sender, msg.content));
        }

        let req = ChatRequest {
            model: "default".to_string(),
            system: "You are an intent classification engine. Output only the category name.".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 50,
            temperature: 0.0,
        };

        match self.llm.chat(req).await {
            Ok(resp) => {
                let text = resp.message.content.trim().to_lowercase();
                if text.contains("support") {
                    Ok(ConversationIntent::Support)
                } else if text.contains("sales") {
                    Ok(ConversationIntent::Sales)
                } else if text.contains("billing") {
                    Ok(ConversationIntent::Billing)
                } else if text.contains("human") || text.contains("escalation") {
                    Ok(ConversationIntent::HumanEscalation)
                } else if text.contains("general") || text.contains("inquiry") {
                    Ok(ConversationIntent::GeneralInquiry)
                } else {
                    Ok(ConversationIntent::Unknown)
                }
            }
            Err(e) => Err(format!("Failed to classify intent: {}", e)),
        }
    }

    /// Replicates Chatwoot's auto-responder bot features.
    pub async fn generate_auto_response(&self, conversation: &Conversation) -> Result<String, String> {
        let mut prompt = "Generate a helpful, concise auto-response for the customer based on the following conversation history.\n\nConversation:\n".to_string();

        for msg in &conversation.messages {
            let sender = if msg.is_from_customer { "Customer" } else { "Agent" };
            prompt.push_str(&format!("{}: {}\n", sender, msg.content));
        }

        let req = ChatRequest {
            model: "default".to_string(),
            system: "You are an automated customer support agent. Provide a polite and helpful response.".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 500,
            temperature: 0.7,
        };

        match self.llm.chat(req).await {
            Ok(resp) => Ok(resp.message.content.trim().to_string()),
            Err(e) => Err(format!("Failed to generate auto-response: {}", e)),
        }
    }

    /// Replicates Chatwoot's Agent Copilot feature (drafting replies for human agents).
    pub async fn draft_copilot_response(&self, conversation: &Conversation, internal_notes: &str) -> Result<String, String> {
        let mut prompt = format!("Draft a reply for the human agent to send to the customer. Use the internal notes provided to inform the response.\n\nInternal Notes:\n{}\n\nConversation:\n", internal_notes);

        for msg in &conversation.messages {
            let sender = if msg.is_from_customer { "Customer" } else { "Agent" };
            prompt.push_str(&format!("{}: {}\n", sender, msg.content));
        }

        let req = ChatRequest {
            model: "default".to_string(),
            system: "You are an AI assistant drafting a reply for a human customer support agent. The draft should be ready to send.".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 500,
            temperature: 0.7,
        };

        match self.llm.chat(req).await {
            Ok(resp) => Ok(resp.message.content.trim().to_string()),
            Err(e) => Err(format!("Failed to draft copilot response: {}", e)),
        }
    }

    /// Manages handoff from bot to human agent, replicating Chatwoot's conversation assignment logic.
    pub fn process_human_handoff(&self, conversation: &mut Conversation) {
        // In a real system, this would update the database to assign the conversation to a human team/agent
        // and pause the bot processing.
        conversation.requires_human = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatResponse, Usage};
    use async_trait::async_trait;
    use tokio::sync::Mutex;

    struct MockOmniLlm {
        response_text: Mutex<String>,
    }

    #[async_trait]
    impl LlmClient for MockOmniLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let text = self.response_text.lock().await.clone();
            Ok(ChatResponse {
                message: Message::assistant(text),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: None,
            })
        }

        async fn generate_embedding(
            &self,
            _text: &str,
        ) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(vec![0.1; 1536])
        }
    }

    fn create_test_conversation() -> Conversation {
        Conversation {
            id: "conv_1".to_string(),
            customer_id: "cust_1".to_string(),
            messages: vec![
                ChatMessage {
                    sender_id: "cust_1".to_string(),
                    content: "My laptop screen is broken, I need help fixing it.".to_string(),
                    is_from_customer: true,
                    timestamp: 1000,
                }
            ],
            intent: None,
            requires_human: false,
        }
    }

    #[tokio::test]
    async fn test_classify_intent() {
        let llm = Arc::new(MockOmniLlm {
            response_text: Mutex::new("Support".to_string()),
        });
        let engine = OmnichannelEngine::new(llm);
        let conv = create_test_conversation();

        let intent = engine.classify_intent(&conv).await.unwrap();
        assert_eq!(intent, ConversationIntent::Support);
    }

    #[tokio::test]
    async fn test_generate_auto_response() {
        let llm = Arc::new(MockOmniLlm {
            response_text: Mutex::new("I'm sorry to hear that! Please provide your order number.".to_string()),
        });
        let engine = OmnichannelEngine::new(llm);
        let conv = create_test_conversation();

        let resp = engine.generate_auto_response(&conv).await.unwrap();
        assert_eq!(resp, "I'm sorry to hear that! Please provide your order number.");
    }

    #[tokio::test]
    async fn test_draft_copilot_response() {
        let llm = Arc::new(MockOmniLlm {
            response_text: Mutex::new("Hi there, the repair will cost $100. Should we proceed?".to_string()),
        });
        let engine = OmnichannelEngine::new(llm);
        let conv = create_test_conversation();

        let resp = engine.draft_copilot_response(&conv, "Repair costs $100").await.unwrap();
        assert_eq!(resp, "Hi there, the repair will cost $100. Should we proceed?");
    }

    #[tokio::test]
    async fn test_human_handoff() {
        let llm = Arc::new(MockOmniLlm {
            response_text: Mutex::new("".to_string()),
        });
        let engine = OmnichannelEngine::new(llm);
        let mut conv = create_test_conversation();

        assert_eq!(conv.requires_human, false);
        engine.process_human_handoff(&mut conv);
        assert_eq!(conv.requires_human, true);
    }
}
