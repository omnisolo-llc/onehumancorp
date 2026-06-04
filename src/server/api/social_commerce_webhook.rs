use axum::{
    extract::{State, Json},
    response::{IntoResponse, Json as JsonResponse},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct SocialCommerceState {
    // For now, testing directly
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SocialWebhookPayload {
    pub channel: String,      // "instagram", "whatsapp", etc.
    pub tenant_id: String,
    pub message: String,
    pub customer_id: String,
    pub product_id: Option<String>,
    pub quantity: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialWebhookResponse {
    pub status: String,
    pub reply_message: Option<String>,
    pub checkout_link: Option<String>,
}

pub async fn handle_social_commerce_webhook(
    State(_state): State<SocialCommerceState>,
    Json(payload): Json<SocialWebhookPayload>,
) -> impl IntoResponse {

    // In a real flow, this message would go through the NLP agent to determine intent.
    // For this edge-cached quote scenario, we determine intent manually based on whether
    // a product_id is provided in the webhook payload, simulating an order request.

    if let Some(_product_id) = payload.product_id {
        let quantity = payload.quantity.unwrap_or(1);

        let amount = 1000 * quantity; // dummy

        let reply_msg = format!("Great! I checked our live inventory and we have {} available. The total is ${}.{}! Here is your secure checkout link to confirm your deposit:",
            quantity,
            amount / 100,
            format!("{:02}", amount % 100)
        );

        let resp = SocialWebhookResponse {
            status: "success".to_string(),
            reply_message: Some(reply_msg),
            checkout_link: Some("https://checkout.stripe.com/pay/dummy".to_string()),
        };

        (StatusCode::OK, JsonResponse(resp))
    } else {
        // Generic conversation without intent to buy a specific product immediately
        let resp = SocialWebhookResponse {
            status: "success".to_string(),
            reply_message: Some("Message received. I will review and get back to you shortly!".to_string()),
            checkout_link: None,
        };
        (StatusCode::OK, JsonResponse(resp))
    }
}
