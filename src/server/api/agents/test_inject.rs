use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::Deserialize;
use crate::orchestration::departments::orchestrator::{DepartmentOrchestrator, ActionRisk};
use crate::orchestration::departments::types::DepartmentType;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct InjectRequest {
    pub tenant_id: String,
    pub description: String,
    pub department: String,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(inject_approval))
        .with_state(orchestrator)
}

async fn inject_approval(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Json(payload): Json<InjectRequest>,
) -> impl IntoResponse {
    let dept = match payload.department.as_str() {
        "sales" | "Sales" => DepartmentType::Sales,
        "marketing" | "Marketing" => DepartmentType::Marketing,
        "finance" | "Finance" => DepartmentType::Finance,
        "legal" | "Legal" => DepartmentType::Legal,
        "business_advisory" => DepartmentType::BusinessAdvisory,
        "operations" | "Operations" => DepartmentType::Operations,
        _ => DepartmentType::CustomerSuccess,
    };

    match orchestrator.execute_action(
        dept,
        payload.description,
        payload.tenant_id,
        ActionRisk::DraftForReview,
        serde_json::json!({}),
    ).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
