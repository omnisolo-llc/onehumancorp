use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::Row;

#[derive(Clone)]
pub struct CalendarBookingState {
    pub db: Arc<crate::db::DB>,
}

#[derive(Deserialize)]
pub struct BookServiceRequest {
    pub service_id: String,
    pub start_time: String,
    pub end_time: String,
    pub customer_email: String,
    pub requires_zoom: bool,
}

#[derive(Serialize)]
pub struct BookServiceResponse {
    pub event_id: String,
    pub join_url: Option<String>,
}

pub async fn book_service_handler(
    State(state): State<CalendarBookingState>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    let tenant_id = match request.extensions().get::<crate::auth::AuthInfo>() {
        Some(auth) => auth.org_id.clone(),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let body_bytes = axum::body::to_bytes(request.into_body(), usize::MAX).await.unwrap_or_default();
    let payload: BookServiceRequest = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid payload").into_response(),
    };

    // Get tokens
    let tokens = match &state.db.store {
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("SELECT google_calendar_token, zoom_token FROM tenants WHERE tenant_id = ?")
                .bind(&tenant_id)
                .fetch_optional(pool)
                .await
                .ok()
                .flatten()
        }
        crate::db::DbStore::Postgres => {
            sqlx::query("SELECT google_calendar_token, zoom_token FROM tenants WHERE tenant_id = $1")
                .bind(&tenant_id)
                .fetch_optional(&state.db.pool)
                .await
                .ok()
                .flatten()
        }
    };

    let (google_token, zoom_token): (Option<String>, Option<String>) = match tokens {
        Some(row) => (row.try_get("google_calendar_token").ok(), row.try_get("zoom_token").ok()),
        None => (None, None),
    };

    let summary = format!("Booking for service {}", payload.service_id);

    use crate::integrations::google_calendar::client::GoogleCalendarClientWrapper;
    let event_id = if let Some(t) = google_token {
        let client = crate::integrations::google_calendar::client::RealGoogleCalendarClient::new(t);
        match client.create_event(&summary, &payload.start_time, &payload.end_time).await {
            Ok(id) => id,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create Google Calendar event").into_response(),
        }
    } else {
        return (StatusCode::BAD_REQUEST, "Google Calendar not connected").into_response();
    };

    let mut join_url = None;
    use crate::integrations::zoom::client::ZoomClientWrapper;
    if payload.requires_zoom {
        if let Some(t) = zoom_token {
            let client = crate::integrations::zoom::client::RealZoomClient::new(t);
            if let Ok(url) = client.create_meeting(&summary, &payload.start_time).await {
                join_url = Some(url);
            }
        }
    }

    (StatusCode::OK, Json(BookServiceResponse {
        event_id,
        join_url,
    })).into_response()
}
