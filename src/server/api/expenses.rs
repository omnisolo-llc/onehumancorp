use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::get_pool;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpenseReceipt {
    pub id: String,
    pub tenant_id: String,
    pub image_path: Option<String>,
    pub vendor: Option<String>,
    pub amount: Option<f64>,
    pub category: Option<String>,
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct UploadExpenseRequest {
    pub image_path: Option<String>,
    pub vendor: Option<String>,
    pub amount: Option<f64>,
    pub date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateExpenseRequest {
    pub vendor: Option<String>,
    pub amount: Option<f64>,
    pub category: Option<String>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

pub struct AppState {
    pub db: sqlx::PgPool,
}

pub fn router(db: sqlx::PgPool) -> Router {
    let state = Arc::new(AppState { db });

    Router::new()
        .route("/api/v1/tenants/:tenant_id/expenses", post(upload_expense).get(list_expenses))
        .route("/api/v1/tenants/:tenant_id/expenses/:expense_id", put(update_expense))
        .with_state(state)
}

async fn upload_expense(
    Path(tenant_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UploadExpenseRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let id = Uuid::new_v4().to_string();
    let status = "pending".to_string();

    let query = r#"
        INSERT INTO ohc_expense_receipts (id, tenant_id, image_path, vendor, amount, date, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, tenant_id, image_path, vendor, amount, category, date, status, notes, created_at, updated_at
    "#;

    let expense = sqlx::query_as::<_, ExpenseReceipt>(query)
        .bind(&id)
        .bind(&tenant_id)
        .bind(&payload.image_path)
        .bind(&payload.vendor)
        .bind(&payload.amount)
        .bind(&payload.date)
        .bind(&status)
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // In a real implementation, we would queue a job here for the Finance Agent (The Accountant)
    // to process the receipt via OCR (e.g., Gemini Pro Vision) and categorize it.

    Ok((StatusCode::CREATED, Json(expense)))
}

async fn list_expenses(
    Path(tenant_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let query = r#"
        SELECT id, tenant_id, image_path, vendor, amount, category, date, status, notes, created_at, updated_at
        FROM ohc_expense_receipts
        WHERE tenant_id = $1
        ORDER BY created_at DESC
    "#;

    let expenses = sqlx::query_as::<_, ExpenseReceipt>(query)
        .bind(&tenant_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(expenses))
}

async fn update_expense(
    Path((tenant_id, expense_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateExpenseRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let query = r#"
        UPDATE ohc_expense_receipts
        SET vendor = COALESCE($1, vendor),
            amount = COALESCE($2, amount),
            category = COALESCE($3, category),
            status = COALESCE($4, status),
            notes = COALESCE($5, notes),
            updated_at = NOW()
        WHERE id = $6 AND tenant_id = $7
        RETURNING id, tenant_id, image_path, vendor, amount, category, date, status, notes, created_at, updated_at
    "#;

    let expense = sqlx::query_as::<_, ExpenseReceipt>(query)
        .bind(&payload.vendor)
        .bind(&payload.amount)
        .bind(&payload.category)
        .bind(&payload.status)
        .bind(&payload.notes)
        .bind(&expense_id)
        .bind(&tenant_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // If status is updated to 'reconciled', we would also write this to the Multi-Tenant Ledger
    // in a real implementation.

    Ok(Json(expense))
}
