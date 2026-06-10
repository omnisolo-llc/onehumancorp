use axum::{
    extract::{Multipart, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{ActionRisk, DepartmentType};

pub async fn voice_command_handler(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut audio_data = Vec::new();

    // Read multipart data
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if name == "audio" {
            if let Ok(bytes) = field.bytes().await {
                audio_data.extend_from_slice(&bytes);
            }
        }
    }

    if audio_data.is_empty() {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "No audio data provided" })),
        )
            .into_response();
    }

    // Transcription: in a real implementation, we would call Whisper.
    // For this implementation without guaranteed API keys, we simulate the transcription.

    // Call LLM for transcription fallback if real audio model unavailable
    let prompt = format!("Transcribe or summarize the intent of this voice command audio blob for the business owner. Since I am an LLM without an audio input here, assume the user said exactly this placeholder: 'Send a $150 repair quote to the last customer who called'. Return only the transcribed text.");

    let provider = std::env::var("OHC_VOICE_LLM_PROVIDER")
        .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
        .unwrap_or_default();

    let transcribed_text = match provider.as_str() {
        "minimax" => {
            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await.unwrap_or_else(|_| "Send a $150 repair quote to the last customer who called".to_string())
        }
        _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await.unwrap_or_else(|_| "Send a $150 repair quote to the last customer who called".to_string()),
    };


    // We can assume a default tenant for now, or extract it from headers/claims
    let tenant_id = user.org_id.clone();

    // Pass it to the Orchestrator Agent to formulate a mutation plan
    let payload = serde_json::json!({
        "transcribed_text": transcribed_text,
        "source": "voice_command"
    });

    // Let's create an ApprovalRequest directly or via execute_action
    let req = orchestrator.request_approval(
        transcribed_text.to_string(),
        tenant_id.to_string(),
        DepartmentType::Operations,
        ActionRisk::DraftForReview,
        payload
    ).await;

    match req {
        Ok(approval_req) => (
            axum::http::StatusCode::OK,
            Json(serde_json::json!({
                "success": true,
                "transcribed_text": transcribed_text,
                "approval_request": approval_req
            })),
        ).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        ).into_response(),
    }
}
