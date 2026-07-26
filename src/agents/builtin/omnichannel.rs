use std::sync::Arc;
use serde::{Deserialize, Serialize};
use ohc_builtin_agent_core::types::{ChatRequest, Message};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Intent {
    Support,
    Sales,
    Complaint,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HandoffReason {
    ComplexQuery,
    FrustratedUser,
    ExplicitRequest,
    HighValueSales,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandoffStatus {
    pub needs_human: bool,
    pub reason: Option<HandoffReason>,
}

#[derive(Clone)]
pub struct OmnichannelEngine {
    llm_client: Arc<dyn crate::llm::LlmClient>,
}

impl OmnichannelEngine {
    pub fn new(llm_client: Arc<dyn crate::llm::LlmClient>) -> Self {
        Self { llm_client }
    }

    /// Native Intent Classification for omnichannel inbox using LLM
    pub async fn intent_classification(&self, message: &str) -> Intent {
        let prompt = format!(
            "Classify the following message into one of these intents: Support, Sales, Complaint, or Unknown.\n\nMessage: \"{}\"\n\nReply with only the exact name of the intent.",
            message
        );

        let req = ChatRequest {
            model: "default".to_string(),
            system: "You are an omnichannel AI assistant specialized in intent classification.".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 10,
            temperature: 0.0,
        };

        if let Ok(resp) = self.llm_client.chat(req).await {
            let reply = resp.message.content.trim().to_lowercase();
            if reply.contains("sales") {
                Intent::Sales
            } else if reply.contains("complaint") {
                Intent::Complaint
            } else if reply.contains("support") {
                Intent::Support
            } else {
                Intent::Unknown
            }
        } else {
            Intent::Unknown
        }
    }

    /// AI Auto-Responder for standard omnichannel queries using LLM
    pub async fn auto_responder(&self, message: &str) -> Option<String> {
        let intent = self.intent_classification(message).await;

        if intent == Intent::Unknown {
            return None;
        }

        let context_info = match intent {
            Intent::Support => "They are asking for help or support.",
            Intent::Sales => "They are interested in buying products or asking for pricing.",
            Intent::Complaint => "They are unhappy and filing a complaint.",
            Intent::Unknown => "",
        };

        let prompt = format!(
            "Write a polite, helpful, and concise auto-reply for the following customer message. {}\n\nCustomer Message: \"{}\"",
            context_info, message
        );

        let req = ChatRequest {
            model: "default".to_string(),
            system: "You are an expert customer service auto-responder AI.".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 150,
            temperature: 0.7,
        };

        if let Ok(resp) = self.llm_client.chat(req).await {
            Some(resp.message.content.trim().to_string())
        } else {
            None
        }
    }

    /// Copilot response drafting for omnichannel inbox using LLM
    pub async fn copilot_drafting(&self, message: &str, context: &str) -> String {
        let prompt = format!(
            "Draft a professional reply to the customer's message using the provided context.\n\nCustomer Message: \"{}\"\nContext: \"{}\"\n\nReply with only the draft text.",
            message, context
        );

        let req = ChatRequest {
            model: "default".to_string(),
            system: "You are an AI copilot helping human agents draft customer replies.".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 300,
            temperature: 0.7,
        };

        if let Ok(resp) = self.llm_client.chat(req).await {
            let intent = self.intent_classification(message).await;
            let draft_prefix = match intent {
                Intent::Sales => "[Sales Draft]: ",
                Intent::Support => "[Support Draft]: ",
                Intent::Complaint => "[Escalation Draft]: ",
                Intent::Unknown => "[General Draft]: ",
            };
            format!("{} {}", draft_prefix, resp.message.content.trim())
        } else {
            format!("[Error]: Failed to generate draft for: {}", message)
        }
    }

    /// Human agent handoff logic using LLM
    pub async fn human_handoff(&self, message: &str) -> HandoffStatus {
        let prompt = format!(
            "Analyze the following customer message to determine if it requires human intervention.
Return 'YES' if it explicitly asks for a human, expresses severe frustration, or discusses a complex/high-value topic (like enterprise or bulk orders).
Return 'NO' if it is a standard inquiry that can be handled automatically.
If YES, also provide a reason: ExplicitRequest, FrustratedUser, HighValueSales, or ComplexQuery.
Format: <YES/NO> - <Reason if YES>
Message: \"{}\"",
            message
        );

        let req = ChatRequest {
            model: "default".to_string(),
            system: "You are an omnichannel routing AI that determines when to hand off to a human agent.".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 50,
            temperature: 0.0,
        };

        if let Ok(resp) = self.llm_client.chat(req).await {
            let reply = resp.message.content.trim().to_lowercase();
            if reply.starts_with("yes") {
                let reason = if reply.contains("explicitrequest") {
                    Some(HandoffReason::ExplicitRequest)
                } else if reply.contains("frustrateduser") {
                    Some(HandoffReason::FrustratedUser)
                } else if reply.contains("highvaluesales") {
                    Some(HandoffReason::HighValueSales)
                } else {
                    Some(HandoffReason::ComplexQuery)
                };
                return HandoffStatus {
                    needs_human: true,
                    reason,
                };
            }
        }

        HandoffStatus {
            needs_human: false,
            reason: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use ohc_builtin_agent_core::types::ChatResponse;

    struct MockOmniLlm {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl crate::llm::LlmClient for MockOmniLlm {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut responses = self.responses.lock().unwrap();
            let content = if responses.is_empty() {
                "Unknown".to_string()
            } else {
                responses.remove(0)
            };

            Ok(ChatResponse {
                message: Message::assistant(&content),
                usage: Default::default(),
                stop_reason: "stop".to_string(),
                response_id: None,
            })
        }
    }

    #[tokio::test]
    async fn test_intent_classification() {
        let llm = Arc::new(MockOmniLlm {
            responses: Mutex::new(vec!["Sales".to_string(), "Complaint".to_string(), "Support".to_string(), "Unknown".to_string()]),
        });
        let engine = OmnichannelEngine::new(llm);
        assert_eq!(engine.intent_classification("What is the price?").await, Intent::Sales);
        assert_eq!(engine.intent_classification("My item is broken!").await, Intent::Complaint);
        assert_eq!(engine.intent_classification("I need help with my account").await, Intent::Support);
        assert_eq!(engine.intent_classification("Hello").await, Intent::Unknown);
    }

    #[tokio::test]
    async fn test_auto_responder() {
        let llm = Arc::new(MockOmniLlm {
            responses: Mutex::new(vec![
                "Sales".to_string(), "We noticed you are interested in our products.".to_string(),
                "Complaint".to_string(), "We apologize for the inconvenience.".to_string(),
                "Support".to_string(), "Thank you for reaching out to support!".to_string(),
                "Unknown".to_string(), // Unknown shouldn't trigger a second call for auto_responder
            ]),
        });
        let engine = OmnichannelEngine::new(llm);
        assert!(engine.auto_responder("What is the price?").await.unwrap().contains("interested in our products"));
        assert!(engine.auto_responder("My item is broken!").await.unwrap().contains("apologize for the inconvenience"));
        assert!(engine.auto_responder("I need help").await.unwrap().contains("reaching out to support"));
        assert_eq!(engine.auto_responder("Hello").await, None);
    }

    #[tokio::test]
    async fn test_copilot_drafting() {
        let llm = Arc::new(MockOmniLlm {
            responses: Mutex::new(vec![
                "Product X costs $50".to_string(), "Sales".to_string(),
            ]),
        });
        let engine = OmnichannelEngine::new(llm);
        let draft = engine.copilot_drafting("What is the price?", "Product X costs $50").await;
        assert!(draft.starts_with("[Sales Draft]"));
        assert!(draft.contains("Product X costs $50"));
    }

    #[tokio::test]
    async fn test_human_handoff() {
        let llm = Arc::new(MockOmniLlm {
            responses: Mutex::new(vec![
                "YES - ExplicitRequest".to_string(),
                "YES - FrustratedUser".to_string(),
                "YES - HighValueSales".to_string(),
                "NO".to_string(),
            ]),
        });
        let engine = OmnichannelEngine::new(llm);

        let status = engine.human_handoff("I want to speak to a human").await;
        assert!(status.needs_human);
        assert_eq!(status.reason, Some(HandoffReason::ExplicitRequest));

        let status = engine.human_handoff("This is terrible!").await;
        assert!(status.needs_human);
        assert_eq!(status.reason, Some(HandoffReason::FrustratedUser));

        let status = engine.human_handoff("I want to make a bulk order").await;
        assert!(status.needs_human);
        assert_eq!(status.reason, Some(HandoffReason::HighValueSales));

        let status = engine.human_handoff("What is the price?").await;
        assert!(!status.needs_human);
        assert_eq!(status.reason, None);
    }
}
