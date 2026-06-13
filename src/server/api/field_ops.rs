use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Appointment {
    pub id: String,
    pub customer_id: String,
    pub customer_name: String,
    pub job_template_id: String,
    pub job_name: String,
    pub status: String,
    pub scheduled_start_time: Option<DateTime<Utc>>,
    pub scheduled_end_time: Option<DateTime<Utc>>,
    pub actual_start_time: Option<DateTime<Utc>>,
    pub actual_end_time: Option<DateTime<Utc>>,
    pub location_address: Option<String>,
    pub notes: Option<String>,
}

#[derive(Serialize)]
pub struct AppointmentsResponse {
    pub appointments: Vec<Appointment>,
}

pub async fn get_appointments(
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let query = r#"
        SELECT
            a.id,
            a.customer_id,
            c.name as customer_name,
            a.job_template_id,
            j.name as job_name,
            a.status,
            a.scheduled_start_time,
            a.scheduled_end_time,
            a.actual_start_time,
            a.actual_end_time,
            a.location_address,
            a.notes
        FROM appointments a
        LEFT JOIN customers c ON a.customer_id = c.id
        LEFT JOIN job_templates j ON a.job_template_id = j.id
        ORDER BY a.scheduled_start_time ASC NULLS LAST
    "#;

    let result = sqlx::query_as::<_, Appointment>(query)
        .fetch_all(&pool)
        .await;

    match result {
        Ok(appointments) => Json(AppointmentsResponse { appointments }).into_response(),
        Err(e) => {
            eprintln!("Error fetching appointments: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Database error" }))).into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateAppointmentRequest {
    pub status: String,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub actual_start_time: Option<DateTime<Utc>>,
    #[serde(default)]
    pub actual_end_time: Option<DateTime<Utc>>,
}

pub async fn update_appointment(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAppointmentRequest>,
) -> impl IntoResponse {
    let query = r#"
        UPDATE appointments
        SET
            status = $1,
            notes = COALESCE($2, notes),
            actual_start_time = COALESCE($3, actual_start_time),
            actual_end_time = COALESCE($4, actual_end_time),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $5
        RETURNING id
    "#;

    let result = sqlx::query(query)
        .bind(&payload.status)
        .bind(&payload.notes)
        .bind(&payload.actual_start_time)
        .bind(&payload.actual_end_time)
        .bind(&id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Json(serde_json::json!({ "success": true })).into_response(),
        Err(e) => {
            eprintln!("Error updating appointment: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Database error" }))).into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct OptimizeRouteRequest {
    pub appointments: Vec<Appointment>,
    pub current_location_lat: Option<f64>,
    pub current_location_lng: Option<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeRouteResponse {
    pub success: bool,
    pub optimized_route: Vec<Appointment>,
    pub agent_suggestion: Option<String>,
}

pub async fn optimize_route(
    Json(payload): Json<OptimizeRouteRequest>,
) -> impl IntoResponse {
    let mut optimized = payload.appointments;

    // Simulate simple reordering: push completed/cancelled to end
    optimized.sort_by(|a, b| {
        let a_done = a.status == "Completed" || a.status == "Cancelled";
        let b_done = b.status == "Completed" || b.status == "Cancelled";
        if a_done && !b_done {
            std::cmp::Ordering::Greater
        } else if !a_done && b_done {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    });

    // Simulate agent suggestion
    let mut agent_suggestion = None;
    for appt in &optimized {
        if appt.status == "Completed" {
            if let (Some(actual_end), Some(scheduled_end)) = (appt.actual_end_time, appt.scheduled_end_time) {
                if actual_end < scheduled_end {
                    agent_suggestion = Some("You finished early! Should I text the next client to see if we can arrive early?".to_string());
                    break;
                }
            }
        }
    }

    Json(OptimizeRouteResponse {
        success: true,
        optimized_route: optimized,
        agent_suggestion,
    }).into_response()
}

pub fn router<S>(pool: PgPool) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/appointments", get(get_appointments))
        .route("/appointments/{id}", put(update_appointment))
        .route("/optimize-route", post(optimize_route))
        .with_state(pool)
}
