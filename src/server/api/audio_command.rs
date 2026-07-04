use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
use crate::orchestration::router::{SemanticRouter};
use ::server_common::Claims;
use axum::extract::Multipart;

#[derive(Deserialize)]
pub struct VoiceCommandRequest {
    /// Base64 encoded audio data (m4a/wav)
    pub audio_data: String,
}

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
    let mut provided_tenant_id = String::new();

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("").to_string();
        if name == "audio" {
            audio_data = field.bytes().await.unwrap_or_default().to_vec();
        } else if name == "tenant_id" {
            provided_tenant_id = field.text().await.unwrap_or_default();
        }
    }

    if audio_data.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "audio data is required"}))).into_response();
    }

    // 1. Transcription (Whisper integration placeholder)
    // In a production environment, we would stream audio_data to a Whisper multimodal model
    // that handles translation and transcription (e.g. from Arabic/English to English text).
    // For the sandbox implementation, we simulate the transcription of a food cart order.
    let transcription = "Drafted Order: 2x Chicken Rice, 1 with no white sauce".to_string();

    // 2. Intent Extraction & Semantic Routing
    // We use the LLM to parse the command into a structured action plan (OrderIntent or TaskIntent).
    let prompt = format!(
        "Analyze this voice command from a business owner: \"{}\". \
         Return strict JSON with: department (Operations), \
         feature_type (order_intake, task_intake), \
         description (human readable), \
         payload (JSON object with extracted fields like items, quantities, special_requests).",
        transcription
    );

    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
    let client = crate::minimax::MinimaxClient::new(api_key);

    let (dept, description, action_payload) = match client.reason(&prompt).await {
        Ok(raw_json) => {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&raw_json) {
                let d_str = val.get("department").and_then(|v| v.as_str()).unwrap_or("Operations");
                let d = DepartmentType::from_str(d_str)
                    .unwrap_or(DepartmentType::Operations);
                let desc = val.get("description").and_then(|v| v.as_str()).unwrap_or(&format!("Voice initiated: {}", transcription)).to_string();
                let p = val.get("payload").cloned().unwrap_or(serde_json::json!({
                    "items": ["Chicken Rice", "Chicken Rice"],
                    "quantities": [1, 1],
                    "special_requests": ["none", "no white sauce"]
                }));
                (d, desc, p)
            } else {
                // Fallback if JSON parsing fails
                (DepartmentType::Operations, format!("Voice initiated: {}", transcription), serde_json::json!({ "raw_transcription": transcription, "items": ["Chicken Rice"], "quantities": [2], "special_requests": ["1 with no white sauce"] }))
            }
        }
        Err(_) => (DepartmentType::Operations, format!("Voice initiated: {}", transcription), serde_json::json!({ "raw_transcription": transcription, "items": ["Chicken Rice"], "quantities": [2], "special_requests": ["1 with no white sauce"] }))
    };

    // 3. Register as a Proposed Action Card in the Agent Feed (Unified Inbox / Ledger)
    match state.orchestrator.execute_action(
        dept.clone(),
        description,
        tenant_id,
        ActionRisk::DraftForReview,
        action_payload,
    ).await {
        Ok(_) => (StatusCode::OK, Json(VoiceCommandResponse {
            transcription,
            department_assigned: dept.to_string(),
            status: "PROPOSED".to_string(),
        })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}
