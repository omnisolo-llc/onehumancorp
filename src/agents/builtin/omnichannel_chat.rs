use crate::types::{ChatRequest, Message};
use ohc_builtin_agent_llm::LlmClient;
use std::sync::Arc;

pub struct OmnichannelChatEngine {
    llm_client: Arc<dyn LlmClient>,
}

impl OmnichannelChatEngine {
    pub fn new(llm_client: Arc<dyn LlmClient>) -> Self {
        Self { llm_client }
    }

    /// Auto-responder: generates a direct customer response.
    pub async fn auto_respond(
        &self,
        customer_message: &str,
        context: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let system_prompt = format!(
            "You are an AI auto-responder for a customer support chat. \
            Respond directly, politely, and concisely to the customer based on this context:\n{}",
            context
        );
        let req = ChatRequest {
            model: "default".to_string(),
            system: system_prompt,
            messages: vec![Message::user(customer_message.to_string())],
            tools: vec![],
            max_tokens: 500,
            temperature: 0.2,
        };
        let resp = self.llm_client.chat(req).await?;
        Ok(resp.message.content)
    }

    /// Copilot response drafting: generates a suggested draft for a human agent.
    pub async fn draft_copilot_response(
        &self,
        customer_message: &str,
        context: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let system_prompt = format!(
            "You are an AI copilot assisting a human customer support agent. \
            Draft a suggested response that the human agent can review and send to the customer. \
            Base the draft on this context:\n{}",
            context
        );
        let req = ChatRequest {
            model: "default".to_string(),
            system: system_prompt,
            messages: vec![Message::user(customer_message.to_string())],
            tools: vec![],
            max_tokens: 500,
            temperature: 0.2,
        };
        let resp = self.llm_client.chat(req).await?;
        Ok(format!("[DRAFT] {}", resp.message.content))
    }

    /// Intent classification: classifies the user's intent into categories.
    pub async fn classify_intent(
        &self,
        customer_message: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let system_prompt = "You are an AI intent classifier for a customer support chat. \
            Classify the customer's intent into one of the following exact categories: \
            'support', 'sales', 'billing', or 'other'. Respond with ONLY the category name."
            .to_string();

        let req = ChatRequest {
            model: "default".to_string(),
            system: system_prompt,
            messages: vec![Message::user(customer_message.to_string())],
            tools: vec![],
            max_tokens: 10, // Short response
            temperature: 0.0,
        };
        let resp = self.llm_client.chat(req).await?;
        let intent = resp.message.content.trim().to_lowercase();

        // Basic validation of the output
        if ["support", "sales", "billing", "other"].contains(&intent.as_str()) {
            Ok(intent)
        } else {
            Ok("other".to_string())
        }
    }

    /// Human agent handoff: returns a standard system message indicating transfer.
    pub async fn handoff_to_human(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // In a full implementation, this would trigger webhooks or database updates
        // to alert human agents and pause the AI auto-responder.
        Ok("Transferring you to a human agent. Please hold on a moment...".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatResponse, Usage};

    struct MockLlmClient {
        response: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(self.response.clone()),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_omnichannel_auto_respond() {
        let llm = Arc::new(MockLlmClient {
            response: "Hello! How can I help you?".to_string(),
        });
        let engine = OmnichannelChatEngine::new(llm);
        let result = engine.auto_respond("Hi", "Context: User is a VIP").await.unwrap();
        assert_eq!(result, "Hello! How can I help you?");
    }

    #[tokio::test]
    async fn test_omnichannel_draft_copilot() {
        let llm = Arc::new(MockLlmClient {
            response: "I can help with your order.".to_string(),
        });
        let engine = OmnichannelChatEngine::new(llm);
        let result = engine.draft_copilot_response("Help with order", "Context: order #123").await.unwrap();
        assert_eq!(result, "[DRAFT] I can help with your order.");
    }

    #[tokio::test]
    async fn test_omnichannel_classify_intent() {
        // Valid intent
        let llm_support = Arc::new(MockLlmClient {
            response: " support \n".to_string(),
        });
        let engine_support = OmnichannelChatEngine::new(llm_support);
        let result_support = engine_support.classify_intent("Help me").await.unwrap();
        assert_eq!(result_support, "support");

        // Invalid intent fallback to 'other'
        let llm_invalid = Arc::new(MockLlmClient {
            response: "unknown".to_string(),
        });
        let engine_invalid = OmnichannelChatEngine::new(llm_invalid);
        let result_invalid = engine_invalid.classify_intent("Help me").await.unwrap();
        assert_eq!(result_invalid, "other");
    }

    #[tokio::test]
    async fn test_omnichannel_handoff() {
        let llm = Arc::new(MockLlmClient {
            response: "".to_string(), // Unused for handoff
        });
        let engine = OmnichannelChatEngine::new(llm);
        let result = engine.handoff_to_human().await.unwrap();
        assert_eq!(result, "Transferring you to a human agent. Please hold on a moment...");
    }
}
