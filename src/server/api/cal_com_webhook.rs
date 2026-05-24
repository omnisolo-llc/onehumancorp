use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use crate::api::billing_webhook::WebhookState;

#[derive(Debug, Deserialize)]
pub struct CalComWebhookPayload {
    pub triggerEvent: String,
    pub payload: CalComWebhookData,
}

#[derive(Debug, Deserialize)]
pub struct CalComWebhookData {
    pub uid: String,
    pub title: String,
    pub startTime: String,
    pub endTime: String,
    pub status: String,
    pub metadata: Option<serde_json::Value>,
    pub attendee: Option<CalComAttendee>,
}

#[derive(Debug, Deserialize)]
pub struct CalComAttendee {
    pub email: String,
    pub name: String,
}

pub async fn cal_com_webhook_handler(
    State(state): State<WebhookState>,
    Json(payload): Json<CalComWebhookPayload>,
) -> impl IntoResponse {
    if payload.triggerEvent != "BOOKING_CREATED" {
        return StatusCode::OK.into_response();
    }

    let metadata = payload.payload.metadata.unwrap_or_else(|| serde_json::json!({}));
    let tenant_id = metadata.get("tenant_id").and_then(|v| v.as_str()).unwrap_or("unknown_tenant").to_string();

    // Attempt to parse existing customer or create a new one based on attendee email
    let attendee_email = payload.payload.attendee.as_ref().map(|a| a.email.clone()).unwrap_or_else(|| "unknown@example.com".to_string());

    // For simplicity, we just hash the email to ensure consistency for now, or would ideally look them up in DB.
    let customer_id = format!("cus_{}", uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, attendee_email.as_bytes()).to_string().replace("-", "")[..16].to_string());

    let booking_id = payload.payload.uid;
    let start_time = payload.payload.startTime;
    let end_time = payload.payload.endTime;
    let status = payload.payload.status;

    let res = match &state.db.store {
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("INSERT INTO bookings (id, tenant_id, customer_id, start_time, end_time, status) VALUES (?, ?, ?, ?, ?, ?)")
                .bind(&booking_id)
                .bind(&tenant_id)
                .bind(&customer_id)
                .bind(&start_time)
                .bind(&end_time)
                .bind(&status)
                .execute(&*pool)
                .await
        }
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO bookings (id, tenant_id, customer_id, start_time, end_time, status) VALUES ($1, $2, $3, $4, $5, $6)")
                .bind(&booking_id)
                .bind(&tenant_id)
                .bind(&customer_id)
                .bind(&start_time)
                .bind(&end_time)
                .bind(&status)
                .execute(&state.db.pool)
                .await
        }
    };

    // Trigger notification to the customer (mocking this with a log as email setup depends on context)
    tracing::info!("Sent booking confirmation for booking {} to customer_id {} in tenant {}", booking_id, customer_id, tenant_id);

    match res {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            tracing::error!("Failed to save booking: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
