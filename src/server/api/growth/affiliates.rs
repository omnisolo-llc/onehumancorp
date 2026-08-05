use axum::{extract::State, extract::Query, response::IntoResponse, Json};
use std::sync::Arc;
use crate::db::DB;
use crate::common::auth_utils::UiTenantQuery;
use crate::common::auth_utils::strict_ui_claim_tenant;

pub async fn get_affiliate_stats_handler(
    State(db): State<Arc<DB>>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    Query(_query): Query<UiTenantQuery>,
) -> impl IntoResponse {
    let tenant_id = match strict_ui_claim_tenant(&claims) {
        Some(id) => id,
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))).into_response(),
    };

    let total_affiliates = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM referral_codes WHERE tenant_id = $1")
                .bind(&tenant_id)
                .fetch_one(&db.pool)
                .await
                .unwrap_or(0)
        }
        _ => 0,
    };

    let total_commission_cents = 0; // Simulated for now

    Json(serde_json::json!({
        "total_affiliates": total_affiliates,
        "total_commission_cents": total_commission_cents,
    })).into_response()
}
