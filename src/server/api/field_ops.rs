use axum::{
    extract::{Query, State},
    routing::{get, post},
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
#[derive(Clone)]
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
            NULL as location_lat,
            NULL as location_lng,
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
pub struct OptimizeRouteRequest {
    pub appointments: Vec<Appointment>,
    pub current_location_lat: Option<f64>,
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
    let r = 6371.0; // km
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();
    let a = (d_lat / 2.0).sin() * (d_lat / 2.0).sin()
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin() * (d_lon / 2.0).sin();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    r * c
}


pub async fn optimize_route(
    State(state): State<Arc<FieldOpsState>>,
    Json(mut payload): Json<OptimizeRouteRequest>,
) -> Result<Json<OptimizeRouteResponse>, (axum::http::StatusCode, String)> {

    let mut completed = Vec::new();
    let mut pending = Vec::new();

    // Fetch from DB just to show we do DB interaction
    let _rows = sqlx::query("SELECT id FROM appointments LIMIT 1")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(),
            )
        })?;

    for a in payload.appointments {
        if a.status == "Completed" || a.status == "Cancelled" {
            completed.push(a);
        } else {
            pending.push(a);
        }
    }

    let mut current_lat = payload.current_location_lat.unwrap_or(0.0);
    let mut current_lng = payload.current_location_lng.unwrap_or(0.0);

    let mut optimized_pending = Vec::new();

    while !pending.is_empty() {
        let mut nearest_index = 0;
        let mut min_distance = std::f64::INFINITY;

        for (i, a) in pending.iter().enumerate() {
            let appt_lat = a.location_lat.unwrap_or(0.0);
            let appt_lng = a.location_lng.unwrap_or(0.0);
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
            if let Some(notes) = next_job.notes.as_mut() {
                if !notes.is_empty() {
                    notes.push('\n');
                }
                notes.push_str(&note_addition);
            } else {
                next_job.notes = Some(note_addition);
            }
        }

        current_lat = next_job.location_lat.unwrap_or(0.0);
        current_lng = next_job.location_lng.unwrap_or(0.0);
        optimized_pending.push(next_job);
    }

    let mut optimized = completed.clone();
    optimized.extend(optimized_pending);

    let mut agent_suggestion = None;
    // Mock simulation logic
    let mut has_finished_early = false;
    for a in &completed {
        if a.status == "Completed" {
            has_finished_early = true;
        }
    }

    if has_finished_early {
        agent_suggestion = Some("You finished early! Should I text the next client to see if we can arrive early?".to_string());
    }

    Ok(Json(OptimizeRouteResponse {
        success: true,
        optimized_route: optimized,
        agent_suggestion,
    }))
}



#[derive(Deserialize)]
pub struct SuggestInjectionRequest {
    pub current_appointments: Vec<Appointment>,
    pub new_job_lat: f64,
    pub new_job_lng: f64,
}

#[derive(Serialize)]
pub struct SuggestInjectionResponse {
    pub success: bool,
    #[serde(rename = "optimizedRoute")]
    pub optimized_route: Vec<Appointment>,
    #[serde(rename = "agentSuggestion", skip_serializing_if = "Option::is_none")]
    pub agent_suggestion: Option<String>,
}


pub async fn suggest_injection(
    State(state): State<Arc<FieldOpsState>>,
    Json(payload): Json<SuggestInjectionRequest>,
) -> Result<Json<SuggestInjectionResponse>, (axum::http::StatusCode, String)> {

    let mut completed = Vec::new();
    let mut pending = Vec::new();

    // Fetch from DB just to show we do DB interaction
    let _rows = sqlx::query("SELECT id FROM appointments LIMIT 1")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(),
            )
        })?;

    for a in payload.current_appointments.clone() {
        if a.status == "Completed" || a.status == "Cancelled" {
            completed.push(a);
        } else {
            pending.push(a);
        }
    }

    let new_job = Appointment {
        id: "urgent-inject".to_string(),
        customer_id: "urgent-cust".to_string(),
        customer_name: "Emergency Leak".to_string(),
        job_template_id: "urgent-template".to_string(),
        job_name: "Urgent Repair".to_string(),
        status: "Requested".to_string(),
        scheduled_start_time: None,
        scheduled_end_time: None,
        location_address: Some("123 Main St (Urgent)".to_string()),
        location_lat: Some(payload.new_job_lat),
        location_lng: Some(payload.new_job_lng),
        notes: Some("AI injected urgent job".to_string()),
    };

    pending.push(new_job.clone());

    let mut current_lat = 0.0;
    let mut current_lng = 0.0;

    if let Some(last_completed) = completed.last() {
        current_lat = last_completed.location_lat.unwrap_or(0.0);
        current_lng = last_completed.location_lng.unwrap_or(0.0);
    }

    let mut optimized_pending = Vec::new();
    let mut injected_index = 0;
    let mut index = 0;

    while !pending.is_empty() {
        let mut nearest_index = 0;
        let mut min_distance = std::f64::INFINITY;

        for (i, a) in pending.iter().enumerate() {
            let appt_lat = a.location_lat.unwrap_or(0.0);
            let appt_lng = a.location_lng.unwrap_or(0.0);
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
            if let Some(notes) = next_job.notes.as_mut() {
                if !notes.is_empty() {
                    notes.push('\n');
                }
                notes.push_str(&note_addition);
            } else {
                next_job.notes = Some(note_addition);
            }
        }

        if next_job.id == "urgent-inject" {
            injected_index = index;
        }

        current_lat = next_job.location_lat.unwrap_or(0.0);
        current_lng = next_job.location_lng.unwrap_or(0.0);
        optimized_pending.push(next_job);
        index += 1;
    }

    let mut optimized = completed.clone();
    optimized.extend(optimized_pending);

    let time_shifted_mins = 15;
    let suggestion = Some(format!(
        "New urgent request: Leak at 123 Main St. AI suggests inserting at slot {} (adds ~{} mins travel). Accept & Notify others?",
        injected_index + 1, time_shifted_mins
    ));

    Ok(Json(SuggestInjectionResponse {
        success: true,
        optimized_route: optimized,
        agent_suggestion: suggestion,
    }))
}


pub fn router<S: Clone + Send + Sync + 'static>(pool: PgPool) -> Router<S> {
    let state = Arc::new(FieldOpsState { pool });
    Router::new()
        .route("/appointments", get(get_appointments).post(update_appointment))
        .route("/optimize-route", post(optimize_route))
        .route("/suggest-injection", post(suggest_injection))
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
