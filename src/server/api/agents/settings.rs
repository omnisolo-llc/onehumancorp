use axum::{
    extract::{Extension, State, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
    Json,
};
use std::sync::Arc;
use serde::Serialize;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentConfig;
use ::server_common::Claims;

#[derive(Serialize)]
pub struct SettingResponse {
    pub success: bool,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/{department}", post(update_setting))
        .with_state(orchestrator)
}

async fn update_setting(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Path(department): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<DepartmentConfig>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(SettingResponse { success: false })).into_response(),
    };

    match orchestrator.update_department_config(&tenant_id, &department, payload).await {
        Ok(_) => (StatusCode::OK, Json(SettingResponse { success: true })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(SettingResponse { success: false })).into_response(),
    }
}
