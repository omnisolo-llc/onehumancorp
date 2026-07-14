use crate::hub::Hub;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;
use crate::services::subscription::service::SubscriptionService;
use axum::http::StatusCode;
use axum::{
    Router,
    extract::{Extension, Json},
    response::IntoResponse,
    routing::{get, post},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::Arc;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SubscriptionPlanResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub amount: i64,
    pub interval: String,
    pub active: bool,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SubscriberResponse {
    pub id: String,
    pub customer_id: String,
    pub status: String,
    pub health_score: Option<i32>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
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

#[derive(Debug, Serialize)]
pub struct SubscriptionOverviewResponse {
    pub plans: Vec<SubscriptionPlanResponse>,
    pub subscribers: Vec<SubscriberResponse>,
    pub batches: Vec<FulfillmentBatchResponse>,
}

fn resolve_subscription_tenant(
    claims: &::server_common::Claims,
    multitenant: bool,
    configured_default: &str,
) -> Result<String, StatusCode> {
    let tenant = if multitenant {
        claims.organization_id.as_deref().unwrap_or_default()
    } else {
        configured_default
    }
    .trim();

    if tenant.is_empty() {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        Ok(tenant.to_string())
    }
}

fn subscription_tenant(claims: &::server_common::Claims) -> Result<String, StatusCode> {
    resolve_subscription_tenant(
        claims,
        ::server_config::get().multitenant,
        &::server_common::auth_utils::get_default_tenant(),
    )
}

async fn fetch_subscription_plans(
    pool: &sqlx::PgPool,
    tenant_id: &str,
) -> Result<Vec<SubscriptionPlanResponse>, sqlx::Error> {
    sqlx::query_as::<_, SubscriptionPlanResponse>(
        "SELECT
            sp.id,
            COALESCE(p.title, sp.name) AS name,
            COALESCE(p.description, sp.description, '') AS description,
            COALESCE(p.price_cents, sp.price_cents)::BIGINT AS amount,
            COALESCE(NULLIF(sp.interval, ''), sp.frequency) AS interval,
            sp.status = 'active' AS active
         FROM subscription_plans sp
         LEFT JOIN products p
           ON sp.product_id = p.id
          AND p.tenant_id = sp.tenant_id
         WHERE sp.tenant_id = $1
         ORDER BY sp.created_at ASC, sp.id ASC",
    )
    .bind(tenant_id)
    .fetch_all(pool)
    .await
}

async fn fetch_subscribers(
    pool: &sqlx::PgPool,
    tenant_id: &str,
) -> Result<Vec<SubscriberResponse>, sqlx::Error> {
    sqlx::query_as::<_, SubscriberResponse>(
        "SELECT
            id,
            customer_id,
            status,
            health_score::INTEGER AS health_score
         FROM subscriptions
         WHERE tenant_id = $1
         ORDER BY created_at ASC, id ASC",
    )
    .bind(tenant_id)
    .fetch_all(pool)
    .await
}

async fn fetch_fulfillment_batches(
    pool: &sqlx::PgPool,
    tenant_id: &str,
) -> Result<Vec<FulfillmentBatchResponse>, sqlx::Error> {
    sqlx::query_as::<_, FulfillmentBatchResponse>(
        "SELECT
            id,
            fulfillment_date::TEXT AS fulfillment_date,
            status,
            subscriber_count::BIGINT AS subscriber_count
         FROM fulfillment_batches
         WHERE tenant_id = $1
         ORDER BY fulfillment_date ASC, created_at ASC, id ASC",
    )
    .bind(tenant_id)
    .fetch_all(pool)
    .await
}

async fn fetch_subscription_overview(
    pool: &sqlx::PgPool,
    tenant_id: &str,
) -> Result<SubscriptionOverviewResponse, sqlx::Error> {
    let (plans, subscribers, batches) = tokio::try_join!(
        fetch_subscription_plans(pool, tenant_id),
        fetch_subscribers(pool, tenant_id),
        fetch_fulfillment_batches(pool, tenant_id),
    )?;

    Ok(SubscriptionOverviewResponse {
        plans,
        subscribers,
        batches,
    })
}

async fn get_subscription_overview(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match subscription_tenant(&claims) {
        Ok(tenant_id) => tenant_id,
        Err(status) => return (status, "Organization is required").into_response(),
    };

    match fetch_subscription_overview(&hub.pool, &tenant_id).await {
        Ok(overview) => (StatusCode::OK, Json(overview)).into_response(),
        Err(error) => {
            ::server_telemetry::record_error_signal("[bug] Failed to fetch subscription overview");
            tracing::error!(tenant_id, %error, "Failed to fetch subscription overview");
            (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response()
        }
    }
}

async fn get_plans(
    Extension(hub): Extension<Arc<Hub>>,

    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match subscription_tenant(&claims) {
        Ok(tenant_id) => tenant_id,
        Err(status) => return (status, "Organization is required").into_response(),
    };

    match fetch_subscription_plans(&hub.pool, &tenant_id).await {
        Ok(plans) => (StatusCode::OK, Json(plans)).into_response(),
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
    let tenant_id = match subscription_tenant(&claims) {
        Ok(tenant_id) => tenant_id,
        Err(status) => return (status, "Organization is required").into_response(),
    };

    match fetch_subscribers(&hub.pool, &tenant_id).await {
        Ok(subscribers) => (StatusCode::OK, Json(subscribers)).into_response(),
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
    let tenant_id = match subscription_tenant(&claims) {
        Ok(tenant_id) => tenant_id,
        Err(status) => return (status, "Organization is required").into_response(),
    };

    match fetch_fulfillment_batches(&hub.pool, &tenant_id).await {
        Ok(batches) => (StatusCode::OK, Json(batches)).into_response(),
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
    let tenant_id = match subscription_tenant(&claims) {
        Ok(tenant_id) => tenant_id,
        Err(status) => return (status, "Organization is required").into_response(),
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
            ::server_telemetry::record_error_signal(
                "[bug] Failed to dispatch fulfillment batch event",
            );
            tracing::error!("Failed to dispatch fulfillment batch event: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Operations dispatch failed",
            )
                .into_response();
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

pub fn sign_magic_link_token(claims: &MagicLinkClaims, secret: &[u8]) -> Result<String, String> {
    if secret.is_empty() {
        return Err("magic link secret is required".to_string());
    }

    let payload = serde_json::to_vec(claims).map_err(|e| format!("invalid claims: {e}"))?;
    let encoded_payload = URL_SAFE_NO_PAD.encode(payload);
    let mut mac = HmacSha256::new_from_slice(secret).map_err(|e| format!("invalid secret: {e}"))?;
    mac.update(encoded_payload.as_bytes());
    let signature = mac.finalize().into_bytes();

    Ok(format!(
        "{}.{}",
        encoded_payload,
        URL_SAFE_NO_PAD.encode(signature)
    ))
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
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Magic link secret is not configured",
            )
                .into_response();
        }
    };
    let claims = match verify_magic_link_token(
        &payload.token,
        secret.as_bytes(),
        chrono::Utc::now().timestamp(),
    ) {
        Ok(claims) if claims.action == payload.action => claims,
        Ok(_) => return (StatusCode::BAD_REQUEST, "Token action mismatch").into_response(),
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid token").into_response(),
    };

    let update = sqlx::query("UPDATE subscribers SET status = $1 WHERE id = $2")
        .bind(status)
        .bind(claims.subscriber_id)
        .execute(&mut *conn)
        .await;

    match update {
        Ok(_) => (StatusCode::OK, Json(MagicLinkResponse { success: true })).into_response(),
        Err(e) => {
            ::server_telemetry::record_error_signal(
                "[bug] Failed to update subscription via magic link",
            );
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
    let tenant_id = match subscription_tenant(&claims) {
        Ok(tenant_id) => tenant_id,
        Err(status) => return (status, "Organization is required").into_response(),
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
         WHERE s.id = $1 AND s.tenant_id = $2",
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

            let next_date: chrono::DateTime<chrono::Utc> = r
                .try_get("nextDeliveryDate")
                .unwrap_or_else(|_| chrono::Utc::now());

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
        }
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
    let tenant_id = match subscription_tenant(&claims) {
        Ok(tenant_id) => tenant_id,
        Err(status) => return (status, "Organization is required").into_response(),
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
        .route("/", get(get_subscription_overview))
        .route("/plans", get(get_plans))
        .route("/subscribers", get(get_subscribers))
        .route(
            "/fulfillment-batches",
            get(get_fulfillment_batches).post(create_fulfillment_batch),
        )
        .route("/magic-link", post(handle_magic_link))
        .route("/{id}", get(get_subscription_by_id))
        .route("/{id}/action", post(subscription_action))
        .layer(Extension(orchestrator))
        .layer(Extension(hub))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn claims(organization_id: Option<&str>) -> ::server_common::Claims {
        ::server_common::Claims {
            sub: "user-1".to_string(),
            exp: i64::MAX,
            iat: 0,
            organization_id: organization_id.map(str::to_string),
            username: "subscriber".to_string(),
            email: "subscriber@example.com".to_string(),
            roles: vec![],
            session_id: None,
            jti: "subscription-test".to_string(),
        }
    }

    #[test]
    fn multitenant_subscription_requests_require_a_non_blank_organization() {
        assert_eq!(
            resolve_subscription_tenant(&claims(None), true, "system"),
            Err(StatusCode::UNAUTHORIZED),
        );
        assert_eq!(
            resolve_subscription_tenant(&claims(Some("  ")), true, "system"),
            Err(StatusCode::UNAUTHORIZED),
        );
    }

    #[test]
    fn explicitly_single_tenant_requests_use_the_configured_default() {
        assert_eq!(
            resolve_subscription_tenant(&claims(None), false, "configured-tenant"),
            Ok("configured-tenant".to_string()),
        );
    }

    #[tokio::test]
    async fn subscription_overview_queries_are_tenant_scoped_in_postgres() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:32768/ohc_test".to_string()
        });
        let admin = match sqlx::PgPool::connect(&database_url).await {
            Ok(pool) => pool,
            Err(error) => {
                eprintln!("skipping subscription postgres test; database unavailable: {error}");
                return;
            }
        };
        let schema = format!("subscription_overview_{}", uuid::Uuid::new_v4().simple());
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .expect("create isolated subscription schema");

        let schema_for_connections = schema.clone();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4)
            .after_connect(move |connection, _metadata| {
                let search_path = format!("SET search_path TO {schema_for_connections}");
                Box::pin(async move {
                    sqlx::query(&search_path).execute(connection).await?;
                    Ok(())
                })
            })
            .connect(&database_url)
            .await
            .expect("connect isolated subscription pool");

        for statement in [
            "CREATE TABLE products (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, title TEXT NOT NULL, description TEXT, price_cents BIGINT NOT NULL)",
            "CREATE TABLE subscription_plans (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, product_id TEXT, name TEXT NOT NULL, description TEXT, price_cents BIGINT NOT NULL, frequency TEXT NOT NULL, interval TEXT, status TEXT NOT NULL, created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE subscriptions (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, customer_id TEXT NOT NULL, status TEXT NOT NULL, health_score INTEGER, created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE fulfillment_batches (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, fulfillment_date DATE NOT NULL, status TEXT NOT NULL, subscriber_count INTEGER NOT NULL, created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP)",
            "INSERT INTO subscription_plans (id, tenant_id, name, description, price_cents, frequency, interval, status) VALUES ('plan-a', 'tenant-a', 'A plan', 'Owned', 2500, 'month', 'month', 'active'), ('plan-b', 'tenant-b', 'B plan', 'Foreign', 9900, 'year', 'year', 'active')",
            "INSERT INTO subscriptions (id, tenant_id, customer_id, status, health_score) VALUES ('subscriber-a', 'tenant-a', 'customer-a', 'ACTIVE', 90), ('subscriber-b', 'tenant-b', 'customer-b', 'ACTIVE', 10)",
            "INSERT INTO fulfillment_batches (id, tenant_id, fulfillment_date, status, subscriber_count) VALUES ('batch-a', 'tenant-a', '2026-08-01', 'PENDING', 3), ('batch-b', 'tenant-b', '2026-09-01', 'PENDING', 99)",
        ] {
            sqlx::query(statement)
                .execute(&pool)
                .await
                .expect("prepare subscription integration data");
        }

        let overview = fetch_subscription_overview(&pool, "tenant-a")
            .await
            .expect("fetch tenant subscription overview");

        assert_eq!(overview.plans.len(), 1);
        assert_eq!(overview.plans[0].id, "plan-a");
        assert_eq!(overview.subscribers.len(), 1);
        assert_eq!(overview.subscribers[0].id, "subscriber-a");
        assert_eq!(overview.batches.len(), 1);
        assert_eq!(overview.batches[0].id, "batch-a");

        pool.close().await;
        sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
            .execute(&admin)
            .await
            .expect("drop isolated subscription schema");
    }

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
        let (payload, signature) = token
            .split_once('.')
            .expect("signed token should have two parts");
        let replacement = if payload.ends_with('A') { "B" } else { "A" };
        let tampered = format!(
            "{}{}.{}",
            &payload[..payload.len() - 1],
            replacement,
            signature
        );

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
