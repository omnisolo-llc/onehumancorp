//! Authoritative tenant-scoped order reads. Lists remain bounded; detail is
//! selected by its ID in SQL and never inferred from presence in a recent list.
use crate::db::DB;
use axum::{
    Json, Router,
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use sqlx::Row;
use std::sync::Arc;

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    Router::new()
        .route("/api/v1/ui/orders", get(list))
        .route("/api/v1/ui/orders/{order_id}", get(detail))
        .with_state(db)
}

fn private_response(status: StatusCode, body: serde_json::Value) -> Response {
    (
        status,
        [
            ("cache-control", "private, no-store"),
            ("vary", "Authorization"),
        ],
        Json(body),
    )
        .into_response()
}

pub async fn list(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<::server_common::Claims>,
    Query(query): Query<::server_common::auth_utils::UiTenantQuery>,
) -> Response {
    let Some(tenant_id) = ::server_common::auth_utils::signed_tenant_id(&claims) else {
        return private_response(
            StatusCode::UNAUTHORIZED,
            serde_json::json!({"error":"authentication required"}),
        );
    };
    match load(
        &db,
        &tenant_id,
        query.mobile_optimized.unwrap_or(false),
        None,
    )
    .await
    {
        Ok(orders) => private_response(
            StatusCode::OK,
            ::server_utils::payload_shaper::shape_payload(
                serde_json::Value::Array(orders),
                query.fields.as_deref(),
            ),
        ),
        Err(error) => {
            ::server_telemetry::record_error_signal("[bug] Failed to fetch UI orders");
            tracing::error!(%error, "Failed to fetch UI orders");
            private_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::json!({"error":"Order data is unavailable"}),
            )
        }
    }
}

pub async fn detail(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(order_id): Path<String>,
) -> Response {
    let Some(tenant_id) = ::server_common::auth_utils::signed_tenant_id(&claims) else {
        return private_response(
            StatusCode::UNAUTHORIZED,
            serde_json::json!({"error":"authentication required"}),
        );
    };
    match load(&db, &tenant_id, false, Some(&order_id)).await {
        Ok(orders) => match orders.into_iter().next() {
            Some(order) => private_response(StatusCode::OK, order),
            None => private_response(
                StatusCode::NOT_FOUND,
                serde_json::json!({"error":"Order not found"}),
            ),
        },
        Err(error) => {
            ::server_telemetry::record_error_signal("[bug] Failed to fetch UI order detail");
            tracing::error!(%error, "Failed to fetch UI order detail");
            private_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::json!({"error":"Order data is unavailable"}),
            )
        }
    }
}

pub async fn load(
    db: &crate::db::DB,
    tenant_id: &str,
    mobile_optimized: bool,
    order_id: Option<&str>,
) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    match &db.store {
        crate::db::DbStore::Postgres => {
            let mut tx = db.pool.begin().await?;
            ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await?;
            let res = if mobile_optimized {
                sqlx::query("SELECT o.id, CAST(COALESCE(o.total_amount, 0.0) AS DOUBLE PRECISION) AS total_amount, COALESCE(o.status, '') AS status FROM orders o WHERE o.tenant_id = $1 AND ($2::text IS NULL OR o.id = $2) ORDER BY o.created_at DESC, o.id DESC LIMIT 50")
                    .bind(tenant_id)
                    .bind(order_id)
                    .fetch_all(&mut *tx)
                    .await.map(|rows| rows.into_iter().map(|row| {
                        serde_json::json!({
                            "id": row.get::<String, _>("id"),
                            "total_amount": row.get::<f64, _>("total_amount"),
                            "status": row.get::<String, _>("status"),
                        })
                    }).collect())
            } else {
                sqlx::query("SELECT o.id, COALESCE(c.name, '') AS customer_name, CAST(COALESCE(o.total_amount, 0.0) AS DOUBLE PRECISION) AS total_amount, COALESCE(o.status, '') AS status, COALESCE(o.created_at::text, '') AS created_at FROM orders o LEFT JOIN customers c ON c.id = o.customer_id AND c.tenant_id = o.tenant_id WHERE o.tenant_id = $1 AND ($2::text IS NULL OR o.id = $2) ORDER BY o.created_at DESC, o.id DESC LIMIT 50")
                    .bind(tenant_id)
                    .bind(order_id)
                    .fetch_all(&mut *tx)
                    .await.map(|rows| rows.into_iter().map(|row| {
                        serde_json::json!({
                            "id": row.get::<String, _>("id"),
                            "customer_name": row.get::<String, _>("customer_name"),
                            "total_amount": row.get::<f64, _>("total_amount"),
                            "status": row.get::<String, _>("status"),
                            "created_at": row.get::<String, _>("created_at")
                        })
                    }).collect())
            };
            tx.commit().await?;
            res
        }
        crate::db::DbStore::Sqlite(pool) => {
            if mobile_optimized {
                sqlx::query("SELECT o.id, CAST(COALESCE(o.total_amount, 0.0) AS REAL) AS total_amount, COALESCE(o.status, '') AS status FROM orders o WHERE o.tenant_id = ?1 AND (?2 IS NULL OR o.id = ?2) ORDER BY o.created_at DESC, o.id DESC LIMIT 50")
                    .bind(tenant_id)
                    .bind(order_id)
                    .fetch_all(pool)
                    .await.map(|rows| rows.into_iter().map(|row| {
                        serde_json::json!({
                            "id": row.get::<String, _>("id"),
                            "total_amount": row.get::<f64, _>("total_amount"),
                            "status": row.get::<String, _>("status"),
                        })
                    }).collect())
            } else {
                sqlx::query("SELECT o.id, COALESCE(c.name, '') AS customer_name, CAST(COALESCE(o.total_amount, 0.0) AS REAL) AS total_amount, COALESCE(o.status, '') AS status, COALESCE(CAST(o.created_at AS TEXT), '') AS created_at FROM orders o LEFT JOIN customers c ON c.id = o.customer_id AND c.tenant_id = o.tenant_id WHERE o.tenant_id = ?1 AND (?2 IS NULL OR o.id = ?2) ORDER BY o.created_at DESC, o.id DESC LIMIT 50")
                    .bind(tenant_id)
                    .bind(order_id)
                    .fetch_all(pool)
                    .await.map(|rows| rows.into_iter().map(|row| {
                        serde_json::json!({
                            "id": row.get::<String, _>("id"),
                            "customer_name": row.get::<String, _>("customer_name"),
                            "total_amount": row.get::<f64, _>("total_amount"),
                            "status": row.get::<String, _>("status"),
                            "created_at": row.get::<String, _>("created_at")
                        })
                    }).collect())
            }
        }
    }
}
