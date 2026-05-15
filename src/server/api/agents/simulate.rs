use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;
use ::server_ohc::orchestration::MeshEvent;

#[derive(Deserialize)]
pub struct ConfigRequest {
    pub department: String,
    pub mode: String,
}

#[derive(Serialize)]
pub struct ConfigResponse {
    pub success: bool,
}

pub async fn simulate_order(
    State(hub): State<Arc<Hub>>,
) -> impl IntoResponse {
    let msg = MeshEvent {
        event_id: uuid::Uuid::new_v4().to_string(),
        topic: "system:order_received".to_string(),
        payload: "order123".as_bytes().to_vec(),
        timestamp: chrono::Utc::now().timestamp(),
    };

    let _ = hub.publish_mesh_event(msg);

    (StatusCode::OK, Json(ConfigResponse { success: true })).into_response()
}

pub async fn save_config(
    State(hub): State<Arc<Hub>>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    Json(payload): Json<ConfigRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(ConfigResponse { success: false })).into_response(),
    };

    let tenant_uuid = uuid::Uuid::parse_str(&tenant_id).unwrap_or_default();

    // We assume the pool sets `app.current_tenant` in `before_acquire`
    // or we just bind the uuid. The RLS relies on `app.current_tenant`.
    // In our codebase, the pool usually does this, or we do it explicitly.
    let pool = &hub.pool;
    let query = "
        INSERT INTO agent_department_config (tenant_id, department, mode)
        VALUES ($1, $2, $3)
        ON CONFLICT (tenant_id, department)
        DO UPDATE SET mode = EXCLUDED.mode, updated_at = NOW();
    ";

    let res = sqlx::query(query)
        .bind(tenant_uuid)
        .bind(payload.department)
        .bind(payload.mode)
        .execute(pool)
        .await;

    match res {
        Ok(_) => (StatusCode::OK, Json(ConfigResponse { success: true })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ConfigResponse { success: false })).into_response(),
    }
}
