#![allow(clippy::empty_line_after_doc_comments)]
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role};
use std::sync::Arc;

/// Master Catalog Harness Innovation: Native Rust Omnichannel Chat Integration
/// Native AI auto-responder, copilot response drafting, intent classification, and human agent handoff features in Rust.

#[derive(Debug, PartialEq, Clone)]
pub enum Intent {
    Sales,
    Support,
    Booking,
    Escalation,
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub enum HandoffReason {
    ExplicitRequest,
    HighFrustration,
    ComplexIntent,
}

pub struct ChatContext {
    pub customer_id: String,
    pub channel: String,
    pub message_history: Vec<Message>,
}

#[async_trait::async_trait]
pub trait OmnichannelLlmClient: Send + Sync {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct IntentClassifier {
    llm: Arc<dyn OmnichannelLlmClient>,
}

impl IntentClassifier {
    pub fn new(llm: Arc<dyn OmnichannelLlmClient>) -> Self {
        Self { llm }
    }

    pub async fn classify_intent(&self, msg: &str) -> Intent {
        let req = ChatRequest {
            model: "".to_string(),
            system: "".to_string(),
            tools: vec![],
            max_tokens: 1000,
            temperature: 0.0,
            messages: vec![
                Message {
                    role: Role::System,
                    content: "Classify the following user message into exactly one of these intents: Sales, Support, Booking, Escalation, or Unknown. Reply with ONLY the intent name.".to_string(),
                    tool_calls: vec![],
                    previous_response_id: None,
                    response_id: None,
                    tool_results: vec![],
                },
                Message {
                    role: Role::User,
                    content: msg.to_string(),
                    tool_calls: vec![],
                    previous_response_id: None,
                    response_id: None,
                    tool_results: vec![],
                },
            ],
        };

        if let Ok(res) = self.llm.chat(req).await {
            let content = res.message.content.trim().to_lowercase();
            if content.contains("sales") {
                Intent::Sales
            } else if content.contains("support") {
                Intent::Support
            } else if content.contains("booking") {
                Intent::Booking
            } else if content.contains("escalation") {
                Intent::Escalation
            } else {
                Intent::Unknown
            }
        } else {
            Intent::Unknown
        }
    }
}

pub struct AutoResponder {
    llm: Arc<dyn OmnichannelLlmClient>,
}

impl AutoResponder {
    pub fn new(llm: Arc<dyn OmnichannelLlmClient>) -> Self {
        Self { llm }
    }

    pub async fn draft_response(&self, intent: &Intent, context: &ChatContext) -> String {
        let intent_str = match intent {
            Intent::Sales => "Sales",
            Intent::Support => "Support",
            Intent::Booking => "Booking",
            Intent::Escalation => "Escalation",
            Intent::Unknown => "Unknown",
        };

        let mut messages = vec![Message {
            role: Role::System,
            content: format!("You are an AI auto-responder. The detected intent is {}. Draft a helpful, concise response. Do not use placeholders.", intent_str),
            tool_calls: vec![],
            previous_response_id: None,
            response_id: None,
            tool_results: vec![],
        }];

        for msg in &context.message_history {
            messages.push(msg.clone());
        }

        let req = ChatRequest {
            model: "".to_string(),
            system: "".to_string(),
            tools: vec![],
            max_tokens: 1000,
            temperature: 0.0,
            messages,
        };

        if let Ok(res) = self.llm.chat(req).await {
            res.message.content.clone()
        } else {
            "I'm sorry, I'm having trouble processing your request right now. Let me connect you with a human agent.".to_string()
        }
    }
}

pub struct HumanAgentHandoff {
    llm: Arc<dyn OmnichannelLlmClient>,
}

impl HumanAgentHandoff {
    pub fn new(llm: Arc<dyn OmnichannelLlmClient>) -> Self {
        Self { llm }
    }

    pub async fn check_handoff(&self, msg: &str, intent: &Intent) -> Option<HandoffReason> {
        if intent == &Intent::Escalation {
            return Some(HandoffReason::ComplexIntent);
        }

        let req = ChatRequest {
            model: "".to_string(),
            system: "".to_string(),
            tools: vec![],
            max_tokens: 1000,
            temperature: 0.0,
            messages: vec![
                Message {
                    role: Role::System,
                    content: "Analyze the user message. If they explicitly request a human/agent/manager, reply EXACTLY with 'ExplicitRequest'. If they are highly frustrated or angry, reply EXACTLY with 'HighFrustration'. Otherwise, reply with 'None'.".to_string(),
                    tool_calls: vec![],
                    previous_response_id: None,
                    response_id: None,
                    tool_results: vec![],
                },
                Message {
                    role: Role::User,
                    content: msg.to_string(),
                    tool_calls: vec![],
                    previous_response_id: None,
                    response_id: None,
                    tool_results: vec![],
                },
            ],
        };

        if let Ok(res) = self.llm.chat(req).await {
            let content = res.message.content.trim().to_lowercase();
            if content.contains("explicitrequest") {
                Some(HandoffReason::ExplicitRequest)
            } else if content.contains("highfrustration") {
                Some(HandoffReason::HighFrustration)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct OmnichannelChatEngine {
    classifier: IntentClassifier,
    responder: AutoResponder,
    handoff: HumanAgentHandoff,
}

impl OmnichannelChatEngine {
    pub fn new(llm: Arc<dyn OmnichannelLlmClient>) -> Self {
        Self {
            classifier: IntentClassifier::new(llm.clone()),
            responder: AutoResponder::new(llm.clone()),
            handoff: HumanAgentHandoff::new(llm.clone()),
        }
    }

    pub async fn process_message(&self, context: &ChatContext) -> (String, Option<HandoffReason>) {
        if let Some(last_msg) = context.message_history.last() {
            let msg_text = last_msg.content.clone();

            let intent = self.classifier.classify_intent(&msg_text).await;
            let handoff_reason = self.handoff.check_handoff(&msg_text, &intent).await;

            if handoff_reason.is_some() {
                let response = self.responder.draft_response(&Intent::Escalation, context).await;
                (response, handoff_reason)
            } else {
                let response = self.responder.draft_response(&intent, context).await;
                (response, None)
            }
        } else {
            ("Hello! How can I help you today?".to_string(), None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{Role, Usage};
    use tokio::sync::Mutex;

    struct MockOmnichannelLlm {
        responses: Mutex<Vec<ChatResponse>>,
    }

    #[async_trait::async_trait]
    impl OmnichannelLlmClient for MockOmnichannelLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            if !resps.is_empty() {
                Ok(resps.remove(0))
            } else {
                Err("No mock responses left".into())
            }
        }
    }

    fn create_mock_response(content: &str) -> ChatResponse {
        ChatResponse {
            response_id: None,
            message: Message {
                role: Role::Assistant,
                content: content.to_string(),
                tool_calls: vec![],
                previous_response_id: None,
                response_id: None,
                tool_results: vec![],
            },
            stop_reason: "stop".to_string(),
            usage: Usage {
                cache_creation_input_tokens: 0,
                cache_read_input_tokens: 0,
                input_tokens: 10,
                output_tokens: 10,
            },
        }
    }

    #[tokio::test]
    async fn test_intent_classification_llm() {
        let llm = Arc::new(MockOmnichannelLlm {
            responses: Mutex::new(vec![
                create_mock_response("Sales"),
                create_mock_response("Unknown"),
            ]),
        });
        let classifier = IntentClassifier::new(llm);

        assert_eq!(classifier.classify_intent("I want to buy").await, Intent::Sales);
        assert_eq!(classifier.classify_intent("Hello").await, Intent::Unknown);
    }

    #[tokio::test]
    async fn test_auto_responder_llm() {
        let llm = Arc::new(MockOmnichannelLlm {
            responses: Mutex::new(vec![
                create_mock_response("Here is the pricing info."),
            ]),
        });
        let responder = AutoResponder::new(llm);
        let ctx = ChatContext {
            customer_id: "1".to_string(),
            channel: "Web".to_string(),
            message_history: vec![],
        };

        let reply = responder.draft_response(&Intent::Sales, &ctx).await;
        assert_eq!(reply, "Here is the pricing info.");
    }

    #[tokio::test]
    async fn test_human_handoff_llm() {
        let llm = Arc::new(MockOmnichannelLlm {
            responses: Mutex::new(vec![
                create_mock_response("ExplicitRequest"),
                create_mock_response("HighFrustration"),
                create_mock_response("None"),
            ]),
        });
        let handoff = HumanAgentHandoff::new(llm);

        assert_eq!(handoff.check_handoff("Human please", &Intent::Support).await, Some(HandoffReason::ExplicitRequest));
        assert_eq!(handoff.check_handoff("I am angry", &Intent::Support).await, Some(HandoffReason::HighFrustration));
        assert_eq!(handoff.check_handoff("How much?", &Intent::Sales).await, None);
        assert_eq!(handoff.check_handoff("Escalate this", &Intent::Escalation).await, Some(HandoffReason::ComplexIntent)); // Does not consume mock
    }

    #[tokio::test]
    async fn test_chat_engine_processing_llm() {
        let llm = Arc::new(MockOmnichannelLlm {
            responses: Mutex::new(vec![
                create_mock_response("Sales"), // Intent
                create_mock_response("None"), // Handoff
                create_mock_response("Sure, I can help with sales."), // Responder
            ]),
        });
        let engine = OmnichannelChatEngine::new(llm);

        let sales_context = ChatContext {
            customer_id: "2".to_string(),
            channel: "WhatsApp".to_string(),
            message_history: vec![Message { role: Role::User, content: "What's the price?".to_string(), tool_calls: vec![], previous_response_id: None, response_id: None, tool_results: vec![] }],
        };

        let (resp, handoff) = engine.process_message(&sales_context).await;
        assert_eq!(resp, "Sure, I can help with sales.");
        assert_eq!(handoff, None);
    }
}
