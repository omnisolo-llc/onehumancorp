use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum ChatIntent {
    Support,
    Sales,
    Billing,
    HandoffRequest,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct ChatContext {
    pub tenant_id: String,
    pub customer_id: String,
    pub conversation_history: Vec<ChatMessage>,
    pub is_auto_responder_enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatAction {
    SendAutoReply(String),
    DraftCopilotResponse(String),
    HandoffToHuman(String),
    NoAction,
}

#[async_trait::async_trait]
pub trait IntentClassifier: Send + Sync {
    async fn classify(&self, message: &str) -> ChatIntent;
}

#[async_trait::async_trait]
pub trait AIAutoResponder: Send + Sync {
    async fn generate_reply(&self, context: &ChatContext, latest_message: &str) -> String;
}

#[async_trait::async_trait]
pub trait CopilotDrafting: Send + Sync {
    async fn draft_response(&self, context: &ChatContext, latest_message: &str) -> String;
}

pub struct OmnichannelEngine {
    classifier: Arc<dyn IntentClassifier>,
    responder: Arc<dyn AIAutoResponder>,
    drafter: Arc<dyn CopilotDrafting>,
}

impl OmnichannelEngine {
    pub fn new(
        classifier: Arc<dyn IntentClassifier>,
        responder: Arc<dyn AIAutoResponder>,
        drafter: Arc<dyn CopilotDrafting>,
    ) -> Self {
        Self {
            classifier,
            responder,
            drafter,
        }
    }

    pub async fn process_incoming_message(
        &self,
        context: &mut ChatContext,
        message: &str,
    ) -> ChatAction {
        let intent = self.classifier.classify(message).await;

        if intent == ChatIntent::HandoffRequest {
            context.is_auto_responder_enabled = false;
            return ChatAction::HandoffToHuman(
                "Customer requested human assistance. Handoff initiated.".to_string(),
            );
        }

        if context.is_auto_responder_enabled {
            let reply = self.responder.generate_reply(context, message).await;
            ChatAction::SendAutoReply(reply)
        } else {
            let draft = self.drafter.draft_response(context, message).await;
            ChatAction::DraftCopilotResponse(draft)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockClassifier {
        intent: ChatIntent,
    }
    #[async_trait::async_trait]
    impl IntentClassifier for MockClassifier {
        async fn classify(&self, _message: &str) -> ChatIntent {
            self.intent.clone()
        }
    }

    struct MockResponder;
    #[async_trait::async_trait]
    impl AIAutoResponder for MockResponder {
        async fn generate_reply(&self, _context: &ChatContext, _message: &str) -> String {
            "Auto reply".to_string()
        }
    }

    struct MockDrafter;
    #[async_trait::async_trait]
    impl CopilotDrafting for MockDrafter {
        async fn draft_response(&self, _context: &ChatContext, _message: &str) -> String {
            "Draft response".to_string()
        }
    }

    #[tokio::test]
    async fn test_auto_responder_flow() {
        let engine = OmnichannelEngine::new(
            Arc::new(MockClassifier {
                intent: ChatIntent::Support,
            }),
            Arc::new(MockResponder),
            Arc::new(MockDrafter),
        );

        let mut context = ChatContext {
            tenant_id: "t1".to_string(),
            customer_id: "c1".to_string(),
            conversation_history: vec![],
            is_auto_responder_enabled: true,
        };

        let action = engine.process_incoming_message(&mut context, "Help").await;
        assert_eq!(action, ChatAction::SendAutoReply("Auto reply".to_string()));
        assert_eq!(context.is_auto_responder_enabled, true);
    }

    #[tokio::test]
    async fn test_copilot_draft_flow() {
        let engine = OmnichannelEngine::new(
            Arc::new(MockClassifier {
                intent: ChatIntent::Sales,
            }),
            Arc::new(MockResponder),
            Arc::new(MockDrafter),
        );

        let mut context = ChatContext {
            tenant_id: "t1".to_string(),
            customer_id: "c1".to_string(),
            conversation_history: vec![],
            is_auto_responder_enabled: false,
        };

        let action = engine.process_incoming_message(&mut context, "Pricing?").await;
        assert_eq!(action, ChatAction::DraftCopilotResponse("Draft response".to_string()));
        assert_eq!(context.is_auto_responder_enabled, false);
    }

    #[tokio::test]
    async fn test_human_handoff_flow() {
        let engine = OmnichannelEngine::new(
            Arc::new(MockClassifier {
                intent: ChatIntent::HandoffRequest,
            }),
            Arc::new(MockResponder),
            Arc::new(MockDrafter),
        );

        let mut context = ChatContext {
            tenant_id: "t1".to_string(),
            customer_id: "c1".to_string(),
            conversation_history: vec![],
            is_auto_responder_enabled: true,
        };

        let action = engine.process_incoming_message(&mut context, "Agent please").await;
        assert_eq!(
            action,
            ChatAction::HandoffToHuman(
                "Customer requested human assistance. Handoff initiated.".to_string()
            )
        );
        // Ensure auto responder is disabled after handoff
        assert_eq!(context.is_auto_responder_enabled, false);
    }
}
