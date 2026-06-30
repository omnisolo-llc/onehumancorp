use axum::{
    extract::{State, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::db::DB;
use crate::hub::Hub;
use ::server_ohc::orchestration::TeammateMeshEvent;
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;

#[derive(Serialize)]
pub struct JobLocation {
    pub id: String,
    pub customer_id: Option<String>,
    pub job_title: String,
    pub address: String,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub status: String,
    pub order_index: i32,
}

#[derive(Serialize)]
pub struct ServiceRoute {
    pub id: String,
    pub staff_id: Option<String>,
    pub route_date: NaiveDate,
    pub status: String,
    pub jobs: Vec<JobLocation>,
}

#[derive(Serialize)]
pub struct TodayRoutesResponse {
    pub routes: Vec<ServiceRoute>,
}

#[derive(Deserialize)]
pub struct UpdateJobStatusRequest {
    pub status: String,
}

#[derive(Serialize)]
pub struct UpdateJobStatusResponse {
    pub success: bool,
    pub error: Option<String>,
}

pub fn router<S>(db: Arc<DB>, hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = AppState { db, hub };
    Router::new()
        .route("/routes/today", get(get_today_routes))
        .route("/jobs/{id}/status", post(update_job_status))
        .with_state(state)
}

#[derive(Clone)]
struct AppState {
    db: Arc<DB>,
    hub: Arc<Hub>,
}

async fn get_today_routes(
    State(state): State<AppState>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
) -> impl IntoResponse {
    let tenant_id = auth_info.org_id;
    if tenant_id.is_empty() {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response();
    }

    let pool = state.db.pool.clone();

    let today = Utc::now().date_naive();

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("failed to begin tx: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response();
        }
    };

    let _ = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;

    use sqlx::Row;
    let routes_result = sqlx::query(
        r#"
        SELECT id, staff_profile_id as staff_id, route_date, status
        FROM service_routes
        WHERE tenant_id = $1 AND route_date = $2
        "#,
    )
    .bind(&tenant_id)
    .bind(today)
    .fetch_all(&mut *tx)
    .await;

    let mut routes = Vec::new();
    if let Ok(routes_rows) = routes_result {
        for r_row in routes_rows {
            let r_id: String = r_row.get("id");
            let jobs_result = sqlx::query(
                r#"
                SELECT
                    jl.id,
                    a.customer_id,
                    COALESCE(jt.name, 'Service Job') as job_title,
                    COALESCE(a.location_address, 'No Address Provided') as address,
                    a.location_lat as lat,
                    a.location_lng as lng,
                    COALESCE(a.scheduled_start_time, NOW()) as scheduled_start,
                    a.scheduled_end_time as scheduled_end,
                    jl.status,
                    jl.sequence_order as order_index
                FROM job_locations jl
                JOIN appointments a ON jl.appointment_id = a.id
                LEFT JOIN job_templates jt ON a.job_template_id = jt.id
                WHERE jl.tenant_id = $1 AND jl.service_route_id = $2
                ORDER BY jl.sequence_order ASC, a.scheduled_start_time ASC
                "#,
            )
            .bind(&tenant_id)
            .bind(&r_id)
            .fetch_all(&mut *tx)
            .await;

            let mut jobs = Vec::new();
            if let Ok(jobs_rows) = jobs_result {
                for j_row in jobs_rows {
                    jobs.push(JobLocation {
                        id: j_row.get("id"),
                        customer_id: j_row.try_get("customer_id").unwrap_or(None),
                        job_title: j_row.get("job_title"),
                        address: j_row.get("address"),
                        lat: j_row.try_get("lat").unwrap_or(None),
                        lng: j_row.try_get("lng").unwrap_or(None),
                        scheduled_start: j_row.get("scheduled_start"),
                        scheduled_end: j_row.try_get("scheduled_end").unwrap_or(None),
                        status: j_row.get("status"),
                        order_index: j_row.get("order_index"),
                    });
                }
            }

            routes.push(ServiceRoute {
                id: r_id,
                staff_id: r_row.try_get("staff_id").unwrap_or(None),
                route_date: r_row.get("route_date"),
                status: r_row.get("status"),
                jobs,
            });
        }
    }

    let _ = tx.commit().await;

    (StatusCode::OK, Json(TodayRoutesResponse { routes })).into_response()
}

async fn update_job_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<UpdateJobStatusRequest>,
) -> impl IntoResponse {
    let tenant_id = auth_info.org_id;
    if tenant_id.is_empty() {
        return (StatusCode::UNAUTHORIZED, Json(UpdateJobStatusResponse { success: false, error: Some("unauthorized".to_string()) })).into_response();
    }

    let pool = state.db.pool.clone();

    let valid_statuses = vec!["pending", "en_route", "on_site", "done", "cancelled"];
    if !valid_statuses.contains(&payload.status.as_str()) {
        return (StatusCode::BAD_REQUEST, Json(UpdateJobStatusResponse { success: false, error: Some("invalid status".to_string()) })).into_response();
    }

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("failed to begin tx: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(UpdateJobStatusResponse { success: false, error: Some("internal error".to_string()) })).into_response();
        }
    };

    let _ = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;

    let update_res = sqlx::query(
        r#"
        UPDATE job_locations
        SET status = $1, updated_at = CURRENT_TIMESTAMP
        WHERE id = $2 AND tenant_id = $3
        RETURNING id
        "#,
    )
    .bind(&payload.status)
    .bind(&id)
    .bind(&tenant_id)
    .fetch_optional(&mut *tx)
    .await;

    match update_res {
        Ok(Some(_)) => {
            let _ = tx.commit().await;

            // Broadcast TeammateMeshEvent
            let payload_json = serde_json::json!({
                "job_id": id,
                "status": payload.status,
            });
            let payload_bytes = serde_json::to_vec(&payload_json).unwrap_or_default();

            let event = TeammateMeshEvent {
                agent_id: "system".to_string(),
                action: "job_status_changed".to_string(),
                status: "ok".to_string(),
                payload: payload_bytes,
                msg_id: Uuid::new_v4().to_string(),
            };

            if let Err(e) = state.hub.publish_teammate_event("job_status_updates".to_string(), event) {
                tracing::warn!("Failed to publish mesh event for job status change: {}", e);
            }

            (StatusCode::OK, Json(UpdateJobStatusResponse { success: true, error: None })).into_response()
        },
        Ok(None) => (StatusCode::NOT_FOUND, Json(UpdateJobStatusResponse { success: false, error: Some("job not found".to_string()) })).into_response(),
        Err(e) => {
            tracing::error!("failed to update job status: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(UpdateJobStatusResponse { success: false, error: Some("failed to update job".to_string()) })).into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dummy() {
        assert!(true);
    }
}
