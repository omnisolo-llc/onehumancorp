use axum::{extract::{Path, Extension}, response::IntoResponse, Json};
use serde_json::json;
use crate::common::auth_utils::set_org_context;


pub async fn customer_360_handler(
    Path(customer_id): Path<String>,
    Extension(user): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let pool = crate::db::get_pool();
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "DB error"}))).into_response();
        }
    };

    let org_id = user.organization_id.unwrap_or_default();
    if let Err(e) = set_org_context(&mut *tx, &org_id).await {
        tracing::error!("Failed to set org context: {}", e);
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "DB error"}))).into_response();
    }


    let resp = crate::domain::repository::customer_360::Customer360Repository::get_customer_360(&mut tx, &org_id, &customer_id).await;
    match resp {
        Ok(Some(data)) => {
            let _ = tx.commit().await;
            (axum::http::StatusCode::OK, Json(data)).into_response()
        },
        Ok(None) => (axum::http::StatusCode::NOT_FOUND, Json(json!({"error": "Customer not found"}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch customer 360 data: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "DB error"}))).into_response()
        }
    }
}
