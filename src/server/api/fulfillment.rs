//! Persisted fulfillment and authenticated provider tracking.
use super::shipping::authority::{self, ShippingAccess};
use ::server_common::Claims;
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Extension, Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use server_auth::commit_authority::AuthorizedPgOwner;
use sha2::{Digest, Sha256};
use sqlx::Row;
use std::sync::Arc;

#[path = "fulfillment/authentication.rs"]
pub(crate) mod authentication;
#[path = "fulfillment/storage.rs"]
pub(crate) mod storage;
#[path = "fulfillment/tracking.rs"]
pub(crate) mod tracking;
use authentication::{
    MAX_BODY_BYTES, ProviderScope, verify_doordash_authorization, verify_shippo_signature,
};
pub use tracking::TrackingUpdate as ShippoTrackingUpdate;
pub use tracking::TrackingUpdate as DoorDashTrackingUpdate;
pub use tracking::parse_doordash as parse_doordash_tracking_webhook;
pub use tracking::parse_shippo as parse_shippo_tracking_webhook;

#[derive(Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub delivery_task_id: String,
    pub fulfillment_mode: Option<String>,
    pub status: String,
    pub customer_name: Option<String>,
    pub items: Vec<String>,
    pub organization_id: String,
    pub driver_status: Option<String>,
    pub driver_id: Option<String>,
    pub driver_lat: Option<f64>,
    pub driver_lng: Option<f64>,
    pub provider_delivery_id: Option<String>,
    pub binding_status: String,
    pub label_url: Option<String>,
}
#[derive(Serialize)]
pub struct QueueResponse {
    pub to_pack: Vec<Order>,
    pub awaiting_pickup: Vec<Order>,
}
#[derive(Deserialize)]
pub struct ExecuteActionRequest {
    pub action: String,
}
pub struct AppState {
    pool: sqlx::PgPool,
}

/// Private routes. The parent mount applies canonical bearer authentication and
/// merges the existing shipping router as the legacy /rates and /label aliases.
pub fn router<S: Clone + Send + Sync + 'static>(access: Arc<ShippingAccess>) -> Router<S> {
    Router::new()
        .route("/", get(get_queue))
        .route("/execute/{id}", post(execute_action))
        .with_state(access)
}
/// Provider authentication is independent of owner bearer authentication.
pub fn webhook_router<S: Clone + Send + Sync + 'static>(pool: sqlx::PgPool) -> Router<S> {
    Router::new()
        .route("/webhook/shippo", post(shippo_webhook))
        .route("/webhook/doordash", post(doordash_webhook))
        .layer(DefaultBodyLimit::max(MAX_BODY_BYTES))
        .with_state(Arc::new(AppState { pool }))
}
fn error(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({"success":false,"error":message}))).into_response()
}
fn tracking_result(result: Result<storage::Outcome, storage::Error>) -> Response {
    match result {
        Ok(outcome) => (
            StatusCode::OK,
            Json(json!({"success":true,"applied":outcome==storage::Outcome::Applied})),
        )
            .into_response(),
        Err(storage::Error::Conflict(message)) => error(StatusCode::CONFLICT, message),
        Err(storage::Error::Database(cause)) => {
            tracing::error!("fulfillment persistence failed: {cause}");
            error(
                StatusCode::SERVICE_UNAVAILABLE,
                "fulfillment storage is unavailable",
            )
        }
    }
}

pub async fn shippo_webhook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Response {
    let secret = match std::env::var("SHIPPO_WEBHOOK_SECRET") {
        Ok(secret) if !secret.trim().is_empty() => secret,
        _ => {
            return error(
                StatusCode::SERVICE_UNAVAILABLE,
                "Shippo webhook authentication is not configured",
            );
        }
    };
    let scope = match ProviderScope::from_environment("SHIPPO") {
        Ok(scope) => scope,
        Err(message) => return error(StatusCode::SERVICE_UNAVAILABLE, message),
    };
    if verify_shippo_signature(&secret, &headers, &body, chrono::Utc::now().timestamp()).is_err() {
        return error(StatusCode::UNAUTHORIZED, "invalid Shippo signature");
    }
    let payload: Value = match serde_json::from_slice(&body) {
        Ok(payload) => payload,
        Err(_) => return error(StatusCode::BAD_REQUEST, "invalid JSON"),
    };
    let update = match parse_shippo_tracking_webhook(&payload) {
        Ok(update) => update,
        Err(message) if message.starts_with("Ignoring Shippo webhook event:") => {
            return (
                StatusCode::OK,
                Json(json!({"success":true,"applied":false,"message":message})),
            )
                .into_response();
        }
        Err(message) => return error(StatusCode::BAD_REQUEST, &message),
    };
    tracking_result(
        storage::apply_tracking(
            &state.pool,
            &scope,
            &update,
            &hex::encode(Sha256::digest(&body)),
        )
        .await,
    )
}
async fn doordash_webhook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Response {
    let expected = match std::env::var("DOORDASH_WEBHOOK_AUTHORIZATION") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            return error(
                StatusCode::SERVICE_UNAVAILABLE,
                "DoorDash webhook authentication is not configured",
            );
        }
    };
    let scope = match ProviderScope::from_environment("DOORDASH") {
        Ok(scope) => scope,
        Err(message) => return error(StatusCode::SERVICE_UNAVAILABLE, message),
    };
    if verify_doordash_authorization(&expected, &headers).is_err() {
        return error(StatusCode::UNAUTHORIZED, "invalid DoorDash authorization");
    }
    let payload: Value = match serde_json::from_slice(&body) {
        Ok(payload) => payload,
        Err(_) => return error(StatusCode::BAD_REQUEST, "invalid JSON"),
    };
    let update = match parse_doordash_tracking_webhook(&payload) {
        Ok(update) => update,
        Err(message) => return error(StatusCode::BAD_REQUEST, &message),
    };
    tracking_result(
        storage::apply_tracking(
            &state.pool,
            &scope,
            &update,
            &hex::encode(Sha256::digest(&body)),
        )
        .await,
    )
}

async fn get_queue(
    State(access): State<Arc<ShippingAccess>>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
) -> Response {
    if server_common::auth_utils::signed_tenant_id(&claims).is_none() {
        return error(StatusCode::UNAUTHORIZED, "a signed tenant is required");
    }
    let owner = match access.authorize_postgres(&claims, &headers).await {
        Ok(owner) => owner,
        Err(error) => return authority::response(error),
    };
    match read_queue(owner).await {
        Ok(queue) => (StatusCode::OK, Json(queue)).into_response(),
        Err(error) => authority::response(error),
    }
}
async fn read_queue(owner: AuthorizedPgOwner) -> Result<QueueResponse, authority::Error> {
    let tenant = owner.tenant_id().to_owned();
    let mut tx = owner.begin().await?;
    let rows=sqlx::query(
        "SELECT d.id,d.order_id,d.provider,d.status,d.provider_delivery_id,d.driver_id,d.delivery_location_lat,d.delivery_location_lng,c.name AS customer_name,
         b.delivery_task_id IS NOT NULL AS is_bound,b.label_url,
         ARRAY(SELECT p.title FROM order_items i JOIN products p ON p.id=i.product_id AND p.tenant_id=i.tenant_id WHERE i.order_id=o.id AND i.tenant_id=o.tenant_id ORDER BY i.id) AS items
         FROM delivery_tasks d JOIN orders o ON o.id=d.order_id AND o.tenant_id=d.organization_id
         LEFT JOIN customers c ON c.id=o.customer_id AND c.tenant_id=o.tenant_id
         LEFT JOIN delivery_provider_bindings b ON b.delivery_task_id=d.id AND b.organization_id=d.organization_id
         WHERE d.organization_id=$1 AND lower(COALESCE(o.status,'')) NOT IN ('cancelled','canceled','fulfilled','returned')
           AND upper(d.status) NOT IN ('DELIVERED','RETURNED','CANCELLED','CANCELED')
         ORDER BY d.updated_at DESC,d.id LIMIT 100")
        .bind(&tenant).fetch_all(tx.connection()).await?;
    let mut queue = QueueResponse {
        to_pack: Vec::new(),
        awaiting_pickup: Vec::new(),
    };
    for row in rows {
        let provider: Option<String> = row.try_get("provider")?;
        let status: String = row.try_get("status")?;
        let order = Order {
            id: row.try_get("order_id")?,
            delivery_task_id: row.try_get::<uuid::Uuid, _>("id")?.to_string(),
            fulfillment_mode: match provider.as_deref() {
                Some("shippo") => Some("Shipping".into()),
                Some("doordash") => Some("LocalDelivery".into()),
                _ => None,
            },
            customer_name: row.try_get("customer_name")?,
            items: row.try_get("items")?,
            organization_id: tenant.clone(),
            driver_status: (provider.as_deref() == Some("doordash")).then(|| status.clone()),
            status: status.clone(),
            driver_id: row.try_get("driver_id")?,
            driver_lat: row.try_get("delivery_location_lat")?,
            driver_lng: row.try_get("delivery_location_lng")?,
            provider_delivery_id: row.try_get("provider_delivery_id")?,
            label_url: row.try_get("label_url")?,
            binding_status: if row.try_get::<bool, _>("is_bound")? {
                "bound"
            } else {
                "reconciliation_required"
            }
            .into(),
        };
        if matches!(
            status.to_ascii_uppercase().as_str(),
            "PENDING" | "PREPARING"
        ) {
            queue.to_pack.push(order);
        } else {
            queue.awaiting_pickup.push(order);
        }
    }
    tx.commit().await?;
    Ok(queue)
}

async fn execute_action(
    State(access): State<Arc<ShippingAccess>>,
    Path(id): Path<String>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
    Json(payload): Json<ExecuteActionRequest>,
) -> Response {
    if server_common::auth_utils::signed_tenant_id(&claims).is_none() {
        return error(StatusCode::UNAUTHORIZED, "a signed tenant is required");
    }
    let owner = match access.authorize_postgres(&claims, &headers).await {
        Ok(owner) => owner,
        Err(error) => return authority::response(error),
    };
    if payload.action == "print_label" {
        return error(
            StatusCode::CONFLICT,
            "Purchase or view the actual shipping label on the order page; printing does not ship an order",
        );
    }
    if payload.action == "request_driver" {
        return error(
            StatusCode::NOT_IMPLEMENTED,
            "Driver dispatch is not connected; no driver was requested",
        );
    }
    if !matches!(payload.action.as_str(), "mark_ready" | "hand_off") {
        return error(StatusCode::BAD_REQUEST, "unsupported fulfillment action");
    }
    match record_manual_action(owner, &id, &payload.action).await {
        Ok(status) => (
            StatusCode::OK,
            Json(json!({"success":true,"status":status})),
        )
            .into_response(),
        Err(error) => authority::response(error),
    }
}
async fn record_manual_action(
    owner: AuthorizedPgOwner,
    order: &str,
    action: &str,
) -> Result<&'static str, authority::Error> {
    let tenant = owner.tenant_id().to_owned();
    let mut tx = owner.begin().await?;
    let rows=sqlx::query("SELECT d.id,d.provider,d.status FROM delivery_tasks d JOIN orders o ON o.id=d.order_id AND o.tenant_id=d.organization_id WHERE d.organization_id=$1 AND d.order_id=$2 AND lower(COALESCE(o.status,'')) NOT IN ('canceled','cancelled','fulfilled','returned') LIMIT 2 FOR UPDATE OF d,o")
        .bind(&tenant).bind(order).fetch_all(tx.connection()).await?;
    if rows.len() != 1 {
        return Err(authority::Error::Conflict(
            "a unique active fulfillment task is required",
        ));
    }
    let row = &rows[0];
    let provider: Option<String> = row.try_get("provider")?;
    if provider.as_deref() != Some("doordash") {
        return Err(authority::Error::Conflict(
            "this action requires a recorded local-delivery task",
        ));
    }
    let prior: String = row.try_get("status")?;
    let status = match action {
        "mark_ready"
            if matches!(
                prior.to_ascii_uppercase().as_str(),
                "PENDING" | "PREPARING" | "READYFORPICKUP"
            ) =>
        {
            "ReadyForPickup"
        }
        "hand_off"
            if matches!(
                prior.to_ascii_uppercase().as_str(),
                "READYFORPICKUP" | "DRIVER_CONFIRMED" | "DRIVER_ENROUTE_TO_PICKUP" | "HANDED_OFF"
            ) =>
        {
            "HANDED_OFF"
        }
        _ => {
            return Err(authority::Error::Conflict(
                "the recorded fulfillment state does not permit this action",
            ));
        }
    };
    sqlx::query("UPDATE delivery_tasks SET status=$3,updated_at=CURRENT_TIMESTAMP WHERE id=$1 AND organization_id=$2")
        .bind(row.try_get::<uuid::Uuid,_>("id")?).bind(&tenant).bind(status).execute(tx.connection()).await?;
    tx.commit().await?;
    Ok(status)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_shippo_tracking_webhook_with_valid_data() {
        let update=parse_shippo_tracking_webhook(&json!({"event":"track_updated","test":true,"data":{"tracking_number":"9499907123456123456781","carrier":"usps","tracking_status":{"status":"DELIVERED","status_date":"2026-10-03T00:00:00Z"}}})).unwrap();
        assert_eq!(
            update.tracking_number.as_deref(),
            Some("9499907123456123456781")
        );
        assert_eq!(update.status, "DELIVERED");
        assert_eq!(update.provider_object_id, None);
    }
    #[test]
    fn rejects_shippo_tracking_webhook_without_tracking_number() {
        let error = parse_shippo_tracking_webhook(
            &json!({"event":"track_updated","data":{"tracking_status":{"status":"DELIVERED"}}}),
        )
        .unwrap_err();
        assert!(error.contains("tracking_number"));
    }
    #[test]
    fn ignores_shippo_webhook_wrong_event() {
        let error = parse_shippo_tracking_webhook(
            &json!({"event":"transaction_created","data":{"tracking_number":"123"}}),
        )
        .unwrap_err();
        assert!(error.starts_with("Ignoring Shippo webhook event: transaction_created"));
    }
    #[test]
    fn parses_doordash_tracking_webhook_with_dasher_coordinates() {
        let update=parse_doordash_tracking_webhook(&json!({"event_name":"DASHER_CONFIRMED","created_at":"2026-10-03T00:00:00Z","data":{"external_delivery_id":"delivery-2","dasher":{"id":"dasher-42"},"dasher_location":{"lat":37.7864,"lng":-122.4051}}})).unwrap();
        assert_eq!(update.provider_object_id.as_deref(), Some("delivery-2"));
        assert_eq!(update.status, "DRIVER_CONFIRMED");
        assert_eq!(update.driver_id.as_deref(), Some("dasher-42"));
        assert_eq!(update.latitude, Some(37.7864));
        assert_eq!(update.longitude, Some(-122.4051));
    }
    #[test]
    fn rejects_doordash_tracking_webhook_without_external_delivery_id() {
        let error =
            parse_doordash_tracking_webhook(&json!({"delivery_status":"enroute_to_dropoff"}))
                .unwrap_err();
        assert!(error.contains("external_delivery_id"));
    }
}
