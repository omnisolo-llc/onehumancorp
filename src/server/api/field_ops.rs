use axum::{
    Json, Router,
    extract::{Extension, State},
    http::HeaderMap,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

pub mod appointments;
mod planning;
pub mod records;
pub use appointments::Appointment;
pub use planning::{
    OptimizeRouteRequest, OptimizeRouteResponse, RunningLateRequest, RunningLateResponse,
    optimize_route, running_late,
};
pub use records::canonical_pool;
use records::{
    FieldAccess, FieldError, Receipt, authority_error, check_transition, conflict, coordinates,
    expected, invalid, missing, unavailable,
};

#[derive(Clone)]
pub struct FieldOpsState {
    pub pool: PgPool,
    pub mesh: Arc<dyn omnisolo_builtin_agent::mesh::transport::MeshTransport>,
    pub access: FieldAccess,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateAppointmentRequest {
    pub id: String,
    pub status: String,
    pub expected_updated_at: Option<DateTime<Utc>>,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub notes: Option<String>,
    pub scheduled_start_time: Option<DateTime<Utc>>,
    pub scheduled_end_time: Option<DateTime<Utc>>,
}
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UpdateAppointmentResponse {
    pub success: bool,
    pub id: String,
    pub status: String,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub notes: Option<String>,
    pub scheduled_start_time: Option<DateTime<Utc>>,
    pub scheduled_end_time: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct AppointmentVersion {
    status: String,
    updated_at: Option<DateTime<Utc>>,
    scheduled_start_time: Option<DateTime<Utc>>,
    scheduled_end_time: Option<DateTime<Utc>>,
}

pub async fn update_appointment(
    headers: HeaderMap,
    State(state): State<Arc<FieldOpsState>>,
    Extension(claims): Extension<server_common::Claims>,
    Json(payload): Json<UpdateAppointmentRequest>,
) -> Result<Json<UpdateAppointmentResponse>, FieldError> {
    let owner = state.access.authorize(&claims, &headers).await?;
    let tenant = owner.tenant_id().to_owned();
    let actor = owner.actor_id().to_owned();
    if !records::valid_id(&payload.id)
        || ![
            "Requested",
            "Pending",
            "Scheduled",
            "Confirmed",
            "En-Route",
            "In-Progress",
            "Completed",
            "Cancelled",
        ]
        .contains(&payload.status.as_str())
    {
        return Err(invalid("Invalid appointment identifier or status"));
    }
    if payload
        .notes
        .as_ref()
        .is_some_and(|notes| notes.len() > 16_384 || notes.contains('\0'))
    {
        return Err(invalid("Appointment notes exceed the supported limit"));
    }
    coordinates(payload.location_lat, payload.location_lng)?;
    let observed = expected(payload.expected_updated_at)?;
    let receipt = Receipt::new(&headers, &owner, "appointment", &payload)?;
    let mut tx = owner.begin().await.map_err(authority_error)?;
    if let Some(receipt) = &receipt
        && let Some(saved) = receipt.replay(tx.connection()).await?
    {
        tx.commit().await.map_err(authority_error)?;
        return Ok(Json(saved));
    }
    let current = sqlx::query_as::<_,AppointmentVersion>(
        "SELECT status,updated_at,scheduled_start_time,scheduled_end_time FROM appointments WHERE id=$1 AND tenant_id=$2 FOR UPDATE"
    ).bind(&payload.id).bind(&tenant).fetch_optional(tx.connection()).await.map_err(unavailable)?;
    let AppointmentVersion {
        status: old_status,
        updated_at,
        scheduled_start_time: start,
        scheduled_end_time: end,
    } = current.ok_or_else(missing)?;
    if updated_at != Some(observed) {
        return Err(conflict());
    }
    check_transition(&old_status, &payload.status)?;
    let start = payload.scheduled_start_time.or(start);
    let end = payload.scheduled_end_time.or(end);
    if start.zip(end).is_some_and(|(start, end)| end < start) {
        return Err(invalid("Appointment end cannot precede its start"));
    }
    let saved = sqlx::query_as::<_, UpdateAppointmentResponse>(
        "UPDATE appointments SET status=$1,notes=COALESCE($2,notes),location_lat=COALESCE($3,location_lat),location_lng=COALESCE($4,location_lng),scheduled_start_time=$5,scheduled_end_time=$6,updated_at=GREATEST(clock_timestamp(),updated_at+INTERVAL '1 microsecond') WHERE id=$7 AND tenant_id=$8 AND updated_at=$9 RETURNING TRUE AS success,id,status,location_lat,location_lng,notes,scheduled_start_time,scheduled_end_time,updated_at"
    ).bind(&payload.status).bind(&payload.notes).bind(payload.location_lat).bind(payload.location_lng)
        .bind(start).bind(end).bind(&payload.id).bind(&tenant).bind(observed)
        .fetch_optional(tx.connection()).await.map_err(unavailable)?.ok_or_else(conflict)?;
    if payload.status == "Completed" && !old_status.eq_ignore_ascii_case("Completed") {
        sqlx::query("INSERT INTO department_tasks(id,tenant_id,department,event_type,payload,status)VALUES($1,$2,'operations','field_ops.job_completed',$3,'PENDING')")
            .bind(uuid::Uuid::new_v4().to_string()).bind(&tenant)
            .bind(serde_json::json!({"appointment_id":saved.id,"status":saved.status,"updated_at":saved.updated_at,"actor_id":actor,"message":"Owner recorded this appointment as completed. Any further external action requires its own authority."}))
            .execute(tx.connection()).await.map_err(unavailable)?;
    }
    if let Some(receipt) = &receipt {
        receipt.save(tx.connection(), &saved).await?;
    }
    tx.commit().await.map_err(authority_error)?;
    // Keep the historical flat event shape. Publish the caller's supplied fields,
    // never unrelated stored notes or coordinates returned by the record read.
    let mut notification = serde_json::to_value(&payload).map_err(unavailable)?;
    notification["tenant_id"] = serde_json::json!(tenant);
    notification["id"] = serde_json::json!(saved.id);
    notification["status"] = serde_json::json!(saved.status);
    notification["updated_at"] = serde_json::json!(saved.updated_at);
    let event = server_omnisolo::orchestration::TeammateMeshEvent {
        agent_id: actor,
        action: "job:status_changed".into(),
        status: "ok".into(),
        msg_id: uuid::Uuid::new_v4().to_string(),
        payload: serde_json::to_vec(&notification).map_err(unavailable)?,
    };
    if let Err(error) = state.mesh.publish("job:status_changed", event).await {
        tracing::warn!(%error, "Committed field status notification unavailable");
    }
    Ok(Json(saved))
}

pub fn router<S: Clone + Send + Sync + 'static>(
    pool: PgPool,
    mesh: Arc<dyn omnisolo_builtin_agent::mesh::transport::MeshTransport>,
    auth_store: Arc<server_auth::Store>,
) -> Router<S> {
    configured_router(pool.clone(), Some(pool), mesh, auth_store)
}
pub fn configured_router<S: Clone + Send + Sync + 'static>(
    pool: PgPool,
    canonical: Option<PgPool>,
    mesh: Arc<dyn omnisolo_builtin_agent::mesh::transport::MeshTransport>,
    auth_store: Arc<server_auth::Store>,
) -> Router<S> {
    let reads = appointments::optional_router(canonical.clone(), auth_store.clone());
    let state = Arc::new(FieldOpsState {
        pool,
        mesh,
        access: FieldAccess {
            pool: canonical,
            store: auth_store.clone(),
        },
    });
    Router::new()
        .route("/appointments", axum::routing::post(update_appointment))
        .route("/optimize-route", axum::routing::post(optimize_route))
        .route("/running-late", axum::routing::post(running_late))
        .route_layer(axum::middleware::from_fn_with_state(
            auth_store,
            server_auth::strict_bearer_auth_middleware,
        ))
        .with_state(state)
        .merge(reads)
        .layer(axum::extract::DefaultBodyLimit::max(256 * 1024))
        .layer(axum::middleware::map_response(records::private_response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    async fn assert_unsigned(method: &str, path: &str, body: &str) {
        let pool = PgPool::connect_lazy("postgres://invalid:invalid@localhost/invalid").unwrap();
        let mesh: Arc<dyn omnisolo_builtin_agent::mesh::transport::MeshTransport> =
            Arc::new(omnisolo_builtin_agent::mesh::transport::InProcessTransport::new());
        let app = router(pool, mesh, Arc::new(server_auth::Store::new()));
        let request = Request::builder()
            .method(method)
            .uri(path)
            .header("Content-Type", "application/json")
            .body(Body::from(body.to_owned()))
            .unwrap();
        assert_eq!(
            app.oneshot(request).await.unwrap().status(),
            StatusCode::UNAUTHORIZED
        );
    }
    #[tokio::test]
    async fn test_get_appointments_rejects_anonymous_before_database() {
        assert_unsigned("GET", "/appointments?tenant_id=t1", "null").await;
    }
    #[tokio::test]
    async fn optimizer_rejects_unsigned_requests_before_storage() {
        assert_unsigned(
            "POST",
            "/optimize-route",
            r#"{"tenantId":"t1","appointments":[]}"#,
        )
        .await;
    }
}
