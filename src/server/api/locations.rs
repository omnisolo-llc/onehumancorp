use sqlx::Row;
use axum::{
    extract::{State, Path, Json},
    routing::{get, post},
    Router,
    http::HeaderMap,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct CreateLocationRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateLocationResponse {
    pub id: String,
}

#[derive(Deserialize)]
pub struct CreateEscalationRequest {
    pub task_id: String,
    pub summary: String,
}

#[derive(Serialize)]
pub struct CreateEscalationResponse {
    pub id: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct LocationTasksResponse {
    pub task_id: String,
    pub title: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct EscalationResponse {
    pub id: String,
    pub task_id: String,
    pub summary: String,
    pub status: String,
}

fn get_tenant_id(headers: &HeaderMap) -> String {
    headers
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default")
        .to_string()
}

pub async fn create_location(
    headers: HeaderMap,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateLocationRequest>,
) -> axum::response::Result<Json<CreateLocationResponse>, axum::http::StatusCode> {
    let tenant_id = get_tenant_id(&headers);
    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Set RLS scope
    let _ = sqlx::query(&format!("SET LOCAL app.current_tenant = '{}'", tenant_id))
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = sqlx::query("INSERT INTO locations (tenant_id, name) VALUES ($1, $2) RETURNING id").bind(tenant_id).bind(payload.name)
        .fetch_one(&mut *tx)
        .await;

    let _ = tx.commit().await;

    match result {
        Ok(record) => Ok(Json(CreateLocationResponse {
            id: record.try_get::<uuid::Uuid, _>("id").unwrap_or_default().to_string(),
        })),
        Err(e) => {
            eprintln!("Failed to create location: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_escalation(
    headers: HeaderMap,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateEscalationRequest>,
) -> axum::response::Result<Json<CreateEscalationResponse>, axum::http::StatusCode> {
    let tenant_id = get_tenant_id(&headers);
    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = sqlx::query(&format!("SET LOCAL app.current_tenant = '{}'", tenant_id))
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = sqlx::query("INSERT INTO escalations (tenant_id, task_id, summary) VALUES ($1, $2, $3) RETURNING id").bind(tenant_id).bind(payload.task_id).bind(payload.summary)
        .fetch_one(&mut *tx)
        .await;

    let _ = tx.commit().await;

    match result {
        Ok(record) => Ok(Json(CreateEscalationResponse {
            id: record.try_get::<uuid::Uuid, _>("id").unwrap_or_default().to_string(),
        })),
        Err(e) => {
            eprintln!("Failed to create escalation: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn list_tasks_by_location(
    headers: HeaderMap,
    State(pool): State<PgPool>,
    Path(location_id): Path<String>,
) -> axum::response::Result<Json<Vec<LocationTasksResponse>>, axum::http::StatusCode> {
    let tenant_id = get_tenant_id(&headers);
    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = sqlx::query(&format!("SET LOCAL app.current_tenant = '{}'", tenant_id))
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let result: Result<Vec<_>, sqlx::Error> = sqlx::query_as::<_, LocationTasksResponse>( "SELECT id as task_id, title, status FROM shared_tasks WHERE location_id = $1").bind(location_id)
        .fetch_all(&mut *tx)
        .await;

    let _ = tx.commit().await;

    match result {
        Ok(records) => Ok(Json(records)),
        Err(e) => {
            eprintln!("Failed to list tasks: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn list_escalations(
    headers: HeaderMap,
    State(pool): State<PgPool>,
) -> axum::response::Result<Json<Vec<EscalationResponse>>, axum::http::StatusCode> {
    let tenant_id = get_tenant_id(&headers);
    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = sqlx::query(&format!("SET LOCAL app.current_tenant = '{}'", tenant_id))
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let result: Result<Vec<_>, sqlx::Error> = sqlx::query_as::<_, EscalationResponse>( "SELECT id::text as id, task_id, summary, status FROM escalations WHERE tenant_id = $1").bind(tenant_id)
        .fetch_all(&mut *tx)
        .await;

    let _ = tx.commit().await;

    match result {
        Ok(records) => Ok(Json(records)),
        Err(e) => {
            eprintln!("Failed to list escalations: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/", post(create_location))
        .route("/{location_id}/tasks", get(list_tasks_by_location))
        .route("/escalations", post(create_escalation).get(list_escalations))
}
