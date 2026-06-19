use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;

#[derive(Clone)]
pub struct FieldOpsState {
    pub pool: PgPool,
}

#[derive(Serialize, Deserialize)]
pub struct Appointment {
    pub id: String,
    pub customer_id: String,
    pub customer_name: String,
    pub job_template_id: String,
    pub job_name: String,
    pub status: String,
    pub scheduled_start_time: Option<DateTime<Utc>>,
    pub scheduled_end_time: Option<DateTime<Utc>>,
    pub location_address: Option<String>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct GetAppointmentsQuery {
    pub tenant_id: String,
}

#[derive(Serialize)]
pub struct GetAppointmentsResponse {
    pub appointments: Vec<Appointment>,
}

pub async fn get_appointments(
    State(state): State<Arc<FieldOpsState>>,
    Query(query): Query<GetAppointmentsQuery>,
) -> Result<Json<GetAppointmentsResponse>, (axum::http::StatusCode, String)> {
    let rows = sqlx::query(
        r#"
        SELECT
            a.id,
            a.customer_id,
            c.name as customer_name,
            a.job_template_id,
            jt.name as job_name,
            a.status,
            a.scheduled_start_time,
            a.scheduled_end_time,
            a.location_address,
            a.notes
        FROM appointments a
        LEFT JOIN customers c ON a.customer_id = c.id
        LEFT JOIN job_templates jt ON a.job_template_id = jt.id
        WHERE a.tenant_id = $1
        ORDER BY a.scheduled_start_time ASC
        "#,
    )
    .bind(&query.tenant_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )
    })?;

    let mut appointments = Vec::new();
    for row in rows {
        appointments.push(Appointment {
            id: row.get("id"),
            customer_id: row.get("customer_id"),
            customer_name: row.get::<Option<String>, _>("customer_name").unwrap_or_default(),
            job_template_id: row.get("job_template_id"),
            job_name: row.get::<Option<String>, _>("job_name").unwrap_or_default(),
            status: row.get("status"),
            scheduled_start_time: row.get("scheduled_start_time"),
            scheduled_end_time: row.get("scheduled_end_time"),
            location_address: row.get("location_address"),
            notes: row.get("notes"),
        });
    }

    Ok(Json(GetAppointmentsResponse { appointments }))
}

#[derive(Deserialize)]
pub struct UpdateAppointmentRequest {
    pub id: String,
    pub status: String,
    pub notes: Option<String>,
    pub scheduled_start_time: Option<DateTime<Utc>>,
    pub scheduled_end_time: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct UpdateAppointmentResponse {
    pub success: bool,
    pub id: String,
    pub status: String,
    pub notes: Option<String>,
    pub scheduled_start_time: Option<DateTime<Utc>>,
    pub scheduled_end_time: Option<DateTime<Utc>>,
}

pub async fn update_appointment(
    State(state): State<Arc<FieldOpsState>>,
    Json(payload): Json<UpdateAppointmentRequest>,
) -> Result<Json<UpdateAppointmentResponse>, (axum::http::StatusCode, String)> {
    sqlx::query(
        r#"
        UPDATE appointments
        SET status = $1,
            notes = COALESCE($2, notes),
            scheduled_start_time = COALESCE($4, scheduled_start_time),
            scheduled_end_time = COALESCE($5, scheduled_end_time),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $3
        "#,
    )
    .bind(&payload.status)
    .bind(&payload.notes)
    .bind(&payload.id)
    .bind(&payload.scheduled_start_time)
    .bind(&payload.scheduled_end_time)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )
    })?;

    Ok(Json(UpdateAppointmentResponse {
        success: true,
        id: payload.id,
        status: payload.status,
        notes: payload.notes,
        scheduled_start_time: payload.scheduled_start_time,
        scheduled_end_time: payload.scheduled_end_time,
    }))
}

pub fn router<S: Clone + Send + Sync + 'static>(pool: PgPool) -> Router<S> {
    let state = Arc::new(FieldOpsState { pool });
    Router::new()
        .route("/appointments", get(get_appointments).post(update_appointment))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_appointments_empty() {
        // Use connect_lazy so it doesn't fail immediately, then it hits the query and fails
        let pool = sqlx::PgPool::connect_lazy("postgres://invalid:invalid@localhost/invalid").unwrap();
        let app = router(pool);
        let req = Request::builder()
            .uri("/appointments?tenant_id=t1")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_update_appointment_invalid_db() {
        let pool = sqlx::PgPool::connect_lazy("postgres://invalid:invalid@localhost/invalid").unwrap();
        let app = router(pool);

        let body = serde_json::json!({
            "id": "apt_123",
            "status": "In-Progress",
            "notes": "Test notes",
            "scheduled_start_time": "2023-10-27T10:00:00Z",
            "scheduled_end_time": "2023-10-27T11:00:00Z"
        });

        let req = Request::builder()
            .method("POST")
            .uri("/appointments")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&body).unwrap()))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}
