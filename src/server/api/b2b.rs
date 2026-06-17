use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::repository::models::{IntakeRequest, Quote, QuoteLineItem};

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/intake", post(create_intake_request))
        .route("/intake/:id", get(get_intake_request))
        .route("/intake", get(list_intake_requests))
}

#[derive(Deserialize)]
pub struct CreateIntakeRequestPayload {
    pub tenant_id: String,
    pub customer_id: Option<uuid::Uuid>,
    pub client_name: String,
    pub client_email: String,
    pub client_company: Option<String>,
    pub description: String,
    pub budget_cents: Option<i64>,
}

async fn create_intake_request(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateIntakeRequestPayload>,
) -> impl IntoResponse {
    let request_id = Uuid::new_v4();

    let res = sqlx::query(
        "INSERT INTO intake_requests (id, tenant_id, customer_id, client_name, client_email, client_company, description, budget_cents, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'NEW', NOW(), NOW())"
    )
    .bind(request_id.to_string())
    .bind(&payload.tenant_id)
    .bind(payload.customer_id)
    .bind(&payload.client_name)
    .bind(&payload.client_email)
    .bind(&payload.client_company)
    .bind(&payload.description)
    .bind(payload.budget_cents)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => (StatusCode::CREATED, Json(serde_json::json!({"id": request_id.to_string()}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to insert intake request: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn get_intake_request(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let req_res = sqlx::query_as::<_, IntakeRequest>("SELECT * FROM intake_requests WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await;

    match req_res {
        Ok(Some(req)) => (StatusCode::OK, Json(req)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch intake request: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn list_intake_requests(
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let reqs_res = sqlx::query_as::<_, IntakeRequest>("SELECT * FROM intake_requests ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await;

    match reqs_res {
        Ok(reqs) => (StatusCode::OK, Json(reqs)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list intake requests: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
