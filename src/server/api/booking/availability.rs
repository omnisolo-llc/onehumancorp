use axum::{
    extract::{State, Query},
    response::IntoResponse,
    http::StatusCode,
    routing::get,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::db::DB;
use chrono::{DateTime, Utc};
use sqlx::Row;

#[derive(Deserialize)]
pub struct CheckAvailabilityQuery {
    pub product_id: String,
    pub date: String, // YYYY-MM-DD
}

#[derive(Serialize)]
pub struct TimeSlot {
    pub start_time: String, // RFC3339
    pub end_time: String,   // RFC3339
}

#[derive(Serialize)]
pub struct CheckAvailabilityResponse {
    pub available_slots: Vec<TimeSlot>,
}

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(check_availability))
        .with_state(db)
}

async fn check_availability(
    State(db): State<Arc<DB>>,
    headers: axum::http::HeaderMap,
    Query(query): Query<CheckAvailabilityQuery>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let product_id = query.product_id;
    let date_str = query.date;

    let date_parsed = match chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => return (StatusCode::BAD_REQUEST, axum::Json(serde_json::json!({"error": "Invalid date format, use YYYY-MM-DD"}))).into_response(),
    };

    let mut tx = match db.pool.begin().await {
        Ok(t) => t,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };

    if let Err(e) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": e.to_string()}))).into_response();
    }

    let rows = match sqlx::query(
        "SELECT start_time, end_time FROM bookings \
         WHERE tenant_id = $1 AND product_id = $2 AND start_time::date = $3::date \
         AND COALESCE(status, 'pending') <> 'cancelled'"
    )
    .bind(&tenant_id)
    .bind(&product_id)
    .bind(&date_str)
    .fetch_all(&mut *tx)
    .await {
        Ok(r) => r,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };

    let existing_slots: Vec<(DateTime<Utc>, DateTime<Utc>)> = rows.into_iter().filter_map(|row| {
        let st: Option<DateTime<Utc>> = row.get("start_time");
        let et: Option<DateTime<Utc>> = row.get("end_time");
        if let (Some(s), Some(e)) = (st, et) { Some((s, e)) } else { None }
    }).collect();

    let ledger_rows = match sqlx::query(
        "SELECT start_time, end_time FROM availability_ledger WHERE tenant_id = $1 AND product_id = $2 AND start_time::date = $3::date AND status IN ('BLOCKED', 'BOOKED')"
    )
    .bind(&tenant_id)
    .bind(&product_id)
    .bind(&date_str)
    .fetch_all(&mut *tx)
    .await {
         Ok(r) => r,
         Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };

    let mut blocked_slots: Vec<(DateTime<Utc>, DateTime<Utc>)> = ledger_rows.into_iter().filter_map(|row| {
        let st: Option<DateTime<Utc>> = row.get("start_time");
        let et: Option<DateTime<Utc>> = row.get("end_time");
        if let (Some(s), Some(e)) = (st, et) { Some((s, e)) } else { None }
    }).collect();

    let schedule_rows = match sqlx::query(
        "SELECT business_hours, exceptions FROM availability_schedules WHERE tenant_id = $1"
    )
    .bind(&tenant_id)
    .fetch_all(&mut *tx)
    .await {
         Ok(r) => r,
         Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };

    for row in schedule_rows {
         let exceptions_json: serde_json::Value = row.try_get("exceptions").unwrap_or(serde_json::json!([]));
         if let Some(arr) = exceptions_json.as_array() {
             for ex in arr {
                  let st_str = ex.get("start_time").and_then(|v| v.as_str()).unwrap_or("");
                  let et_str = ex.get("end_time").and_then(|v| v.as_str()).unwrap_or("");
                  if let (Ok(st), Ok(et)) = (DateTime::parse_from_rfc3339(st_str), DateTime::parse_from_rfc3339(et_str)) {
                      blocked_slots.push((st.with_timezone(&Utc), et.with_timezone(&Utc)));
                  }
             }
         }
    }

    let _ = tx.commit().await;

    let mut available_slots = vec![];
    for hour in 9..17 {
        let st_naive = date_parsed.and_hms_opt(hour, 0, 0).unwrap();
        let et_naive = date_parsed.and_hms_opt(hour + 1, 0, 0).unwrap();
        let st = DateTime::<Utc>::from_naive_utc_and_offset(st_naive, Utc);
        let et = DateTime::<Utc>::from_naive_utc_and_offset(et_naive, Utc);

        let mut overlap = false;
        let all_busy = existing_slots.iter().chain(blocked_slots.iter());
        for (est, eet) in all_busy {
            if st < *eet && et > *est {
                overlap = true;
                break;
            }
        }

        if !overlap {
            available_slots.push(TimeSlot {
                start_time: st.to_rfc3339(),
                end_time: et.to_rfc3339(),
            });
        }
    }

    (StatusCode::OK, axum::Json(CheckAvailabilityResponse { available_slots })).into_response()
}
