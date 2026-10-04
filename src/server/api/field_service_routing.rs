use crate::api::field_ops::records::{
    self, FieldAccess, FieldError, Receipt, authority_error, conflict, expected, invalid, missing,
    unavailable,
};
use crate::{db::DB, hub::Hub};
use axum::{
    Json, Router,
    extract::{Extension, Path, Query, State},
    http::HeaderMap,
    routing::{get, post},
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct JobLocation {
    pub id: String,
    pub customer_id: Option<String>,
    pub job_title: String,
    pub address: String,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub status: String,
    pub order_index: i32,
    pub updated_at: DateTime<Utc>,
    #[serde(skip)]
    pub service_route_id: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ServiceRoute {
    pub id: String,
    pub staff_id: Option<String>,
    pub route_date: NaiveDate,
    pub status: String,
    pub jobs: Vec<JobLocation>,
}
#[derive(Serialize)]
pub struct TodayRoutesResponse {
    pub routes: Vec<ServiceRoute>,
}
#[derive(Serialize, Deserialize)]
pub struct UpdateJobStatusRequest {
    pub status: String,
    pub expected_updated_at: Option<DateTime<Utc>>,
}
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UpdateJobStatusResponse {
    pub success: bool,
    pub error: Option<String>,
    pub id: String,
    pub status: String,
    pub updated_at: DateTime<Utc>,
}
#[derive(Clone)]
struct AppState {
    hub: Arc<Hub>,
    access: FieldAccess,
}
pub fn router<S>(
    db: Arc<DB>,
    hub: Arc<Hub>,
    auth_store: Arc<server_auth::Store>,
    canonical: Option<sqlx::PgPool>,
) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let _ = db;
    let state = AppState {
        hub,
        access: FieldAccess {
            pool: canonical,
            store: auth_store.clone(),
        },
    };
    Router::new()
        .route("/routes/today", get(get_today_routes))
        .route("/jobs/{id}/status", post(update_job_status))
        .route_layer(axum::middleware::from_fn_with_state(
            auth_store,
            server_auth::strict_bearer_auth_middleware,
        ))
        .with_state(state)
        .layer(axum::extract::DefaultBodyLimit::max(64 * 1024))
        .layer(axum::middleware::map_response(records::private_response))
}
#[derive(Deserialize)]
pub struct GetTodayRoutesQuery {
    pub mobile_optimized: Option<bool>,
}
async fn get_today_routes(
    headers: HeaderMap,
    State(state): State<AppState>,
    Extension(claims): Extension<server_common::Claims>,
    Query(query): Query<GetTodayRoutesQuery>,
) -> Result<Json<TodayRoutesResponse>, FieldError> {
    let owner = state.access.authorize(&claims, &headers).await?;
    let tenant = owner.tenant_id().to_owned();
    let mut tx = owner.begin().await.map_err(authority_error)?;
    // Read both parent and child rows from the same tenant-bound transaction.
    // Global cached relationship snapshots cannot authorize current tenant ownership.
    let routes:Vec<(String,Option<String>,NaiveDate,String)>=sqlx::query_as("SELECT id,agent_id,route_date,status FROM service_routes WHERE tenant_id=$1 AND route_date=$2 ORDER BY id FOR SHARE")
        .bind(&tenant).bind(Utc::now().date_naive()).fetch_all(tx.connection()).await.map_err(unavailable)?;
    let ids: Vec<_> = routes.iter().map(|row| row.0.clone()).collect();
    let malformed:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM job_locations j LEFT JOIN appointments a ON a.id=j.appointment_id AND a.tenant_id=j.tenant_id WHERE j.tenant_id=$1 AND j.service_route_id=ANY($2) AND a.id IS NULL)")
        .bind(&tenant).bind(&ids).fetch_one(tx.connection()).await.map_err(unavailable)?;
    if malformed {
        return Err(unavailable("Invalid stored field appointment relationship"));
    }
    let jobs=sqlx::query_as::<_,JobLocation>("SELECT j.id,c.id AS customer_id,COALESCE(t.name,'Service Job') AS job_title,COALESCE(a.location_address,'No Address Provided') AS address,CASE WHEN $3 THEN NULL::double precision ELSE a.location_lat END AS lat,CASE WHEN $3 THEN NULL::double precision ELSE a.location_lng END AS lng,a.scheduled_start_time AS scheduled_start,a.scheduled_end_time AS scheduled_end,j.status,j.sequence_order AS order_index,j.updated_at,j.service_route_id FROM job_locations j JOIN appointments a ON a.id=j.appointment_id AND a.tenant_id=j.tenant_id JOIN service_routes r ON r.id=j.service_route_id AND r.tenant_id=j.tenant_id LEFT JOIN job_templates t ON t.id=a.job_template_id AND t.tenant_id=a.tenant_id LEFT JOIN customers c ON c.id=a.customer_id AND c.tenant_id=a.tenant_id WHERE j.tenant_id=$1 AND j.service_route_id=ANY($2) ORDER BY j.service_route_id,j.sequence_order,j.id")
        .bind(&tenant).bind(&ids).bind(query.mobile_optimized.unwrap_or(false)).fetch_all(tx.connection()).await.map_err(unavailable)?;
    let mut grouped: HashMap<String, Vec<JobLocation>> = HashMap::new();
    for job in jobs {
        grouped
            .entry(job.service_route_id.clone())
            .or_default()
            .push(job);
    }
    let routes = routes
        .into_iter()
        .map(|(id, staff_id, route_date, status)| ServiceRoute {
            jobs: grouped.remove(&id).unwrap_or_default(),
            id,
            staff_id,
            route_date,
            status,
        })
        .collect();
    tx.commit().await.map_err(authority_error)?;
    Ok(Json(TodayRoutesResponse { routes }))
}
async fn update_job_status(
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(claims): Extension<server_common::Claims>,
    Json(payload): Json<UpdateJobStatusRequest>,
) -> Result<Json<UpdateJobStatusResponse>, FieldError> {
    let owner = state.access.authorize(&claims, &headers).await?;
    let tenant = owner.tenant_id().to_owned();
    let actor = owner.actor_id().to_owned();
    if !records::valid_id(&id) || !valid_status(&payload.status) {
        return Err(invalid("Invalid job identifier or status"));
    }
    let observed = expected(payload.expected_updated_at)?;
    let receipt = Receipt::new(
        &headers,
        &owner,
        "routing_job",
        &serde_json::json!({"id":id,"payload":payload}),
    )?;
    let mut tx = owner.begin().await.map_err(authority_error)?;
    if let Some(receipt) = &receipt
        && let Some(saved) = receipt.replay(tx.connection()).await?
    {
        tx.commit().await.map_err(authority_error)?;
        return Ok(Json(saved));
    }
    let row:Option<(String,DateTime<Utc>)>=sqlx::query_as("SELECT j.status,j.updated_at FROM job_locations j JOIN appointments a ON a.id=j.appointment_id AND a.tenant_id=j.tenant_id JOIN service_routes r ON r.id=j.service_route_id AND r.tenant_id=j.tenant_id WHERE j.id=$1 AND j.tenant_id=$2 FOR UPDATE OF j FOR SHARE OF a,r")
        .bind(&id).bind(&tenant).fetch_optional(tx.connection()).await.map_err(unavailable)?;
    let (current, updated_at) = row.ok_or_else(missing)?;
    if observed != updated_at {
        return Err(conflict());
    }
    records::check_transition(&current, &payload.status)?;
    let saved=sqlx::query_as::<_,UpdateJobStatusResponse>("UPDATE job_locations SET status=$1,updated_at=GREATEST(clock_timestamp(),updated_at+INTERVAL '1 microsecond') WHERE id=$2 AND tenant_id=$3 AND updated_at=$4 RETURNING TRUE AS success,NULL::text AS error,id,status,updated_at")
        .bind(&payload.status).bind(&id).bind(&tenant).bind(observed).fetch_optional(tx.connection()).await.map_err(unavailable)?.ok_or_else(conflict)?;
    if let Some(receipt) = &receipt {
        receipt.save(tx.connection(), &saved).await?;
    }
    tx.commit().await.map_err(authority_error)?;
    let event=server_omnisolo::orchestration::TeammateMeshEvent{agent_id:actor,action:"job_status_changed".into(),status:"ok".into(),msg_id:uuid::Uuid::new_v4().to_string(),payload:serde_json::to_vec(&serde_json::json!({"tenant_id":tenant,"job_id":saved.id,"status":saved.status,"updated_at":saved.updated_at})).map_err(unavailable)?};
    if let Err(error) = state
        .hub
        .publish_teammate_event("job_status_updates".into(), event)
        .await
    {
        tracing::warn!(%error,"Committed route status notification unavailable");
    }
    Ok(Json(saved))
}

fn valid_status(status: &str) -> bool {
    [
        "pending",
        "en_route",
        "on_site",
        "done",
        "cancelled",
        "completed",
    ]
    .contains(&status)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_statuses() {
        for status in [
            "pending",
            "en_route",
            "on_site",
            "done",
            "cancelled",
            "completed",
        ] {
            assert!(valid_status(status));
        }
        assert!(!valid_status("unknown_status"));
    }
    #[test]
    fn test_update_job_status_request() {
        let req: UpdateJobStatusRequest = serde_json::from_value(
            serde_json::json!({"status":"done","expected_updated_at":"2026-10-03T00:00:00Z"}),
        )
        .unwrap();
        assert_eq!(req.status, "done");
        assert_eq!(req.expected_updated_at.unwrap().timestamp(), 1790985600);
    }
}
