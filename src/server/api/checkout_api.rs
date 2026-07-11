use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;
use axum::http::HeaderMap;

#[derive(Deserialize)]
pub struct CreateCheckoutSessionRequest {
    pub tenant_id: String,
    pub r#type: String,
    pub amount_cents: i64,
    pub device_id: Option<String>,
    pub cart_payload: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct CreateCheckoutSessionResponse {
    pub session_id: String,
    pub success: bool,
    pub error_message: Option<String>,
}

pub async fn create_checkout_session_handler(
    _headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    mut req_data: axum::extract::Json<CreateCheckoutSessionRequest>,
) -> axum::response::Response {
    if !auth_info.spiffe_id.is_empty() {
        req_data.tenant_id = auth_info.spiffe_id.clone();
    } else {
        return (axum::http::StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({"success": false, "error_message": "missing tenant identity in session"}))).into_response();
    }
    let session_id = uuid::Uuid::new_v4().to_string();
    let tenant_id = req_data.tenant_id.clone();

    let mut db_tx = match hub.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false, "error_message": e.to_string()}))).into_response()
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *db_tx, &tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false, "error_message": e.to_string()}))).into_response()
    }

    let query = sqlx::query(
        "INSERT INTO checkout_sessions (id, tenant_id, type, amount_cents, device_id, cart_payload, status)
         VALUES ($1, $2, $3, $4, $5, $6, 'PENDING')"
    )
    .bind(&session_id)
    .bind(&tenant_id)
    .bind(&req_data.r#type)
    .bind(req_data.amount_cents)
    .bind(&req_data.device_id)
    .bind(&req_data.cart_payload);

    match query.execute(&mut *db_tx).await {
        Ok(_) => {
            let _ = db_tx.commit().await;
            (StatusCode::OK, Json(CreateCheckoutSessionResponse {
                session_id,
                success: true,
                error_message: None,
            })).into_response()
        },
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(CreateCheckoutSessionResponse {
                session_id: "".to_string(),
                success: false,
                error_message: Some(e.to_string()),
            })).into_response()
        }
    }
}

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new().route("/session", axum::routing::post(create_checkout_session_handler)).with_state(hub)
}
