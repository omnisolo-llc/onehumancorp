//! Read-only appointments bound to the verified bearer tenant.
use axum::{
    Json, Router,
    extract::{Extension, Query, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Appointment {
    pub id: String,
    pub customer_id: String,
    pub customer_name: String,
    pub job_template_id: String,
    pub job_name: String,
    pub status: String,
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
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
    pub tenant_id: Option<String>,
    pub mobile_optimized: Option<bool>,
}
#[derive(Serialize)]
pub struct GetAppointmentsResponse {
    pub appointments: Vec<Appointment>,
}

pub fn router<S: Clone + Send + Sync + 'static>(
    pool: PgPool,
    store: Arc<server_auth::Store>,
) -> Router<S> {
    optional_router(Some(pool), store)
}
pub fn optional_router<S: Clone + Send + Sync + 'static>(
    pool: Option<PgPool>,
    store: Arc<server_auth::Store>,
) -> Router<S> {
    Router::new()
        .route("/appointments", get(get_appointments))
        .route_layer(axum::middleware::from_fn_with_state(
            store,
            server_auth::strict_bearer_auth_middleware,
        ))
        .with_state(pool)
}
fn failure(status: StatusCode, message: &'static str) -> Response {
    (
        status,
        [(header::CACHE_CONTROL, "private, no-store")],
        Json(serde_json::json!({"error":message})),
    )
        .into_response()
}
pub async fn get_appointments(
    State(pool): State<Option<PgPool>>,
    Extension(claims): Extension<server_common::Claims>,
    Query(query): Query<GetAppointmentsQuery>,
) -> Response {
    let Some(tenant) = server_common::auth_utils::signed_tenant_id(&claims) else {
        return failure(
            StatusCode::FORBIDDEN,
            "A signed business identity is required",
        );
    };
    if query
        .tenant_id
        .as_ref()
        .is_some_and(|supplied| supplied != &tenant)
    {
        return failure(
            StatusCode::FORBIDDEN,
            "Appointment tenant does not match the current session",
        );
    }
    let Some(pool) = pool else {
        return failure(
            StatusCode::SERVICE_UNAVAILABLE,
            "Canonical appointment storage is unavailable",
        );
    };
    match read_appointments(&pool, &tenant, query.mobile_optimized.unwrap_or(false)).await {
        Ok(appointments) => (
            [(header::CACHE_CONTROL, "private, no-store")],
            Json(GetAppointmentsResponse { appointments }),
        )
            .into_response(),
        Err(_) => {
            tracing::warn!("Tenant appointment read failed");
            failure(
                StatusCode::SERVICE_UNAVAILABLE,
                "Appointments are temporarily unavailable",
            )
        }
    }
}
async fn read_appointments(
    pool: &PgPool,
    tenant: &str,
    mobile: bool,
) -> Result<Vec<Appointment>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("SET TRANSACTION READ ONLY")
        .execute(&mut *tx)
        .await?;
    server_common::auth_utils::set_org_context(&mut *tx, tenant).await?;
    sqlx::query("SET LOCAL statement_timeout = '2s'")
        .execute(&mut *tx)
        .await?;
    let rows = sqlx::query_as::<_, Appointment>(
        r#"
        WITH latest_route AS (
            SELECT id FROM service_routes
            WHERE tenant_id=$1 AND route_date=$3 AND status IN ('prepared','active')
            ORDER BY created_at DESC NULLS LAST,id DESC LIMIT 1
        )
        SELECT a.id, COALESCE(c.id,'') AS customer_id, COALESCE(c.name,'') AS customer_name,
               COALESCE(jt.id,'') AS job_template_id, COALESCE(jt.name,'') AS job_name,
               a.status,a.updated_at,a.scheduled_start_time,a.scheduled_end_time,
               CASE WHEN $2 THEN NULL::text ELSE a.location_address END AS location_address,
               CASE WHEN $2 THEN NULL::double precision ELSE a.location_lat END AS location_lat,
               CASE WHEN $2 THEN NULL::double precision ELSE a.location_lng END AS location_lng,
               CASE WHEN $2 THEN NULL::text ELSE a.notes END AS notes
        FROM appointments a
        LEFT JOIN customers c ON c.id=a.customer_id AND c.tenant_id=a.tenant_id
        LEFT JOIN job_templates jt ON jt.id=a.job_template_id AND jt.tenant_id=a.tenant_id
        LEFT JOIN LATERAL (
            SELECT MIN(jl.sequence_order) AS sequence_order FROM job_locations jl
            JOIN latest_route lr ON lr.id=jl.service_route_id
            WHERE jl.appointment_id=a.id AND jl.tenant_id=a.tenant_id
        ) route ON true
        WHERE a.tenant_id=$1
        ORDER BY COALESCE(route.sequence_order,9999),a.scheduled_start_time ASC NULLS LAST,a.id
    "#,
    )
    .bind(tenant)
    .bind(mobile)
    .bind(Utc::now().date_naive())
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(rows)
}
