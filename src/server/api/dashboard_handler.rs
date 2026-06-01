use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct DashboardSummary {
    pub total_orders: i64,
    pub total_revenue: f64,
    pub total_customers: i64,
    pub total_bookings: i64,
    pub active_ai_tasks: i64,
}

#[derive(Deserialize)]
pub struct DashboardRequest {
    pub tenant_id: String,
}

pub async fn get_dashboard_metrics(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<DashboardRequest>,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    let result = sqlx::query_as::<_, DashboardSummary>(
        r#"
        SELECT
            COALESCE(total_orders, 0)::bigint as total_orders,
            COALESCE(total_revenue, 0)::float8 as total_revenue,
            COALESCE(total_customers, 0)::bigint as total_customers,
            COALESCE(total_bookings, 0)::bigint as total_bookings,
            COALESCE(active_ai_tasks, 0)::bigint as active_ai_tasks
        FROM dashboard_summary
        WHERE tenant_id = $1
        "#
    )
    .bind(payload.tenant_id)
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(Some(summary)) => {
            let json = serde_json::json!({
                "total_sales": summary.total_revenue,
                "active_customers": summary.total_customers,
                "total_orders": summary.total_orders,
                "total_bookings": summary.total_bookings,
                "active_ai_tasks": summary.active_ai_tasks,
            });
            (axum::http::StatusCode::OK, Json(json)).into_response()
        }
        Ok(None) => {
            // Return empty/zeroed state if no summary exists for the tenant
            let json = serde_json::json!({
                "total_sales": 0.0,
                "active_customers": 0,
                "total_orders": 0,
                "total_bookings": 0,
                "active_ai_tasks": 0,
            });
            (axum::http::StatusCode::OK, Json(json)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch dashboard summary: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Internal Server Error"}))).into_response()
        }
    }
}
