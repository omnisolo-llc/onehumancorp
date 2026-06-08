use axum::{
    extract::{Extension, State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;

#[derive(Deserialize)]
pub struct ClientIntakeRequest {
    pub message: String,
    pub service_name: Option<String>,
    pub estimated_price: Option<f64>,
}

#[derive(Serialize)]
pub struct ClientIntakeResponse {
    pub success: bool,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(process_client_intake))
        .with_state(orchestrator)
}

#[derive(Deserialize)]
pub struct IntakeQueryParams {
    pub tenant_id: String,
}

async fn process_client_intake(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    axum::extract::Query(query): axum::extract::Query<IntakeQueryParams>,
    Json(payload): Json<ClientIntakeRequest>,
) -> impl IntoResponse {
    let tenant_id = query.tenant_id.clone();

    let event = DepartmentEvent {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id,
        event_type: "tenant.intake.received".to_string(),
        payload: serde_json::json!({
            "message": payload.message,
            "service_name": payload.service_name,
            "estimated_price": payload.estimated_price,
        }),
    };

    match orchestrator.dispatch_event(event).await {
        Ok(_) => (StatusCode::OK, Json(ClientIntakeResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to dispatch client intake event: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ClientIntakeResponse { success: false })).into_response()
        }
    }
}
