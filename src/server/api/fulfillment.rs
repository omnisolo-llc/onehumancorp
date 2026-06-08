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
use ::server_common::Claims;

#[derive(Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub fulfillment_mode: String, // Shipping, LocalDelivery, Pickup
    pub status: String, // Preparing, ReadyForPickup, Shipped, Delivered
    pub customer_name: String,
    pub items: Vec<String>,
    pub organization_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_lng: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_delivery_id: Option<String>,
}

#[derive(Serialize)]
pub struct QueueResponse {
    pub to_pack: Vec<Order>,
    pub awaiting_pickup: Vec<Order>,
}

#[derive(Deserialize)]
pub struct ExecuteActionRequest {
    pub action: String, // e.g. "print_label", "mark_ready", "hand_off"
}

struct AppState {
    orders: RwLock<Vec<Order>>,
    pool: sqlx::PgPool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoorDashTrackingUpdate {
    pub external_delivery_id: String,
    pub status: String,
    pub driver_id: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Deserialize)]
pub struct FetchRatesRequest {
    #[serde(rename = "orderId")]
    pub order_id: String,
    pub weight: String,
    pub dimensions: String,
}

#[derive(Serialize)]
pub struct Rate {
    pub id: String,
    pub carrier: String,
    pub service: String,
    pub amount: String,
    pub days: u32,
}

#[derive(Serialize)]
pub struct FetchRatesResponse {
    pub rates: Vec<Rate>,
}

#[derive(Deserialize)]
pub struct PurchaseLabelRequest {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "rateId")]
    pub rate_id: String,
}

#[derive(Serialize)]
pub struct PurchaseLabelResponse {
    pub success: bool,
    #[serde(rename = "labelUrl")]
    pub label_url: String,
    #[serde(rename = "trackingNumber")]
    pub tracking_number: String,
    pub carrier: String,
}

pub fn router<S>(pool: sqlx::PgPool) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let initial_orders = vec![
        Order {
            id: "ord-1".to_string(),
            fulfillment_mode: "Shipping".to_string(),
            status: "Preparing".to_string(),
            customer_name: "John Doe".to_string(),
            items: vec!["2 Summer Dresses".to_string()],
            organization_id: "default".to_string(),
            driver_status: None,
            driver_id: None,
            driver_lat: None,
            driver_lng: None,
            provider_delivery_id: None,
        },
        Order {
            id: "ord-2".to_string(),
            fulfillment_mode: "LocalDelivery".to_string(),
            status: "Preparing".to_string(),
            customer_name: "Jane Smith".to_string(),
            items: vec!["Chocolate Cake".to_string()],
            organization_id: "default".to_string(),
            driver_status: None,
            driver_id: None,
            driver_lat: None,
            driver_lng: None,
            provider_delivery_id: None,
        },
        Order {
            id: "ord-3".to_string(),
            fulfillment_mode: "Pickup".to_string(),
            status: "ReadyForPickup".to_string(),
            customer_name: "Alice Johnson".to_string(),
            items: vec!["Coffee and Bagel".to_string()],
            organization_id: "default".to_string(),
            driver_status: None,
            driver_id: None,
            driver_lat: None,
            driver_lng: None,
            provider_delivery_id: None,
        },
    ];

    let state = Arc::new(AppState {
        orders: RwLock::new(initial_orders),
        pool,
    });

    Router::new()
        .route("/", get(get_queue))
        .route("/execute/{id}", post(execute_action))
        .route("/rates", post(fetch_rates))
        .route("/label", post(purchase_label))
        .route("/webhook/doordash", post(doordash_webhook))
        .with_state(state)
}

async fn fetch_rates(
    Extension(_claims): Extension<Claims>,
    Json(payload): Json<FetchRatesRequest>,
) -> impl IntoResponse {
    let weight: f64 = payload.weight.parse().unwrap_or(16.0);

    let api_key = match std::env::var("SHIPPO_API_TOKEN") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({"error": "SHIPPO_API_TOKEN is required"})),
            )
                .into_response();
        }
    };
    let client = crate::integrations::shippo::provider::ShippoProvider::new(api_key);
    let rates = match client.fetch_rates(weight, &payload.dimensions).await {
        Ok(rates) => rates,
        Err(err) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": err})),
            )
                .into_response();
        }
    };
    let rates = rates
        .into_iter()
        .map(|rate| Rate {
            id: rate.id,
            carrier: rate.carrier,
            service: rate.service,
            amount: rate.amount,
            days: rate.days,
        })
        .collect();

    (StatusCode::OK, Json(FetchRatesResponse { rates })).into_response()
}

async fn purchase_label(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<PurchaseLabelRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => "default".to_string(),
    };

    let api_key = match std::env::var("SHIPPO_API_TOKEN") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({"error": "SHIPPO_API_TOKEN is required"})),
            )
                .into_response();
        }
    };
    let client = crate::integrations::shippo::provider::ShippoProvider::new(api_key);
    let label = match client.purchase_label(&payload.rate_id).await {
        Ok(label) => label,
        Err(err) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": err})),
            )
                .into_response();
        }
    };

    let mut orders = state.orders.write().unwrap();
    for order in orders.iter_mut() {
        if order.id == payload.order_id && order.organization_id == tenant_id {
            if order.fulfillment_mode == "Shipping" {
                order.status = "Shipped".to_string();
            }
            break;
        }
    }

    (StatusCode::OK, Json(PurchaseLabelResponse {
        success: true,
        label_url: label.label_url,
        tracking_number: label.tracking_number,
        carrier: label.carrier,
    })).into_response()
}

async fn get_queue(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => "default".to_string(), // fallback for testing if claims are empty or mock
    };

    let orders = state.orders.read().unwrap();
    let mut to_pack = Vec::new();
    let mut awaiting_pickup = Vec::new();

    for order in orders.iter() {
        if order.organization_id != tenant_id {
            continue;
        }

        match order.status.as_str() {
            "Preparing" => {
                to_pack.push(order.clone());
            }
            "ReadyForPickup" | "DriverRequested" | "DriverTracking" => {
                awaiting_pickup.push(order.clone());
            }
            _ => {}
        }
    }

    (StatusCode::OK, Json(QueueResponse { to_pack, awaiting_pickup })).into_response()
}

async fn execute_action(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Path(id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ExecuteActionRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => "default".to_string(),
    };

    let mut orders = state.orders.write().unwrap();
    let mut found = false;

    for order in orders.iter_mut() {
        if order.id == id && order.organization_id == tenant_id {
            found = true;
            match payload.action.as_str() {
                "print_label" => {
                    if order.fulfillment_mode == "Shipping" {
                        order.status = "Shipped".to_string();
                    }
                }
                "mark_ready" => {
                    if order.fulfillment_mode == "LocalDelivery" || order.fulfillment_mode == "Pickup" {
                        order.status = "ReadyForPickup".to_string();
                    }
                }
                "request_driver" => {
                    if order.fulfillment_mode == "LocalDelivery" {
                        order.status = "DriverRequested".to_string();
                    }
                }
                "hand_off" => {
                    if order.status == "ReadyForPickup" || order.status == "DriverRequested" {
                        order.status = "Delivered".to_string();
                    }
                }
                _ => {}
            }
            break;
        }
    }

    if found {
        (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Order not found or unauthorized"}))).into_response()
    }
}

async fn doordash_webhook(
    State(state): State<Arc<AppState>>,
    claims: Option<Extension<Claims>>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let update = match parse_doordash_tracking_webhook(&payload) {
        Ok(update) => update,
        Err(err) => {
            return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": err}))).into_response();
        }
    };
    let tenant_id = match claims
        .and_then(|Extension(claims)| claims.organization_id)
        .or_else(|| {
            headers
                .get("x-tenant-id")
                .and_then(|value| value.to_str().ok())
                .map(|value| value.to_string())
        })
        .or_else(|| find_string_by_key(&payload, &["organization_id", "tenant_id"]))
    {
        Some(id) if !id.trim().is_empty() => id,
        _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    match persist_doordash_tracking_update(&state.pool, &tenant_id, &update).await {
        Ok(_) => {
            apply_doordash_tracking_update_to_queue(&state, &tenant_id, &update);
            (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
        }
        Err(err) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({"error": err})),
        )
            .into_response(),
    }
}

pub fn parse_doordash_tracking_webhook(payload: &Value) -> Result<DoorDashTrackingUpdate, String> {
    let external_delivery_id = find_string_by_key(payload, &["external_delivery_id"])
        .ok_or_else(|| "DoorDash webhook missing external_delivery_id".to_string())?;
    let status = find_string_by_key(payload, &["delivery_status", "status"])
        .or_else(|| find_string_by_key(payload, &["event_type"]))
        .ok_or_else(|| "DoorDash webhook missing delivery status".to_string())?;
    let driver_id = find_string_by_key(payload, &["dasher_id", "driver_id"])
        .or_else(|| find_nested_object_string(payload, &["dasher", "driver"], &["id"]));
    let (latitude, longitude) = find_location(payload)?;

    Ok(DoorDashTrackingUpdate {
        external_delivery_id,
        status,
        driver_id,
        latitude,
        longitude,
    })
}

pub async fn persist_doordash_tracking_update(
    pool: &sqlx::PgPool,
    tenant_id: &str,
    update: &DoorDashTrackingUpdate,
) -> Result<u64, String> {
    let mut tx = pool.begin().await.map_err(|err| err.to_string())?;
    ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id)
        .await
        .map_err(|err| err.to_string())?;

    let result = if let (Some(latitude), Some(longitude)) = (update.latitude, update.longitude) {
        sqlx::query(
            "UPDATE delivery_tasks
             SET provider = 'doordash',
                 provider_delivery_id = COALESCE(provider_delivery_id, $2),
                 status = $3,
                 driver_id = COALESCE($4, driver_id),
                 delivery_location_lat = $5,
                 delivery_location_lng = $6,
                 updated_at = CURRENT_TIMESTAMP
             WHERE organization_id = $1
               AND ((provider = 'doordash' AND provider_delivery_id = $2) OR order_id = $2)",
        )
        .bind(tenant_id)
        .bind(&update.external_delivery_id)
        .bind(&update.status)
        .bind(&update.driver_id)
        .bind(latitude)
        .bind(longitude)
        .execute(&mut *tx)
        .await
        .map_err(|err| err.to_string())?
    } else {
        sqlx::query(
            "UPDATE delivery_tasks
             SET provider = 'doordash',
                 provider_delivery_id = COALESCE(provider_delivery_id, $2),
                 status = $3,
                 driver_id = COALESCE($4, driver_id),
                 updated_at = CURRENT_TIMESTAMP
             WHERE organization_id = $1
               AND ((provider = 'doordash' AND provider_delivery_id = $2) OR order_id = $2)",
        )
        .bind(tenant_id)
        .bind(&update.external_delivery_id)
        .bind(&update.status)
        .bind(&update.driver_id)
        .execute(&mut *tx)
        .await
        .map_err(|err| err.to_string())?
    };

    tx.commit().await.map_err(|err| err.to_string())?;
    Ok(result.rows_affected())
}

fn apply_doordash_tracking_update_to_queue(
    state: &Arc<AppState>,
    tenant_id: &str,
    update: &DoorDashTrackingUpdate,
) {
    if let Ok(mut orders) = state.orders.write() {
        for order in orders.iter_mut() {
            if order.organization_id == tenant_id
                && (order.id == update.external_delivery_id
                    || order.provider_delivery_id.as_deref() == Some(update.external_delivery_id.as_str()))
            {
                order.driver_status = Some(update.status.clone());
                order.driver_id = update.driver_id.clone().or_else(|| order.driver_id.clone());
                order.driver_lat = update.latitude.or(order.driver_lat);
                order.driver_lng = update.longitude.or(order.driver_lng);
                order.provider_delivery_id = Some(update.external_delivery_id.clone());
                if order.status == "DriverRequested" || order.status == "ReadyForPickup" {
                    order.status = "DriverTracking".to_string();
                }
            }
        }
    }
}

fn find_string_by_key(value: &Value, keys: &[&str]) -> Option<String> {
    match value {
        Value::Object(map) => {
            for key in keys {
                if let Some(found) = map.get(*key).and_then(|value| value.as_str()) {
                    let trimmed = found.trim();
                    if !trimmed.is_empty() {
                        return Some(trimmed.to_string());
                    }
                }
            }
            for nested in map.values() {
                if let Some(found) = find_string_by_key(nested, keys) {
                    return Some(found);
                }
            }
            None
        }
        Value::Array(values) => values
            .iter()
            .find_map(|nested| find_string_by_key(nested, keys)),
        _ => None,
    }
}

fn find_nested_object_string(value: &Value, object_keys: &[&str], field_keys: &[&str]) -> Option<String> {
    match value {
        Value::Object(map) => {
            for object_key in object_keys {
                if let Some(found) = map
                    .get(*object_key)
                    .and_then(|nested| find_string_by_key(nested, field_keys))
                {
                    return Some(found);
                }
            }
            for nested in map.values() {
                if let Some(found) = find_nested_object_string(nested, object_keys, field_keys) {
                    return Some(found);
                }
            }
            None
        }
        Value::Array(values) => values
            .iter()
            .find_map(|nested| find_nested_object_string(nested, object_keys, field_keys)),
        _ => None,
    }
}

fn find_location(value: &Value) -> Result<(Option<f64>, Option<f64>), String> {
    match value {
        Value::Object(map) => {
            let latitude = map
                .get("lat")
                .or_else(|| map.get("latitude"))
                .and_then(|value| value.as_f64());
            let longitude = map
                .get("lng")
                .or_else(|| map.get("longitude"))
                .and_then(|value| value.as_f64());
            if latitude.is_some() || longitude.is_some() {
                let latitude = latitude.ok_or_else(|| "DoorDash location missing latitude".to_string())?;
                let longitude = longitude.ok_or_else(|| "DoorDash location missing longitude".to_string())?;
                if !(-90.0..=90.0).contains(&latitude) {
                    return Err("DoorDash latitude is out of range".to_string());
                }
                if !(-180.0..=180.0).contains(&longitude) {
                    return Err("DoorDash longitude is out of range".to_string());
                }
                return Ok((Some(latitude), Some(longitude)));
            }
            for nested in map.values() {
                let found = find_location(nested)?;
                if found.0.is_some() || found.1.is_some() {
                    return Ok(found);
                }
            }
            Ok((None, None))
        }
        Value::Array(values) => {
            for nested in values {
                let found = find_location(nested)?;
                if found.0.is_some() || found.1.is_some() {
                    return Ok(found);
                }
            }
            Ok((None, None))
        }
        _ => Ok((None, None)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_doordash_tracking_webhook_with_dasher_coordinates() {
        let payload = json!({
            "event_type": "DASHER_CONFIRMED",
            "data": {
                "external_delivery_id": "ord-2",
                "delivery_status": "dasher_confirmed",
                "dasher": {
                    "id": "dasher-42"
                },
                "dasher_location": {
                    "lat": 37.7864,
                    "lng": -122.4051
                }
            }
        });

        let update = parse_doordash_tracking_webhook(&payload).expect("valid DoorDash tracking payload");

        assert_eq!(update.external_delivery_id, "ord-2");
        assert_eq!(update.status, "dasher_confirmed");
        assert_eq!(update.driver_id.as_deref(), Some("dasher-42"));
        assert_eq!(update.latitude, Some(37.7864));
        assert_eq!(update.longitude, Some(-122.4051));
    }

    #[test]
    fn rejects_doordash_tracking_webhook_without_external_delivery_id() {
        let payload = json!({
            "delivery_status": "enroute_to_dropoff",
            "dasher_location": {
                "latitude": 37.7864,
                "longitude": -122.4051
            }
        });

        let err = parse_doordash_tracking_webhook(&payload).unwrap_err();

        assert!(err.contains("external_delivery_id"));
    }
}
