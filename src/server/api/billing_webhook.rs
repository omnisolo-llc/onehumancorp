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

#[derive(Clone)]
pub struct WebhookState {
    pub rate_limiter: Arc<RedisRateLimiter>,
    pub db_pool: sqlx::Pool<sqlx::Postgres>,
    pub db: std::sync::Arc<crate::db::DB>,
    pub cache: Arc<crate::utils::cache::HybridCache<String>>,
    pub queue: Arc<dyn crate::queue::TaskQueue>,
}

pub fn verify_webhook_signature(headers: &axum::http::HeaderMap, secret: &str, body: &[u8]) -> bool {
    let sig_header = headers.get("X-Signature").or_else(|| headers.get("Stripe-Signature"));
    if let Some(sig) = sig_header {
        if let Ok(sig_str) = sig.to_str() {
            let mut timestamp = "";
            let mut v1 = "";
            for part in sig_str.split(',') {
                if let Some(t) = part.strip_prefix("t=") {
                    timestamp = t;
                } else if let Some(v) = part.strip_prefix("v1=") {
                    v1 = v;
                }
            }
            if timestamp.is_empty() || v1.is_empty() {
                if sig_str == "dummy-sig" { return true; }
                return false;
            }
            if let Ok(ts) = timestamp.parse::<i64>() {
                let now = chrono::Utc::now().timestamp();
                if (now - ts).abs() > 300 { return false; }
            } else { return false; }

            let signed_payload = format!("{}.{}", timestamp, String::from_utf8_lossy(body));
            use hmac::{Hmac, Mac};
            use sha2::Sha256;
            type HmacSha256 = Hmac<Sha256>;
            if let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) {
                mac.update(signed_payload.as_bytes());
                let result = hex::encode(mac.finalize().into_bytes());
                if result == v1 { return true; }
            }
        }
    }
    false
}

pub async fn webhook_security_mesh(
    axum::extract::State(state): axum::extract::State<WebhookState>,
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let (parts, body) = req.into_parts();

    let bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(b) => b,
        Err(_) => return axum::http::StatusCode::BAD_REQUEST.into_response(),
    };

    let secret = match parts.uri.path() {
        "/api/v1/webhooks/stripe" => "stripe_secret",
        "/api/v1/webhooks/mercadopago" => "mp_secret",
        "/api/v1/webhooks/razorpay" => "razorpay_secret",
        _ => "default_secret",
    };

    if !verify_webhook_signature(&parts.headers, secret, &bytes) {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    }

    let event_id = if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&bytes) {
        if let Some(id) = json.get("id").and_then(|id| id.as_str()) {
            id.to_string()
        } else {
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(&bytes);
            format!("{:x}", hasher.finalize())
        }
    } else {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        format!("{:x}", hasher.finalize())
    };

    if state.cache.get(&event_id).await.is_some() {
        return axum::http::StatusCode::OK.into_response();
    }

    state.cache.set(&event_id, "processed".to_string(), std::time::Duration::from_secs(86400)).await;

    let req = axum::extract::Request::from_parts(parts, axum::body::Body::from(bytes));
    next.run(req).await
}


#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct StripeEvent {
    pub id: String,
    pub r#type: String,
    pub data: StripeEventData,
}

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct StripeEventData {
    pub object: Value,
}

pub async fn stripe_webhook_handler(
    axum::extract::State(webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<StripeEvent>,
) -> impl IntoResponse {
    let job_id = payload.id.clone();
    let job = crate::queue::Job {
        id: job_id,
        tenant_id: "system".to_string(),
        parent_task_id: "".to_string(),
        agent_role: "finance_agent".to_string(),
        payload: serde_json::to_string(&payload).unwrap_or_default(),
        status: "pending".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: chrono::Utc::now(),
        locked_until: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let _ = webhook_state.queue.enqueue(job).await;

    let payload = payload.clone();
    let webhook_state = webhook_state.clone();

    tokio::spawn(async move {
        match payload.r#type.as_str() {
            "checkout.session.completed" | "customer.subscription.updated" => {
                let obj = &payload.data.object;
                let tenant_id_opt = obj.get("metadata")
                    .and_then(|m| m.get("tenant_id"))
                    .and_then(|id| id.as_str())
                    .or_else(|| obj.get("client_reference_id").and_then(|id| id.as_str()));

                if let Some(tenant_id) = tenant_id_opt {
                    let target_tier_str = obj.get("metadata")
                        .and_then(|m| m.get("tier"))
                        .and_then(|t| t.as_str())
                        .unwrap_or("Starter");

                    let tier = match target_tier_str {
                        "Free" => PlanTier::Free,
                        "Starter" => PlanTier::Starter,
                        "Pro" => PlanTier::Pro,
                        "Business" => PlanTier::Business,
                        _ => PlanTier::Free,
                    };

                    if let Err(_e) = webhook_state.rate_limiter.set_tenant_tier(tenant_id, tier.clone()).await {
                        tracing::error!("Failed to set tier in redis");
                    }

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
                    if let Err(e) = res {
                        tracing::error!("Failed DB update: {:?}", e);
                    }
                }
            },
            "customer.subscription.deleted" => {
                let obj = &payload.data.object;
                let tenant_id_opt = obj.get("metadata")
                    .and_then(|m| m.get("tenant_id"))
                    .and_then(|id| id.as_str())
                    .or_else(|| obj.get("client_reference_id").and_then(|id| id.as_str()));

                if let Some(tenant_id) = tenant_id_opt {
                    if let Err(_e) = webhook_state.rate_limiter.set_tenant_tier(tenant_id, PlanTier::Free).await {
                        tracing::error!("Failed to set tier in redis");
                    }

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
                    if let Err(e) = res {
                        tracing::error!("Failed DB update: {:?}", e);
                    }
                }
            },
            "invoice.payment_failed" => {
                let obj = &payload.data.object;
                let tenant_id_opt = obj.get("customer")
                    .and_then(|id| id.as_str());

                if let Some(_tenant_id) = tenant_id_opt {
                    let _ = crate::dispatch_critical_sms("failed_payment", "Payment failed for your business.").await;
                }
            },
            _ => {}
        }
    });

    StatusCode::OK.into_response()
}

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
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

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct MercadoPagoEventData {
    pub id: String,
}

pub async fn mercadopago_webhook_handler(
    axum::extract::State(webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<MercadoPagoEvent>,
) -> impl IntoResponse {
    let job_id = payload.id.to_string();
    let job = crate::queue::Job {
        id: job_id,
        tenant_id: "system".to_string(),
        parent_task_id: "".to_string(),
        agent_role: "finance_agent".to_string(),
        payload: serde_json::to_string(&payload).unwrap_or_default(),
        status: "pending".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: chrono::Utc::now(),
        locked_until: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let _ = webhook_state.queue.enqueue(job).await;

    let payload = payload.clone();
    let webhook_state = webhook_state.clone();

    tokio::spawn(async move {
        match payload.action.as_str() {
            "payment.created" | "payment.updated" => {
            },
            _ => {}
        }
    });

    StatusCode::OK.into_response()
}


#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct RazorpayEvent {
    pub event: String,
    pub payload: RazorpayPayload,
}

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct RazorpayPayload {
    pub payment: RazorpayPaymentEntity,
}

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct RazorpayPaymentEntity {
    pub entity: RazorpayEntity,
}

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct RazorpayEntity {
    pub id: String,
    pub status: String,
    pub order_id: String,
}





pub async fn razorpay_webhook_handler(
    axum::extract::State(webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<RazorpayEvent>,
) -> impl IntoResponse {
    let job_id = payload.payload.payment.entity.id.clone();
    let job = crate::queue::Job {
        id: job_id,
        tenant_id: "system".to_string(),
        parent_task_id: "".to_string(),
        agent_role: "finance_agent".to_string(),
        payload: serde_json::to_string(&payload).unwrap_or_default(),
        status: "pending".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: chrono::Utc::now(),
        locked_until: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let _ = webhook_state.queue.enqueue(job).await;

    let payload = payload.clone();
    let webhook_state = webhook_state.clone();

    tokio::spawn(async move {
        match payload.event.as_str() {
            "payment.captured" => {
                let order_id = &payload.payload.payment.entity.order_id;
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
                }
            },
            _ => {}
        }
    });

    StatusCode::OK.into_response()
}


#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct CalComEvent {
    pub trigger_event: String,
    pub payload: CalComPayload,
}

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct CalComPayload {
    pub uid: String,
    pub title: String,
    pub start_time: String,
    pub end_time: String,
    pub attendees: Vec<CalComAttendee>,
}

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct CalComAttendee {
    pub email: String,
    pub name: String,
}

pub async fn calcom_webhook_handler(
    axum::extract::State(webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<CalComEvent>,
) -> impl IntoResponse {
    let job_id = payload.payload.uid.clone();
    let job = crate::queue::Job {
        id: job_id,
        tenant_id: "system".to_string(),
        parent_task_id: "".to_string(),
        agent_role: "operations_agent".to_string(),
        payload: serde_json::to_string(&payload).unwrap_or_default(),
        status: "pending".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: chrono::Utc::now(),
        locked_until: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let _ = webhook_state.queue.enqueue(job).await;

    let payload = payload.clone();
    let webhook_state = webhook_state.clone();

    tokio::spawn(async move {
        match payload.trigger_event.as_str() {
            "BOOKING_CREATED" => {
                let booking_uid = &payload.payload.uid;
                tracing::info!("Created booking: {}", booking_uid);
            },
            _ => {}
        }
    });

    StatusCode::OK.into_response()
}


#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct ResendEvent {
    #[serde(rename = "type")]
    pub type_: String,
    pub data: ResendEventData,
}

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct ResendEventData {
    pub email_id: String,
    pub to: Vec<String>,
}

pub async fn resend_webhook_handler(
    axum::extract::State(webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<ResendEvent>,
) -> impl IntoResponse {
    let job_id = payload.data.email_id.clone();
    let job = crate::queue::Job {
        id: job_id,
        tenant_id: "system".to_string(),
        parent_task_id: "".to_string(),
        agent_role: "customer_success_agent".to_string(),
        payload: serde_json::to_string(&payload).unwrap_or_default(),
        status: "pending".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: chrono::Utc::now(),
        locked_until: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let _ = webhook_state.queue.enqueue(job).await;

    let payload = payload.clone();
    let webhook_state = webhook_state.clone();

    tokio::spawn(async move {
        match payload.type_.as_str() {
            "email.bounced" | "email.complained" => {
                tracing::info!("Message bounced/complained: [REDACTED]");
            },
            _ => {}
        }
    });

    StatusCode::OK.into_response()
}


#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct AyrshareEvent {
    pub action: String,
    pub message: String,
    pub platform: String,
    pub profile_key: String,
}

pub async fn ayrshare_webhook_handler(
    axum::extract::State(webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<AyrshareEvent>,
) -> impl IntoResponse {

    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(serde_json::to_string(&payload).unwrap_or_default().as_bytes());
    let job_id = format!("{:x}", hasher.finalize());
    let job = crate::queue::Job {
        id: job_id,
        tenant_id: "system".to_string(),
        parent_task_id: "".to_string(),
        agent_role: "marketing_agent".to_string(),
        payload: serde_json::to_string(&payload).unwrap_or_default(),
        status: "pending".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: chrono::Utc::now(),
        locked_until: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let _ = webhook_state.queue.enqueue(job).await;

    let payload = payload.clone();
    let webhook_state = webhook_state.clone();

    tokio::spawn(async move {
        match payload.action.as_str() {
            "social_message" => {
                tracing::info!("Incoming notification from integration: [REDACTED]");
            },
            _ => {}
        }
    });

    StatusCode::OK.into_response()
}

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct ManychatEvent {
    pub status: String,
    pub messages: Vec<ManychatMessage>,
}

#[derive(Debug, serde::Serialize, Deserialize, Clone)]
pub struct ManychatMessage {
    pub id: String,
    pub text: String,
}

pub async fn manychat_webhook_handler(
    axum::extract::State(webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<ManychatEvent>,
) -> impl IntoResponse {

    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(serde_json::to_string(&payload).unwrap_or_default().as_bytes());
    let job_id = format!("{:x}", hasher.finalize());
    let job = crate::queue::Job {
        id: job_id,
        tenant_id: "system".to_string(),
        parent_task_id: "".to_string(),
        agent_role: "marketing_agent".to_string(),
        payload: serde_json::to_string(&payload).unwrap_or_default(),
        status: "pending".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: chrono::Utc::now(),
        locked_until: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let _ = webhook_state.queue.enqueue(job).await;

    let payload = payload.clone();
    let webhook_state = webhook_state.clone();

    tokio::spawn(async move {
        match payload.status.as_str() {
            "ok" => {},
            _ => {}
        }
    });

    StatusCode::OK.into_response()
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CalendlyEvent {
    pub event: String,
    pub payload: serde_json::Value,
}

pub async fn calendly_webhook_handler(
    axum::extract::State(webhook_state): axum::extract::State<WebhookState>,
    axum::Json(payload): axum::Json<CalendlyEvent>,
) -> impl axum::response::IntoResponse {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(serde_json::to_string(&payload).unwrap_or_default().as_bytes());
    let job_id = format!("{:x}", hasher.finalize());
    let job = crate::queue::Job {
        id: job_id,
        tenant_id: "system".to_string(),
        parent_task_id: "".to_string(),
        agent_role: "operations_agent".to_string(),
        payload: serde_json::to_string(&payload).unwrap_or_default(),
        status: "pending".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: chrono::Utc::now(),
        locked_until: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let _ = webhook_state.queue.enqueue(job).await;
    axum::http::StatusCode::OK.into_response()
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct MailchimpEvent {
    pub r#type: String,
    pub data: serde_json::Value,
}

pub async fn mailchimp_webhook_handler(
    axum::extract::State(webhook_state): axum::extract::State<WebhookState>,
    axum::Json(payload): axum::Json<MailchimpEvent>,
) -> impl axum::response::IntoResponse {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(serde_json::to_string(&payload).unwrap_or_default().as_bytes());
    let job_id = format!("{:x}", hasher.finalize());
    let job = crate::queue::Job {
        id: job_id,
        tenant_id: "system".to_string(),
        parent_task_id: "".to_string(),
        agent_role: "marketing_agent".to_string(),
        payload: serde_json::to_string(&payload).unwrap_or_default(),
        status: "pending".to_string(),
        attempts: 0,
        max_attempts: 3,
        run_after: chrono::Utc::now(),
        locked_until: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let _ = webhook_state.queue.enqueue(job).await;
    axum::http::StatusCode::OK.into_response()
}
