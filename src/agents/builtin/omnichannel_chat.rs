use crate::llm::LlmClient;
use ohc_builtin_agent_core::types::{ChatRequest, Message};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Native Rust Omnichannel Chat System Standard
/// Chatwoot Retirement & Custom Rust Omnichannel Chat System Standard (MANDATORY):
/// Native AI auto-responder, copilot response drafting, intent classification, and human agent handoff features in Rust.

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Intent {
    Support,
    Sales,
    Complaint,
    Spam,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub session_id: String,
    pub sender: String,
    pub content: String,
    pub timestamp: String,
}

pub struct IntentClassifier {
    llm: Arc<dyn LlmClient>,
}

impl IntentClassifier {
    pub fn new(llm: Arc<dyn LlmClient>) -> Self {
        Self { llm }
    }

    pub async fn classify(&self, message: &str) -> Result<Intent, String> {
        let prompt = format!(
            "Classify the following customer message into one of these categories: Support, Sales, Complaint, Spam. Respond with ONLY the category name.\n\nMessage: {}",
            message
        );

        let req = ChatRequest {
            model: "default".to_string(),
            system: "You are an intent classifier for an omnichannel chat system.".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 10,
            temperature: 0.0,
        };

        match self.llm.chat(req).await {
            Ok(resp) => {
                let text = resp.message.content.trim().to_lowercase();
                if text.contains("support") {
                    Ok(Intent::Support)
                } else if text.contains("sales") {
                    Ok(Intent::Sales)
                } else if text.contains("complaint") {
                    Ok(Intent::Complaint)
                } else if text.contains("spam") {
                    Ok(Intent::Spam)
                } else {
                    Ok(Intent::Unknown)
                }
            }
            Err(e) => Err(format!("LLM Error: {}", e)),
        }
    }
}

pub struct CopilotDrafter {
    llm: Arc<dyn LlmClient>,
}

impl CopilotDrafter {
    pub fn new(llm: Arc<dyn LlmClient>) -> Self {
        Self { llm }
    }

    pub async fn draft_response(&self, history: &[ChatMessage], tone: &str) -> Result<String, String> {
        let mut prompt = String::new();
        prompt.push_str("Based on the following chat history, draft a response for the human agent to send to the customer.\n");
        prompt.push_str(&format!("Tone: {}\n\nHistory:\n", tone));

        for msg in history {
            prompt.push_str(&format!("{}: {}\n", msg.sender, msg.content));
        }

        let req = ChatRequest {
            model: "default".to_string(),
            system: "You are an AI copilot drafting a response for a customer support agent.".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 1000,
            temperature: 0.7,
        };

        match self.llm.chat(req).await {
            Ok(resp) => Ok(resp.message.content.trim().to_string()),
            Err(e) => Err(format!("LLM Error: {}", e)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AutoResponderAction {
    Reply(String),
    Handoff(String), // Reason for handoff
}

pub struct AutoResponder {
    llm: Arc<dyn LlmClient>,
    #[allow(dead_code)]
    confidence_threshold: f32, // Not directly used since LLMs don't output pure confidence scores easily, but modeled here
}

impl AutoResponder {
    pub fn new(llm: Arc<dyn LlmClient>, confidence_threshold: f32) -> Self {
        Self { llm, confidence_threshold }
    }

    pub async fn process(&self, message: &str, intent: &Intent) -> Result<AutoResponderAction, String> {
        if *intent == Intent::Complaint || *intent == Intent::Unknown {
            return Ok(AutoResponderAction::Handoff("Intent requires human empathy or is ambiguous.".to_string()));
        }

        let prompt = format!(
            "Generate an automatic response to the following customer message. The intent is {:?}. If you cannot safely and confidently answer, respond with exactly 'HANDOFF'.\n\nMessage: {}",
            intent, message
        );

        let req = ChatRequest {
            model: "default".to_string(),
            system: "You are an AI auto-responder. Answer questions directly or say HANDOFF.".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 500,
            temperature: 0.1,
        };

        match self.llm.chat(req).await {
            Ok(resp) => {
                let content = resp.message.content.trim();
                if content == "HANDOFF" {
                    Ok(AutoResponderAction::Handoff("AI decided it could not answer confidently.".to_string()))
                } else {
                    Ok(AutoResponderAction::Reply(content.to_string()))
                }
            }
            Err(e) => Err(format!("LLM Error: {}", e)),
        }
    }
}

pub struct HandoffManager {
    // In a real system, this would interact with a routing DB or pub/sub, but for this component, we use redis via a simple redis client abstraction.
    redis: redis::Client,
}

impl HandoffManager {
    pub fn new(redis_url: &str) -> Result<Self, String> {
        let redis = redis::Client::open(redis_url).map_err(|e| format!("Redis connection error: {}", e))?;
        Ok(Self {
            redis,
        })
    }

    pub fn trigger_handoff(&self, session_id: &str, reason: &str) -> Result<(), String> {
        let mut conn = self.redis.get_connection().map_err(|e| format!("Redis error: {}", e))?;
        let _ : () = redis::cmd("SADD").arg("active_handoffs").arg(session_id).query(&mut conn).map_err(|e| format!("Redis error: {}", e))?;
        let _ : () = redis::cmd("HSET").arg(format!("handoff_reason:{}", session_id)).arg("reason").arg(reason).query(&mut conn).map_err(|e| format!("Redis error: {}", e))?;
        println!("Handoff triggered for session {}: {}", session_id, reason);
        Ok(())
    }

    pub fn resolve_handoff(&self, session_id: &str) -> Result<(), String> {
        let mut conn = self.redis.get_connection().map_err(|e| format!("Redis error: {}", e))?;
        let _ : () = redis::cmd("SREM").arg("active_handoffs").arg(session_id).query(&mut conn).map_err(|e| format!("Redis error: {}", e))?;
        let _ : () = redis::cmd("DEL").arg(format!("handoff_reason:{}", session_id)).query(&mut conn).map_err(|e| format!("Redis error: {}", e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::ChatResponse;

    struct MockLlm {
        response_text: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlm {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(self.response_text.clone()),
                usage: ohc_builtin_agent_core::types::Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_intent_classifier() {
        let classifier = IntentClassifier::new(Arc::new(MockLlm { response_text: "Support".to_string() }));
        let result = classifier.classify("I need help with my account.").await.unwrap();
        assert_eq!(result, Intent::Support);

        let classifier = IntentClassifier::new(Arc::new(MockLlm { response_text: "spam".to_string() }));
        let result = classifier.classify("Buy vi4gra").await.unwrap();
        assert_eq!(result, Intent::Spam);
    }

    #[tokio::test]
    async fn test_copilot_drafter() {
        let drafter = CopilotDrafter::new(Arc::new(MockLlm { response_text: "Sure, let me check that for you.".to_string() }));
        let history = vec![
            ChatMessage { id: "1".to_string(), session_id: "s1".to_string(), sender: "Customer".to_string(), content: "Where is my order?".to_string(), timestamp: "now".to_string() }
        ];
        let result = drafter.draft_response(&history, "professional").await.unwrap();
        assert_eq!(result, "Sure, let me check that for you.");
    }

    #[tokio::test]
    async fn test_auto_responder_reply() {
        let responder = AutoResponder::new(Arc::new(MockLlm { response_text: "Our hours are 9-5.".to_string() }), 0.9);
        let result = responder.process("When do you open?", &Intent::Support).await.unwrap();
        assert_eq!(result, AutoResponderAction::Reply("Our hours are 9-5.".to_string()));
    }

    #[tokio::test]
    async fn test_auto_responder_handoff_on_complaint() {
        let responder = AutoResponder::new(Arc::new(MockLlm { response_text: "Doesn't matter".to_string() }), 0.9);
        let result = responder.process("I hate your service", &Intent::Complaint).await.unwrap();
        if let AutoResponderAction::Handoff(reason) = result {
            assert!(reason.contains("human empathy"));
        } else {
            panic!("Expected handoff");
        }
    }

    #[tokio::test]
    async fn test_auto_responder_handoff_from_llm() {
        let responder = AutoResponder::new(Arc::new(MockLlm { response_text: "HANDOFF".to_string() }), 0.9);
        let result = responder.process("I have a complex issue.", &Intent::Support).await.unwrap();
        if let AutoResponderAction::Handoff(reason) = result {
            assert!(reason.contains("could not answer confidently"));
        } else {
            panic!("Expected handoff");
        }
    }
}
