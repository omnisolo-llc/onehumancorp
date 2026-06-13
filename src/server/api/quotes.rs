use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{post, get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use crate::db::DB;

#[derive(Serialize, Deserialize)]
pub struct QuoteAcceptResponse {
    pub success: bool,
    pub project_id: Option<String>,
    pub invoice_id: Option<String>,
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    Router::new()
        .route("/:id/accept", post(accept_quote))
        .route("/projects", get(get_projects))
        .with_state(db)
}

async fn accept_quote(
    State(db): State<Arc<DB>>,
    Path(quote_id): Path<String>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(QuoteAcceptResponse { success: false, project_id: None, invoice_id: None })).into_response(),
    };

    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(QuoteAcceptResponse { success: false, project_id: None, invoice_id: None })).into_response();
        }
    };

    if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
    {
        tracing::error!("Failed to set tenant context: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(QuoteAcceptResponse { success: false, project_id: None, invoice_id: None })).into_response();
    }

    // 1. Mark quote as accepted
    let update_quote = sqlx::query("UPDATE quotes SET status = 'ACCEPTED', updated_at = $1 WHERE id = $2 AND tenant_id = $3")
        .bind(Utc::now())
        .bind(&quote_id)
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await;

    if update_quote.is_err() {
        tracing::error!("Failed to update quote status");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(QuoteAcceptResponse { success: false, project_id: None, invoice_id: None })).into_response();
    }

    // 2. Create Project
    let project_id = Uuid::new_v4().to_string();
    let insert_project = sqlx::query(
        "INSERT INTO projects (id, tenant_id, quote_id, title, status, created_at, updated_at) VALUES ($1, $2, $3, $4, 'PENDING', $5, $5)"
    )
    .bind(&project_id)
    .bind(&tenant_id)
    .bind(&quote_id)
    .bind("New Project from Quote")
    .bind(Utc::now())
    .execute(&mut *tx)
    .await;

    if insert_project.is_err() {
        tracing::error!("Failed to insert project");
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(QuoteAcceptResponse { success: false, project_id: None, invoice_id: None })).into_response();
    }

    // 3. Create Tasks
    let tasks = vec!["Review Requirements", "Schedule Kickoff", "Begin Execution", "Client Review"];
    for (_i, task_title) in tasks.iter().enumerate() {
        let task_id = Uuid::new_v4().to_string();
        let _ = sqlx::query(
            "INSERT INTO tasks (id, project_id, title, status, created_at, updated_at) VALUES ($1, $2, $3, 'PENDING', $4, $4)"
        )
        .bind(&task_id)
        .bind(&project_id)
        .bind(task_title)
        .bind(Utc::now())
        .execute(&mut *tx)
        .await;
    }

    // 4. Create Invoice for deposit
    let invoice_id = Uuid::new_v4().to_string();
    let insert_invoice = sqlx::query(
        "INSERT INTO invoices (id, tenant_id, client_id, client_name, status, due_date, currency, total_amount, stripe_invoice_id, stripe_payment_link, created_at, updated_at)
         VALUES ($1, $2, $3, $4, 'pending', $5, 'USD', $6, $7, $8, $9, $9)"
    )
    .bind(&invoice_id)
    .bind(&tenant_id)
    .bind("temp_client") // In real flow, fetch from quote
    .bind("Client") // Fetch from quote/customer
    .bind(Utc::now().timestamp() + 86400 * 7) // Due in 7 days
    .bind(500.0) // Fetch deposit amount from quote
    .bind("")
    .bind("")
    .bind(Utc::now().timestamp())
    .execute(&mut *tx)
    .await;

    if insert_invoice.is_err() {
        tracing::error!("Failed to insert invoice: {:?}", insert_invoice.err());
    }

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit transaction: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(QuoteAcceptResponse { success: false, project_id: None, invoice_id: None })).into_response();
    }

    (StatusCode::OK, Json(QuoteAcceptResponse { success: true, project_id: Some(project_id), invoice_id: Some(invoice_id) })).into_response()
}

#[derive(Serialize)]
pub struct ProjectDto {
    pub id: String,
    pub quote_id: Option<String>,
    pub title: String,
    pub status: String,
    pub tasks: Vec<TaskDto>,
}

#[derive(Serialize)]
pub struct TaskDto {
    pub id: String,
    pub title: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct ProjectsResponse {
    pub projects: Vec<ProjectDto>,
    pub invoices: Vec<InvoiceDto>,
}

#[derive(Serialize)]
pub struct InvoiceDto {
    pub id: String,
    pub status: String,
    pub total_amount: f64,
}

async fn get_projects(
    State(db): State<Arc<DB>>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(ProjectsResponse { projects: vec![], invoices: vec![] })).into_response(),
    };

    // simplified fetch
    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ProjectsResponse { projects: vec![], invoices: vec![] })).into_response(),
    };

    let _ = sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_id).execute(&mut *tx).await;

    use sqlx::Row;

    let mut projects = vec![];
    let proj_rows = sqlx::query("SELECT id, quote_id, title, status FROM projects WHERE tenant_id = $1 ORDER BY created_at DESC")
        .bind(&tenant_id)
        .fetch_all(&mut *tx)
        .await
        .unwrap_or_default();

    for row in proj_rows {
        let p_id: String = row.get("id");
        let task_rows = sqlx::query("SELECT id, title, status FROM tasks WHERE project_id = $1")
            .bind(&p_id)
            .fetch_all(&mut *tx)
            .await
            .unwrap_or_default();

        let tasks = task_rows.into_iter().map(|tr| TaskDto {
            id: tr.get("id"),
            title: tr.get("title"),
            status: tr.get("status"),
        }).collect();

        projects.push(ProjectDto {
            id: p_id,
            quote_id: row.try_get("quote_id").ok(),
            title: row.get("title"),
            status: row.get("status"),
            tasks,
        });
    }

    let inv_rows = sqlx::query("SELECT id, status, total_amount FROM invoices WHERE tenant_id = $1 ORDER BY created_at DESC")
        .bind(&tenant_id)
        .fetch_all(&mut *tx)
        .await
        .unwrap_or_default();

    let invoices = inv_rows.into_iter().map(|ir| InvoiceDto {
        id: ir.get("id"),
        status: ir.get("status"),
        total_amount: ir.get("total_amount"),
    }).collect();

    let _ = tx.commit().await;

    (StatusCode::OK, Json(ProjectsResponse { projects, invoices })).into_response()
}
