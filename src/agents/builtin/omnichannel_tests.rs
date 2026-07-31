use ohc_builtin_agent::omnichannel::{OmnichannelEngine, Conversation, ChatMessage, ConversationIntent};
use ohc_builtin_agent::types::{ChatRequest, ChatResponse, Message, Usage};
use ohc_builtin_agent::llm::LlmClient;
use std::sync::Arc;
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
async fn test_classify_intent_sales() {
    let llm = Arc::new(MockOmniLlm {
        response_text: Mutex::new("Sales".to_string()),
    });
    let engine = OmnichannelEngine::new(llm);
    let mut conv = create_test_conversation();
    conv.messages[0].content = "I want to buy a new subscription plan.".to_string();

    let intent = engine.classify_intent(&conv).await.unwrap();
    assert_eq!(intent, ConversationIntent::Sales);
}

#[tokio::test]
async fn test_classify_intent_billing() {
    let llm = Arc::new(MockOmniLlm {
        response_text: Mutex::new("Billing".to_string()),
    });
    let engine = OmnichannelEngine::new(llm);
    let mut conv = create_test_conversation();
    conv.messages[0].content = "I need a refund for my last invoice.".to_string();

    let intent = engine.classify_intent(&conv).await.unwrap();
    assert_eq!(intent, ConversationIntent::Billing);
}

#[tokio::test]
async fn test_classify_intent_escalation() {
    let llm = Arc::new(MockOmniLlm {
        response_text: Mutex::new("HumanEscalation".to_string()),
    });
    let engine = OmnichannelEngine::new(llm);
    let mut conv = create_test_conversation();
    conv.messages[0].content = "Let me speak to a manager right now!".to_string();

    let intent = engine.classify_intent(&conv).await.unwrap();
    assert_eq!(intent, ConversationIntent::HumanEscalation);
}

#[tokio::test]
async fn test_classify_intent_inquiry() {
    let llm = Arc::new(MockOmniLlm {
        response_text: Mutex::new("GeneralInquiry".to_string()),
    });
    let engine = OmnichannelEngine::new(llm);
    let mut conv = create_test_conversation();
    conv.messages[0].content = "What time do you open?".to_string();

    let intent = engine.classify_intent(&conv).await.unwrap();
    assert_eq!(intent, ConversationIntent::GeneralInquiry);
}

#[tokio::test]
async fn test_human_handoff_already_handed_off() {
    let llm = Arc::new(MockOmniLlm {
        response_text: Mutex::new("".to_string()),
    });
    let engine = OmnichannelEngine::new(llm);
    let mut conv = create_test_conversation();

    conv.requires_human = true;
    engine.process_human_handoff(&mut conv);
    assert_eq!(conv.requires_human, true);
}

#[tokio::test]
async fn test_classify_intent_error_handling() {
    struct MockErrorLlm;
    #[async_trait]
    impl LlmClient for MockErrorLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Err("Simulated LLM network error".into())
        }
        async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(vec![0.1; 1536])
        }
    }

    let llm = Arc::new(MockErrorLlm);
    let engine = OmnichannelEngine::new(llm);
    let conv = create_test_conversation();

    let result = engine.classify_intent(&conv).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to classify intent"));
}

#[tokio::test]
async fn test_generate_auto_response_error_handling() {
    struct MockErrorLlm;
    #[async_trait]
    impl LlmClient for MockErrorLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Err("Simulated LLM network error".into())
        }
        async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(vec![0.1; 1536])
        }
    }

    let llm = Arc::new(MockErrorLlm);
    let engine = OmnichannelEngine::new(llm);
    let conv = create_test_conversation();

    let result = engine.generate_auto_response(&conv).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to generate auto-response"));
}

#[tokio::test]
async fn test_draft_copilot_response_error_handling() {
    struct MockErrorLlm;
    #[async_trait]
    impl LlmClient for MockErrorLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Err("Simulated LLM network error".into())
        }
        async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(vec![0.1; 1536])
        }
    }

    let llm = Arc::new(MockErrorLlm);
    let engine = OmnichannelEngine::new(llm);
    let conv = create_test_conversation();

    let result = engine.draft_copilot_response(&conv, "internal notes").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to draft copilot response"));
}
