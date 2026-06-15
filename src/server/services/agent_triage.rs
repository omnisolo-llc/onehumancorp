use axum::{Json, routing::get, Router};
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
pub struct TriageItem {
    id: String,
    source: String,
    content: String,
    urgency: String,
    suggested_action: String,
    suggested_draft: String,
    timestamp: String,
}

pub async fn get_triage_feed() -> Json<Vec<TriageItem>> {
    let now = Utc::now().to_rfc3339();

    let mock_data = vec![
        TriageItem {
            id: "triage-1".to_string(),
            source: "Instagram DM".to_string(),
            content: "Customer Maya asked about a vegan cake for Saturday.".to_string(),
            urgency: "high".to_string(),
            suggested_action: "Draft Reply".to_string(),
            suggested_draft: "Hi Maya! We can absolutely do a vegan cake for Saturday. The total is $50. You can pay here: [Payment Link]".to_string(),
            timestamp: now.clone(),
        },
        TriageItem {
            id: "triage-2".to_string(),
            source: "Stripe".to_string(),
            content: "Payment failed for Invoice #102.".to_string(),
            urgency: "high".to_string(),
            suggested_action: "Send Reminder".to_string(),
            suggested_draft: "Hi there, your recent payment failed. Please update your payment method.".to_string(),
            timestamp: now,
        }
    ];

    Json(mock_data)
}

pub fn router() -> Router {
    Router::new()
        .route("/api/ui/triage", get(get_triage_feed))
}
