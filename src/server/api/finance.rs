use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{post},
    Json, Router, Extension,
};
use serde::{Deserialize, Serialize};
use crate::db::DB;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptMicroLoanRequest {
    pub loan_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptMicroLoanResponse {
    pub success: bool,
    pub message: String,
}

pub async fn accept_micro_loan_handler(
    Extension(user): Extension<::server_common::Claims>,
    Extension(db): Extension<Arc<DB>>,
    Json(payload): Json<AcceptMicroLoanRequest>,
) -> impl IntoResponse {
    let tenant_id = user.organization_id.unwrap_or_default();

    let now = chrono::Utc::now();
    let rows_affected = match &db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                "UPDATE micro_loans SET status = 'ACCEPTED', accepted_at = $1 WHERE id = $2 AND tenant_id = $3 AND status = 'PENDING'"
            )
            .bind(now)
            .bind(&payload.loan_id)
            .bind(&tenant_id)
            .execute(&db.pool).await;
            res.map(|r| r.rows_affected())
        },
        crate::db::DbStore::Sqlite(pool) => {
            let res = sqlx::query(
                "UPDATE micro_loans SET status = 'ACCEPTED', accepted_at = ? WHERE id = ? AND tenant_id = ? AND status = 'PENDING'"
            )
            .bind(now.format("%Y-%m-%d %H:%M:%S").to_string())
            .bind(&payload.loan_id)
            .bind(&tenant_id)
            .execute(pool).await;
            res.map(|r| r.rows_affected())
        }
    };

    match rows_affected {
        Ok(count) => {
            if count > 0 {
                (StatusCode::OK, Json(AcceptMicroLoanResponse { success: true, message: "Loan accepted".to_string() })).into_response()
            } else {
                (StatusCode::NOT_FOUND, Json(AcceptMicroLoanResponse { success: false, message: "Loan not found or already accepted".to_string() })).into_response()
            }
        },
        Err(e) => {
            tracing::error!("Failed to accept loan: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(AcceptMicroLoanResponse { success: false, message: "Database error".to_string() })).into_response()
        }
    }
}
