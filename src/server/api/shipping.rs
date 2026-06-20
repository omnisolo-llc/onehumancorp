use axum::{
    extract::{Extension, Path, State},
    response::IntoResponse,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::RwLock;
use std::sync::Arc;
use crate::hub::Hub;
use crate::integrations::carrier::mock::{MockCarrierClient, ShippingRate, ShippingLabel};

#[derive(Serialize, Deserialize)]
pub struct RateRequest {
    pub order_id: String,
    pub weight_grams: i64,
}

#[derive(Serialize)]
pub struct RateResponse {
    pub rates: Vec<ShippingRate>,
}

#[derive(Serialize, Deserialize)]
pub struct LabelRequest {
    pub order_id: String,
    pub carrier: String,
    pub service_level: String,
    pub cost_cents: i64,
}

#[derive(Serialize)]
pub struct LabelResponse {
    pub tracking_number: String,
    pub label_url: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct TrackingEvent {
    pub id: String,
    pub tenant_id: String,
    pub tracking_number: String,
    pub status: String,
    pub message: String,
}

use ohc_builtin_agent::mesh::transport::MeshTransport;

pub fn router(hub: Arc<Hub>) -> Router<Arc<dyn MeshTransport>> {
    Router::new()
        .route("/rates", post(get_rates))
        .route("/label", post(generate_label))
        .route("/tracking/:tracking_number", get(get_tracking))
        .with_state(hub)
}

async fn get_rates(
    State(_hub): State<Arc<Hub>>,
    headers: HeaderMap,
    Json(payload): Json<RateRequest>,
) -> impl IntoResponse {
    let tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");
    let client = MockCarrierClient;
    match client.get_rates(tenant_id, payload.weight_grams).await {
        Ok(rates) => (StatusCode::OK, Json(RateResponse { rates })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

async fn generate_label(
    State(hub): State<Arc<Hub>>,
    headers: HeaderMap,
    Json(payload): Json<LabelRequest>,
) -> impl IntoResponse {
    let tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");
    let client = MockCarrierClient;
    match client.generate_label(tenant_id, &payload.carrier).await {
        Ok(label) => {
            // Persist the label to tracking_events (simulated)
            let _ = sqlx::query("INSERT INTO tracking_events (id, tenant_id, tracking_number, status, message) VALUES ($1, $2, $3, 'LABEL_CREATED', 'Shipping label generated') ON CONFLICT DO NOTHING")
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(tenant_id)
                .bind(&label.tracking_number)
                .execute(&hub.pool)
                .await;

            // Trigger ambassador agent draft (simulated)
            let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, source, agent_type, action_type, payload, status) VALUES ($1, $2, 'shipping', 'ambassador', 'draft_shipping_update', $3, 'pending') ON CONFLICT DO NOTHING")
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(tenant_id)
                .bind(serde_json::json!({"tracking_number": label.tracking_number}))
                .execute(&hub.pool)
                .await;

            (StatusCode::OK, Json(LabelResponse {
                tracking_number: label.tracking_number,
                label_url: label.label_url,
            })).into_response()
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))).into_response(),
    }
}

async fn get_tracking(
    State(hub): State<Arc<Hub>>,
    headers: HeaderMap,
    Path(tracking_number): Path<String>,
) -> impl IntoResponse {
    let tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");

    match sqlx::query_as::<_, TrackingEvent>("SELECT id, tenant_id, tracking_number, status, message FROM tracking_events WHERE tenant_id = $1 AND tracking_number = $2")
        .bind(tenant_id)
        .bind(tracking_number)
        .fetch_all(&hub.pool)
        .await {
            Ok(events) => (StatusCode::OK, Json(events)).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to fetch tracking"}))).into_response(),
        }
}
