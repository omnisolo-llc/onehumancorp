mod models;
use std::sync::Arc;
use ohc_builtin_agent::llm::LlmClient;
use ohc_builtin_agent::types::{ChatRequest, Message as LlmMessage, Role};
use crate::models::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Intent {
    Support,
    Sales,
    Billing,
    General,
    HumanHandoffRequest,
}

pub struct IntentClassifier {
    llm: Arc<dyn LlmClient>,
}

impl IntentClassifier {
    pub fn new(llm: Arc<dyn LlmClient>) -> Self {
        Self { llm }
    }

    pub async fn classify(&self, message: &Message) -> Result<Intent, String> {
        let prompt = format!(
            "Classify the following customer message into one of these intents: Support, Sales, Billing, General, or HumanHandoffRequest.\n\nMessage: \"{}\"\n\nRespond with exactly ONE word representing the intent from the list.",
            message.content
        );

        let req = ChatRequest {
            messages: vec![LlmMessage {
                role: Role::User,
                content: prompt,
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            }],
            tools: vec![],
            model: "default".to_string(),
            temperature: 0.0,
            max_tokens: 10,
            system: "".to_string(),

        };

        let resp = self.llm.chat(req).await.map_err(|e| e.to_string())?;
        let text = resp.message.content.trim().to_lowercase();

        if text.contains("support") {
            Ok(Intent::Support)
        } else if text.contains("sales") {
            Ok(Intent::Sales)
        } else if text.contains("billing") {
            Ok(Intent::Billing)
        } else if text.contains("handoff") || text.contains("human") {
            Ok(Intent::HumanHandoffRequest)
        } else {
            Ok(Intent::General)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent::types::{ChatResponse, Usage};

    struct MockLlm {
        response: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: LlmMessage::assistant(&self.response),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: None,
            })
        }
    }

    #[tokio::test]
    async fn test_classify_support() {
        let llm = Arc::new(MockLlm { response: "Support".to_string() });
        let classifier = IntentClassifier::new(llm);
        let msg = Message {
            id: "1".to_string(),
            conversation_id: "1".to_string(),
            content: "My app is crashing".to_string(),
            message_type: crate::models::MessageType::Incoming,
            sender_id: None,
            private: false,
            created_at: 0,
        };
        let intent = classifier.classify(&msg).await.unwrap();
        assert_eq!(intent, Intent::Support);
    }
}
