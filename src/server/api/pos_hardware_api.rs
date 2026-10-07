use crate::hub::Hub;
use axum::{Json, extract::State, response::IntoResponse};
use std::sync::Arc;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct HardwareDevice {
    pub id: String,
    pub tenant_id: String,
    pub stripe_reader_id: String,
    pub status: String,
    pub last_seen: Option<chrono::DateTime<chrono::Utc>>,
    pub battery_level: Option<i32>,
}

#[derive(serde::Deserialize)]
pub struct RegisterHardwareRequest {
    pub stripe_reader_id: String,
    pub status: Option<String>,
    pub battery_level: Option<i32>,
}

#[derive(serde::Serialize)]
pub struct RegisterHardwareResponse {
    pub id: String,
    pub success: bool,
}

pub async fn list_hardware_handler(
    State(hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
) -> axum::response::Response {
    let tenant_id = match auth_info {
        Some(auth)
            if !auth.org_id.trim().is_empty()
                && !auth.org_id.trim().eq_ignore_ascii_case("system") =>
        {
            auth.org_id.clone()
        }
        _ => return (
            axum::http::StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error":"Authentication required."})),
        ).into_response(),
    };

    let rows = match sqlx::query_as::<_, HardwareDevice>(
        "SELECT id, tenant_id, stripe_reader_id, status, last_seen, battery_level FROM pos_hardware_devices WHERE tenant_id = $1"
    )
    .bind(tenant_id)
    .fetch_all(&hub.pool)
    .await {
        Ok(rows) => rows,
        Err(e) => {
            tracing::error!("Failed to list pos_hardware_devices: {}", e);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to list POS hardware devices"})),
            ).into_response();
        }
    };

    Json(rows).into_response()
}

pub async fn register_hardware_handler(
    State(hub): State<Arc<Hub>>,
    auth_info: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    Json(req): Json<RegisterHardwareRequest>,
) -> axum::response::Response {
    let tenant_id = match auth_info {
        Some(auth)
            if !auth.org_id.trim().is_empty()
                && !auth.org_id.trim().eq_ignore_ascii_case("system") =>
        {
            auth.org_id.clone()
        }
        _ => return (
            axum::http::StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error":"Authentication required."})),
        ).into_response(),
    };

    let id = uuid::Uuid::new_v4().to_string();
    let status = req.status.unwrap_or_else(|| "ACTIVE".to_string());
    let battery = req.battery_level.unwrap_or(100);

    match sqlx::query_scalar::<_, String>(
        "INSERT INTO pos_hardware_devices (id, tenant_id, stripe_reader_id, status, battery_level) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (tenant_id, stripe_reader_id) DO UPDATE SET status = EXCLUDED.status, battery_level = EXCLUDED.battery_level, last_seen = CURRENT_TIMESTAMP RETURNING id"
    )
    .bind(id)
    .bind(tenant_id)
    .bind(req.stripe_reader_id)
    .bind(status)
    .bind(battery)
    .fetch_one(&hub.pool)
    .await {
        Ok(record) => {
            Json(RegisterHardwareResponse {
                id: record,
                success: true,
            }).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to register pos_hardware_devices: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to register POS hardware device"})),
            ).into_response()
        }
    }
}

pub fn router(
    hub: Arc<Hub>,
) -> axum::Router<Arc<dyn omnisolo_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route("/", axum::routing::get(list_hardware_handler))
        .route(
            "/register",
            axum::routing::post(register_hardware_handler),
        )
        .with_state(hub)
}
