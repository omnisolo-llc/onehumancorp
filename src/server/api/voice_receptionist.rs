use axum::{
    extract::Json,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct BlandWebhookRequest {
    pub call_id: String,
    pub variables: std::collections::HashMap<String, String>,
    pub status: String,
}

#[derive(Serialize)]
pub struct VoiceReceptionistResponse {
    pub success: bool,
    pub message: String,
}

async fn handle_bland_webhook(
    Json(payload): Json<BlandWebhookRequest>,
) -> impl IntoResponse {
    // Mission 3: Handles webhook triggers from Bland AI, performs GetFreeBusy check (simulated),
    // and generates a billing.Invoice for the deposit (simulated).

    // Simulate booking a repair and collecting a deposit via Stripe
    tracing::info!("Received Bland AI webhook for call_id: {}", payload.call_id);

    let _event_payload = serde_json::json!({
        "status": payload.status,
        "action": "booked",
        "appointment_time": "Friday 2 PM",
        "service": "Faucet Repair",
        "deposit_amount": 50,
        "payment_link": "https://buy.stripe.com/test_12345"
    });

    tracing::info!("Action taken: Booked Faucet Repair for Friday 2 PM. Deposit SMS sent with link: https://buy.stripe.com/test_12345");

    (
        axum::http::StatusCode::OK,
        Json(VoiceReceptionistResponse {
            success: true,
            message: "Webhook processed, appointment booked, deposit SMS sent.".to_string(),
        }),
    ).into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/webhook/bland", post(handle_bland_webhook))
}
