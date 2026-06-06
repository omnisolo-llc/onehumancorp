use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
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

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    // Initialize with mock data
    let mock_orders = vec![
        Order {
            id: "ord-1".to_string(),
            fulfillment_mode: "Shipping".to_string(),
            status: "Preparing".to_string(),
            customer_name: "John Doe".to_string(),
            items: vec!["2 Summer Dresses".to_string()],
            organization_id: "default".to_string(),
        },
        Order {
            id: "ord-2".to_string(),
            fulfillment_mode: "LocalDelivery".to_string(),
            status: "Preparing".to_string(),
            customer_name: "Jane Smith".to_string(),
            items: vec!["Chocolate Cake".to_string()],
            organization_id: "default".to_string(),
        },
        Order {
            id: "ord-3".to_string(),
            fulfillment_mode: "Pickup".to_string(),
            status: "ReadyForPickup".to_string(),
            customer_name: "Alice Johnson".to_string(),
            items: vec!["Coffee and Bagel".to_string()],
            organization_id: "default".to_string(),
        },
    ];

    let state = Arc::new(AppState {
        orders: RwLock::new(mock_orders),
    });

    Router::new()
        .route("/", get(get_queue))
        .route("/execute/:id", post(execute_action))
        .route("/rates", post(fetch_rates))
        .route("/label", post(purchase_label))
        .with_state(state)
}

async fn fetch_rates(
    Extension(_claims): Extension<Claims>,
    Json(payload): Json<FetchRatesRequest>,
) -> impl IntoResponse {
    let weight: f64 = payload.weight.parse().unwrap_or(16.0);

    let client = crate::integrations::shippo::provider::ShippoProvider::new("dummy_token".to_string());
    let _ = client.fetch_rates(weight, &payload.dimensions).await;

    let mock_rates = vec![
        Rate { id: "rate_usps_1".to_string(), carrier: "USPS".to_string(), service: "Priority Mail".to_string(), amount: "8.50".to_string(), days: 2 },
        Rate { id: "rate_usps_2".to_string(), carrier: "USPS".to_string(), service: "First-Class Mail".to_string(), amount: "4.20".to_string(), days: 4 },
        Rate { id: "rate_ups_1".to_string(), carrier: "UPS".to_string(), service: "Ground".to_string(), amount: "9.75".to_string(), days: 3 },
    ];

    (StatusCode::OK, Json(FetchRatesResponse { rates: mock_rates })).into_response()
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

    let client = crate::integrations::shippo::provider::ShippoProvider::new("dummy_token".to_string());
    let label_url = client.purchase_label(&payload.rate_id).await.unwrap_or("https://api.goshippo.com/v1/mock_label.pdf".to_string());

    let carrier = if payload.rate_id.contains("ups") { "UPS".to_string() } else { "USPS".to_string() };
    let rand_num = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().subsec_nanos() % 1000;
    let tracking_number = format!("1Z999999999999999{}", rand_num);

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
        label_url,
        tracking_number,
        carrier,
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
            "ReadyForPickup" | "DriverRequested" => {
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
                    // Simulate ops agent printing label and shipping
                    if order.fulfillment_mode == "Shipping" {
                        order.status = "Shipped".to_string();
                    }
                }
                "mark_ready" => {
                    // Simulate ops agent dispatching courier / notifying customer
                    if order.fulfillment_mode == "LocalDelivery" || order.fulfillment_mode == "Pickup" {
                        order.status = "ReadyForPickup".to_string();
                    }
                }
                "request_driver" => {
                    if order.fulfillment_mode == "LocalDelivery" {
                        // Mocking driver dispatch via DoorDash Drive
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
