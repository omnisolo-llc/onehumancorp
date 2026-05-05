use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use std::sync::Arc;
use crate::billing::Tracker;
use serde_json::Value;
use crate::integrations::stripe::webhook::verify_signature;

pub async fn handle_webhook(
    State(_tracker): State<Arc<Tracker>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let sig = if let Some(sig) = headers.get("stripe-signature") {
        sig.to_str().unwrap_or("")
    } else {
        return (StatusCode::BAD_REQUEST, "Missing stripe-signature header").into_response();
    };

    let secret = std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_else(|_| "whsec_test".to_string());

    let payload_str = match std::str::from_utf8(&body) {
        Ok(s) => s,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid UTF-8").into_response(),
    };

    if !verify_signature(payload_str, sig, &secret) {
        return (StatusCode::BAD_REQUEST, "Invalid signature").into_response();
    }

    let payload: Value = match serde_json::from_str(payload_str) {
        Ok(p) => p,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid JSON").into_response(),
    };

    let event_type = payload["type"].as_str().unwrap_or("");

    match event_type {
        "customer.subscription.updated" | "customer.subscription.created" => {
        }
        "customer.subscription.deleted" => {
        }
        _ => {
        }
    }

    (StatusCode::OK, "Webhook received").into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_webhook_missing_signature() {
        let headers = HeaderMap::new();
        let tracker = Arc::new(Tracker::new());
        let body = Bytes::from(r#"{"type":"customer.subscription.updated"}"#);

        let response = handle_webhook(State(tracker), headers, body).await.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
