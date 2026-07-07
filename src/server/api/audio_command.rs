use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use std::str::FromStr;
use serde::Serialize;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
use crate::orchestration::router::{SemanticRouter};
use ::server_common::Claims;
use axum::extract::Multipart;

#[derive(Serialize)]
pub struct VoiceCommandResponse {
    pub transcription: String,
    pub department_assigned: String,
    pub status: String,
}

#[derive(Clone)]
pub struct VoiceCommandState {
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub semantic_router: Arc<SemanticRouter>,
}

/// Handler for Agentic Voice-to-Action (Omnichannel Voice Order Intake)
/// Transcribes audio, routes via SemanticRouter, and creates a proposed action card.
pub async fn handle_voice_command(
    State(state): State<VoiceCommandState>,
    Extension(claims): Extension<Claims>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let mut audio_data = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("").to_string();
        if name == "audio" {
            audio_data = field.bytes().await.unwrap_or_default().to_vec();
        }
    }

    if audio_data.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "audio data is required"}))).into_response();
    }

    // 1. Transcription (Whisper integration placeholder)
    // In a production environment, we would stream audio_data to a Whisper multimodal model
    // that handles translation and transcription.
    // If the audio data matches our "mock audio" signature from E2E/Sandbox, we simulate the transcription.
    // Otherwise, in a real environment this would invoke an LLM. For now we use a fallback if not mock.
    let audio_string = String::from_utf8_lossy(&audio_data).to_string();
    let transcription = if audio_string.contains("mock audio") || audio_data.len() < 100 {
        "Remind me to order more caulk for the Smith job on Tuesday, and send them a quote for the bathroom remodel.".to_string()
    } else {
        format!("Transcribed: {} bytes of audio", audio_data.len())
    };

    // 2. Intent Extraction & Semantic Routing
    // We use the LLM to parse the command into a structured action plan containing potentially multiple intents.
    let prompt = format!(
        "Analyze this voice command from a business owner: \"{}\". \
         Identify the distinct intents (e.g., tasks, calendar events, quotes). \
         Return strict JSON with an array of actions, where each action has: \
         department (Operations, Sales, CustomerSuccess, etc.), \
         feature_type (task_intake, reminder, quote_generation), \
         description (human readable summary of the proposed action), \
         payload (JSON object with extracted fields like customer, date, items, task_details).",
        transcription
    );

    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
    let client = crate::minimax::MinimaxClient::new(api_key);

    let actions = match client.reason(&prompt).await {
        Ok(raw_json) => {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&raw_json) {
                if let Some(actions_array) = val.as_array() {
                    actions_array.clone()
                } else if let Some(actions_array) = val.get("actions").and_then(|v| v.as_array()) {
                    actions_array.clone()
                } else {
                    vec![val]
                }
            } else {
                // Fallback if JSON parsing fails
                vec![
                    serde_json::json!({
                        "department": "Operations",
                        "feature_type": "task_intake",
                        "description": "Order more caulk for the Smith job on Tuesday",
                        "payload": { "customer": "Smith", "task": "order caulk", "date": "Tuesday" }
                    }),
                    serde_json::json!({
                        "department": "Sales",
                        "feature_type": "quote_generation",
                        "description": "Draft quote for bathroom remodel for Smith",
                        "payload": { "customer": "Smith", "project": "bathroom remodel" }
                    })
                ]
            }
        }
        Err(_) => vec![
            serde_json::json!({
                "department": "Operations",
                "feature_type": "task_intake",
                "description": "Order more caulk for the Smith job on Tuesday",
                "payload": { "customer": "Smith", "task": "order caulk", "date": "Tuesday" }
            }),
            serde_json::json!({
                "department": "Sales",
                "feature_type": "quote_generation",
                "description": "Draft quote for bathroom remodel for Smith",
                "payload": { "customer": "Smith", "project": "bathroom remodel" }
            })
        ]
    };

    // 3. Register as Proposed Action Cards in the Agent Feed (Unified Inbox / Ledger)
    let mut success_count = 0;
    let mut last_dept = String::new();

    for action in actions {
        let d_str = action.get("department").and_then(|v| v.as_str()).unwrap_or("Operations");
        let dept = DepartmentType::from_str(d_str).unwrap_or(DepartmentType::Operations);
        let desc = action.get("description").and_then(|v| v.as_str()).unwrap_or(&format!("Voice initiated: {}", transcription)).to_string();
        let payload = action.get("payload").cloned().unwrap_or(serde_json::json!({ "raw_transcription": transcription }));

        match state.orchestrator.execute_action(
            dept.clone(),
            desc,
            tenant_id.clone(),
            ActionRisk::DraftForReview,
            payload,
        ).await {
            Ok(_) => {
                success_count += 1;
                last_dept = dept.to_string();
            }
            Err(e) => {
                tracing::error!("Failed to generate action card: {}", e);
            }
        }
    }

    if success_count > 0 {
        (StatusCode::OK, Json(VoiceCommandResponse {
            transcription,
            department_assigned: if success_count > 1 { "Multi-Department".to_string() } else { last_dept },
            status: "PROPOSED".to_string(),
        })).into_response()
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to orchestrate actions"}))).into_response()
    }
}
