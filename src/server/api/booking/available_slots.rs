use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::db::DB;

#[derive(Deserialize)]
pub struct AvailableSlotsRequest {
    pub tenant_id: Option<String>,
    pub product_id: String,
    pub date: String,
}

#[derive(Serialize)]
pub struct AvailableSlotsResponse {
    pub available_slots: Vec<Slot>,
}

#[derive(Serialize)]
pub struct Slot {
    pub start_time: String,
    pub end_time: String,
}

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_get_available_slots))
        .with_state(db)
}

async fn handle_get_available_slots(
    State(db): State<Arc<DB>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<AvailableSlotsRequest>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (axum::http::StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let service_id = payload.product_id;

    // Construct date string for parsing and querying
    // The UI sends "YYYY-MM-DD" in `payload.date`
    let start_date = match chrono::NaiveDate::parse_from_str(&payload.date, "%Y-%m-%d") {
        Ok(d) => d.and_hms_opt(0, 0, 0).unwrap().and_local_timezone(chrono::Utc).unwrap(),
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "invalid date format"})),
            )
                .into_response();
        }
    };
    let end_date = start_date + chrono::Duration::days(1);

    use sqlx::Row;

    let slots_result = sqlx::query(
        r#"
        SELECT start_time, end_time
        FROM availability_blocks
        WHERE tenant_id = $1 AND service_id = $2
          AND start_time >= $3 AND start_time < $4
          AND is_available = true
        ORDER BY start_time ASC
        "#
    )
    .bind(&tenant_id)
    .bind(&service_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(&db.pool)
    .await;

    let slots = match slots_result {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Failed to query available slots: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to fetch available slots"})),
            )
                .into_response();
        }
    };

    let mut response_slots = Vec::new();
    for slot in slots {
        let st: chrono::DateTime<chrono::Utc> = slot.get("start_time");
        let et: chrono::DateTime<chrono::Utc> = slot.get("end_time");
        response_slots.push(Slot {
            start_time: st.to_rfc3339(),
            end_time: et.to_rfc3339(),
        });
    }

    (
        StatusCode::OK,
        Json(AvailableSlotsResponse { available_slots: response_slots }),
    ).into_response()
}
