use axum::{
    extract::{State, Form},
    response::{IntoResponse, Html},
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::Deserialize;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
use uuid::Uuid;
use crate::db::get_pool;

#[derive(Deserialize, Debug)]
pub struct TwilioCallPayload {
    pub CallSid: String,
    pub From: String,
    pub To: String,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_voice_webhook))
        .with_state(orchestrator)
}

async fn handle_voice_webhook(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Form(payload): Form<TwilioCallPayload>,
) -> impl IntoResponse {
    // In a real implementation we would dynamically grab the tenant based on the 'To' number.
    // For this demonstration, we use a default tenant id or extract from DB.
    let tenant_id = "default".to_string();

    // Simulate finding the business profile based on tenant id to inform the AI
    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();

    let generated_response = if !api_key.is_empty() {
        let client = crate::minimax::MinimaxClient::new(api_key);
        client.reason("A customer is calling your business. Generate a short, welcoming voice greeting in English saying you will send them an SMS to book.").await.unwrap_or_else(|_| "Hi, I am the AI receptionist. I'll text you a link to book an appointment.".to_string())
    } else {
        "Hello, this is the AI Receptionist. I am sending you a text message with a link to complete your request.".to_string()
    };

    let description = format!("Incoming voice call from {}", payload.From);
    let risk = ActionRisk::DraftForReview;
    let id = Uuid::new_v4().to_string();
    let status = "pending";
    let pool = get_pool();

    // Save the "transcription" or summary to the inbox
    if let Ok(mut tx) = pool.begin().await {
        let _ = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;
        let _ = sqlx::query(
            "INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&id)
        .bind(&tenant_id)
        .bind("voice_call")
        .bind(format!("Voice call received. AI greeted caller and sent booking SMS."))
        .bind(&generated_response)
        .bind(&status)
        .execute(&mut *tx)
        .await;
        let _ = tx.commit().await;
    }

    // Dispatch event to Customer Success department to notify the owner
    let _ = orchestrator.execute_action(
        DepartmentType::CustomerSuccess,
        description,
        tenant_id,
        risk,
        serde_json::json!({
            "source": "voice_call",
            "feature_type": "voice_call",
            "message": "Missed call handled by AI Receptionist.",
            "caller": payload.From,
            "transcription": "Caller asked about availability.",
            "generated_response": generated_response,
            "inbox_message_id": id,
            "audio_url": "https://example.com/mock_audio.mp3",
        }),
    ).await;

    // Send SMS as a follow-up (mocked for now, or using real twilio provider if configured)
    tokio::spawn(async move {
        let _ = crate::dispatch_critical_sms("booking_link", "Here is the link to book: https://onehumancorp.com/book").await;
    });

    let twiml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<Response><Say>{}</Say></Response>",
        generated_response
    );

    (StatusCode::OK, [("Content-Type", "text/xml")], twiml).into_response()
}
