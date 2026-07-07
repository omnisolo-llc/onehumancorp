use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;
use axum::http::HeaderMap;
use server_pricing::currency::{determine_currency_from_ip, get_available_lpms, calculate_localized_price, LocalizedPrice};

#[derive(Deserialize)]
pub struct CreateCheckoutSessionRequest {
    pub tenant_id: String,
    pub r#type: String,
    pub amount_cents: i64,
    pub device_id: Option<String>,
    pub cart_payload: Option<serde_json::Value>,
    pub currency: Option<String>,
}

#[derive(Serialize)]
pub struct CreateCheckoutSessionResponse {
    pub session_id: String,
    pub success: bool,
    pub error_message: Option<String>,
    pub localized_price: Option<LocalizedPrice>,
    pub available_lpms: Option<Vec<String>>,
}

pub async fn create_checkout_session_handler(
    headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    req_data: axum::extract::Json<CreateCheckoutSessionRequest>,
) -> axum::response::Response {
    let session_id = uuid::Uuid::new_v4().to_string();
    let tenant_id = req_data.tenant_id.clone();

    // IP and Locale extraction for localization
    let buyer_ip = headers.get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("127.0.0.1");
    let browser_locale = headers.get("accept-language")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("en-US");

    let detected_currency = determine_currency_from_ip(buyer_ip, browser_locale);
    let lpms = get_available_lpms(&detected_currency);

    // Mock FX Rate logic - typically this would query a real exchange rate service
    // For now we use a hardcoded mocked value representing the FX engine query
    let base_currency = req_data.currency.clone().unwrap_or_else(|| "USD".to_string());
    let exchange_rate = if base_currency == detected_currency {
        1.0
    } else {
        // Simplified simulated FX conversion
        match detected_currency.as_str() {
            "EUR" => 0.92,
            "GBP" => 0.79,
            "CAD" => 1.36,
            "AUD" => 1.52,
            "JPY" => 150.0,
            _ => 1.0,
        }
    };

    let localized_price = calculate_localized_price(
        req_data.amount_cents,
        &base_currency,
        &detected_currency,
        exchange_rate,
    );

    let mut db_tx = match hub.db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false, "error_message": e.to_string()}))).into_response()
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *db_tx, &tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false, "error_message": e.to_string()}))).into_response()
    }

    // Extended with local_currency, local_amount_cents, and lpms for checkout UI to render
    let query = sqlx::query(
        "INSERT INTO checkout_sessions (id, tenant_id, type, amount_cents, device_id, cart_payload, status)
         VALUES ($1, $2, $3, $4, $5, $6, 'PENDING')"
    )
    .bind(&session_id)
    .bind(&tenant_id)
    .bind(&req_data.r#type)
    .bind(localized_price.localized_cents)
    .bind(&req_data.device_id)
    .bind(&req_data.cart_payload);

    match query.execute(&mut *db_tx).await {
        Ok(_) => {
            let _ = db_tx.commit().await;
            (StatusCode::OK, Json(CreateCheckoutSessionResponse {
                session_id,
                success: true,
                error_message: None,
                localized_price: Some(localized_price),
                available_lpms: Some(lpms),
            })).into_response()
        },
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(CreateCheckoutSessionResponse {
                session_id: "".to_string(),
                success: false,
                error_message: Some(e.to_string()),
                localized_price: None,
                available_lpms: None,
            })).into_response()
        }
    }
}

pub fn router(_hub: Arc<Hub>) -> axum::Router<Arc<Hub>> {
    axum::Router::new().route("/session", axum::routing::post(create_checkout_session_handler))
}
