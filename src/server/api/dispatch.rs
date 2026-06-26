use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use crate::api::field_ops::Appointment;

#[derive(Clone)]
pub struct DispatchState {
    pub pool: PgPool,
}

#[derive(Deserialize)]
pub struct GetRunSheetQuery {
    pub tenant_id: String,
    pub date: String,
}

#[derive(Serialize)]
pub struct RouteStop {
    pub id: String,
    pub service_route_id: String,
    pub appointment_id: Option<String>,
    pub sequence_order: i32,
    pub estimated_arrival: Option<DateTime<Utc>>,
    pub status: String,
    pub notes: Option<String>,
    pub appointment: Option<Appointment>,
}

#[derive(Serialize)]
pub struct ServiceRoute {
    pub id: String,
    pub date: String,
    pub status: String,
    pub stops: Vec<RouteStop>,
}

pub async fn get_run_sheet(
    State(state): State<Arc<DispatchState>>,
    Query(query): Query<GetRunSheetQuery>,
) -> Result<Json<ServiceRoute>, (axum::http::StatusCode, String)> {
    let route_row = sqlx::query(
        "SELECT id, date, status FROM service_routes WHERE tenant_id = $1 AND date = $2 LIMIT 1",
    )
    .bind(&query.tenant_id)
    .bind(&query.date)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(row) = route_row {
        let route_id: String = row.get("id");

        let stops_rows = sqlx::query(
            r#"
            SELECT
                rs.id, rs.service_route_id, rs.appointment_id, rs.sequence_order, rs.estimated_arrival, rs.status, rs.notes,
                a.id as a_id, a.customer_id as a_customer_id, a.job_template_id as a_job_template_id, a.status as a_status,
                a.scheduled_start_time as a_scheduled_start_time, a.scheduled_end_time as a_scheduled_end_time,
                a.location_address as a_location_address, a.location_lat as a_location_lat, a.location_lng as a_location_lng, a.notes as a_notes,
                c.name as a_customer_name, jt.name as a_job_name
            FROM route_stops rs
            LEFT JOIN appointments a ON rs.appointment_id = a.id
            LEFT JOIN customers c ON a.customer_id = c.id
            LEFT JOIN job_templates jt ON a.job_template_id = jt.id
            WHERE rs.service_route_id = $1
            ORDER BY rs.sequence_order ASC
            "#
        )
        .bind(&route_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let mut stops = Vec::new();
        for r in stops_rows {
            let appt_id: Option<String> = r.try_get("a_id").unwrap_or(None);
            let appointment = if let Some(id) = appt_id {
                Some(Appointment {
                    id,
                    customer_id: r.try_get("a_customer_id").unwrap_or_default(),
                    customer_name: r.try_get("a_customer_name").unwrap_or_default(),
                    job_template_id: r.try_get("a_job_template_id").unwrap_or_default(),
                    job_name: r.try_get("a_job_name").unwrap_or_default(),
                    status: r.try_get("a_status").unwrap_or_default(),
                    scheduled_start_time: r.try_get("a_scheduled_start_time").unwrap_or(None),
                    scheduled_end_time: r.try_get("a_scheduled_end_time").unwrap_or(None),
                    location_address: r.try_get("a_location_address").unwrap_or(None),
                    location_lat: r.try_get("a_location_lat").unwrap_or(None),
                    location_lng: r.try_get("a_location_lng").unwrap_or(None),
                    notes: r.try_get("a_notes").unwrap_or(None),
                })
            } else {
                None
            };

            stops.push(RouteStop {
                id: r.get("id"),
                service_route_id: r.get("service_route_id"),
                appointment_id: r.try_get("appointment_id").unwrap_or(None),
                sequence_order: r.get("sequence_order"),
                estimated_arrival: r.try_get("estimated_arrival").unwrap_or(None),
                status: r.get("status"),
                notes: r.try_get("notes").unwrap_or(None),
                appointment,
            });
        }

        Ok(Json(ServiceRoute {
            id: route_id,
            date: query.date,
            status: row.get("status"),
            stops,
        }))
    } else {
        // Return an empty schedule if none exists
        Ok(Json(ServiceRoute {
            id: uuid::Uuid::new_v4().to_string(),
            date: query.date,
            status: "Pending".to_string(),
            stops: vec![],
        }))
    }
}

#[derive(Deserialize)]
pub struct InjectJobRequest {
    pub tenant_id: String,
    pub date: String,
    pub appointment_id: String,
}

#[derive(Serialize)]
pub struct InjectJobResponse {
    pub success: bool,
    pub proposed_slot: Option<String>,
    pub impact: Option<String>,
}

pub async fn inject_job(
    State(state): State<Arc<DispatchState>>,
    Json(payload): Json<InjectJobRequest>,
) -> Result<Json<InjectJobResponse>, (axum::http::StatusCode, String)> {
    // A mock for the AI ops agent route engine calculating optimal insertion point
    Ok(Json(InjectJobResponse {
        success: true,
        proposed_slot: Some("1:00 PM".to_string()),
        impact: Some("+15m delay for PM jobs".to_string()),
    }))
}

pub fn router<S: Clone + Send + Sync + 'static>(pool: PgPool) -> Router<S> {
    let state = Arc::new(DispatchState { pool });
    Router::new()
        .route("/run-sheet", get(get_run_sheet))
        .route("/inject-job", post(inject_job))
        .with_state(state)
}
