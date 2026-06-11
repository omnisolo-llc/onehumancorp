use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use sqlx::PgPool;
use sqlx::Row;
use ::server_ohc::app::{
    ListAppointmentsRequest, ListAppointmentsResponse, Appointment,
    UpdateAppointmentStatusRequest, UpdateAppointmentStatusResponse,
};

pub async fn list_appointments(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<ListAppointmentsRequest>,
) -> impl IntoResponse {
    let tenant_id = payload.tenant_id;

    let appointments_result = sqlx::query(
        r#"
        SELECT id, customer_id, job_template_id, assigned_staff_id,
               scheduled_start_time, scheduled_end_time, actual_start_time, actual_end_time,
               location_lat, location_lng, location_address, status
        FROM appointments
        WHERE tenant_id = $1
        ORDER BY scheduled_start_time ASC
        "#,
    )
    .bind(&tenant_id)
    .fetch_all(&pool)
    .await;

    match appointments_result {
        Ok(rows) => {
            let appointments = rows.into_iter().map(|row| {
                let scheduled_start: Option<chrono::DateTime<chrono::Utc>> = row.get("scheduled_start_time");
                let scheduled_end: Option<chrono::DateTime<chrono::Utc>> = row.get("scheduled_end_time");
                let actual_start: Option<chrono::DateTime<chrono::Utc>> = row.get("actual_start_time");
                let actual_end: Option<chrono::DateTime<chrono::Utc>> = row.get("actual_end_time");

                Appointment {
                    id: row.get("id"),
                    customer_id: row.get("customer_id"),
                    job_template_id: row.get("job_template_id"),
                    assigned_staff_id: row.try_get("assigned_staff_id").unwrap_or_default(),
                    scheduled_start_time: scheduled_start.map(|t| t.to_rfc3339()).unwrap_or_default(),
                    scheduled_end_time: scheduled_end.map(|t| t.to_rfc3339()).unwrap_or_default(),
                    actual_start_time: actual_start.map(|t| t.to_rfc3339()).unwrap_or_default(),
                    actual_end_time: actual_end.map(|t| t.to_rfc3339()).unwrap_or_default(),
                    location_lat: row.try_get("location_lat").unwrap_or_default(),
                    location_lng: row.try_get("location_lng").unwrap_or_default(),
                    location_address: row.try_get("location_address").unwrap_or_default(),
                    status: row.get("status"),
                }
            }).collect();

            Json(ListAppointmentsResponse { appointments }).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch appointments: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_appointment_status(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<UpdateAppointmentStatusRequest>,
) -> impl IntoResponse {
    let tenant_id = payload.tenant_id;
    let appointment_id = payload.appointment_id;
    let new_status = payload.status;
    let now = chrono::Utc::now();

    // Determine if we need to update actual_start_time or actual_end_time
    let mut update_query = String::from("UPDATE appointments SET status = $1, updated_at = $4");
    if new_status == "In-Progress" || new_status == "En-Route" {
        update_query.push_str(", actual_start_time = COALESCE(actual_start_time, $4)");
    } else if new_status == "Completed" {
        update_query.push_str(", actual_end_time = COALESCE(actual_end_time, $4)");
    }
    update_query.push_str(" WHERE id = $2 AND tenant_id = $3 RETURNING *");

    let result = sqlx::query(&update_query)
        .bind(&new_status)
        .bind(&appointment_id)
        .bind(&tenant_id)
        .bind(&now)
        .fetch_optional(&pool)
        .await;

    match result {
        Ok(Some(row)) => {
            let scheduled_start: Option<chrono::DateTime<chrono::Utc>> = row.get("scheduled_start_time");
            let scheduled_end: Option<chrono::DateTime<chrono::Utc>> = row.get("scheduled_end_time");
            let actual_start: Option<chrono::DateTime<chrono::Utc>> = row.get("actual_start_time");
            let actual_end: Option<chrono::DateTime<chrono::Utc>> = row.get("actual_end_time");

            let appointment = Appointment {
                id: row.get("id"),
                customer_id: row.get("customer_id"),
                job_template_id: row.get("job_template_id"),
                assigned_staff_id: row.try_get("assigned_staff_id").unwrap_or_default(),
                scheduled_start_time: scheduled_start.map(|t| t.to_rfc3339()).unwrap_or_default(),
                scheduled_end_time: scheduled_end.map(|t| t.to_rfc3339()).unwrap_or_default(),
                actual_start_time: actual_start.map(|t| t.to_rfc3339()).unwrap_or_default(),
                actual_end_time: actual_end.map(|t| t.to_rfc3339()).unwrap_or_default(),
                location_lat: row.try_get("location_lat").unwrap_or_default(),
                location_lng: row.try_get("location_lng").unwrap_or_default(),
                location_address: row.try_get("location_address").unwrap_or_default(),
                status: row.get("status"),
            };

            Json(UpdateAppointmentStatusResponse {
                success: true,
                appointment: Some(appointment),
                error: String::new(),
            }).into_response()
        }
        Ok(None) => {
             Json(UpdateAppointmentStatusResponse {
                success: false,
                appointment: None,
                error: "Appointment not found".to_string(),
            }).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to update appointment status: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
