use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UpsellRequest {
    pub cart_items: Vec<String>,
}

#[derive(Serialize)]
pub struct UpsellProduct {
    pub id: String,
    pub name: String,
    pub price: String,
    pub image: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct UpsellResponse {
    pub recommendations: Vec<UpsellProduct>,
}

async fn handle_upsell_recommend(
    Json(payload): Json<UpsellRequest>,
) -> impl IntoResponse {
    // Basic heuristic simulation for the Upsell Engine
    let mut recommendations = Vec::new();

    // Always recommend something to ensure the feature works in UI,
    // unless the cart is totally empty.
    if !payload.cart_items.is_empty() {
        recommendations.push(UpsellProduct {
            id: "upsell_1".to_string(),
            name: "Premium Matches".to_string(),
            price: "5.00".to_string(),
            image: "🔥".to_string(),
            description: "Perfect pair for your items".to_string(),
        });
        recommendations.push(UpsellProduct {
            id: "upsell_2".to_string(),
            name: "Gift Wrapping".to_string(),
            price: "3.50".to_string(),
            image: "🎁".to_string(),
            description: "Make it special".to_string(),
        });
    }

    Json(UpsellResponse { recommendations })
}

use std::sync::Arc;
use ohc_builtin_agent::mesh::transport::MeshTransport;

pub fn router() -> Router<Arc<dyn MeshTransport>> {
    Router::new().route("/recommend", post(handle_upsell_recommend))
}
