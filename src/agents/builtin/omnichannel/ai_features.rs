use std::sync::Arc;
use ohc_builtin_agent_llm::LlmClient;
use ohc_builtin_agent_core::types::{ChatRequest, Message};

pub struct AiFeatureProcessor {
    pub llm: Arc<dyn LlmClient>,
}

impl AiFeatureProcessor {
    pub fn new(llm: Arc<dyn LlmClient>) -> Self {
        Self { llm }
    }

    /// Auto-Responder: Automatically drafts a response based on conversation history
    pub async fn draft_auto_response(&self, history: &[super::models::Message]) -> Result<String, String> {
        let mut prompt = String::from("You are a helpful customer support AI. Draft a response for the following conversation:\n");
        for msg in history {
            prompt.push_str(&format!("{}: {}\n", msg.message_type, msg.content));
        }

        let request = ChatRequest {
            model: "default".to_string(),
            system: "".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            temperature: 0.7,
            max_tokens: 1000,
        };

        let response = self.llm.chat(request).await.map_err(|e| e.to_string())?;
        Ok(response.message.content)
    }

    /// Intent Classification: Determine the user's intent from their message
    pub async fn classify_intent(&self, message: &str) -> Result<String, String> {
        let prompt = format!("Classify the intent of the following customer message into one of: 'support', 'sales', 'billing', 'feedback', 'other'. Message: '{}'", message);

        let request = ChatRequest {
            model: "default".to_string(),
            system: "".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            temperature: 0.0,
            max_tokens: 50,
        };

        let response = self.llm.chat(request).await.map_err(|e| e.to_string())?;
        Ok(response.message.content.trim().to_lowercase())
    }
}
