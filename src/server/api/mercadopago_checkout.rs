use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::Row;

#[derive(Clone)]
pub struct MercadoPagoCheckoutState {
    pub db: Arc<crate::db::DB>,
}

#[derive(Deserialize)]
pub struct CreateCheckoutRequest {
    pub price_id: String,
}

#[derive(Serialize)]
pub struct CreateCheckoutResponse {
    pub url: String,
}

pub async fn create_checkout_handler(
    State(state): State<MercadoPagoCheckoutState>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    let tenant_id = match request.extensions().get::<crate::auth::AuthInfo>() {
        Some(auth) => auth.org_id.clone(),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let body_bytes = axum::body::to_bytes(request.into_body(), usize::MAX).await.unwrap_or_default();
    let payload: CreateCheckoutRequest = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid payload").into_response(),
    };

    let token: Option<String> = match &state.db.store {
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("SELECT mercadopago_token FROM tenants WHERE tenant_id = ?")
                .bind(&tenant_id)
                .fetch_optional(pool)
                .await
                .ok()
                .flatten()
                .map(|r| r.get("mercadopago_token"))
        }
        crate::db::DbStore::Postgres => {
            sqlx::query("SELECT mercadopago_token FROM tenants WHERE tenant_id = $1")
                .bind(&tenant_id)
                .fetch_optional(&state.db.pool)
                .await
                .ok()
                .flatten()
                .map(|r| r.get("mercadopago_token"))
        }
    };

    let token = match token {
        Some(t) => t,
        None => return (StatusCode::BAD_REQUEST, "Mercado Pago not connected").into_response(),
    };

    let mp_client = crate::integrations::mercadopago::client::MercadoPagoClient::new(token);

    match mp_client.create_checkout_preference(&payload.price_id, &tenant_id).await {
        Ok(url) => (StatusCode::OK, Json(CreateCheckoutResponse { url })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create checkout preference").into_response(),
    }
}
