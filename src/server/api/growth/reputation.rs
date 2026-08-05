use axum::{extract::State, extract::Query, response::IntoResponse, Json};
use std::sync::Arc;
use crate::db::DB;
use crate::common::auth_utils::UiTenantQuery;
use crate::common::auth_utils::strict_ui_claim_tenant;

pub async fn get_reputation_stats_handler(
    State(db): State<Arc<DB>>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    Query(_query): Query<UiTenantQuery>,
) -> impl IntoResponse {
    let tenant_id = match strict_ui_claim_tenant(&claims) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))).into_response(),
    };

    let (average_rating, total_reviews) = match &db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query_as::<_, (Option<f64>, i64)>("SELECT average_rating, total_reviews FROM reputation_profiles WHERE tenant_id = $1")
                .bind(&tenant_id)
                .fetch_one(&db.pool)
                .await;
            match res {
                Ok((Some(rating), count)) => (rating, count),
                Ok((None, count)) => (0.0, count),
                Err(_) => (0.0, 0),
            }
        }
        _ => (0.0, 0),
    };

    Json(serde_json::json!({
        "average_rating": average_rating,
        "total_reviews": total_reviews,
    })).into_response()
}
