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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct GetAppointmentsQuery {
    pub tenant_id: String,
    pub mobile_optimized: Option<bool>,
}

#[derive(Serialize)]
pub struct GetAppointmentsResponse {
    pub appointments: Vec<Appointment>,
}

pub async fn get_appointments(
    State(state): State<Arc<FieldOpsState>>,
    Query(query): Query<GetAppointmentsQuery>,
) -> Result<Json<GetAppointmentsResponse>, (axum::http::StatusCode, String)> {



    let query_str = if query.mobile_optimized.unwrap_or(false) {
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
            NULL as location_address,
            NULL as notes
        FROM appointments a
        LEFT JOIN customers c ON a.customer_id = c.id
        LEFT JOIN job_templates jt ON a.job_template_id = jt.id
        WHERE a.tenant_id = $1
        ORDER BY a.scheduled_start_time ASC
"#
    } else {
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
"#
    };

    let rows = sqlx::query(query_str)
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
            location_address: row.try_get("location_address").unwrap_or(None),
            notes: row.try_get("notes").unwrap_or(None),
        });
    }

    Ok(Json(GetAppointmentsResponse { appointments }))
}

#[derive(Deserialize)]
pub struct UpdateAppointmentRequest {
    pub id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Serialize)]
pub struct UpdateAppointmentResponse {
    pub success: bool,
    pub id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

pub async fn update_appointment(
    State(state): State<Arc<FieldOpsState>>,
    Json(payload): Json<UpdateAppointmentRequest>,
) -> Result<Json<UpdateAppointmentResponse>, (axum::http::StatusCode, String)> {
    sqlx::query(
        r#"
        UPDATE appointments
        SET status = $1, notes = COALESCE($2, notes), updated_at = CURRENT_TIMESTAMP
        WHERE id = $3
        "#,
    )
    .bind(&payload.status)
    .bind(&payload.notes)
    .bind(&payload.id)
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
    }))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunningLateRequest {
    pub delay_job_id: String,
    pub appointments: Vec<Appointment>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunningLateResponse {
    pub success: bool,
    pub subsequent_count: usize,
    pub agent_suggestion: Option<String>,
    pub optimized_route: Vec<Appointment>,
}

pub fn calculate_delay_and_shift(mut appointments: Vec<Appointment>, delay_job_id: &str) -> (Vec<Appointment>, usize) {
    let mut delay_idx = None;
    for (i, appt) in appointments.iter().enumerate() {
        if appt.id == delay_job_id {
            delay_idx = Some(i);
            break;
        }
    }

    let mut subsequent_count = 0;
    if let Some(idx) = delay_idx {
        for appt in appointments.iter_mut().skip(idx + 1) {
            subsequent_count += 1;
            if let Some(st) = appt.scheduled_start_time {
                appt.scheduled_start_time = Some(st + chrono::Duration::minutes(15));
            }
            if let Some(et) = appt.scheduled_end_time {
                appt.scheduled_end_time = Some(et + chrono::Duration::minutes(15));
            }
        }
    }

    (appointments, subsequent_count)
}

pub async fn running_late(
    State(_state): State<Arc<FieldOpsState>>,
    Json(payload): Json<RunningLateRequest>,
) -> Result<Json<RunningLateResponse>, (axum::http::StatusCode, String)> {
    let (optimized_route, subsequent_count) = calculate_delay_and_shift(payload.appointments, &payload.delay_job_id);

    let agent_suggestion = if subsequent_count > 0 {
        Some(format!("Draft SMS for {} customers: 'Hi, I am running about 15 minutes late but will be there soon!'", subsequent_count))
    } else {
        None
    };

    Ok(Json(RunningLateResponse {
        success: true,
        subsequent_count,
        agent_suggestion,
        optimized_route,
    }))
}

pub fn router<S: Clone + Send + Sync + 'static>(pool: PgPool) -> Router<S> {
    let state = Arc::new(FieldOpsState { pool });
    Router::new()
        .route("/appointments", get(get_appointments).post(update_appointment))
        .route("/running-late", axum::routing::post(running_late))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
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

    #[test]
    fn test_calculate_delay_and_shift() {
        let t1 = Utc.with_ymd_and_hms(2023, 10, 10, 9, 0, 0).unwrap();
        let t2 = Utc.with_ymd_and_hms(2023, 10, 10, 10, 0, 0).unwrap();
        let t3 = Utc.with_ymd_and_hms(2023, 10, 10, 11, 0, 0).unwrap();

        let appointments = vec![
            Appointment {
                id: "job1".to_string(),
                customer_id: "c1".to_string(),
                customer_name: "Customer 1".to_string(),
                job_template_id: "jt1".to_string(),
                job_name: "Job 1".to_string(),
                status: "Completed".to_string(),
                scheduled_start_time: Some(t1),
                scheduled_end_time: Some(t1 + chrono::Duration::hours(1)),
                location_address: None,
                notes: None,
            },
            Appointment {
                id: "job2".to_string(), // This one is delayed
                customer_id: "c2".to_string(),
                customer_name: "Customer 2".to_string(),
                job_template_id: "jt2".to_string(),
                job_name: "Job 2".to_string(),
                status: "In-Progress".to_string(),
                scheduled_start_time: Some(t2),
                scheduled_end_time: Some(t2 + chrono::Duration::hours(1)),
                location_address: None,
                notes: None,
            },
            Appointment {
                id: "job3".to_string(), // Should be shifted
                customer_id: "c3".to_string(),
                customer_name: "Customer 3".to_string(),
                job_template_id: "jt3".to_string(),
                job_name: "Job 3".to_string(),
                status: "Scheduled".to_string(),
                scheduled_start_time: Some(t3),
                scheduled_end_time: Some(t3 + chrono::Duration::hours(1)),
                location_address: None,
                notes: None,
            },
        ];

        let (optimized_route, subsequent_count) = calculate_delay_and_shift(appointments, "job2");

        assert_eq!(subsequent_count, 1);
        assert_eq!(optimized_route.len(), 3);

        assert_eq!(optimized_route[0].scheduled_start_time.unwrap(), t1);
        assert_eq!(optimized_route[1].scheduled_start_time.unwrap(), t2);
        assert_eq!(optimized_route[2].scheduled_start_time.unwrap(), t3 + chrono::Duration::minutes(15));
    }
}
