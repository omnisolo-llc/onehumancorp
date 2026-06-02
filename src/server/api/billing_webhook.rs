use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use std::sync::Arc;
use serde_json::Value;

use ::server_pricing::rate_limit::{PlanTier, RedisRateLimiter};
use crate::db::DbStore;

use std::collections::HashMap;

#[derive(Clone)]
pub struct WebhookState {
    pub rate_limiter: Arc<RedisRateLimiter>,
    pub db_pool: sqlx::Pool<sqlx::Postgres>,
    pub db: std::sync::Arc<crate::db::DB>,
    pub secrets: Arc<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct StripeEvent {
    pub id: String,
    pub r#type: String,
    pub data: StripeEventData,
}

#[derive(Debug, Deserialize)]
pub struct StripeEventData {
    pub object: Value,
}

use axum::middleware::Next;
use axum::extract::Request;
use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};

type HmacSha256 = Hmac<Sha256>;

pub async fn webhook_security_middleware(
    axum::extract::State(state): axum::extract::State<WebhookState>,
    req: Request,
    next: Next,
) -> impl IntoResponse {
    let headers = req.headers().clone();
    let path = req.uri().path().to_string();
    let provider = path.split('/').last().unwrap_or("unknown");

    // Extract body bytes for signature check and replay protection hashing
    let (parts, body) = req.into_parts();

    let bytes = match axum::body::to_bytes(body, 5 * 1024 * 1024).await { // 5MB limit
        Ok(b) => b,
        Err(_) => return StatusCode::PAYLOAD_TOO_LARGE.into_response(),
    };

    // Verify Cryptographic Signature
    let is_valid_signature = if provider == "stripe" {
        let secret = match state.secrets.get("STRIPE_WEBHOOK_SECRET") {
            Some(s) => s.clone(),
            None => {
                tracing::error!("STRIPE_WEBHOOK_SECRET environment variable is missing.");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };
        if let Some(sig_header) = headers.get("Stripe-Signature") {
            let sig_str = sig_header.to_str().unwrap_or("");

            let t_val = sig_str.split(',').find_map(|p| p.strip_prefix("t=")).unwrap_or("");
            let mut is_verified = false;

            if !t_val.is_empty() {
                if let Ok(timestamp) = t_val.parse::<i64>() {
                    let now = chrono::Utc::now().timestamp();
                    let drift = (now - timestamp).abs();
                    if drift <= 300 {
                        let signed_payload = [t_val.as_bytes(), b".", &bytes].concat();
                        if let Ok(mac) = HmacSha256::new_from_slice(secret.as_bytes()) {
                            for v1 in sig_str.split(',').filter_map(|p| p.strip_prefix("v1=")) {
                                let mut mac_clone = mac.clone();
                                mac_clone.update(&signed_payload);
                                if let Ok(expected_sig) = hex::decode(v1) {
                                    if mac_clone.verify_slice(&expected_sig).is_ok() {
                                        is_verified = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            is_verified
        } else {
            false
        }
    } else {
        // Generic HMAC SHA256 fallback for other providers that use X-Signature
        let secret_key = format!("{}_WEBHOOK_SECRET", provider.to_uppercase());
        let secret = match state.secrets.get(&secret_key) {
            Some(s) => s.clone(),
            None => {
                tracing::error!("{} environment variable is missing.", secret_key);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };
        if let Some(sig_header) = headers.get("X-Signature") {
            let sig_str = sig_header.to_str().unwrap_or("");
            if let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) {
                mac.update(&bytes);
                if let Ok(expected_sig) = hex::decode(sig_str) {
                    mac.verify_slice(&expected_sig).is_ok()
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    };

    if !is_valid_signature {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    // Hash the raw body bytes to generate deterministic ID
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash_result = hasher.finalize();
    let event_id = format!("{}_{}", provider, hex::encode(hash_result));

    // Check replay protection
    match state.rate_limiter.check_and_set_webhook_id(&event_id).await {
        Ok(true) => {} // Proceed
        Ok(false) => return StatusCode::OK.into_response(), // Already processed
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }

    // Reconstruct request
    let req = Request::from_parts(parts, axum::body::Body::from(bytes));
    let response = next.run(req).await;

    // If the handler fails to process the request, delete the idempotency key to allow retries.
    if response.status().is_server_error() || response.status().is_client_error() {
        let _ = state.rate_limiter.delete_webhook_id(&event_id).await;
    }

    response
}

pub async fn stripe_webhook_handler(
    _headers: axum::http::HeaderMap,
    axum::extract::State(webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<StripeEvent>,
) -> impl IntoResponse {

    match payload.r#type.as_str() {
        "checkout.session.completed" | "customer.subscription.updated" => {
            let obj = &payload.data.object;

            // Extract tenant ID. Depending on your Stripe setup, this might be in metadata
            // or client_reference_id. Here we assume it's in metadata.tenant_id.
            let tenant_id_opt = obj.get("metadata")
                .and_then(|m| m.get("tenant_id"))
                .and_then(|id| id.as_str())
                .or_else(|| obj.get("client_reference_id").and_then(|id| id.as_str()));

            if let Some(tenant_id) = tenant_id_opt {
                // Determine new tier based on price ID or plan name or metadata
                // For this example, let's assume we pass the target tier in metadata.tier
                // or we deduce it. For simplicity in this demo, let's read metadata.tier
                // and fallback to "Starter" if a payment succeeded.
                let tier_str = obj.get("metadata")
                    .and_then(|m| m.get("tier"))
                    .and_then(|t| t.as_str())
                    .unwrap_or("Starter");

                let tier = match tier_str {
                    "Starter" => PlanTier::Starter,
                    "Pro" => PlanTier::Pro,
                    "Business" => PlanTier::Business,
                    _ => PlanTier::Free,
                };


                // Update Redis Rate Limiter
                if let Err(_e) = webhook_state.rate_limiter.set_tenant_tier(tenant_id, tier.clone()).await {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }

                // Update Database
                let tier_string = match tier {
                    PlanTier::Free => "Free",
                    PlanTier::Starter => "Starter",
                    PlanTier::Pro => "Pro",
                    PlanTier::Business => "Business",
                };

                let res = match &webhook_state.db.store {
                    DbStore::Sqlite(pool) => {
                        sqlx::query("UPDATE tenants SET tier = ? WHERE tenant_id = ?")
                            .bind(tier_string)
                            .bind(tenant_id)
                            .execute(pool)
                            .await
                            .map(|_| ())
                    }
                    DbStore::Postgres => {
                        sqlx::query("UPDATE tenants SET tier = $1 WHERE tenant_id = $2")
                            .bind(tier_string)
                            .bind(tenant_id)
                            .execute(&webhook_state.db.pool)
                            .await
                            .map(|_| ())
                    }
                };

                if let Err(_e) = res {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }

                StatusCode::OK.into_response()
            } else {
                StatusCode::BAD_REQUEST.into_response()
            }
        },
        "customer.subscription.deleted" => {
            let obj = &payload.data.object;
            let tenant_id_opt = obj.get("metadata")
                .and_then(|m| m.get("tenant_id"))
                .and_then(|id| id.as_str())
                .or_else(|| obj.get("client_reference_id").and_then(|id| id.as_str()));

            if let Some(tenant_id) = tenant_id_opt {

                // Update Redis
                if let Err(_e) = webhook_state.rate_limiter.set_tenant_tier(tenant_id, PlanTier::Free).await {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }

                // Update DB
                let res = match &webhook_state.db.store {
                    DbStore::Sqlite(pool) => {
                        sqlx::query("UPDATE tenants SET tier = ? WHERE tenant_id = ?")
                            .bind("Free")
                            .bind(tenant_id)
                            .execute(pool)
                            .await
                            .map(|_| ())
                    }
                    DbStore::Postgres => {
                        sqlx::query("UPDATE tenants SET tier = $1 WHERE tenant_id = $2")
                            .bind("Free")
                            .bind(tenant_id)
                            .execute(&webhook_state.db.pool)
                            .await
                            .map(|_| ())
                    }
                };

                if let Err(_e) = res {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }

                StatusCode::OK.into_response()
            } else {
                StatusCode::BAD_REQUEST.into_response()
            }
        },
        "invoice.payment_failed" => {
            let obj = &payload.data.object;
            let tenant_id_opt = obj.get("customer")
                .and_then(|id| id.as_str());

            if let Some(_tenant_id) = tenant_id_opt {
                // Trigger SMS notification
                tokio::spawn(async move {
                    let _ = crate::dispatch_critical_sms("failed_payment", "Payment failed for your business.").await;
                });
            }
            StatusCode::OK.into_response()
        },
        _ => {
            // Unhandled event types are ignored successfully
            StatusCode::OK.into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MercadoPagoEvent {
    pub id: i64,
    pub live_mode: bool,
    pub r#type: String,
    pub date_created: String,
    pub application_id: i64,
    pub user_id: i64,
    pub version: i32,
    pub api_version: String,
    pub action: String,
    pub data: MercadoPagoEventData,
}

#[derive(Debug, Deserialize)]
pub struct MercadoPagoEventData {
    pub id: String,
}

pub async fn mercadopago_webhook_handler(
    axum::extract::State(_webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<MercadoPagoEvent>,
) -> impl IntoResponse {
    match payload.action.as_str() {
        "payment.created" | "payment.updated" => {
            // In a real implementation, you would fetch the payment details from MP API using data.id
            // and extract the tenant_id and tier from the metadata.
            // For mock purposes, assume we process it similarly to Stripe.
            // We just return OK.
            StatusCode::OK.into_response()
        },
        _ => StatusCode::OK.into_response()
    }
}


#[derive(Debug, Deserialize)]
pub struct RazorpayEvent {
    pub event: String,
    pub payload: RazorpayPayload,
}

#[derive(Debug, Deserialize)]
pub struct RazorpayPayload {
    pub payment: RazorpayPaymentEntity,
}

#[derive(Debug, Deserialize)]
pub struct RazorpayPaymentEntity {
    pub entity: RazorpayEntity,
}

#[derive(Debug, Deserialize)]
pub struct RazorpayEntity {
    pub id: String,
    pub status: String,
    pub order_id: String,
}


pub async fn razorpay_webhook_handler(
    _headers: axum::http::HeaderMap,
    axum::extract::State(webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<RazorpayEvent>,
) -> impl IntoResponse {
    match payload.event.as_str() {
        "payment.captured" => {
            let order_id = &payload.payload.payment.entity.order_id;

            // In a real app, transition OHC orders from "Pending" to "Paid"
            let res = match &webhook_state.db.store {
                DbStore::Sqlite(pool) => {
                    sqlx::query("UPDATE orders SET status = 'Paid' WHERE order_id = ?")
                        .bind(order_id)
                        .execute(pool)
                        .await
                        .map(|_| ())
                }
                DbStore::Postgres => {
                    sqlx::query("UPDATE orders SET status = 'Paid' WHERE order_id = $1")
                        .bind(order_id)
                        .execute(&webhook_state.db.pool)
                        .await
                        .map(|_| ())
                }
            };

            if let Err(e) = res {
                tracing::error!("Failed to update order status: {:?}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }

            StatusCode::OK.into_response()
        },
        _ => StatusCode::OK.into_response()
    }
}


#[derive(Debug, Deserialize)]
pub struct CalComEvent {
    pub trigger_event: String,
    pub payload: CalComPayload,
}

#[derive(Debug, Deserialize)]
pub struct CalComPayload {
    pub uid: String,
    pub title: String,
    pub start_time: String,
    pub end_time: String,
    pub attendees: Vec<CalComAttendee>,
}

#[derive(Debug, Deserialize)]
pub struct CalComAttendee {
    pub email: String,
    pub name: String,
}

pub async fn calcom_webhook_handler(
    axum::extract::State(_webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<CalComEvent>,
) -> impl IntoResponse {
    match payload.trigger_event.as_str() {
        "BOOKING_CREATED" => {
            let booking_uid = &payload.payload.uid;

            // In a real app, create calendar events in the OHC dashboard
            // and auto-generate meeting links (e.g., Zoom).
            tracing::info!("Created booking: {}", booking_uid);
            StatusCode::OK.into_response()
        },
        _ => StatusCode::OK.into_response()
    }
}


#[derive(Debug, Deserialize)]
pub struct ResendEvent {
    #[serde(rename = "type")]
    pub type_: String,
    pub data: ResendEventData,
}

#[derive(Debug, Deserialize)]
pub struct ResendEventData {
    pub email_id: String,
    pub to: Vec<String>,
}

pub async fn resend_webhook_handler(
    axum::extract::State(_webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<ResendEvent>,
) -> impl IntoResponse {
    match payload.type_.as_str() {
        "email.bounced" | "email.complained" => {
            // Automatically clean the tenant's mailing list
            tracing::info!("Message bounced/complained: [REDACTED]");
            StatusCode::OK.into_response()
        },
        _ => StatusCode::OK.into_response()
    }
}


#[derive(Debug, Deserialize)]
pub struct AyrshareEvent {
    pub action: String,
    pub message: String,
    pub platform: String,
    pub profile_key: String,
}

pub async fn ayrshare_webhook_handler(
    axum::extract::State(_webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<AyrshareEvent>,
) -> impl IntoResponse {
    match payload.action.as_str() {
        "social_message" => {
            // Ingest inbound messages into a unified OHC inbox table
            tracing::info!("Incoming notification from integration: [REDACTED]");
            StatusCode::OK.into_response()
        },
        _ => StatusCode::OK.into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct ManychatEvent {
    pub status: String,
    pub messages: Vec<ManychatMessage>,
}

#[derive(Debug, Deserialize)]
pub struct ManychatMessage {
    pub id: String,
    pub text: String,
}

pub async fn manychat_webhook_handler(
    axum::extract::State(_webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<ManychatEvent>,
) -> impl IntoResponse {
    match payload.status.as_str() {
        "ok" => StatusCode::OK.into_response(),
        _ => StatusCode::OK.into_response()
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct CalendlyEvent {
    pub event: String,
    pub payload: serde_json::Value,
}

pub async fn calendly_webhook_handler(
    axum::extract::State(__webhook_state): axum::extract::State<WebhookState>,
    axum::Json(_payload): axum::Json<CalendlyEvent>,
) -> impl axum::response::IntoResponse {
    axum::http::StatusCode::OK.into_response()
}

#[derive(Debug, serde::Deserialize)]
pub struct MailchimpEvent {
    pub r#type: String,
    pub data: serde_json::Value,
}

pub async fn mailchimp_webhook_handler(
    axum::extract::State(__webhook_state): axum::extract::State<WebhookState>,
    axum::Json(_payload): axum::Json<MailchimpEvent>,
) -> impl axum::response::IntoResponse {
    axum::http::StatusCode::OK.into_response()
}
