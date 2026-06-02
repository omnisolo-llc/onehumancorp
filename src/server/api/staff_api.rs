use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::db::DB;

#[derive(Clone)]
pub struct StaffApiState {
    pub db: Arc<DB>,
}

pub fn staff_routes(db: Arc<DB>) -> Router {
    let state = StaffApiState { db };
    Router::new()
        .route("/api/team/staff", get(list_staff).post(add_staff))
        .route("/api/team/staff/:id/pin", post(set_pin))
        .route("/api/team/staff/sync_timecards", post(sync_timecards))
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct StaffMember {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub phone_number: String,
    pub role: String,
    #[serde(skip_serializing)]
    pub hashed_pin: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct AddStaffRequest {
    pub tenant_id: String,
    pub name: String,
    pub phone_number: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct AddStaffResponse {
    pub id: String,
    pub invite_link: String,
}

#[derive(Debug, Deserialize)]
pub struct SetPinRequest {
    pub pin: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimecardEvent {
    pub id: String,
    pub tenant_id: String,
    pub staff_member_id: String,
    pub event_type: String, // 'CLOCK_IN', 'CLOCK_OUT'
    pub occurred_at: DateTime<Utc>,
    pub synced_at: DateTime<Utc>,
    pub is_offline_sync: bool,
}

#[derive(Debug, Deserialize)]
pub struct SyncTimecardsRequest {
    pub tenant_id: String,
    pub events: Vec<TimecardEventPayload>,
}

#[derive(Debug, Deserialize)]
pub struct TimecardEventPayload {
    pub id: String,
    pub staff_member_id: String,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
}

async fn list_staff(
    State(state): State<StaffApiState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let tenant_id = params.get("tenant_id").ok_or((StatusCode::BAD_REQUEST, "Missing tenant_id".to_string()))?;

    let pool = &state.db.pool;
    let staff = sqlx::query_as::<_, StaffMember>(
        "SELECT id, tenant_id, name, phone_number, role, hashed_pin, created_at, updated_at FROM staff_members WHERE tenant_id = $1",
    )
    .bind(tenant_id)
    .fetch_all(pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(staff)))
}

async fn add_staff(
    State(state): State<StaffApiState>,
    Json(payload): Json<AddStaffRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let pool = &state.db.pool;
    let new_id = Uuid::new_v4().to_string();

    // Using a temporary hash until they set it via the invite link
    let temp_hash = "PENDING_SETUP".to_string();

    sqlx::query(
        "INSERT INTO staff_members (id, tenant_id, name, phone_number, role, hashed_pin) VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(new_id.clone())
    .bind(payload.tenant_id)
    .bind(payload.name)
    .bind(payload.phone_number)
    .bind(payload.role)
    .bind(temp_hash)
    .execute(pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // In a real implementation, we would send an SMS here.
    let invite_link = format!("https://ohc.app/invite/staff/{}/setup", new_id);

    Ok((StatusCode::CREATED, Json(AddStaffResponse { id: new_id, invite_link })))
}

async fn set_pin(
    State(state): State<StaffApiState>,
    Path(id): Path<String>,
    Json(payload): Json<SetPinRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let pool = &state.db.pool;

    // In a real implementation, use bcrypt or argon2 to hash the PIN
    // For this demonstration, we'll do a simple mock hash
    let mock_hashed_pin = format!("HASHED_{}", payload.pin);

    sqlx::query(
        "UPDATE staff_members SET hashed_pin = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2"
    )
    .bind(mock_hashed_pin)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}

async fn sync_timecards(
    State(state): State<StaffApiState>,
    Json(payload): Json<SyncTimecardsRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let pool = &state.db.pool;
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for event in payload.events {
        sqlx::query(
            "INSERT INTO timecard_events (id, tenant_id, staff_member_id, event_type, occurred_at, is_offline_sync)
             VALUES ($1, $2, $3, $4, $5, true)
             ON CONFLICT (id) DO NOTHING"
        )
        .bind(event.id)
        .bind(payload.tenant_id.clone())
        .bind(event.staff_member_id)
        .bind(event.event_type)
        .bind(event.occurred_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}
