use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;

#[derive(Serialize, Deserialize)]
pub struct GlobalCommerceSettings {
    pub base_currency: String,
    pub enabled_currencies: Vec<String>,
}

pub async fn get_settings(
    State(hub): State<Arc<Hub>>,
    request: axum::extract::Request,
) -> axum::response::Response {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) if !auth.org_id.is_empty() => auth.org_id.clone(),
        Some(_) => "default".to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let pool = &hub.pool;
    let row = sqlx::query("SELECT base_currency, enabled_currencies FROM tenants WHERE id = $1").bind(&tenant_id)
        .fetch_optional(pool)
        .await;

    match row {
        Ok(Some(record)) => {
            use sqlx::Row;
            let base_currency: Option<String> = record.try_get("base_currency").ok();
            let enabled_currencies: Option<serde_json::Value> = record.try_get("enabled_currencies").ok();
            let enabled = enabled_currencies
                .and_then(|v| serde_json::from_value(v).ok())
                .unwrap_or_else(|| vec!["USD".to_string()]);

            let settings = GlobalCommerceSettings {
                base_currency: base_currency.unwrap_or_else(|| "USD".to_string()),
                enabled_currencies: enabled,
            };
            (StatusCode::OK, Json(serde_json::json!({ "tenant": settings }))).into_response()
        },
        Ok(None) => (StatusCode::NOT_FOUND, "Tenant not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn update_settings(
    State(hub): State<Arc<Hub>>,
    request: axum::extract::Request,
) -> axum::response::Response {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) if !auth.org_id.is_empty() => auth.org_id.clone(),
        Some(_) => "default".to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let body_bytes = axum::body::to_bytes(request.into_body(), 1024 * 64).await.unwrap();
    let payload: GlobalCommerceSettings = serde_json::from_slice(&body_bytes).unwrap();

    let pool = &hub.pool;
    let enabled_json = serde_json::to_value(&payload.enabled_currencies).unwrap_or(serde_json::json!(["USD"]));

    let res = sqlx::query("UPDATE tenants SET base_currency = $1, enabled_currencies = $2 WHERE id = $3")
    .bind(payload.base_currency)
    .bind(enabled_json)
    .bind(tenant_id)
    .execute(pool)
    .await;

    match res {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({ "success": true }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route("/", axum::routing::get(get_settings).put(update_settings))
        .with_state(hub)
}
