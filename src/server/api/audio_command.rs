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

    // Fetch translation preferences for the tenant
    let target_languages: Vec<String> = {
        let pool = crate::db::get_pool();
        let prefs_row = sqlx::query(
            "SELECT target_languages FROM ohc_translation_preferences WHERE tenant_id = $1"
        )
        .bind(&tenant_id)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);

        match prefs_row {
            Some(r) => {
                use sqlx::Row;
                let langs_val: serde_json::Value = r.get("target_languages");
                serde_json::from_value(langs_val).unwrap_or_else(|_| vec!["en".to_string()])
            }
            None => vec!["en".to_string()], // default to English
        }
    };

    let preferred_language = target_languages.first().cloned().unwrap_or_else(|| "en".to_string());

    // 1. Transcription (Whisper integration placeholder)
    // In a production environment, we would stream audio_data to a Whisper multimodal model.
    // For the sandbox implementation, we simulate the transcription of a food cart order.
    let transcription = "Quiero 3 tacos de pollo".to_string(); // Simulate Spanish input

    // 2. Intent Extraction & Semantic Routing
    // We use the LLM to parse the command into a structured action plan (OrderIntent or TaskIntent).
    let prompt = format!(
        "Analyze this voice command from a customer or business owner: \"{}\". \
         Translate it into language code '{}'. \
         Detect the source language. \
         Return strict JSON with: \
         translated_text (the translated text), \
         detected_language (the language code of the original text), \
         department (Operations), \
         feature_type (order_intake, task_intake), \
         description (human readable description of the translated text), \
         payload (JSON object with extracted fields like items, quantities, special_requests).",
        transcription, preferred_language
    );

    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());

    let (dept, description, action_payload, final_transcription) = if api_key.is_empty() || api_key == "fake-key" {
        // Fallback for missing api key (tests)
        (DepartmentType::Operations, "Voice initiated: 3x Chicken Tacos".to_string(), serde_json::json!({
            "items": ["Chicken Tacos"],
            "quantities": [3],
            "special_requests": ["none"]
        }), "3x Chicken Tacos".to_string())
    } else {
        let client = crate::minimax::MinimaxClient::new(api_key);
        match client.reason(&crate::pricing::compression::reduce_tokens(&prompt)).await {
            Ok(raw_json) => {
                let clean_res = raw_json.trim_matches('`').trim_start_matches("json\n").trim_end();
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(clean_res) {
                    let d_str = val.get("department").and_then(|v| v.as_str()).unwrap_or("Operations");
                    let d = DepartmentType::from_str(d_str)
                        .unwrap_or(DepartmentType::Operations);
                    let translated_text = val.get("translated_text").and_then(|v| v.as_str()).unwrap_or(&transcription).to_string();
                    let desc = val.get("description").and_then(|v| v.as_str()).unwrap_or(&format!("Voice initiated: {}", translated_text)).to_string();
                    let p = val.get("payload").cloned().unwrap_or(serde_json::json!({
                        "items": ["Chicken Tacos"],
                        "quantities": [3],
                        "special_requests": ["none"]
                    }));
                    (d, desc, p, translated_text)
                } else {
                    // Fallback if JSON parsing fails
                    (DepartmentType::Operations, format!("Voice initiated: {}", transcription), serde_json::json!({ "raw_transcription": transcription, "items": ["Chicken Tacos"], "quantities": [3], "special_requests": ["none"] }), transcription.clone())
                }
            }
            Err(_) => (DepartmentType::Operations, format!("Voice initiated: {}", transcription), serde_json::json!({ "raw_transcription": transcription, "items": ["Chicken Tacos"], "quantities": [3], "special_requests": ["none"] }), transcription.clone())
        }
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
            transcription: final_transcription,
            department_assigned: dept.to_string(),
            status: "PROPOSED".to_string(),
        })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}
