use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use crate::integrations::registry::IntegrationsRegistry;

#[derive(Deserialize, Debug)]
pub struct CalendlyWebhookPayload {
    pub event: String,
    pub payload: serde_json::Value,
}

use axum::http::HeaderMap;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use crate::api::billing_webhook::WebhookState;
use axum::extract::Query;
use std::collections::HashMap;

pub async fn calendly_webhook_handler(
    State(state): State<WebhookState>,
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
    body: String,
) -> impl IntoResponse {
    let signature_header = match headers.get("Calendly-Webhook-Signature") {
        Some(h) => h.to_str().unwrap_or(""),
        None => return (StatusCode::UNAUTHORIZED, "Missing signature").into_response(),
    };

    #[cfg(not(test))]
    let webhook_secret = match std::env::var("CALENDLY_WEBHOOK_SECRET") {
        Ok(secret) => secret,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Webhook secret not configured").into_response(),
    };

    #[cfg(test)]
    let webhook_secret = std::env::var("CALENDLY_WEBHOOK_SECRET").unwrap_or_else(|_| "test_secret".to_string());

    // Calendly signature format: t=<timestamp>,v1=<signature>
    let parts: Vec<&str> = signature_header.split(',').collect();
    if parts.len() != 2 {
        return (StatusCode::UNAUTHORIZED, "Invalid signature format").into_response();
    }

    let t_part = parts[0].strip_prefix("t=").unwrap_or("");
    let v1_part = parts[1].strip_prefix("v1=").unwrap_or("");

    let payload = format!("{}.{}", t_part, body);

    let mut mac = match Hmac::<Sha256>::new_from_slice(webhook_secret.as_bytes()) {
        Ok(m) => m,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Key error").into_response(),
    };
    mac.update(payload.as_bytes());

    let provided_sig_bytes = match hex::decode(v1_part) {
        Ok(b) => b,
        Err(_) => return (StatusCode::UNAUTHORIZED, "Invalid signature encoding").into_response(),
    };

    if mac.verify_slice(&provided_sig_bytes).is_err() {
        return (StatusCode::UNAUTHORIZED, "Invalid signature").into_response();
    }

    let parsed_payload: Result<CalendlyWebhookPayload, _> = serde_json::from_str(&body);
    let payload = match parsed_payload {
        Ok(p) => p,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid JSON").into_response(),
    };

    tracing::info!("Received verified Calendly webhook: event={}", payload.event);

    let tenant_id = match params.get("tenant_id") {
        Some(id) => id.clone(),
        None => return (StatusCode::BAD_REQUEST, "Missing tenant tracking parameter").into_response(),
    };

    match payload.event.as_str() {
        "invitee.created" => {
            tracing::info!("Syncing booking to OHC dashboard for tenant {}.", tenant_id);
            let payload_str = payload.payload.to_string();
            let default_uuid = uuid::Uuid::new_v4().to_string();
            let booking_id = payload.payload.get("uri").and_then(|u| u.as_str()).unwrap_or(&default_uuid);
            let service_id = "calendly_event";
            let start_time = payload.payload.get("event").and_then(|e| e.get("start_time")).and_then(|t| t.as_str()).unwrap_or("1970-01-01T00:00:00Z");
            let end_time = payload.payload.get("event").and_then(|e| e.get("end_time")).and_then(|t| t.as_str()).unwrap_or("1970-01-01T01:00:00Z");
            let customer_id = payload.payload.get("email").and_then(|c| c.as_str()).unwrap_or("unknown_customer");
            let status = "active";

            let res = match &state.db.store {
                crate::db::DbStore::Sqlite(pool) => {
                    sqlx::query("INSERT INTO bookings (id, tenant_id, customer_id, service_id, start_time, end_time, status) VALUES (?, ?, ?, ?, ?, ?, ?)")
                        .bind(booking_id)
                        .bind(tenant_id)
                        .bind(customer_id)
                        .bind(service_id)
                        .bind(start_time)
                        .bind(end_time)
                        .bind(status)
                        .execute(pool)
                        .await
                        .map(|_| ())
                }
                crate::db::DbStore::Postgres => {
                    sqlx::query("INSERT INTO bookings (id, tenant_id, customer_id, service_id, start_time, end_time, status) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                        .bind(booking_id)
                        .bind(tenant_id)
                        .bind(customer_id)
                        .bind(service_id)
                        .bind(start_time)
                        .bind(end_time)
                        .bind(status)
                        .execute(&state.db_pool)
                        .await
                        .map(|_| ())
                }
            };

            if let Err(e) = res {
                tracing::error!("Failed to insert booking for tenant {}: {}", tenant_id, e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
            }
        }
        "invitee.canceled" => {
            tracing::info!("Syncing booking cancellation to OHC dashboard for tenant {}.", tenant_id);
            let default_uuid = uuid::Uuid::new_v4().to_string();
            let booking_id = payload.payload.get("uri").and_then(|u| u.as_str()).unwrap_or(&default_uuid);

            let res = match &state.db.store {
                crate::db::DbStore::Sqlite(pool) => {
                    sqlx::query("UPDATE bookings SET status = 'canceled' WHERE id = ? AND tenant_id = ?")
                        .bind(booking_id)
                        .bind(tenant_id)
                        .execute(pool)
                        .await
                        .map(|_| ())
                }
                crate::db::DbStore::Postgres => {
                    sqlx::query("UPDATE bookings SET status = 'canceled' WHERE id = $1 AND tenant_id = $2")
                        .bind(booking_id)
                        .bind(tenant_id)
                        .execute(&state.db_pool)
                        .await
                        .map(|_| ())
                }
            };

            if let Err(e) = res {
                tracing::error!("Failed to update booking for tenant {}: {}", tenant_id, e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
            }
        }
        _ => {
            tracing::debug!("Unhandled Calendly event type: {}", payload.event);
        }
    }

    StatusCode::OK.into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;
    use std::env;

    #[tokio::test]
    async fn test_calendly_webhook_handler_missing_signature() {
        let state = WebhookState {
            rate_limiter: Arc::new(crate::pricing::rate_limit::MockRateLimiter::new()),
            db_pool: crate::db::get_pool(),
            db: Arc::new(crate::db::Database::new("mock").await.unwrap()),
        };
        let headers = HeaderMap::new();
        let body = "{}".to_string();
        let mut params = HashMap::new();
        params.insert("tenant_id".to_string(), "mock_tenant".to_string());

        let response = calendly_webhook_handler(State(state), Query(params), headers, body).await.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_calendly_webhook_handler_invalid_signature() {
        let state = WebhookState {
            rate_limiter: Arc::new(crate::pricing::rate_limit::MockRateLimiter::new()),
            db_pool: crate::db::get_pool(),
            db: Arc::new(crate::db::Database::new("mock").await.unwrap()),
        };
        let mut headers = HeaderMap::new();
        headers.insert("Calendly-Webhook-Signature", HeaderValue::from_static("t=123,v1=invalid"));
        let body = "{}".to_string();
        let mut params = HashMap::new();
        params.insert("tenant_id".to_string(), "mock_tenant".to_string());

        let response = calendly_webhook_handler(State(state), Query(params), headers, body).await.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_calendly_webhook_handler_valid_signature() {
        let state = WebhookState {
            rate_limiter: Arc::new(crate::pricing::rate_limit::MockRateLimiter::new()),
            db_pool: crate::db::get_pool(),
            db: Arc::new(crate::db::Database::new("mock").await.unwrap()),
        };

        let body = r#"{"event": "invitee.created", "payload": {"uri": "https://api.calendly.com/scheduled_events/123", "email": "customer@example.com", "event": {"start_time": "2023-01-01T10:00:00Z", "end_time": "2023-01-01T11:00:00Z"}}}"#.to_string();
        let t_part = "1234567890";
        let payload = format!("{}.{}", t_part, body);
        let mut mac = Hmac::<Sha256>::new_from_slice("test_secret".as_bytes()).unwrap();
        mac.update(payload.as_bytes());
        let valid_sig = hex::encode(mac.finalize().into_bytes());

        let mut headers = HeaderMap::new();
        let sig_header = format!("t={},v1={}", t_part, valid_sig);
        headers.insert("Calendly-Webhook-Signature", HeaderValue::from_str(&sig_header).unwrap());

        let mut params = HashMap::new();
        params.insert("tenant_id".to_string(), "mock_tenant".to_string());

        let response = calendly_webhook_handler(State(state), Query(params), headers, body).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
