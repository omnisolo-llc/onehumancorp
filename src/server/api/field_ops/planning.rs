//! Local route preparation and explicitly committed owned route records.
use super::records::{
    self, FieldError, Receipt, authority_error, conflict, invalid, missing, unavailable,
};
use super::{Appointment, FieldOpsState};
use axum::{
    Json,
    extract::{Extension, State},
    http::{HeaderMap, StatusCode},
};
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[derive(Serialize, Deserialize)]
pub struct OptimizeRouteRequest {
    #[serde(rename = "tenantId")]
    pub tenant_id: Option<String>,
    pub appointments: Vec<Appointment>,
    /// Older clients requested persistence by supplying tenantId; false explicitly previews.
    pub commit: Option<bool>,
    #[serde(rename = "currentLocationLat")]
    pub current_location_lat: Option<f64>,
    #[serde(rename = "currentLocationLng")]
    pub current_location_lng: Option<f64>,
}
#[derive(Serialize, Deserialize)]
pub struct OptimizeRouteResponse {
    pub success: bool,
    pub committed: bool,
    #[serde(rename = "routeId", skip_serializing_if = "Option::is_none")]
    pub route_id: Option<String>,
    #[serde(rename = "optimizedRoute")]
    pub optimized_route: Vec<Appointment>,
    #[serde(rename = "agentSuggestion", skip_serializing_if = "Option::is_none")]
    pub agent_suggestion: Option<String>,
}
/// Lock in stable ID order before comparing the exact observed persisted timestamps.
async fn owned_snapshot(
    connection: &mut PgConnection,
    tenant: &str,
    supplied: &[Appointment],
) -> Result<Vec<Appointment>, FieldError> {
    if supplied.len() > 256 {
        return Err(invalid("At most 256 appointments can be prepared together"));
    }
    let ids: Vec<String> = supplied.iter().map(|row| row.id.clone()).collect();
    if ids.iter().any(|id| !records::valid_id(id))
        || ids.iter().collect::<HashSet<_>>().len() != ids.len()
    {
        return Err(invalid("Appointment identifiers must be valid and unique"));
    }
    let locked: Vec<String> = sqlx::query_scalar(
        "SELECT id FROM appointments WHERE tenant_id=$1 AND id=ANY($2) ORDER BY id FOR UPDATE",
    )
    .bind(tenant)
    .bind(&ids)
    .fetch_all(&mut *connection)
    .await
    .map_err(unavailable)?;
    if locked.len() != ids.len() {
        return Err(missing());
    }
    let rows = sqlx::query_as::<_,Appointment>(
        "SELECT a.id,COALESCE(c.id,'') AS customer_id,COALESCE(c.name,'') AS customer_name,COALESCE(jt.id,'') AS job_template_id,COALESCE(jt.name,'') AS job_name,a.status,a.updated_at,a.scheduled_start_time,a.scheduled_end_time,a.location_address,a.location_lat,a.location_lng,a.notes FROM appointments a LEFT JOIN customers c ON c.id=a.customer_id AND c.tenant_id=a.tenant_id LEFT JOIN job_templates jt ON jt.id=a.job_template_id AND jt.tenant_id=a.tenant_id WHERE a.tenant_id=$1 AND a.id=ANY($2)"
    ).bind(tenant).bind(&ids).fetch_all(connection).await.map_err(unavailable)?;
    let mut by_id: HashMap<_, _> = rows.into_iter().map(|row| (row.id.clone(), row)).collect();
    supplied
        .iter()
        .map(|requested| {
            let row = by_id.remove(&requested.id).ok_or_else(missing)?;
            if Some(records::expected(requested.updated_at)?) != row.updated_at {
                return Err(conflict());
            }
            Ok(row)
        })
        .collect()
}
fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let a = ((lat2 - lat1).to_radians() / 2.0).sin().powi(2)
        + lat1.to_radians().cos()
            * lat2.to_radians().cos()
            * ((lon2 - lon1).to_radians() / 2.0).sin().powi(2);
    6371.0 * 2.0 * a.clamp(0.0, 1.0).sqrt().asin()
}
fn location(row: &Appointment) -> Option<(f64, f64)> {
    let (lat, lng) = row.location_lat.zip(row.location_lng)?;
    records::coordinates(Some(lat), Some(lng)).ok()?;
    Some((lat, lng))
}
fn order_route(rows: Vec<Appointment>, mut current: Option<(f64, f64)>) -> Vec<Appointment> {
    let (mut finished, mut pending): (Vec<_>, Vec<_>) = rows
        .into_iter()
        .partition(|row| records::is_terminal(&row.status));
    while !pending.is_empty() {
        let nearest = current.and_then(|(lat, lng)| {
            pending
                .iter()
                .enumerate()
                .filter_map(|(index, row)| {
                    location(row)
                        .map(|(rlat, rlng)| (index, haversine_distance(lat, lng, rlat, rlng)))
                })
                .min_by(|a, b| a.1.total_cmp(&b.1))
        });
        let index = nearest.map_or(0, |v| v.0);
        let mut row = pending.remove(index);
        if let Some((_, distance)) = nearest
            && distance > 0.0
        {
            let estimate = (distance * 2.0).round() as i64 + 5;
            let note = format!("[Travel estimate: ~{estimate} mins]");
            row.notes = Some(
                row.notes
                    .filter(|notes| !notes.is_empty())
                    .map_or(note.clone(), |notes| format!("{notes}\n{note}")),
            );
        }
        // Unknown coordinates never invent a geographic origin or travel time.
        current = location(&row);
        finished.push(row);
    }
    finished
}
pub async fn optimize_route(
    headers: HeaderMap,
    State(state): State<Arc<FieldOpsState>>,
    Extension(claims): Extension<server_common::Claims>,
    Json(payload): Json<OptimizeRouteRequest>,
) -> Result<Json<OptimizeRouteResponse>, FieldError> {
    let owner = state.access.authorize(&claims, &headers).await?;
    let tenant = owner.tenant_id().to_owned();
    if payload.tenant_id.as_ref().is_some_and(|id| id != &tenant) {
        return Err((
            StatusCode::FORBIDDEN,
            "Route tenant does not match the current session".into(),
        ));
    }
    records::coordinates(payload.current_location_lat, payload.current_location_lng)?;
    let commit = payload.commit.unwrap_or(payload.tenant_id.is_some());
    let receipt = if commit {
        Receipt::new(&headers, &owner, "route", &payload)?
    } else {
        None
    };
    if commit && receipt.is_none() {
        return Err((
            StatusCode::PRECONDITION_REQUIRED,
            "Idempotency-Key is required to save a route".into(),
        ));
    }
    let mut tx = owner.begin().await.map_err(authority_error)?;
    if let Some(receipt) = &receipt
        && let Some(saved) = receipt.replay(tx.connection()).await?
    {
        tx.commit().await.map_err(authority_error)?;
        return Ok(Json(saved));
    }
    let rows = owned_snapshot(tx.connection(), &tenant, &payload.appointments).await?;
    let optimized = order_route(
        rows,
        payload
            .current_location_lat
            .zip(payload.current_location_lng),
    );
    let route_id = if commit {
        if optimized.is_empty() {
            return Err(invalid("A saved route requires at least one appointment"));
        }
        let id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO service_routes(id,tenant_id,route_date,status,created_at)VALUES($1,$2,$3,'prepared',clock_timestamp())",
        )
        .bind(&id)
        .bind(&tenant)
        .bind(chrono::Utc::now().date_naive())
        .execute(tx.connection())
        .await
        .map_err(unavailable)?;
        for (index, appointment) in optimized.iter().enumerate() {
            let status = match appointment.status.to_ascii_lowercase().as_str() {
                "completed" | "done" => "done",
                "cancelled" | "canceled" => "cancelled",
                "en-route" => "en_route",
                "in-progress" => "on_site",
                _ => "pending",
            };
            sqlx::query("INSERT INTO job_locations(id,tenant_id,service_route_id,appointment_id,sequence_order,status)VALUES($1,$2,$3,$4,$5,$6)")
                .bind(uuid::Uuid::new_v4().to_string()).bind(&tenant).bind(&id).bind(&appointment.id).bind(index as i32).bind(status).execute(tx.connection()).await.map_err(unavailable)?;
        }
        Some(id)
    } else {
        None
    };
    let response=OptimizeRouteResponse {success:true,committed:commit,route_id,optimized_route:optimized,agent_suggestion:Some(if commit {"Route saved for review. No staff assignment or customer notification was dispatched."}else{"Route preview only. No schedule changes or notifications have been saved or sent."}.into())};
    if let Some(receipt) = &receipt {
        receipt.save(tx.connection(), &response).await?;
    }
    tx.commit().await.map_err(authority_error)?;
    Ok(Json(response))
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
    pub committed: bool,
    #[serde(rename = "optimizedRoute")]
    pub optimized_route: Vec<Appointment>,
    #[serde(rename = "subsequentCount")]
    pub subsequent_count: i32,
    #[serde(rename = "agentSuggestion")]
    pub agent_suggestion: Option<String>,
}
pub async fn running_late(
    headers: HeaderMap,
    State(state): State<Arc<FieldOpsState>>,
    Extension(claims): Extension<server_common::Claims>,
    Json(payload): Json<RunningLateRequest>,
) -> Result<Json<RunningLateResponse>, FieldError> {
    let owner = state.access.authorize(&claims, &headers).await?;
    let tenant = owner.tenant_id().to_owned();
    let mut tx = owner.begin().await.map_err(authority_error)?;
    let mut rows = owned_snapshot(tx.connection(), &tenant, &payload.appointments).await?;
    let index = rows
        .iter()
        .position(|row| row.id == payload.delay_job_id)
        .ok_or_else(missing)?;
    let mut count = 0;
    for row in rows
        .iter_mut()
        .skip(index + 1)
        .filter(|row| !records::is_terminal(&row.status))
    {
        for time in [&mut row.scheduled_start_time, &mut row.scheduled_end_time] {
            if let Some(value) = *time {
                *time = Some(
                    value
                        .checked_add_signed(chrono::Duration::minutes(30))
                        .ok_or_else(|| invalid("Appointment time exceeds the supported range"))?,
                );
            }
        }
        count += 1;
    }
    tx.commit().await.map_err(authority_error)?;
    Ok(Json(RunningLateResponse {
        success: true,
        committed: false,
        optimized_route: rows,
        subsequent_count: count,
        agent_suggestion: Some(format!(
            "Review a 30-minute schedule change for {count} subsequent appointments. No customer notifications have been sent."
        )),
    }))
}
