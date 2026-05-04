use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use redis::Client;
use serde::Deserialize;
use std::sync::Arc;
use crate::pricing::rate_limit::{PlanTier, RedisRateLimiter};

#[derive(Deserialize, Debug)]
pub struct StripeWebhookEvent {
    pub id: String,
    pub r#type: String,
    pub data: EventData,
}

#[derive(Deserialize, Debug)]
pub struct EventData {
    pub object: StripeObject,
}

#[derive(Deserialize, Debug)]
pub struct StripeObject {
    pub id: String,
    pub customer: Option<String>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub status: Option<String>,
    pub items: Option<StripeItems>,
}

#[derive(Deserialize, Debug)]
pub struct StripeItems {
    pub data: Vec<StripeItemData>,
}

#[derive(Deserialize, Debug)]
pub struct StripeItemData {
    pub price: StripePrice,
}

#[derive(Deserialize, Debug)]
pub struct StripePrice {
    pub id: String,
}

pub struct WebhookState {
    pub rate_limiter: Arc<RedisRateLimiter>,
}

pub fn router(redis_client: Client) -> Router {
    let state = Arc::new(WebhookState {
        rate_limiter: Arc::new(RedisRateLimiter::new(redis_client)),
    });
    Router::new()
        .route("/", post(handle_webhook))
        .with_state(state)
}

async fn handle_webhook(
    State(state): State<Arc<WebhookState>>,
    Json(event): Json<StripeWebhookEvent>,
) -> StatusCode {
    if event.r#type == "customer.subscription.updated" || event.r#type == "customer.subscription.created" {
        let tenant_id = match event.data.object.metadata.as_ref().and_then(|m| m.get("tenant_id")) {
            Some(id) => id,
            None => return StatusCode::BAD_REQUEST, // Tenant ID must be in metadata
        };

        let price_id = match event.data.object.items.as_ref().and_then(|items| items.data.first()).map(|item| &item.price.id) {
            Some(id) => id,
            None => return StatusCode::BAD_REQUEST,
        };

        let tier = match price_id.as_str() {
            "price_starter" => PlanTier::Starter,
            "price_pro" => PlanTier::Pro,
            "price_business" => PlanTier::Business,
            _ => PlanTier::Free, // Default to free if unknown price or cancelled
        };

        if let Err(e) = state.rate_limiter.set_tenant_tier(tenant_id, tier).await {
            eprintln!("Failed to update tenant tier: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    } else if event.r#type == "customer.subscription.deleted" {
         let tenant_id = match event.data.object.metadata.as_ref().and_then(|m| m.get("tenant_id")) {
            Some(id) => id,
            None => return StatusCode::BAD_REQUEST, // Tenant ID must be in metadata
        };
        if let Err(e) = state.rate_limiter.set_tenant_tier(tenant_id, PlanTier::Free).await {
            eprintln!("Failed to reset tenant tier to Free: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }

    StatusCode::OK
}
