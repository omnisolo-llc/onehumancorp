use std::sync::Arc;
use crate::llm::LlmClient;
use crate::types::{ChatRequest, Message};

pub struct OmnichannelChatEngine {
    llm: Arc<dyn LlmClient>,
    model: String,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Intent {
    Support,
    Sales,
    Billing,
    General,
    Escalation,
}

impl OmnichannelChatEngine {
    pub fn new(llm: Arc<dyn LlmClient>, model: String) -> Self {
        Self { llm, model }
    }

    pub async fn auto_respond(&self, customer_message: &str) -> Result<String, String> {
        let req = ChatRequest {
            model: self.model.clone(),
            system: "You are an AI auto-responder for an omnichannel customer support system. Provide a helpful, concise answer.".to_string(),
            messages: vec![Message::user(customer_message)],
            tools: vec![],
            max_tokens: 500,
            temperature: 0.5,
        };

        match self.llm.chat(req).await {
            Ok(resp) => Ok(resp.message.content),
            Err(e) => Err(format!("Failed to auto-respond: {}", e)),
        }
    }

    pub async fn draft_copilot_response(
        &self,
        customer_message: &str,
        agent_context: &str,
    ) -> Result<String, String> {
        let prompt = format!(
            "Context: {}\nCustomer Message: {}\nDraft a response for the human agent to review.",
            agent_context, customer_message
        );
        let req = ChatRequest {
            model: self.model.clone(),
            system: "You are an AI Copilot assisting a human agent. Draft a professional, empathetic response based on the provided context.".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 500,
            temperature: 0.7,
        };

        match self.llm.chat(req).await {
            Ok(resp) => Ok(resp.message.content),
            Err(e) => Err(format!("Failed to draft response: {}", e)),
        }
    }

    pub async fn classify_intent(&self, customer_message: &str) -> Result<Intent, String> {
        let req = ChatRequest {
            model: self.model.clone(),
            system: "Classify the intent of the following customer message into one of these categories: Support, Sales, Billing, General, Escalation. Only output the category name.".to_string(),
            messages: vec![Message::user(customer_message)],
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
                } else if text.contains("billing") {
                    Ok(Intent::Billing)
                } else if text.contains("escalation") {
                    Ok(Intent::Escalation)
                } else {
                    Ok(Intent::General)
                }
            }
            Err(e) => Err(format!("Failed to classify intent: {}", e)),
        }
    }

    pub async fn should_handoff_to_human(
        &self,
        customer_message: &str,
        intent: &Intent,
    ) -> bool {
        if matches!(intent, Intent::Escalation) {
            return true;
        }

        let req = ChatRequest {
            model: self.model.clone(),
            system: "Determine if the following customer message requires human intervention. Answer YES or NO.".to_string(),
            messages: vec![Message::user(customer_message)],
            tools: vec![],
            max_tokens: 10,
            temperature: 0.0,
        };

        match self.llm.chat(req).await {
            Ok(resp) => {
                resp.message.content.trim().to_uppercase().contains("YES")
            }
            Err(_) => true, // Handoff by default on error
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatResponse, Usage};

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
                message: Message::assistant(&self.response_text),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_auto_respond() {
        let llm = Arc::new(MockLlm {
            response_text: "We can help you with that right away!".to_string(),
        });
        let engine = OmnichannelChatEngine::new(llm, "test-model".to_string());

        let response = engine.auto_respond("I need help").await.unwrap();
        assert_eq!(response, "We can help you with that right away!");
    }

    #[tokio::test]
    async fn test_draft_copilot_response() {
        let llm = Arc::new(MockLlm {
            response_text: "Here is a draft response based on the context.".to_string(),
        });
        let engine = OmnichannelChatEngine::new(llm, "test-model".to_string());

        let response = engine.draft_copilot_response("I need help", "Customer is a VIP").await.unwrap();
        assert_eq!(response, "Here is a draft response based on the context.");
    }

    #[tokio::test]
    async fn test_classify_intent() {
        // Support
        let llm = Arc::new(MockLlm {
            response_text: "Support".to_string(),
        });
        let engine = OmnichannelChatEngine::new(llm, "test-model".to_string());
        let intent = engine.classify_intent("How do I fix this error?").await.unwrap();
        assert_eq!(intent, Intent::Support);

        // Sales
        let llm_sales = Arc::new(MockLlm {
            response_text: "Sales".to_string(),
        });
        let engine_sales = OmnichannelChatEngine::new(llm_sales, "test-model".to_string());
        let intent_sales = engine_sales.classify_intent("I want to buy the premium plan").await.unwrap();
        assert_eq!(intent_sales, Intent::Sales);

        // Escalation
        let llm_esc = Arc::new(MockLlm {
            response_text: "Escalation".to_string(),
        });
        let engine_esc = OmnichannelChatEngine::new(llm_esc, "test-model".to_string());
        let intent_esc = engine_esc.classify_intent("I demand to speak to a manager right now").await.unwrap();
        assert_eq!(intent_esc, Intent::Escalation);
    }

    #[tokio::test]
    async fn test_should_handoff_to_human() {
        // Direct Escalation intent
        let llm_any = Arc::new(MockLlm {
            response_text: "NO".to_string(), // LLM response shouldn't matter if intent is Escalation
        });
        let engine = OmnichannelChatEngine::new(llm_any, "test-model".to_string());

        let handoff = engine.should_handoff_to_human("Some message", &Intent::Escalation).await;
        assert!(handoff);

        // Based on LLM response (YES)
        let llm_yes = Arc::new(MockLlm {
            response_text: "YES".to_string(),
        });
        let engine_yes = OmnichannelChatEngine::new(llm_yes, "test-model".to_string());
        let handoff_yes = engine_yes.should_handoff_to_human("Please help me figure this out, I'm confused.", &Intent::Support).await;
        assert!(handoff_yes);

        // Based on LLM response (NO)
        let llm_no = Arc::new(MockLlm {
            response_text: "NO".to_string(),
        });
        let engine_no = OmnichannelChatEngine::new(llm_no, "test-model".to_string());
        let handoff_no = engine_no.should_handoff_to_human("Where are the docs?", &Intent::General).await;
        assert!(!handoff_no);
    }
}
