use axum::{
    extract::Path,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::domain::chat::channel::ChannelType;
use crate::services::chat::ingestion::IngestionService;
use crate::services::chat::auto_reply::AutoReplyEngine;

#[derive(Debug, Deserialize, Serialize)]
pub struct IncomingWebhookPayload {
    pub channel_identifier: String,
    pub content: String,
    pub customer_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub auto_reply_generated: bool,
    pub requires_attention: bool,
}

pub fn chat_routes() -> Router {
    Router::new()
        .route("/webhooks/chat/{org_id}/{channel_type}", post(handle_incoming_chat))
}

async fn handle_incoming_chat(
    Path((org_id, channel_type_str)): Path<(String, String)>,
    Json(payload): Json<IncomingWebhookPayload>,
) -> impl IntoResponse {
    let channel_type = match channel_type_str.as_str() {
        "instagram" => ChannelType::InstagramDm,
        "whatsapp" => ChannelType::Whatsapp,
        "sms" => ChannelType::Sms,
        "web" => ChannelType::WebChat,
        _ => ChannelType::WebChat, // Fallback
    };

    let (mut conversation, message) = IngestionService::handle_incoming_message(
        org_id,
        channel_type,
        payload.channel_identifier,
        payload.content,
        payload.customer_id,
    );

    // Process with Auto-Reply Engine
    let reply_opt = AutoReplyEngine::process_conversation(&mut conversation, &message).await;

    let response = WebhookResponse {
        success: true,
        auto_reply_generated: reply_opt.as_ref().map_or(false, |m| !m.is_draft),
        requires_attention: reply_opt.as_ref().map_or(false, |m| m.is_draft),
    };

    Json(response)
}
