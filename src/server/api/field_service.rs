use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct FieldServiceJob {
    pub id: String,
    pub customer_id: Option<String>,
    pub customer_name: String,
    pub service_requested: String,
    pub status: String,
    pub notes: Option<String>,
    pub scheduled_at: String,
    pub location: Option<String>,
}

#[derive(Serialize)]
pub struct RosterResponse {
    pub success: bool,
    pub jobs: Vec<FieldServiceJob>,
}

pub async fn get_roster_handler(
    State(db): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id") {
        Some(t) => t.to_str().unwrap_or("default").to_string(),
        None => "default".to_string(),
    };

    // The Operations Agent analyzes locations and jobs to suggest the most efficient route.
    let query = "SELECT id, customer_id, customer_name, service_requested, status, notes, scheduled_at::text as scheduled_at, location FROM field_service_jobs WHERE tenant_id = $1 ORDER BY scheduled_at ASC";

    match sqlx::query_as::<_, FieldServiceJob>(query)
    .bind(&tenant_id)
    .fetch_all(&db)
    .await {
        Ok(jobs) => (StatusCode::OK, Json(RosterResponse { success: true, jobs })).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch field service jobs: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(RosterResponse { success: false, jobs: vec![] })).into_response()
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct JobSyncMutation {
    pub id: String,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SyncRequest {
    pub mutations: Vec<JobSyncMutation>,
}

#[derive(Serialize)]
pub struct SyncResponse {
    pub success: bool,
}

pub async fn sync_handler(
    State(db): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<SyncRequest>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id") {
        Some(t) => t.to_str().unwrap_or("default").to_string(),
        None => "default".to_string(),
    };

    let mut db_tx = match db.begin().await {
        Ok(tx) => tx,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(SyncResponse { success: false })).into_response(),
    };

    for mutation in payload.mutations {
        let _ = sqlx::query("UPDATE field_service_jobs SET status = $1, notes = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND tenant_id = $4")
            .bind(&mutation.status)
            .bind(&mutation.notes)
            .bind(&mutation.id)
            .bind(&tenant_id)
            .execute(&mut *db_tx)
            .await;

        if mutation.status == "COMPLETED" {
            // "The Sales Agent reads unstructured notes and drafts a professional estimate."
            let event_payload = serde_json::json!({
                "job_id": mutation.id,
                "notes": mutation.notes.unwrap_or_default()
            }).to_string();

            let _ = sqlx::query(
                "INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status)
                 VALUES ($1, $2, 'sales', 'field_service.job.completed', $3::jsonb, 'PENDING')"
            )
            .bind(uuid::Uuid::new_v4().to_string())
            .bind(&tenant_id)
            .bind(&event_payload)
            .execute(&mut *db_tx)
            .await;
        }
    }

    match db_tx.commit().await {
        Ok(_) => (StatusCode::OK, Json(SyncResponse { success: true })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(SyncResponse { success: false })).into_response(),
    }
}

// Handler for fetching if the Sales Agent has completed drafting an estimate for the job.
// We check if the department_tasks table has the corresponding task completed.
#[derive(Serialize)]
pub struct DraftEstimateResponse {
    pub success: bool,
    pub is_ready: bool,
}

pub async fn check_draft_estimate_handler(
    State(db): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    axum::extract::Path(job_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id") {
        Some(t) => t.to_str().unwrap_or("default").to_string(),
        None => "default".to_string(),
    };

    let query = "SELECT id FROM department_tasks WHERE tenant_id = $1 AND department = 'sales' AND event_type = 'field_service.job.completed' AND payload->>'job_id' = $2 LIMIT 1";
    let exists = sqlx::query(query)
        .bind(&tenant_id)
        .bind(&job_id)
        .fetch_optional(&db)
        .await
        .unwrap_or(None);

    (StatusCode::OK, Json(DraftEstimateResponse { success: true, is_ready: exists.is_some() })).into_response()
}
