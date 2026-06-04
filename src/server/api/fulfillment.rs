use axum::{
    extract::{Extension, Path, State},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use crate::integrations::doordash::client::DoorDashClient;
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

#[derive(Deserialize)]
pub struct QuoteRequest {
    pub delivery_address: String,
}

#[derive(Serialize)]
pub struct QuoteResponse {
    pub eligible: bool,
    pub fee: Option<f64>,
}

struct AppState {
    orders: RwLock<Vec<Order>>,
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
        .route("/quote", post(get_quote))
        .with_state(state)
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
                        let api_key = std::env::var("DOORDASH_API_KEY").unwrap_or_default();
                        let client = DoorDashClient::new(api_key);
                        let order_id = order.id.clone();
                        let pickup = "Store Address";
                        let dropoff = "Customer Address";

                        order.status = "DriverRequested".to_string();

                        tokio::spawn(async move {
                            if let Err(e) = client.dispatch_delivery(pickup, dropoff, &order_id).await {
                                tracing::error!("Failed to dispatch DoorDash driver: {}", e);
                            }
                        });
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

async fn get_quote(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<QuoteRequest>,
) -> impl IntoResponse {
    let api_key = std::env::var("DOORDASH_API_KEY").unwrap_or_default();
    let client = DoorDashClient::new(api_key);

    match client.get_delivery_quote("Store Address", &payload.delivery_address).await {
        Ok(quote) => {
            (StatusCode::OK, Json(QuoteResponse { eligible: true, fee: Some(quote.fee) })).into_response()
        }
        Err(_) => {
            (StatusCode::OK, Json(QuoteResponse { eligible: false, fee: None })).into_response()
        }
    }
}
