use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::repository::models::ServiceRequest;
use ::server_common::Claims;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/", post(create_service_request))
        .route("/{id}", get(get_service_request))
        .route("/{id}", put(update_service_request))
}

#[derive(Deserialize)]
pub struct CreateServiceRequest {
    pub customer_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub urgency: Option<String>,
    pub location: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateServiceRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub urgency: Option<String>,
    pub location: Option<String>,
    pub status: Option<String>,
}

async fn create_service_request(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateServiceRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let sr_id = Uuid::new_v4().to_string();

    let res = sqlx::query(
        "INSERT INTO service_requests (id, tenant_id, customer_id, title, description, urgency, location, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, 'open', NOW(), NOW())"
    )
    .bind(&sr_id)
    .bind(tenant_id)
    .bind(payload.customer_id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.urgency)
    .bind(&payload.location)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => (StatusCode::CREATED, Json(serde_json::json!({"id": sr_id}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to create service request: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn get_service_request(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let res = sqlx::query_as::<_, ServiceRequest>("SELECT * FROM service_requests WHERE id = $1 AND tenant_id = $2")
        .bind(&id)
        .bind(tenant_id)
        .fetch_optional(&pool)
        .await;

    match res {
        Ok(Some(sr)) => (StatusCode::OK, Json(sr)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch service request: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn update_service_request(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateServiceRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let res = sqlx::query(
        "UPDATE service_requests SET
         title = COALESCE($1, title),
         description = COALESCE($2, description),
         urgency = COALESCE($3, urgency),
         location = COALESCE($4, location),
         status = COALESCE($5, status),
         updated_at = NOW()
         WHERE id = $6 AND tenant_id = $7"
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.urgency)
    .bind(&payload.location)
    .bind(&payload.status)
    .bind(&id)
    .bind(tenant_id)
    .execute(&pool)
    .await;

    match res {
        Ok(result) if result.rows_affected() > 0 => (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response(),
        Ok(_) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to update service request: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
