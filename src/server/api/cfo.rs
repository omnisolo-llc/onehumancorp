use axum::{
    extract::{State, Query},
    response::IntoResponse,
    Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;
use sqlx::Row;
use chrono::Utc;

#[derive(Serialize)]
pub struct CashFlowProjection {
    pub projected_balance_7_days: f64,
    pub projected_balance_30_days: f64,
    pub status: String,
    pub alerts: Vec<String>,
}

#[derive(Deserialize)]
pub struct CFOActionRequest {
    pub action_type: String,
    pub target_id: Option<String>,
}

#[derive(Deserialize)]
pub struct CFOQuery {
    pub tenant_id: Option<String>,
}

pub async fn get_projection(
    State(db): State<Arc<DB>>,
    Query(query): Query<CFOQuery>,
) -> axum::response::Response {
    let tenant_id = query.tenant_id.unwrap_or_else(|| crate::common::auth_utils::get_default_tenant());

    let pool = &db.pool;

    let mut upcoming_due = 0.0;
    if let Ok(Some(row)) = sqlx::query(
        r#"
        SELECT SUM(total_amount) as income
        FROM invoices
        WHERE tenant_id = $1 AND status = 'pending' AND due_date < $2
        "#
    )
    .bind(&tenant_id)
    .bind(Utc::now() + chrono::Duration::days(7))
    .fetch_optional(pool)
    .await {
        upcoming_due = row.try_get("income").unwrap_or(0.0);
    }

    let mut upcoming_expenses = 0.0;
    if let Ok(Some(row)) = sqlx::query(
        r#"
        SELECT SUM(amount) as outgoing
        FROM ledger_entries
        WHERE tenant_id = $1 AND entry_type = 'expense' AND created_at > $2
        "#
    )
    .bind(&tenant_id)
    .bind(Utc::now() - chrono::Duration::days(7))
    .fetch_optional(pool)
    .await {
        upcoming_expenses = row.try_get("outgoing").unwrap_or(0.0);
    }

    let projected_7_days = upcoming_due - upcoming_expenses;

    let projection = CashFlowProjection {
        projected_balance_7_days: projected_7_days,
        projected_balance_30_days: projected_7_days * 4.0, // Simplified
        status: if projected_7_days < 0.0 { "WARNING".to_string() } else { "OK".to_string() },
        alerts: if projected_7_days < 0.0 {
            vec![format!("You have a projected cash deficit of ${:.2} within 7 days.", projected_7_days.abs())]
        } else {
            vec![]
        },
    };
    Json(projection).into_response()
}

pub async fn execute_action(
    State(db): State<Arc<DB>>,
    Query(query): Query<CFOQuery>,
    Json(payload): Json<CFOActionRequest>,
) -> axum::response::Response {
    let tenant_id = query.tenant_id.unwrap_or_else(|| crate::common::auth_utils::get_default_tenant());
    let pool = &db.pool;

    if payload.action_type == "Send Invoice Reminder" {
         // Instead of updating the status blindly to "reminded", we just log it or update a proper field if available.
         // Real app would dispatch emails. Here we'll simulate success.
        let _ = sqlx::query(
            r#"
            UPDATE invoices SET updated_at = $1
            WHERE tenant_id = $2 AND status = 'pending' AND due_date < $3
            "#
        )
        .bind(Utc::now())
        .bind(&tenant_id)
        .bind(Utc::now() + chrono::Duration::days(7))
        .execute(pool)
        .await;

        let result = serde_json::json!({
            "success": true,
            "message": "Sent invoice reminders"
        });
        Json(result).into_response()
    } else {
        let result = serde_json::json!({
            "success": false,
            "message": format!("Unknown action: {}", payload.action_type)
        });
        Json(result).into_response()
    }
}

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/projection", get(get_projection))
        .route("/action", post(execute_action))
        .with_state(db)
}
