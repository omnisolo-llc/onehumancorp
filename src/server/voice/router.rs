use std::sync::Arc;
use ::server_integrations_twilio::provider::TwilioProvider;
use super::engine::VoiceAIEdgeEngine;

pub struct VoiceContextRouter {
    engine: Arc<VoiceAIEdgeEngine>,
    twilio: Arc<TwilioProvider>,
}

impl VoiceContextRouter {
    pub fn new(engine: Arc<VoiceAIEdgeEngine>, twilio: Arc<TwilioProvider>) -> Self {
        Self { engine, twilio }
    }

    pub async fn process_user_input(&self, session_id: &str, user_text: &str, merchant_phone: &str) -> String {
        self.engine.log_transcript(session_id, "USER", user_text).await;

        let user_text_lower = user_text.to_lowercase();
        let ai_response;

        if user_text_lower.contains("book") || user_text_lower.contains("appointment") || user_text_lower.contains("opening") {
            // Mock Calendar DB interaction
            self.engine.log_intent_action(session_id, "CHECK_AVAILABILITY", serde_json::json!({"action": "checking calendar"})).await;
            ai_response = "Yes, I have an opening tomorrow at 2 PM! Should I book it for you?".to_string();
        } else if user_text_lower.contains("yes") {
            // Mock Booking interaction
            self.engine.log_intent_action(session_id, "BOOK_APPOINTMENT", serde_json::json!({"action": "booking slot"})).await;

            // Trigger SMS link
            let calls = self.engine.active_calls.lock().await;
            if let Some(call) = calls.iter().find(|c| c.session_id == session_id) {
                let caller_phone = call.caller_phone.clone();
                let _ = self.twilio.send_sms(
                    &caller_phone,
                    merchant_phone,
                    "Here is your secure link to pay the deposit and confirm your booking: https://pay.ohc.com/book/123",
                ).await;
            }

            ai_response = "All set! I've texted you a confirmation with a link to pay the deposit.".to_string();
        } else {
            ai_response = "I'm sorry, I can help you with bookings or answering questions about our menu. How can I assist you?".to_string();
        }

        self.engine.log_transcript(session_id, "AI", &ai_response).await;
        ai_response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::server_integrations_twilio::client::TwilioClientWrapper;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockTwilioClient {
        sent_messages: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl TwilioClientWrapper for MockTwilioClient {
        async fn send_sms(&self, _to: &str, _from: &str, _body: &str) -> Result<(), String> {
            self.sent_messages.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_voice_context_router_booking_flow() {
        let engine = Arc::new(VoiceAIEdgeEngine::new());
        let sent = Arc::new(AtomicUsize::new(0));
        let mock_twilio = Arc::new(TwilioProvider::with_client(Arc::new(MockTwilioClient { sent_messages: sent.clone() })));

        let router = VoiceContextRouter::new(engine.clone(), mock_twilio);

        let session_id = engine.handle_incoming_call("merchant_123", "+1234567890").await;

        let response1 = router.process_user_input(&session_id, "Do you have an opening tomorrow?", "+0987654321").await;
        assert!(response1.contains("2 PM"));

        let response2 = router.process_user_input(&session_id, "Yes, please.", "+0987654321").await;
        assert!(response2.contains("texted you"));

        // Ensure SMS was sent
        assert_eq!(sent.load(Ordering::SeqCst), 1);

        // Ensure intents were logged
        let actions = engine.actions.lock().await;
        assert_eq!(actions.len(), 2);
        assert_eq!(actions[0].intent_type, "CHECK_AVAILABILITY");
        assert_eq!(actions[1].intent_type, "BOOK_APPOINTMENT");
    }
}
