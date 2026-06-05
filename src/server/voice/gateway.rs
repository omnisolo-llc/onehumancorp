use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use super::engine::VoiceAIEdgeEngine;
use super::router::VoiceContextRouter;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum TwilioMessage {
    Start {
        start: StartData,
    },
    Media {
        media: MediaData,
    },
    Stop {
        stop: StopData,
    },
    Other(()),
}

#[derive(Debug, Deserialize)]
struct StartData {
    stream_sid: String,
}

#[derive(Debug, Deserialize)]
struct MediaData {
}

#[derive(Debug, Deserialize)]
struct StopData {
    call_sid: String,
}

#[derive(Debug, Serialize)]
struct TwilioMediaOut {
    event: String,
    stream_sid: String,
    media: MediaOutData,
}

#[derive(Debug, Serialize)]
struct MediaOutData {
    payload: String,
}

#[derive(Clone)]
pub struct VoiceGatewayState {
    pub engine: Arc<VoiceAIEdgeEngine>,
    pub router: Arc<VoiceContextRouter>,
}

pub async fn twilio_ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<VoiceGatewayState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_twilio_stream(socket, state))
}

async fn handle_twilio_stream(mut socket: WebSocket, state: VoiceGatewayState) {
    let mut stream_sid: Option<String> = None;
    let session_id = Uuid::new_v4().to_string(); // In a real app we'd get this from Twilio/Custom params

    // Using a simple lock to track transcription buffer, for mock purposes
    let transcript_buffer = Arc::new(Mutex::new(String::new()));

    // Add call to engine
    // Assuming merchant_id is passed somewhere or deduced. Mocking it here.
    let merchant_id = "merchant_mock";
    let caller_phone = "+1234567890";
    state.engine.handle_incoming_call(merchant_id, caller_phone).await;

    while let Some(Ok(msg)) = socket.next().await {
        if let Message::Text(text) = msg {
            if let Ok(twilio_msg) = serde_json::from_str::<TwilioMessage>(&text) {
                match twilio_msg {
                    TwilioMessage::Start { start, .. } => {
                        stream_sid = Some(start.stream_sid);
                        println!("Twilio stream started: {:?}", stream_sid);
                    }
                    TwilioMessage::Media { .. } => {
                        // In a real application, we would stream this base64 PCM audio to OpenAI Realtime API.
                        // Here we mock receiving some audio.
                        let mut buffer = transcript_buffer.lock().await;
                        // Just append some bytes logic or mock text if it was a real STT
                        // For demonstration, let's say after certain packets we process the "speech"
                        buffer.push_str("some speech data ");

                        if buffer.len() > 100 {
                            let simulated_text = "Do you have an opening tomorrow? Yes, please."; // Mock STT output
                            let _ai_response = state.router.process_user_input(&session_id, simulated_text, "+0987654321").await;

                            // Mocking TTS response to Twilio
                            if let Some(ref sid) = stream_sid {
                                let out_msg = TwilioMediaOut {
                                    event: "media".to_string(),
                                    stream_sid: sid.clone(),
                                    media: MediaOutData {
                                        payload: "ZHVtbXlfYXVkaW9fYnl0ZXM=".to_string(), // base64 encoded "dummy_audio_bytes"
                                    },
                                };
                                let out_json = serde_json::to_string(&out_msg).unwrap();
                                let _ = socket.send(Message::Text(out_json.into())).await;
                            }
                            buffer.clear();
                        }
                    }
                    TwilioMessage::Stop { stop, .. } => {
                        println!("Twilio stream stopped for call: {}", stop.call_sid);
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    // Clean up
    state.engine.end_call(&session_id).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::engine::VoiceAIEdgeEngine;
    use super::super::router::VoiceContextRouter;
    use ::server_integrations_twilio::provider::TwilioProvider;
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
    async fn test_voice_gateway_mock() {
        let engine = Arc::new(VoiceAIEdgeEngine::new());
        let sent = Arc::new(AtomicUsize::new(0));
        let mock_twilio = Arc::new(TwilioProvider::with_client(Arc::new(MockTwilioClient { sent_messages: sent.clone() })));
        let router = Arc::new(VoiceContextRouter::new(engine.clone(), mock_twilio));

        let state = VoiceGatewayState {
            engine: engine.clone(),
            router: router.clone(),
        };

        // Testing the underlying router and engine mock setup works
        // as the websocket cannot be tested without an HTTP server harness easily here
        let ai_response = state.router.process_user_input("session_1", "I need to book an appointment", "merchant_123").await;
        assert!(ai_response.contains("tomorrow at 2 PM"));
    }
}
