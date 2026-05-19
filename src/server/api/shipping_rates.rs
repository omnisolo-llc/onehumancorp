use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::Row;

#[derive(Clone)]
pub struct ShippingRatesState {
    pub db: Arc<crate::db::DB>,
}

#[derive(Deserialize)]
pub struct GetRatesRequest {
    pub address_to: String,
    pub address_from: String,
    pub parcel_details: String,
}

#[derive(Serialize)]
pub struct GetRatesResponse {
    pub shipment_id: String,
}

pub async fn get_rates_handler(
    State(state): State<ShippingRatesState>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    let tenant_id = match request.extensions().get::<crate::auth::AuthInfo>() {
        Some(auth) => auth.tenant_id.clone(),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let body_bytes = axum::body::to_bytes(request.into_body(), usize::MAX).await.unwrap_or_default();
    let payload: GetRatesRequest = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid payload").into_response(),
    };

    let api_key: Option<String> = match &state.db.store {
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("SELECT shippo_api_key FROM tenants WHERE tenant_id = ?")
                .bind(&tenant_id)
                .fetch_optional(pool)
                .await
                .ok()
                .flatten()
                .map(|r| r.get("shippo_api_key"))
        }
        crate::db::DbStore::Postgres => {
            sqlx::query("SELECT shippo_api_key FROM tenants WHERE tenant_id = $1")
                .bind(&tenant_id)
                .fetch_optional(&state.db.pool)
                .await
                .ok()
                .flatten()
                .map(|r| r.get("shippo_api_key"))
        }
    };

    let api_key = match api_key {
        Some(k) => k,
        None => return (StatusCode::BAD_REQUEST, "Shippo API key not configured").into_response(),
    };

    use crate::integrations::shippo::client::ShippoClientWrapper;
    let client = crate::integrations::shippo::client::RealShippoClient::new(api_key);

    match client.create_shipment(&payload.address_to, &payload.address_from, &payload.parcel_details).await {
        Ok(shipment_id) => (StatusCode::OK, Json(GetRatesResponse { shipment_id })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get shipping rates").into_response(),
    }
}
