use axum::extract;
use axum::{
    extract::{State, Query, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::db::DB;
use crate::common::auth_utils::{UiTenantQuery, ui_tenant_id};
use crate::domain::repository::ucal_repo::UcalRepository;
use crate::domain::repository::models::UcalResource;

#[derive(Deserialize)]
pub struct CreateResourceRequest {
    pub name: String,
    pub resource_type: String,
    pub base_capacity: i32,
}

#[derive(Deserialize)]
pub struct LockCapacityRequest {
    pub resource_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub units: i32,
    pub status: String,
    pub reference_id: Option<String>,
}

#[derive(Deserialize)]
pub struct LedgerQuery {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/resources", get(get_resources).post(create_resource))
        .route("/ledger", get(get_ledger))
        .route("/lock", post(lock_capacity))
        .with_state(db)
}

async fn create_resource(
    State(db): State<Arc<DB>>,
    Query(query): Query<UiTenantQuery>,
    Json(payload): Json<CreateResourceRequest>,
) -> impl IntoResponse {
    let tenant_id = ui_tenant_id(&query);
    let repo = UcalRepository::new(db);

    let resource = UcalResource {
        id: Uuid::new_v4(),
        tenant_id: tenant_id.clone(),
        name: payload.name,
        resource_type: payload.resource_type,
        base_capacity: payload.base_capacity,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    match repo.create_resource(resource).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn get_resources(
    State(db): State<Arc<DB>>,
    Query(query): Query<UiTenantQuery>,
) -> impl IntoResponse {
    let tenant_id = ui_tenant_id(&query);
    let repo = UcalRepository::new(db);

    match repo.get_resources(&tenant_id).await {
        Ok(res) => Json(res).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn lock_capacity(
    State(db): State<Arc<DB>>,
    Query(query): Query<UiTenantQuery>,
    Json(payload): Json<LockCapacityRequest>,
) -> impl IntoResponse {
    let tenant_id = ui_tenant_id(&query);
    let repo = UcalRepository::new(db);

    match repo.check_and_lock_capacity(
        &tenant_id,
        payload.resource_id,
        payload.start_time,
        payload.end_time,
        payload.units,
        &payload.status,
        payload.reference_id.as_deref(),
    ).await {
        Ok(ledger) => Json(ledger).into_response(),
        Err(e) => (StatusCode::CONFLICT, e).into_response(),
    }
}

async fn get_ledger(
    State(db): State<Arc<DB>>,
    Query(ui_query): Query<UiTenantQuery>,
    Query(ledger_query): Query<LedgerQuery>,
) -> impl IntoResponse {
    let tenant_id = ui_tenant_id(&ui_query);
    let repo = UcalRepository::new(db);

    match repo.get_ledger_entries(&tenant_id, ledger_query.start_time, ledger_query.end_time).await {
        Ok(entries) => Json(entries).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}
