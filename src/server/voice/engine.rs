use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CallStatus {
    Ringing,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallSession {
    pub session_id: String,
    pub merchant_id: String,
    pub caller_phone: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: CallStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptLog {
    pub id: String,
    pub session_id: String,
    pub role: String,
    pub text: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentAction {
    pub id: String,
    pub session_id: String,
    pub intent_type: String,
    pub details: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

pub struct VoiceAIEdgeEngine {
    // In a real implementation, this would interface with a WebRTC/SIP provider like Twilio
    // For now, we mock the core functionality.
    pub active_calls: Arc<Mutex<Vec<CallSession>>>,
    pub transcripts: Arc<Mutex<Vec<TranscriptLog>>>,
    pub actions: Arc<Mutex<Vec<IntentAction>>>,
}

impl VoiceAIEdgeEngine {
    pub fn new() -> Self {
        Self {
            active_calls: Arc::new(Mutex::new(Vec::new())),
            transcripts: Arc::new(Mutex::new(Vec::new())),
            actions: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn handle_incoming_call(&self, merchant_id: &str, caller_phone: &str) -> String {
        let session_id = uuid::Uuid::new_v4().to_string();
        let session = CallSession {
            session_id: session_id.clone(),
            merchant_id: merchant_id.to_string(),
            caller_phone: caller_phone.to_string(),
            start_time: Utc::now(),
            end_time: None,
            status: CallStatus::InProgress,
        };

        let mut calls = self.active_calls.lock().await;
        calls.push(session);

        // Log the initial AI greeting
        self.log_transcript(&session_id, "AI", "Hello! Thank you for calling. How can I help you today?").await;

        session_id
    }

    pub async fn log_transcript(&self, session_id: &str, role: &str, text: &str) {
        let transcript = TranscriptLog {
            id: uuid::Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            role: role.to_string(),
            text: text.to_string(),
            timestamp: Utc::now(),
        };
        let mut logs = self.transcripts.lock().await;
        logs.push(transcript);
    }

    pub async fn log_intent_action(&self, session_id: &str, intent_type: &str, details: serde_json::Value) {
        let action = IntentAction {
            id: uuid::Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            intent_type: intent_type.to_string(),
            details,
            timestamp: Utc::now(),
        };
        let mut actions_guard = self.actions.lock().await;
        actions_guard.push(action);
    }

    pub async fn get_transcripts(&self, session_id: &str) -> Vec<TranscriptLog> {
        let transcripts = self.transcripts.lock().await;
        transcripts.iter().filter(|t| t.session_id == session_id).cloned().collect()
    }

    pub async fn end_call(&self, session_id: &str) {
        let mut calls = self.active_calls.lock().await;
        if let Some(call) = calls.iter_mut().find(|c| c.session_id == session_id) {
            call.end_time = Some(Utc::now());
            call.status = CallStatus::Completed;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_voice_engine_call_flow() {
        let engine = VoiceAIEdgeEngine::new();
        let session_id = engine.handle_incoming_call("merchant_123", "+1234567890").await;

        {
            let calls = engine.active_calls.lock().await;
            assert_eq!(calls.len(), 1);
            assert_eq!(calls[0].status, CallStatus::InProgress);
        }

        engine.log_transcript(&session_id, "USER", "Do you have any openings tomorrow?").await;

        {
            let transcripts = engine.transcripts.lock().await;
            assert_eq!(transcripts.len(), 2); // Greeting + User msg
        }

        engine.log_intent_action(&session_id, "CHECK_AVAILABILITY", serde_json::json!({"date": "tomorrow"})).await;

        {
            let actions = engine.actions.lock().await;
            assert_eq!(actions.len(), 1);
        }

        engine.end_call(&session_id).await;

        {
            let calls = engine.active_calls.lock().await;
            assert_eq!(calls[0].status, CallStatus::Completed);
            assert!(calls[0].end_time.is_some());
        }
    }
}
