use axum::{
    extract::{Path, State},
    response::IntoResponse,
    http::StatusCode,
    routing::get,
    Router,
    Json,
};
use std::sync::Arc;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::Customer360;
use axum::extract::Extension;
use ::server_common::Claims;

#[derive(Clone)]
pub struct CustomersState {
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = CustomersState { orchestrator };
    Router::new()
        .route("/", get(list_customers))
        .route("/{id}", get(get_customer))
        .with_state(state)
}

async fn list_customers(
    State(state): State<CustomersState>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json::<Vec<Customer360>>(vec![])).into_response(),
    };

    match state.orchestrator.list_customer360(&tenant_id).await {
        Ok(customers) => (StatusCode::OK, Json(customers)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list customers: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<Customer360>>(vec![])).into_response()
        }
    }
}

async fn get_customer(
    State(state): State<CustomersState>,
    Extension(claims): Extension<Claims>,
    Path(customer_id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(None::<Customer360>)).into_response(),
    };

    match state.orchestrator.get_customer360(&tenant_id, &customer_id).await {
        Ok(Some(customer)) => (StatusCode::OK, Json(Some(customer))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(None::<Customer360>)).into_response(),
        Err(e) => {
            tracing::error!("Failed to get customer: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(None::<Customer360>)).into_response()
        }
    }
}
