use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::Arc;
use crate::hub::Hub;
use axum::http::StatusCode;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;
use crate::services::subscription::service::SubscriptionService;

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize)]
pub struct SubscriptionPlanResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub amount: i64,
    pub interval: String,
    pub active: bool,
}

#[derive(Serialize)]
pub struct SubscriberResponse {
    pub id: String,
    pub customer_id: String,
    pub status: String,
    pub health_score: Option<i32>,
}

#[derive(Serialize)]
pub struct FulfillmentBatchResponse {
    pub id: String,
    pub fulfillment_date: String,
    pub status: String,
    pub subscriber_count: i64,
}

#[derive(Deserialize)]
pub struct CreateFulfillmentBatchRequest {
    pub subscription_plan_id: String,
    pub fulfillment_date: String,
}

async fn get_plans(
    Extension(hub): Extension<Arc<Hub>>,

    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match ::server_config::get().multitenant {
        true => claims.organization_id.clone().unwrap_or_else(|| "".to_string()),
        false => ::server_common::auth_utils::get_default_tenant(),
    };

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response(),
    };

    let result = sqlx::query(
        "SELECT sp.id as id, COALESCE(p.title, sp.name) as name, COALESCE(p.description, sp.description) as description, COALESCE(p.price_cents, sp.price_cents) as amount, COALESCE(sp.interval, sp.frequency) as interval, sp.status = 'active' as active FROM subscription_plans sp LEFT JOIN products p ON sp.product_id = p.id WHERE sp.tenant_id = $1"
    )
    .bind(tenant_id)
    .fetch_all(&mut *conn)
    .await;

    match result {
        Ok(rows) => {
            use sqlx::Row;
            let plans: Vec<SubscriptionPlanResponse> = rows.into_iter().map(|r| SubscriptionPlanResponse {
                id: r.try_get("id").unwrap_or_default(),
                name: r.try_get("name").unwrap_or_default(),
                description: r.try_get("description").unwrap_or_default(),
                amount: r.try_get::<i64, _>("amount").unwrap_or(0),
                interval: r.try_get("interval").unwrap_or_default(),
                active: r.try_get("active").unwrap_or(true),
            }).collect();
            (StatusCode::OK, Json(plans)).into_response()
        },
        Err(e) => {
            ::server_telemetry::record_error_signal("[bug] Failed to fetch subscription plans");
            tracing::error!("Failed to fetch subscription plans: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response()
        }
    }
}

async fn get_subscribers(
    Extension(hub): Extension<Arc<Hub>>,

    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match ::server_config::get().multitenant {
        true => claims.organization_id.clone().unwrap_or_else(|| "".to_string()),
        false => ::server_common::auth_utils::get_default_tenant(),
    };

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response(),
    };

    let result = sqlx::query(
        "SELECT id, customer_id, status FROM subscriptions WHERE tenant_id = $1"
    )
    .bind(tenant_id)
    .fetch_all(&mut *conn)
    .await;

    match result {
        Ok(rows) => {
            use sqlx::Row;
            let subscribers: Vec<SubscriberResponse> = rows.into_iter().map(|r| SubscriberResponse {
                id: r.try_get("id").unwrap_or_default(),
                customer_id: r.try_get("customer_id").unwrap_or_default(),
                status: r.try_get("status").unwrap_or_default(),
                health_score: r.try_get("health_score").unwrap_or(None),
            }).collect();
            (StatusCode::OK, Json(subscribers)).into_response()
        },
        Err(e) => {
            ::server_telemetry::record_error_signal("[bug] Failed to fetch subscribers");
            tracing::error!("Failed to fetch subscribers: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response()
        }
    }
}

async fn get_fulfillment_batches(
    Extension(hub): Extension<Arc<Hub>>,

    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match ::server_config::get().multitenant {
        true => claims.organization_id.clone().unwrap_or_else(|| "".to_string()),
        false => ::server_common::auth_utils::get_default_tenant(),
    };

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response(),
    };

    let result = sqlx::query(
        "SELECT id, fulfillment_date::text AS fulfillment_date, status, subscriber_count
         FROM fulfillment_batches
         WHERE tenant_id = $1
         ORDER BY fulfillment_date ASC, created_at ASC"
    )
    .bind(tenant_id)
    .fetch_all(&mut *conn)
    .await;

    match result {
        Ok(rows) => {
            use sqlx::Row;
            let batches: Vec<FulfillmentBatchResponse> = rows.into_iter().map(|r| FulfillmentBatchResponse {
                id: r.try_get("id").unwrap_or_default(),
                fulfillment_date: r.try_get("fulfillment_date").unwrap_or_default(),
                status: r.try_get("status").unwrap_or_default(),
                subscriber_count: r.try_get("subscriber_count").unwrap_or(0),
            }).collect();
            (StatusCode::OK, Json(batches)).into_response()
        },
        Err(e) => {
            ::server_telemetry::record_error_signal("[bug] Failed to fetch fulfillment batches");
            tracing::error!("Failed to fetch fulfillment batches: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response()
        }
    }
}

async fn create_fulfillment_batch(
    Extension(hub): Extension<Arc<Hub>>,

    Extension(claims): Extension<::server_common::Claims>,
    Extension(orchestrator): Extension<Option<Arc<DepartmentOrchestrator>>>,
    Json(payload): Json<CreateFulfillmentBatchRequest>,
) -> impl IntoResponse {
    let tenant_id = match ::server_config::get().multitenant {
        true => claims.organization_id.clone().unwrap_or_else(|| "".to_string()),
        false => ::server_common::auth_utils::get_default_tenant(),
    };

    let service = SubscriptionService::new(Arc::new(hub.pool.clone()));
    let batch = match service
        .generate_fulfillment_schedule(
            &tenant_id,
            &payload.subscription_plan_id,
            &payload.fulfillment_date,
        )
        .await
    {
        Ok(batch) => batch,
        Err(e) => {
            ::server_telemetry::record_error_signal("[bug] Failed to generate fulfillment batch");
            tracing::error!("Failed to generate fulfillment batch: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response();
        }
    };

    let event_payload = service.fulfillment_schedule_event_payload(&batch);
    if let Some(orchestrator) = orchestrator {
        let event = DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.subscription.fulfillment_batch.created".to_string(),
            payload: event_payload,
        };
        if let Err(e) = orchestrator.dispatch_event(event).await {
            ::server_telemetry::record_error_signal("[bug] Failed to dispatch fulfillment batch event");
            tracing::error!("Failed to dispatch fulfillment batch event: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Operations dispatch failed").into_response();
        }
    }

    (
        StatusCode::OK,
        Json(FulfillmentBatchResponse {
            id: batch.id,
            fulfillment_date: batch.fulfillment_date,
            status: "PENDING".to_string(),
            subscriber_count: batch.subscriber_count,
        }),
    )
        .into_response()
}

#[derive(Deserialize)]
pub struct MagicLinkRequest {
    pub token: String,
    pub action: String, // "pause", "resume", "cancel"
}

#[derive(Serialize)]
pub struct MagicLinkResponse {
    pub success: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MagicLinkClaims {
    pub subscriber_id: String,
    pub action: String,
    pub exp_unix: i64,
}

pub fn sign_magic_link_token(
    claims: &MagicLinkClaims,
    secret: &[u8],
) -> Result<String, String> {
    if secret.is_empty() {
        return Err("magic link secret is required".to_string());
    }

    let payload = serde_json::to_vec(claims).map_err(|e| format!("invalid claims: {e}"))?;
    let encoded_payload = URL_SAFE_NO_PAD.encode(payload);
    let mut mac = HmacSha256::new_from_slice(secret).map_err(|e| format!("invalid secret: {e}"))?;
    mac.update(encoded_payload.as_bytes());
    let signature = mac.finalize().into_bytes();

    Ok(format!("{}.{}", encoded_payload, URL_SAFE_NO_PAD.encode(signature)))
}

pub fn verify_magic_link_token(
    token: &str,
    secret: &[u8],
    now_unix: i64,
) -> Result<MagicLinkClaims, String> {
    if secret.is_empty() {
        return Err("magic link secret is required".to_string());
    }

    let (encoded_payload, encoded_signature) = token
        .split_once('.')
        .ok_or_else(|| "invalid token format".to_string())?;
    let signature = URL_SAFE_NO_PAD
        .decode(encoded_signature)
        .map_err(|_| "invalid token signature".to_string())?;

    let mut mac = HmacSha256::new_from_slice(secret).map_err(|e| format!("invalid secret: {e}"))?;
    mac.update(encoded_payload.as_bytes());
    mac.verify_slice(&signature)
        .map_err(|_| "invalid token signature".to_string())?;

    let payload = URL_SAFE_NO_PAD
        .decode(encoded_payload)
        .map_err(|_| "invalid token payload".to_string())?;
    let claims: MagicLinkClaims =
        serde_json::from_slice(&payload).map_err(|_| "invalid token claims".to_string())?;
    if claims.subscriber_id.trim().is_empty() {
        return Err("subscriber id is required".to_string());
    }
    if claims.exp_unix <= now_unix {
        return Err("magic link token has expired".to_string());
    }

    Ok(claims)
}

async fn handle_magic_link(
    Extension(hub): Extension<Arc<Hub>>,
    Json(payload): Json<MagicLinkRequest>,
) -> impl IntoResponse {
    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response(),
    };

    let status = match payload.action.as_str() {
        "pause" => "Paused",
        "resume" => "Active",
        "cancel" => "Canceled",
        _ => return (StatusCode::BAD_REQUEST, "Invalid action").into_response(),
    };
    let secret = match std::env::var("OHC_MAGIC_LINK_SECRET")
        .or_else(|_| std::env::var("MAGIC_LINK_SECRET"))
    {
        Ok(secret) if !secret.trim().is_empty() => secret,
        _ => return (StatusCode::INTERNAL_SERVER_ERROR, "Magic link secret is not configured").into_response(),
    };
    let claims = match verify_magic_link_token(&payload.token, secret.as_bytes(), chrono::Utc::now().timestamp()) {
        Ok(claims) if claims.action == payload.action => claims,
        Ok(_) => return (StatusCode::BAD_REQUEST, "Token action mismatch").into_response(),
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid token").into_response(),
    };

    let update = sqlx::query(
        "UPDATE subscribers SET status = $1 WHERE id = $2"
    )
    .bind(status)
    .bind(claims.subscriber_id)
    .execute(&mut *conn)
    .await;

    match update {
        Ok(_) => (StatusCode::OK, Json(MagicLinkResponse { success: true })).into_response(),
        Err(e) => {
            ::server_telemetry::record_error_signal("[bug] Failed to update subscription via magic link");
            tracing::error!("Failed to update subscription via magic link: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response()
        }
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    router_with_orchestrator(hub, None)
}

async fn get_subscription_by_id(
    Extension(hub): Extension<Arc<Hub>>,
    axum::extract::Path(id): axum::extract::Path<String>,
Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match ::server_config::get().multitenant {
        true => claims.organization_id.clone().unwrap_or_else(|| "".to_string()),
        false => ::server_common::auth_utils::get_default_tenant(),
    };

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response(),
    };

    let result = sqlx::query(
        "SELECT
            s.id,
            p.title as productName,
            sp.interval as frequency,
            s.status,
            s.current_period_end as nextDeliveryDate,
            p.price_cents as price,
            sp.discount_percentage
         FROM subscriptions s
         JOIN subscription_plans sp ON s.plan_id = sp.id
         JOIN products p ON sp.product_id = p.id
         WHERE s.id = $1 AND s.tenant_id = $2"
    )
    .bind(&id)
    .bind(tenant_id)
    .fetch_optional(&mut *conn)
    .await;

    match result {
        Ok(Some(r)) => {
            use sqlx::Row;
            let price: i64 = r.try_get("price").unwrap_or(0);
            let discount_percentage: i32 = r.try_get("discount_percentage").unwrap_or(0);
            let price_f64 = (price as f64) / 100.0;
            let discounted_price = price_f64 * (1.0 - (discount_percentage as f64 / 100.0));

            let next_date: chrono::DateTime<chrono::Utc> = r.try_get("nextDeliveryDate").unwrap_or_else(|_| chrono::Utc::now());

            let sub = serde_json::json!({
                "id": r.try_get::<String, _>("id").unwrap_or_default(),
                "productName": r.try_get::<String, _>("productName").unwrap_or_default(),
                "frequency": r.try_get::<String, _>("frequency").unwrap_or_default(),
                "status": r.try_get::<String, _>("status").unwrap_or_default(),
                "nextDeliveryDate": next_date.format("%Y-%m-%d").to_string(),
                "price": price_f64,
                "discountedPrice": discounted_price,
            });
            (StatusCode::OK, Json(sub)).into_response()
        },
        Ok(None) => (StatusCode::NOT_FOUND, "Subscription not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch subscription by id: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct SubscriptionActionRequest {
    pub action: String, // "pause", "skip", "cancel"
}

async fn subscription_action(
    axum::extract::Path(id): axum::extract::Path<String>,
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<SubscriptionActionRequest>,
) -> impl IntoResponse {
    let tenant_id = match ::server_config::get().multitenant {
        true => claims.organization_id.clone().unwrap_or_else(|| "".to_string()),
        false => ::server_common::auth_utils::get_default_tenant(),
    };

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response(),
    };

    let update = match payload.action.as_str() {
        "pause" => sqlx::query("UPDATE subscriptions SET status = 'paused' WHERE id = $1 AND tenant_id = $2")
            .bind(&id).bind(&tenant_id).execute(&mut *conn).await,
        "cancel" => sqlx::query("UPDATE subscriptions SET status = 'canceled' WHERE id = $1 AND tenant_id = $2")
            .bind(&id).bind(&tenant_id).execute(&mut *conn).await,
        "skip" => sqlx::query("UPDATE subscriptions SET current_period_end = current_period_end + interval '1 month' WHERE id = $1 AND tenant_id = $2")
            .bind(&id).bind(&tenant_id).execute(&mut *conn).await,
        _ => return (StatusCode::BAD_REQUEST, "Invalid action").into_response(),
    };

    match update {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({ "success": true }))).into_response(),
        Err(e) => {
            tracing::error!("Failed to update subscription: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response()
        }
    }
}

pub fn router_with_orchestrator<S: Clone + Send + Sync + 'static>(
    hub: Arc<Hub>,
    orchestrator: Option<Arc<DepartmentOrchestrator>>,
) -> Router<S> {
    Router::new()
        .route("/plans", get(get_plans))
        .route("/subscribers", get(get_subscribers))
        .route("/fulfillment-batches", get(get_fulfillment_batches).post(create_fulfillment_batch))
        .route("/magic-link", post(handle_magic_link))
        .route("/{id}", get(get_subscription_by_id))
        .route("/{id}/action", post(subscription_action))
        .layer(Extension(orchestrator))
        .layer(Extension(hub))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_magic_link_token_round_trips_claims() {
        let claims = MagicLinkClaims {
            subscriber_id: "sub_123".to_string(),
            action: "pause".to_string(),
            exp_unix: 1_900_000_000,
        };

        let token = sign_magic_link_token(&claims, b"test-secret").expect("token should sign");
        let verified = verify_magic_link_token(&token, b"test-secret", 1_800_000_000)
            .expect("token should verify");

        assert_eq!(verified, claims);
    }

    #[test]
    fn magic_link_rejects_tampered_payload() {
        let claims = MagicLinkClaims {
            subscriber_id: "sub_123".to_string(),
            action: "cancel".to_string(),
            exp_unix: 1_900_000_000,
        };

        let token = sign_magic_link_token(&claims, b"test-secret").expect("token should sign");
        let (payload, signature) = token.split_once('.').expect("signed token should have two parts");
        let replacement = if payload.ends_with('A') { "B" } else { "A" };
        let tampered = format!("{}{}.{}", &payload[..payload.len() - 1], replacement, signature);

        assert!(verify_magic_link_token(&tampered, b"test-secret", 1_800_000_000).is_err());
    }

    #[test]
    fn magic_link_rejects_expired_tokens() {
        let claims = MagicLinkClaims {
            subscriber_id: "sub_123".to_string(),
            action: "resume".to_string(),
            exp_unix: 1_700_000_000,
        };

        let token = sign_magic_link_token(&claims, b"test-secret").expect("token should sign");

        assert!(verify_magic_link_token(&token, b"test-secret", 1_800_000_000).is_err());
    }
}
