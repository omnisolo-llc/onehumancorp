use std::sync::Arc;
use ::server_integrations_twilio::provider::TwilioProvider;
use super::engine::VoiceAIEdgeEngine;

#[cfg(ohc_bazel_package)]
use ::minimax::{MinimaxClient, LocalLLMClient};
#[cfg(not(ohc_bazel_package))]
use crate::minimax::{MinimaxClient, LocalLLMClient};

pub struct VoiceContextRouter {
    engine: Arc<VoiceAIEdgeEngine>,
    twilio: Arc<TwilioProvider>,
    planner: Arc<dyn VoiceTurnPlanner>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoiceTurnPlan {
    pub intent_type: Option<String>,
    pub ai_response: String,
    pub sms_body: Option<String>,
    pub structured_order: Option<serde_json::Value>,
}

#[async_trait::async_trait]
pub trait VoiceTurnPlanner: Send + Sync {
    async fn plan_turn(&self, session_id: &str, user_text: &str, tenant_lang: &str) -> Result<VoiceTurnPlan, String>;
}

pub struct LlmVoiceTurnPlanner;

#[async_trait::async_trait]
impl VoiceTurnPlanner for LlmVoiceTurnPlanner {
    async fn plan_turn(&self, session_id: &str, user_text: &str, tenant_lang: &str) -> Result<VoiceTurnPlan, String> {
        let prompt = format!(
            "You are the OneHumanCorp voice receptionist planner. Return strict JSON with keys intent_type, ai_response, sms_body, and structured_order. intent_type must be CHECK_AVAILABILITY, BOOK_APPOINTMENT, GENERAL_HELP, ORDER_FOOD, or GENERAL_INQUIRY. Use sms_body only when the caller explicitly confirms a booking and a secure confirmation/deposit link should be sent, or if the caller wants to place an order (ORDER_FOOD), immediately offer to send them a secure ordering link via SMS and include the link (e.g., https://pay.ohc.com/store/voice) in the sms_body. Do not invent exact appointment availability; ask a concise follow-up when calendar data is not present. If intent_type is ORDER_FOOD and the caller has provided enough details, generate a structured JSON object in structured_order containing 'items' (array of strings) and 'pickup_time' (string) representing the finalized order. VERY IMPORTANT: The structured_order must be translated into the tenant's preferred language: {tenant_lang}. Session: {session_id}. Caller said: {user_text}"
        );

        let provider = std::env::var("OHC_VOICE_LLM_PROVIDER")
            .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
            .unwrap_or_default();

        let raw: String = match provider.as_str() {
            "minimax" => {
                let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                MinimaxClient::new(api_key).reason(&prompt).await
            }
            _ => LocalLLMClient::new().reason(&prompt).await,
        }?;

        parse_voice_turn_plan(&raw)
    }
}

impl VoiceContextRouter {
    pub fn new(engine: Arc<VoiceAIEdgeEngine>, twilio: Arc<TwilioProvider>) -> Self {
        Self::with_planner(engine, twilio, Arc::new(LlmVoiceTurnPlanner))
    }

    pub fn with_planner(
        engine: Arc<VoiceAIEdgeEngine>,
        twilio: Arc<TwilioProvider>,
        planner: Arc<dyn VoiceTurnPlanner>,
    ) -> Self {
        Self { engine, twilio, planner }
    }

    pub async fn process_user_input(&self, session_id: &str, user_text: &str, merchant_phone: &str, tenant_lang: &str) -> String {
        self.engine.log_transcript(session_id, "USER", user_text).await;

        let plan = match self.planner.plan_turn(session_id, user_text, tenant_lang).await {
            Ok(plan) => plan,
            Err(err) => {
                ::server_telemetry::record_error_signal("[bug] VoiceContextRouter LLM planning failed");
                tracing::error!("VoiceContextRouter LLM planning failed: {}", err);
                VoiceTurnPlan {
                    intent_type: None,
                    ai_response: "I'm having trouble reaching the voice assistant right now. Please try again in a moment.".to_string(),
                    sms_body: None,
                }
            }
        };

        if let Some(intent_type) = plan.intent_type.as_deref() {
            let mut details = serde_json::json!({"source": "voice_llm_planner"});
            if let Some(structured_order) = &plan.structured_order {
                if let Some(obj) = details.as_object_mut() {
                    obj.insert("structured_order".to_string(), structured_order.clone());
                }
            }

            self.engine
                .log_intent_action(
                    session_id,
                    intent_type,
                    details,
                )
                .await;
        }

        if let Some(sms_body) = plan.sms_body.as_deref() {
            let calls = self.engine.active_calls.lock().await;
            if let Some(call) = calls.iter().find(|c| c.session_id == session_id) {
                let caller_phone = call.caller_phone.clone();
                if let Err(err) = self.twilio.send_sms(&caller_phone, merchant_phone, sms_body).await {
                    ::server_telemetry::record_error_signal("[bug] VoiceContextRouter SMS dispatch failed");
                    tracing::error!("VoiceContextRouter SMS dispatch failed: {}", err);
                }
            }
        }

        self.engine.log_transcript(session_id, "AI", &plan.ai_response).await;
        plan.ai_response
    }
}

fn parse_voice_turn_plan(raw: &str) -> Result<VoiceTurnPlan, String> {
    let value: serde_json::Value = match serde_json::from_str(raw) {
        Ok(value) => value,
        Err(_) => {
            let start = raw.find('{').ok_or_else(|| "voice planner response missing JSON object".to_string())?;
            let end = raw.rfind('}').ok_or_else(|| "voice planner response missing JSON object".to_string())?;
            serde_json::from_str(&raw[start..=end])
                .map_err(|e| format!("failed to parse voice planner JSON: {e}"))?
        }
    };

    let ai_response = value
        .get("ai_response")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .ok_or_else(|| "voice planner response missing ai_response".to_string())?
        .to_string();

    let intent_type = value
        .get("intent_type")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(ToString::to_string);

    let sms_body = value
        .get("sms_body")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(ToString::to_string);

    let structured_order = value
        .get("structured_order")
        .cloned();

    Ok(VoiceTurnPlan {
        intent_type,
        ai_response,
        sms_body,
        structured_order,
    })
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
        async fn send_whatsapp(&self, _to: &str, _from: &str, _body: &str) -> Result<(), String> {
            self.sent_messages.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }

        async fn provision_number(&self, _area_code: &str) -> Result<String, String> {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let last_four: u32 = rng.gen_range(1000..9999);
            Ok(format!("+1555123{}", last_four))
        }
    }

    struct ScriptedVoiceTurnPlanner {
        plans: tokio::sync::Mutex<Vec<VoiceTurnPlan>>,
    }

    #[async_trait]
    impl VoiceTurnPlanner for ScriptedVoiceTurnPlanner {
        async fn plan_turn(&self, _session_id: &str, _user_text: &str, _tenant_lang: &str) -> Result<VoiceTurnPlan, String> {
            Ok(self.plans.lock().await.remove(0))
        }
    }

    #[test]
    fn test_parse_voice_turn_plan_from_llm_json() {
        let plan = parse_voice_turn_plan(
            r#"{"intent_type":"BOOK_APPOINTMENT","ai_response":"I sent the confirmation link.","sms_body":"Confirm here: https://ohc.example/confirm"}"#,
        )
        .unwrap();

        assert_eq!(plan.intent_type.as_deref(), Some("BOOK_APPOINTMENT"));
        assert_eq!(plan.ai_response, "I sent the confirmation link.");
        assert_eq!(plan.sms_body.as_deref(), Some("Confirm here: https://ohc.example/confirm"));

        let plan2 = parse_voice_turn_plan(
            r#"{"intent_type":"ORDER_FOOD","ai_response":"I am an automated assistant. I'm texting you a link to our online menu right now so you can place your order. Please check your messages!","sms_body":"Order here: https://pay.ohc.com/store/voice"}"#,
        )
        .unwrap();
        assert_eq!(plan2.intent_type.as_deref(), Some("ORDER_FOOD"));
        assert_eq!(plan2.ai_response, "I am an automated assistant. I'm texting you a link to our online menu right now so you can place your order. Please check your messages!");
        assert_eq!(plan2.sms_body.as_deref(), Some("Order here: https://pay.ohc.com/store/voice"));
    }

    #[tokio::test]
    async fn test_voice_context_router_booking_flow() {
        let engine = Arc::new(VoiceAIEdgeEngine::new());
        let sent = Arc::new(AtomicUsize::new(0));
        let mock_twilio = Arc::new(TwilioProvider::with_client(Arc::new(MockTwilioClient { sent_messages: sent.clone() })));
        let planner = Arc::new(ScriptedVoiceTurnPlanner {
            plans: tokio::sync::Mutex::new(vec![
                VoiceTurnPlan {
                    intent_type: Some("CHECK_AVAILABILITY".to_string()),
                    ai_response: "I can check availability for you. What day works best?".to_string(),
                    sms_body: None,
                },
                VoiceTurnPlan {
                    intent_type: Some("BOOK_APPOINTMENT".to_string()),
                    ai_response: "All set. I texted you the secure confirmation link.".to_string(),
                    sms_body: Some("Confirm your booking: https://pay.ohc.com/book/session".to_string()),
                },
            ]),
        });

        let router = VoiceContextRouter::with_planner(engine.clone(), mock_twilio, planner);

        let session_id = engine.handle_incoming_call("merchant_123", "+1234567890").await;

        let response1 = router.process_user_input(&session_id, "Do you have an opening tomorrow?", "+0987654321", "English").await;
        assert!(response1.contains("check availability"));

        let response2 = router.process_user_input(&session_id, "Yes, please.", "+0987654321", "English").await;
        assert!(response2.contains("secure confirmation"));

        // Ensure SMS was sent
        assert_eq!(sent.load(Ordering::SeqCst), 1);

        // Ensure intents were logged
        let actions = engine.actions.lock().await;
        assert_eq!(actions.len(), 2);
        assert_eq!(actions[0].intent_type, "CHECK_AVAILABILITY");
        assert_eq!(actions[1].intent_type, "BOOK_APPOINTMENT");
    }

    #[tokio::test]
    async fn test_voice_context_router_ordering_flow() {
        let engine = Arc::new(VoiceAIEdgeEngine::new());
        let sent = Arc::new(AtomicUsize::new(0));
        let mock_twilio = Arc::new(TwilioProvider::with_client(Arc::new(MockTwilioClient { sent_messages: sent.clone() })));
        let planner = Arc::new(ScriptedVoiceTurnPlanner {
            plans: tokio::sync::Mutex::new(vec![
                VoiceTurnPlan {
                    intent_type: Some("ORDER_FOOD".to_string()),
                    ai_response: "I am an automated assistant. I'm texting you a link to our online menu right now so you can place your order. Please check your messages!".to_string(),
                    sms_body: Some("Order here: https://pay.ohc.com/store/voice".to_string()),
                },
            ]),
        });

        let router = VoiceContextRouter::with_planner(engine.clone(), mock_twilio, planner);
        let session_id = engine.handle_incoming_call("merchant_456", "+1122334455").await;

        let response = router.process_user_input(&session_id, "I want to place an order for pickup", "+0987654321", "English").await;
        assert!(response.contains("texting you a link"));

        // Ensure SMS was sent
        assert_eq!(sent.load(Ordering::SeqCst), 1);

        // Ensure intent was logged
        let actions = engine.actions.lock().await;
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].intent_type, "ORDER_FOOD");
    }
}
