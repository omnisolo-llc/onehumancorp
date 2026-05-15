use serde::{Deserialize, Serialize};
use crate::services::onboarding::unified_models::UnifiedModelsManager;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OnboardingState {
    Greeting,
    AskBusinessType,
    AskBusinessDetails,
    AskStorefrontItems,
    ClarifyItemDetails,
    ReviewAndConfirm,
    HandlingCorrection,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub state: OnboardingState,
    pub messages: Vec<ChatMessage>,
    pub business_type: Option<String>,
    pub business_name: Option<String>,
    pub business_description: Option<String>,
    pub items: Vec<serde_json::Value>,
    pub context_buffer: String,
}

pub struct ConversationalOnboardingService {
    models_manager: UnifiedModelsManager,
}

impl ConversationalOnboardingService {
    pub fn new() -> Self {
        Self {
            models_manager: UnifiedModelsManager::new(),
        }
    }

    pub fn start_session(&self) -> ChatSession {
        ChatSession {
            id: format!("sess-{}", Uuid::new_v4()),
            state: OnboardingState::Greeting,
            messages: vec![ChatMessage {
                role: "assistant".to_string(),
                content: "Welcome to OHC! What kind of business are you starting today?".to_string(),
            }],
            business_type: None,
            business_name: None,
            business_description: None,
            items: vec![],
            context_buffer: String::new(),
        }
    }

    pub fn handle_message(&self, mut session: ChatSession, user_message: &str) -> ChatSession {
        session.messages.push(ChatMessage {
            role: "user".to_string(),
            content: user_message.to_string(),
        });

        // Intent parsing (simplified)
        let msg_lower = user_message.to_lowercase();
        if msg_lower.contains("cancel") || msg_lower.contains("stop") {
            session.state = OnboardingState::Failed("User cancelled onboarding".to_string());
            session.messages.push(ChatMessage {
                role: "assistant".to_string(),
                content: "Onboarding cancelled. Let me know if you want to restart.".to_string(),
            });
            return session;
        }

        if msg_lower.contains("wait, no") || msg_lower.contains("no wait") || msg_lower.contains("correction") || msg_lower.contains("let me change") {
            session.state = OnboardingState::HandlingCorrection;
            session.messages.push(ChatMessage {
                role: "assistant".to_string(),
                content: "No problem. What would you like to change?".to_string(),
            });
            return session;
        }

        match session.state {
            OnboardingState::HandlingCorrection => {
                // Return to appropriate state based on context (mocked for simplicity)
                session.state = OnboardingState::AskStorefrontItems;
                session.messages.push(ChatMessage {
                    role: "assistant".to_string(),
                    content: "Got it. Let's continue. What products or services do you offer?".to_string(),
                });
            }
            OnboardingState::Greeting => {
                if msg_lower.len() < 3 {
                    session.messages.push(ChatMessage {
                        role: "assistant".to_string(),
                        content: "Could you provide a bit more detail about your business type?".to_string(),
                    });
                } else {
                    session.business_type = Some(user_message.to_string());
                    session.state = OnboardingState::AskBusinessDetails;
                    session.messages.push(ChatMessage {
                        role: "assistant".to_string(),
                        content: "Great! What is the name of your business and a short description?".to_string(),
                    });
                }
            }
            OnboardingState::AskBusinessType => {
                 session.messages.push(ChatMessage {
                    role: "assistant".to_string(),
                    content: "Could you specify the business type again?".to_string(),
                });
            }
            OnboardingState::AskBusinessDetails => {
                session.business_name = Some(user_message.to_string());
                session.business_description = Some(user_message.to_string());
                session.state = OnboardingState::AskStorefrontItems;
                session.messages.push(ChatMessage {
                    role: "assistant".to_string(),
                    content: "Got it. Tell me about the first product or service you'd like to offer, including its price.".to_string(),
                });
            }
            OnboardingState::AskStorefrontItems => {
                if !msg_lower.contains("$") && !msg_lower.contains("price") && !msg_lower.contains("cost") {
                    session.context_buffer = user_message.to_string();
                    session.state = OnboardingState::ClarifyItemDetails;
                    session.messages.push(ChatMessage {
                        role: "assistant".to_string(),
                        content: "Could you also provide the price for that?".to_string(),
                    });
                } else {
                    let item_type = if session.business_type.as_deref().unwrap_or("").to_lowercase().contains("service") { "booking" } else { "physical" };
                    let item = self.models_manager.create_item("temp-org", user_message, "Generated from chat", item_type, 1000, None);
                    session.items.push(serde_json::to_value(item).unwrap());
                    session.state = OnboardingState::ReviewAndConfirm;
                    session.messages.push(ChatMessage {
                        role: "assistant".to_string(),
                        content: "Excellent! We're ready to create your storefront. Type 'confirm' to finalize.".to_string(),
                    });
                }
            }
            OnboardingState::ClarifyItemDetails => {
                 let item_type = if session.business_type.as_deref().unwrap_or("").to_lowercase().contains("service") { "booking" } else { "physical" };
                 let combined_desc = format!("{} - {}", session.context_buffer, user_message);
                 let item = self.models_manager.create_item("temp-org", &session.context_buffer, &combined_desc, item_type, 1000, None);
                 session.items.push(serde_json::to_value(item).unwrap());
                 session.state = OnboardingState::ReviewAndConfirm;
                 session.messages.push(ChatMessage {
                     role: "assistant".to_string(),
                     content: "Got the price! We're ready to create your storefront. Type 'confirm' to finalize.".to_string(),
                 });
                 session.context_buffer.clear();
            }
            OnboardingState::ReviewAndConfirm => {
                if msg_lower.contains("confirm") || msg_lower.contains("yes") || msg_lower.contains("looks good") {
                    session.state = OnboardingState::Completed;
                    session.messages.push(ChatMessage {
                        role: "assistant".to_string(),
                        content: "Storefront created! Generating your website and AI agents...".to_string(),
                    });
                } else {
                    session.messages.push(ChatMessage {
                        role: "assistant".to_string(),
                        content: "Please type 'confirm' to proceed, or let me know if you need to change anything.".to_string(),
                    });
                }
            }
            OnboardingState::Completed => {
                session.messages.push(ChatMessage {
                    role: "assistant".to_string(),
                    content: "Your setup is already complete!".to_string(),
                });
            }
            OnboardingState::Failed(_) => {
                 session.messages.push(ChatMessage {
                    role: "assistant".to_string(),
                    content: "Onboarding is cancelled. Refresh to start over.".to_string(),
                });
            }
        }

        session
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversational_flow_happy_path() {
        let service = ConversationalOnboardingService::new();
        let mut session = service.start_session();

        assert_eq!(session.state, OnboardingState::Greeting);
        assert_eq!(session.messages.len(), 1);

        session = service.handle_message(session, "A bakery");
        assert_eq!(session.state, OnboardingState::AskBusinessDetails);
        assert_eq!(session.business_type, Some("A bakery".to_string()));

        session = service.handle_message(session, "Maya's Cakes, the best vegan cakes.");
        assert_eq!(session.state, OnboardingState::AskStorefrontItems);
        assert_eq!(session.business_name, Some("Maya's Cakes, the best vegan cakes.".to_string()));

        session = service.handle_message(session, "Vegan Chocolate Cake $25");
        assert_eq!(session.state, OnboardingState::ReviewAndConfirm);
        assert_eq!(session.items.len(), 1);

        session = service.handle_message(session, "confirm");
        assert_eq!(session.state, OnboardingState::Completed);

        session = service.handle_message(session, "hello");
        assert_eq!(session.state, OnboardingState::Completed);
    }

    #[test]
    fn test_conversational_flow_too_short_greeting() {
        let service = ConversationalOnboardingService::new();
        let mut session = service.start_session();

        session = service.handle_message(session, "hi");
        assert_eq!(session.state, OnboardingState::Greeting); // Should stay
    }

    #[test]
    fn test_conversational_flow_cancellation() {
        let service = ConversationalOnboardingService::new();
        let mut session = service.start_session();

        session = service.handle_message(session, "cancel onboarding");
        match session.state {
            OnboardingState::Failed(_) => assert!(true),
            _ => assert!(false, "Expected Failed state"),
        }

        session = service.handle_message(session, "hello");
        match session.state {
            OnboardingState::Failed(_) => assert!(true),
            _ => assert!(false, "Expected Failed state"),
        }
    }

    #[test]
    fn test_conversational_flow_correction() {
        let service = ConversationalOnboardingService::new();
        let mut session = service.start_session();

        session = service.handle_message(session, "wait, no, let me change that");
        assert_eq!(session.state, OnboardingState::HandlingCorrection);

        session = service.handle_message(session, "I want to sell books");
        assert_eq!(session.state, OnboardingState::AskStorefrontItems);
    }

    #[test]
    fn test_conversational_flow_clarify_price() {
        let service = ConversationalOnboardingService::new();
        let mut session = service.start_session();

        session = service.handle_message(session, "bakery");
        session = service.handle_message(session, "test biz");

        // No price given
        session = service.handle_message(session, "chocolate cake");
        assert_eq!(session.state, OnboardingState::ClarifyItemDetails);
        assert_eq!(session.context_buffer, "chocolate cake");

        session = service.handle_message(session, "it costs $20");
        assert_eq!(session.state, OnboardingState::ReviewAndConfirm);
        assert_eq!(session.items.len(), 1);
        assert!(session.context_buffer.is_empty());
    }

    #[test]
    fn test_conversational_flow_invalid_confirm() {
        let service = ConversationalOnboardingService::new();
        let mut session = service.start_session();

        session = service.handle_message(session, "bakery");
        session = service.handle_message(session, "test biz");
        session = service.handle_message(session, "chocolate cake $20");

        // At ReviewAndConfirm
        assert_eq!(session.state, OnboardingState::ReviewAndConfirm);

        session = service.handle_message(session, "no wait");
        assert_eq!(session.state, OnboardingState::HandlingCorrection);
    }

    #[test]
    fn test_conversational_flow_handle_cancel_midway() {
        let service = ConversationalOnboardingService::new();
        let mut session = service.start_session();

        session = service.handle_message(session, "bakery");
        session = service.handle_message(session, "cancel");
        match session.state {
            OnboardingState::Failed(_) => assert!(true),
            _ => assert!(false, "Expected Failed state"),
        }
    }

    #[test]
    fn test_conversational_flow_handle_stop() {
        let service = ConversationalOnboardingService::new();
        let mut session = service.start_session();

        session = service.handle_message(session, "bakery");
        session = service.handle_message(session, "stop");
        match session.state {
            OnboardingState::Failed(_) => assert!(true),
            _ => assert!(false, "Expected Failed state"),
        }
    }

    #[test]
    fn test_conversational_flow_handle_correction_advanced() {
        let service = ConversationalOnboardingService::new();
        let mut session = service.start_session();

        session = service.handle_message(session, "bakery");
        session = service.handle_message(session, "wait, no");
        assert_eq!(session.state, OnboardingState::HandlingCorrection);
    }
}
