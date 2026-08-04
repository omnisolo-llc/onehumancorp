pub mod chat_engine;
pub mod models;
pub mod routing;
pub mod webhooks;
pub mod ai_features;

pub use chat_engine::ChatEngine;

#[cfg(test)]
mod tests {
    use super::*;
    use models::{Conversation, Message};

    #[tokio::test]
    async fn test_omnichannel_chat_engine_creation() {
        let _engine = ChatEngine::new("sqlite::memory:").await.unwrap();
    }

    #[tokio::test]
    async fn test_omnichannel_chat_engine_add_message() {
        let engine = ChatEngine::new("sqlite::memory:").await.unwrap();

        let conv = Conversation {
            id: "conv-1".to_string(),
            account_id: "acc-1".to_string(),
            inbox_id: "inbox-1".to_string(),
            status: "open".to_string(),
            assignee_id: None,
            created_at: 1000,
        };

        engine.create_conversation(conv).await.unwrap();

        let msg = Message {
            id: "msg-1".to_string(),
            conversation_id: "conv-1".to_string(),
            content: "Hello from OHC Omnichannel".to_string(),
            message_type: "incoming".to_string(),
            sender_id: Some("contact-1".to_string()),
            created_at: 1001,
        };

        engine.add_message(msg).await.unwrap();

        let history = engine.get_conversation_history("conv-1").await.unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].content, "Hello from OHC Omnichannel");
    }
}

#[cfg(test)]
mod ai_tests {
    use super::*;
    use ai_features::AiFeatureProcessor;
    use ohc_builtin_agent_llm::LlmClient;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message as CoreMessage, Usage};
    use std::sync::Arc;

    struct MockLlmClient {
        response_text: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: CoreMessage::assistant(self.response_text.clone()),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: None,
            })
        }
        async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(vec![])
        }
    }

    #[tokio::test]
    async fn test_omnichannel_auto_responder() {
        let llm = Arc::new(MockLlmClient {
            response_text: "I can help you with that.".to_string(),
        });
        let processor = AiFeatureProcessor::new(llm);

        let history = vec![
            models::Message {
                id: "1".to_string(),
                conversation_id: "c1".to_string(),
                content: "I need help.".to_string(),
                message_type: "incoming".to_string(),
                sender_id: None,
                created_at: 1000,
            }
        ];

        let response = processor.draft_auto_response(&history).await.unwrap();
        assert_eq!(response, "I can help you with that.");
    }

    #[tokio::test]
    async fn test_omnichannel_intent_classification() {
        let llm = Arc::new(MockLlmClient {
            response_text: "Billing".to_string(),
        });
        let processor = AiFeatureProcessor::new(llm);

        let intent = processor.classify_intent("I want a refund").await.unwrap();
        assert_eq!(intent, "billing");
    }
}
