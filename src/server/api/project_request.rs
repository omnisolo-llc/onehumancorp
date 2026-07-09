use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use std::sync::Arc;
use uuid::Uuid;
use tracing::{error, instrument};

use crate::hub::Hub;
use ohc_builtin_agent::mesh::transport::MeshTransport;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct ProjectRequest {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub raw_intent: String,
    pub extracted_requirements: Option<String>,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequestDto {
    pub customer_id: String,
    pub raw_intent: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequestStatusDto {
    pub status: String,
}

pub fn router<S: Clone + Send + Sync + 'static>(pool: PgPool, hub: Arc<Hub>, msgbus: Arc<dyn MeshTransport>) -> Router<S> {
    Router::new()
        .route("/", get(list_project_requests).post(create_project_request))
        .route("/{id}", get(get_project_request).put(update_project_request))
        .with_state((pool, hub, msgbus))
}

type AppState = (PgPool, Arc<Hub>, Arc<dyn MeshTransport>);

#[instrument(skip(state))]
async fn list_project_requests(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<ProjectRequest>>, (StatusCode, String)> {
    let pool = &state.0;

    // Simplistic RLS handling simulation. In a real app we'd set current_setting.
    let tenant_id = params.get("tenant").cloned().unwrap_or_default();

    let reqs = sqlx::query_as::<_, ProjectRequest>(
        "SELECT * FROM project_requests WHERE tenant_id = $1 ORDER BY created_at DESC"
    )
    .bind(&tenant_id)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!("DB error: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to list project requests".to_string())
    })?;

    Ok(Json(reqs))
}

#[instrument(skip(state))]
async fn get_project_request(
    State(state): State<AppState>,
    Path(id): Path<String>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ProjectRequest>, (StatusCode, String)> {
    let pool = &state.0;
    let tenant_id = params.get("tenant").cloned().unwrap_or_default();

    let req = sqlx::query_as::<_, ProjectRequest>(
        "SELECT * FROM project_requests WHERE id = $1 AND tenant_id = $2"
    )
    .bind(&id)
    .bind(&tenant_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        error!("DB error: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get project request".to_string())
    })?
    .ok_or((StatusCode::NOT_FOUND, "Project request not found".to_string()))?;

    Ok(Json(req))
}

#[instrument(skip(state))]
async fn create_project_request(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
    Json(payload): Json<CreateProjectRequestDto>,
) -> Result<(StatusCode, Json<ProjectRequest>), (StatusCode, String)> {
    let (pool, _, msgbus) = state;
    let tenant_id = params.get("tenant").cloned().unwrap_or_default();

    let id = Uuid::new_v4().to_string();

    // Simulate simple AI extraction (in a real scenario, this would be a complex prompt)
    // The SalesAgent handles this via msgbus, but we might do an initial pass here
    let extracted = format!("Extracted: {}", payload.raw_intent);

    let req = sqlx::query_as::<_, ProjectRequest>(
        r#"
        INSERT INTO project_requests (id, tenant_id, customer_id, raw_intent, extracted_requirements, status)
        VALUES ($1, $2, $3, $4, $5, 'NEW')
        RETURNING *
        "#
    )
    .bind(&id)
    .bind(&tenant_id)
    .bind(&payload.customer_id)
    .bind(&payload.raw_intent)
    .bind(&extracted)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        error!("DB error: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create project request".to_string())
    })?;

    // Publish event for SalesAgent to draft a proposal
    let event_payload = serde_json::json!({
        "project_request_id": id,
        "tenant_id": tenant_id,
        "action": "draft_proposal"
    });

    if let Err(e) = msgbus.publish("agent_tasks", ohc_builtin_agent::mesh::transport::Message {
        agent_id: "api_gateway".into(),
        action: "draft_proposal".into(),
        status: "ok".into(),
        payload: event_payload.to_string().into_bytes(),
        msg_id: id.clone(),
    }).await {
        error!("Failed to publish project request event: {:?}", e);
    }

    Ok((StatusCode::CREATED, Json(req)))
}

#[instrument(skip(state))]
async fn update_project_request(
    State(state): State<AppState>,
    Path(id): Path<String>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
    Json(payload): Json<UpdateProjectRequestStatusDto>,
) -> Result<Json<ProjectRequest>, (StatusCode, String)> {
    let pool = &state.0;
    let tenant_id = params.get("tenant").cloned().unwrap_or_default();

    let req = sqlx::query_as::<_, ProjectRequest>(
        r#"
        UPDATE project_requests
        SET status = $1, updated_at = CURRENT_TIMESTAMP
        WHERE id = $2 AND tenant_id = $3
        RETURNING *
        "#
    )
    .bind(&payload.status)
    .bind(&id)
    .bind(&tenant_id)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        error!("DB error: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update project request".to_string())
    })?;

    Ok(Json(req))
}
