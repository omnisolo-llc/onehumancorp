use axum::{
    extract::Extension,
    extract::{State, Json, Path},
    http::StatusCode,
    routing::{post, get},
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::DB;
use crate::services::customer_memory_graph::service::{CustomerMemoryGraphService, CustomerProfileSummary};

#[derive(Clone)]
pub struct CustomerMemoryState {
    pub db: Arc<DB>,
}

#[derive(Deserialize)]
pub struct IngestEventPayload {
    pub tenant_id: String,
    pub customer_id: String,
    pub channel: String,
    pub raw_content: String,
}

#[derive(Serialize)]
pub struct IngestEventResponse {
    pub event_id: Uuid,
}

pub async fn ingest_event(
    State(state): State<CustomerMemoryState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<IngestEventPayload>,
) -> Result<Json<IngestEventResponse>, StatusCode> {
    if claims.organization_id.as_deref() != Some(&payload.tenant_id) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let service = CustomerMemoryGraphService::new(state.db.pool.clone());

    match service.ingest_interaction(&payload.tenant_id, &payload.customer_id, &payload.channel, &payload.raw_content).await {
        Ok(event_id) => Ok(Json(IngestEventResponse { event_id })),
        Err(e) => {
            tracing::error!("Failed to ingest event: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_profile_summary(
    State(state): State<CustomerMemoryState>,
    Extension(claims): Extension<::server_common::Claims>,
    Path((tenant_id, customer_id)): Path<(String, String)>,
) -> Result<Json<CustomerProfileSummary>, StatusCode> {
    if claims.organization_id.as_deref() != Some(&tenant_id) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let service = CustomerMemoryGraphService::new(state.db.pool.clone());

    match service.get_profile_summary(&tenant_id, &customer_id).await {
        Ok(summary) => Ok(Json(summary)),
        Err(e) => {
            tracing::error!("Failed to fetch profile summary: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Admin endpoint to trigger processing (in reality this would be a background worker)
pub async fn trigger_processing(
    State(state): State<CustomerMemoryState>,
) -> Result<StatusCode, StatusCode> {
    let service = CustomerMemoryGraphService::new(state.db.pool.clone());

    match service.process_pending_jobs().await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            tracing::error!("Failed to process jobs: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn router(db: Arc<DB>) -> Router {
    let state = CustomerMemoryState { db };
    Router::new()
        .route("/ingest", post(ingest_event))
        .route("/process", post(trigger_processing))
        .route("/summary/{tenant_id}/{customer_id}", get(get_profile_summary))
        .with_state(state)
}
