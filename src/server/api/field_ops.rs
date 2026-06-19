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

#[derive(Serialize, Deserialize, Clone)]
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
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
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
            a.location_lat,
            a.location_lng,
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
            location_lat: row.get("location_lat"),
            location_lng: row.get("location_lng"),
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
        id: payload.id.clone(),
        status: payload.status.clone(),
        notes: payload.notes.clone(),
        scheduled_start_time: payload.scheduled_start_time,
        scheduled_end_time: payload.scheduled_end_time,
    }))
}

#[derive(Deserialize)]
pub struct DelayRequest {
    pub tenant_id: String,
    pub appointment_id: String,
    pub delay_minutes: i64,
}

#[derive(Serialize)]
pub struct DelayResponse {
    pub success: bool,
    pub message: String,
    pub affected_count: u64,
    pub appointments: Vec<Appointment>,
}

pub async fn delay_appointment(
    State(state): State<Arc<FieldOpsState>>,
    axum::extract::Json(payload): axum::extract::Json<DelayRequest>,
) -> Result<Json<DelayResponse>, (axum::http::StatusCode, String)> {

    let appointment_row = sqlx::query("SELECT tenant_id FROM appointments WHERE id = $1 AND tenant_id = $2")
        .bind(&payload.appointment_id)
        .bind(&payload.tenant_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let tenant_id: String = match appointment_row {
        Some(row) => row.get("tenant_id"),
        None => return Err((axum::http::StatusCode::NOT_FOUND, "Appointment not found or access denied".to_string())),
    };

    let delay_start_row = sqlx::query("SELECT scheduled_start_time FROM appointments WHERE id = $1")
        .bind(&payload.appointment_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let start_time: Option<DateTime<Utc>> = delay_start_row.get("scheduled_start_time");
    let mut affected_count = 0;

    if let Some(start) = start_time {
        use chrono::Timelike;
        let end_of_day = start.with_hour(23).unwrap().with_minute(59).unwrap().with_second(59).unwrap();

        let result = sqlx::query(
            r#"
            UPDATE appointments
            SET scheduled_start_time = scheduled_start_time + make_interval(mins => $1::int),
                scheduled_end_time = scheduled_end_time + make_interval(mins => $1::int),
                updated_at = CURRENT_TIMESTAMP
            WHERE tenant_id = $2 AND scheduled_start_time >= $3 AND scheduled_start_time <= $4
            "#
        )
        .bind(payload.delay_minutes as i32)
        .bind(&tenant_id)
        .bind(start)
        .bind(end_of_day)
        .execute(&state.pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        affected_count = result.rows_affected();
    }

    let message = if affected_count > 1 {
        format!("Drafting delay notifications for the next {} clients.", affected_count - 1)
    } else {
        "No subsequent clients affected today.".to_string()
    };

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
            a.location_lat,
            a.location_lng,
            a.notes
        FROM appointments a
        LEFT JOIN customers c ON a.customer_id = c.id
        LEFT JOIN job_templates jt ON a.job_template_id = jt.id
        WHERE a.tenant_id = $1
        ORDER BY a.scheduled_start_time ASC
        "#,
    )
    .bind(&tenant_id)
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
            location_lat: row.get("location_lat"),
            location_lng: row.get("location_lng"),
            notes: row.get("notes"),
        });
    }

    Ok(Json(DelayResponse {
        success: true,
        message,
        affected_count,
        appointments
    }))
}

#[derive(Deserialize)]
pub struct OptimizeRouteRequest {
    pub tenant_id: String,
    pub appointments: Vec<Appointment>,
    #[serde(alias = "currentLocationLat")]
    pub current_location_lat: Option<f64>,
    #[serde(alias = "currentLocationLng")]
    pub current_location_lng: Option<f64>,
}

#[derive(Serialize)]
pub struct OptimizeRouteResponse {
    pub success: bool,
    #[serde(rename = "optimizedRoute")]
    pub optimized_route: Vec<Appointment>,
    #[serde(rename = "agentSuggestion")]
    pub agent_suggestion: Option<String>,
}

fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let earth_radius_km = 6371.0;

    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();

    let lat1 = lat1.to_radians();
    let lat2 = lat2.to_radians();

    let a = (d_lat / 2.0).sin().powi(2) +
            lat1.cos() * lat2.cos() *
            (d_lon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    earth_radius_km * c
}

pub async fn optimize_route(
    State(state): State<Arc<FieldOpsState>>,
    axum::extract::Json(payload): axum::extract::Json<OptimizeRouteRequest>,
) -> Result<Json<OptimizeRouteResponse>, (axum::http::StatusCode, String)> {

    let mut pending_appointments: Vec<Appointment> = payload.appointments.iter()
        .filter(|a| a.status != "Completed" && a.status != "Cancelled")
        .cloned()
        .collect();

    let mut completed_appointments: Vec<Appointment> = payload.appointments.iter()
        .filter(|a| a.status == "Completed" || a.status == "Cancelled")
        .cloned()
        .collect();

    let mut current_lat = payload.current_location_lat.unwrap_or(0.0);
    let mut current_lng = payload.current_location_lng.unwrap_or(0.0);

    let mut optimized_pending = Vec::new();

    while !pending_appointments.is_empty() {
        let mut nearest_idx = 0;
        let mut min_distance = f64::MAX;

        for (i, appt) in pending_appointments.iter().enumerate() {
            let lat = appt.location_lat.unwrap_or(0.0);
            let lng = appt.location_lng.unwrap_or(0.0);

            if lat != 0.0 && lng != 0.0 && current_lat != 0.0 && current_lng != 0.0 {
                let dist = calculate_distance(current_lat, current_lng, lat, lng);
                if dist < min_distance {
                    min_distance = dist;
                    nearest_idx = i;
                }
            } else {
                if min_distance == f64::MAX {
                    nearest_idx = 0;
                }
            }
        }

        let mut next_appt = pending_appointments.remove(nearest_idx);

        current_lat = next_appt.location_lat.unwrap_or(current_lat);
        current_lng = next_appt.location_lng.unwrap_or(current_lng);

        if min_distance != f64::MAX && !optimized_pending.is_empty() {
            let travel_mins = (min_distance * 10.0) as i64;
            let last_appt: &Appointment = optimized_pending.last().unwrap();

            if let Some(last_end) = last_appt.scheduled_end_time {
                let new_start = last_end + chrono::Duration::try_minutes(travel_mins + 15).unwrap();

                if let Some(current_start) = next_appt.scheduled_start_time {
                    if current_start < new_start {
                        let duration = next_appt.scheduled_end_time.unwrap_or(current_start) - current_start;
                        next_appt.scheduled_start_time = Some(new_start);
                        next_appt.scheduled_end_time = Some(new_start + duration);
                    }
                }
            }
        }

        optimized_pending.push(next_appt);
    }

    // Save the new optimized route to DB
    for appt in &optimized_pending {
        sqlx::query(
            r#"
            UPDATE appointments
            SET scheduled_start_time = $1,
                scheduled_end_time = $2,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $3 AND tenant_id = $4
            "#
        )
        .bind(appt.scheduled_start_time)
        .bind(appt.scheduled_end_time)
        .bind(&appt.id)
        .bind(&payload.tenant_id)
        .execute(&state.pool)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    let mut agent_suggestion = None;
    if !completed_appointments.is_empty() && !optimized_pending.is_empty() {
        if let Some(last_completed) = completed_appointments.last() {
            if let (Some(_actual_end), Some(_scheduled_end)) = (last_completed.scheduled_end_time, last_completed.scheduled_end_time) {
                agent_suggestion = Some("You finished early! Should I text the next client to see if we can arrive early?".to_string());
            }
        }
    }

    let mut optimized_route = Vec::new();
    optimized_route.append(&mut completed_appointments);
    optimized_route.append(&mut optimized_pending);

    Ok(Json(OptimizeRouteResponse {
        success: true,
        optimized_route,
        agent_suggestion,
    }))
}

pub fn router<S: Clone + Send + Sync + 'static>(pool: PgPool) -> Router<S> {
    let state = Arc::new(FieldOpsState { pool });
    Router::new()
        .route("/appointments", get(get_appointments).post(update_appointment))
        .route("/delay", axum::routing::post(delay_appointment))
        .route("/optimize-route", axum::routing::post(optimize_route))
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
}
