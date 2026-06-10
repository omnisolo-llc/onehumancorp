use axum::{
    routing::get,
    Json, Router,
};
use serde_json::{json, Value};

pub fn pos_routes() -> Router {
    Router::new()
        .route("/orders", get(get_orders))
        .route("/inventory", get(get_inventory))
}

async fn get_orders() -> Json<Value> {
    Json(json!({
        "orders": [
            {
                "id": "ord_1",
                "status": "pending",
                "items": [{"name": "Coffee", "quantity": 2}]
            }
        ]
    }))
}

async fn get_inventory() -> Json<Value> {
    Json(json!({
        "inventory": [
            {
                "id": "inv_1",
                "name": "Coffee Beans",
                "stock": 100
            }
        ]
    }))
}
