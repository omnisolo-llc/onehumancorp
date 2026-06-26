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
    pub location_lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_lng: Option<f64>,
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
            NULL::double precision as location_lat,
            NULL::double precision as location_lng,
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
            a.location_lat,
            a.location_lng,
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
            location_lat: row.try_get("location_lat").unwrap_or(None),
            location_lng: row.try_get("location_lng").unwrap_or(None),
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
    pub location_lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_lng: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Serialize)]
pub struct UpdateAppointmentResponse {
    pub success: bool,
    pub id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_lng: Option<f64>,
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
        location_lat: payload.location_lat,
        location_lng: payload.location_lng,
    }))
}


#[derive(Deserialize)]
pub struct OptimizeRouteRequest {
    pub appointments: Vec<Appointment>,
    #[serde(rename = "currentLocationLat")]
    pub current_location_lat: Option<f64>,
    #[serde(rename = "currentLocationLng")]
    pub current_location_lng: Option<f64>,
}

#[derive(Serialize)]
pub struct OptimizeRouteResponse {
    pub success: bool,
    #[serde(rename = "optimizedRoute")]
    pub optimized_route: Vec<Appointment>,
    #[serde(rename = "agentSuggestion", skip_serializing_if = "Option::is_none")]
    pub agent_suggestion: Option<String>,
}

fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let to_rad = |x: f64| x * std::f64::consts::PI / 180.0;
    let r = 6371.0; // km
    let d_lat = to_rad(lat2 - lat1);
    let d_lon = to_rad(lon2 - lon1);
    let a = (d_lat / 2.0).sin() * (d_lat / 2.0).sin()
        + to_rad(lat1).cos() * to_rad(lat2).cos() * (d_lon / 2.0).sin() * (d_lon / 2.0).sin();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    r * c
}

pub async fn optimize_route(
    Json(payload): Json<OptimizeRouteRequest>,
) -> Result<Json<OptimizeRouteResponse>, (axum::http::StatusCode, String)> {
    let mut completed = Vec::new();
    let mut pending = Vec::new();

    for appt in payload.appointments {
        if appt.status == "Completed" || appt.status == "Cancelled" {
            completed.push(appt);
        } else {
            pending.push(appt);
        }
    }

    let mut current_lat = payload.current_location_lat.unwrap_or(0.0);
    let mut current_lng = payload.current_location_lng.unwrap_or(0.0);

    let mut optimized_pending = Vec::new();

    while !pending.is_empty() {
        let mut nearest_index = 0;
        let mut min_distance = std::f64::INFINITY;

        for (i, appt) in pending.iter().enumerate() {
            let appt_lat = appt.location_lat.unwrap_or(0.0);
            let appt_lng = appt.location_lng.unwrap_or(0.0);
            let dist = haversine_distance(current_lat, current_lng, appt_lat, appt_lng);
            if dist < min_distance {
                min_distance = dist;
                nearest_index = i;
            }
        }

        let mut next_job = pending.remove(nearest_index);
        let travel_time_mins = (min_distance * 2.0).round() as i32 + 5;

        if min_distance > 0.0 {
            let note_addition = format!("[Travel: ~{} mins]", travel_time_mins);
            next_job.notes = match next_job.notes {
                Some(n) if !n.is_empty() => Some(format!("{}\n{}", n, note_addition)),
                _ => Some(note_addition),
            };
        }

        current_lat = next_job.location_lat.unwrap_or(0.0);
        current_lng = next_job.location_lng.unwrap_or(0.0);

        optimized_pending.push(next_job);
    }

    let mut optimized = completed;
    optimized.extend(optimized_pending);

    let agent_suggestion = if optimized.iter().any(|a| a.status == "Completed") {
        Some("You finished early! Should I text the next client to see if we can arrive early?".to_string())
    } else {
        None
    };

    Ok(Json(OptimizeRouteResponse {
        success: true,
        optimized_route: optimized,
        agent_suggestion,
    }))
}

#[derive(Deserialize)]
pub struct RunningLateRequest {
    pub appointments: Vec<Appointment>,
    #[serde(rename = "delayJobId")]
    pub delay_job_id: String,
}

#[derive(Serialize)]
pub struct RunningLateResponse {
    pub success: bool,
    #[serde(rename = "optimizedRoute")]
    pub optimized_route: Vec<Appointment>,
    #[serde(rename = "subsequentCount")]
    pub subsequent_count: i32,
    #[serde(rename = "agentSuggestion", skip_serializing_if = "Option::is_none")]
    pub agent_suggestion: Option<String>,
}

pub async fn running_late(
    Json(payload): Json<RunningLateRequest>,
) -> Result<Json<RunningLateResponse>, (axum::http::StatusCode, String)> {
    let mut delay_index = None;
    for (i, appt) in payload.appointments.iter().enumerate() {
        if appt.id == payload.delay_job_id {
            delay_index = Some(i);
            break;
        }
    }

    if delay_index.is_none() {
        return Err((axum::http::StatusCode::NOT_FOUND, "Job not found".to_string()));
    }
    let delay_index = delay_index.unwrap();

    let delay_minutes = 30;
    let delay_duration = chrono::Duration::minutes(delay_minutes);

    let mut subsequent_count = 0;
    let mut optimized_route = Vec::new();

    for (i, mut appt) in payload.appointments.into_iter().enumerate() {
        if i > delay_index && appt.status != "Completed" && appt.status != "Cancelled" {
            subsequent_count += 1;
            if let Some(start) = appt.scheduled_start_time {
                appt.scheduled_start_time = Some(start + delay_duration);
            }
            if let Some(end) = appt.scheduled_end_time {
                appt.scheduled_end_time = Some(end + delay_duration);
            }
        }
        optimized_route.push(appt);
    }

    let agent_suggestion = if subsequent_count > 0 {
        Some(format!("Drafting delay notifications for the next {} clients. Approve?", subsequent_count))
    } else {
        Some("No subsequent appointments to notify.".to_string())
    };

    Ok(Json(RunningLateResponse {
        success: true,
        optimized_route,
        subsequent_count,
        agent_suggestion,
    }))
}

pub fn router<S: Clone + Send + Sync + 'static>(pool: PgPool) -> Router<S> {
    let state = Arc::new(FieldOpsState { pool });
    Router::new()
        .route("/appointments", get(get_appointments).post(update_appointment))
        .route("/optimize-route", axum::routing::post(optimize_route))
        .route("/running-late", axum::routing::post(running_late))
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
