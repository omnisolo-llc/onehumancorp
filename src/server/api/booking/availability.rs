use axum::{
    extract::Query,
    response::IntoResponse,
    http::StatusCode,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::Row;

#[derive(Deserialize)]
pub struct AvailabilityQuery {
    pub tenant_id: String,
    pub product_id: String,
    pub date: String,
}

#[derive(Serialize)]
pub struct TimeSlot {
    pub start_time: String,
    pub end_time: String,
}

#[derive(Serialize)]
pub struct AvailabilityResponse {
    pub available_slots: Vec<TimeSlot>,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(handle_check_availability))
}

async fn handle_check_availability(
    Query(query): Query<AvailabilityQuery>,
) -> impl IntoResponse {
    let tenant_id = query.tenant_id;
    let product_id = query.product_id;
    let date_str = query.date;

    let date_parsed = match chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => return (StatusCode::BAD_REQUEST, axum::Json(serde_json::json!({"error": "Invalid date format, use YYYY-MM-DD"}))).into_response(),
    };

    let pool = crate::db::get_pool();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "db_error"}))).into_response(),
    };

    if let Err(_) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "auth_error"}))).into_response();
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
        Ok(rows) => rows,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "db_query_error"}))).into_response(),
    };

    let mut blocked_slots: Vec<(DateTime<Utc>, DateTime<Utc>)> = rows.into_iter().filter_map(|row| {
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
        Ok(rows) => rows,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "db_query_error"}))).into_response(),
    };

    blocked_slots.extend(ledger_rows.into_iter().filter_map(|row| {
        let st: Option<DateTime<Utc>> = row.get("start_time");
        let et: Option<DateTime<Utc>> = row.get("end_time");
        if let (Some(s), Some(e)) = (st, et) { Some((s, e)) } else { None }
    }));

    let _ = tx.commit().await;

    let mut available_slots = vec![];
    for hour in 9..17 {
        let st_naive = date_parsed.and_hms_opt(hour, 0, 0).unwrap();
        let et_naive = date_parsed.and_hms_opt(hour + 1, 0, 0).unwrap();
        let st = DateTime::<Utc>::from_naive_utc_and_offset(st_naive, Utc);
        let et = DateTime::<Utc>::from_naive_utc_and_offset(et_naive, Utc);

        let mut overlap = false;
        for (b_start, b_end) in &blocked_slots {
            if st < *b_end && et > *b_start {
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

    (StatusCode::OK, axum::Json(AvailabilityResponse { available_slots })).into_response()
}
