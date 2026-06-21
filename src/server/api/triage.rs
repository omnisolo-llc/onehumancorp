use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{post, get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use chrono::Utc;
use uuid::Uuid;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<crate::db::DB>,
}

#[derive(Debug, Deserialize)]
pub struct WebhookPayload {
    pub tenant_id: String,
    pub source: String,
    pub identifier: String,
    pub message: String,
}

pub fn router(db: Arc<crate::db::DB>) -> Router {
    let state = AppState {
        db,
    };

    Router::new()
        .route("/webhooks/omnichannel", post(handle_webhook))
        .route("/api/v1/triage/feed", get(get_action_required))
        .route("/api/v1/triage/feed/{item_id}/approve", post(approve_action))
        .with_state(state)
}

pub async fn handle_webhook(
    State(state): State<AppState>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    info!("Received omnichannel webhook from {} for tenant {}", payload.source, payload.tenant_id);

    let message_id = Uuid::new_v4().to_string();
    let job_id = Uuid::new_v4().to_string();

    let job_payload = serde_json::json!({
        "message_id": message_id,
        "content": payload.message,
        "source": payload.source,
        "customer_id": payload.identifier,
    });

    let result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'triage_ingestion', $3, 'PENDING')"
            )
            .bind(&job_id)
            .bind(&payload.tenant_id)
            .bind(job_payload.to_string())
            .execute(&state.db.pool).await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'triage_ingestion', ?, 'PENDING')"
            )
            .bind(&job_id)
            .bind(&payload.tenant_id)
            .bind(job_payload.to_string())
            .execute(pool).await.map(|_| ())
        }
    };

    match result {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"status": "queued"}))).into_response(),
        Err(e) => {
            error!("Failed to enqueue triage job: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to enqueue job"}))).into_response()
        }
    }
}

pub async fn get_action_required(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let tenant_id = match params.get("tenant_id") {
        Some(id) => id.clone(),
        None => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Missing tenant_id"}))).into_response()
    };

    let items = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "SELECT id, tenant_id, source_id, source_type, content, intent, status FROM triage_items WHERE tenant_id = $1 AND status = 'PENDING'"
            )
            .bind(&tenant_id)
            .fetch_all(&state.db.pool)
            .await.map(|rows| {
                rows.into_iter().map(|row| {
                    use sqlx::Row;
                    serde_json::json!({
                        "id": row.get::<String, _>("id"),
                        "tenant_id": row.get::<String, _>("tenant_id"),
                        "source_id": row.get::<String, _>("source_id"),
                        "source_type": row.get::<String, _>("source_type"),
                        "content": row.get::<String, _>("content"),
                        "intent": row.get::<String, _>("intent"),
                        "status": row.get::<String, _>("status"),
                    })
                }).collect::<Vec<_>>()
            })
        },
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query(
                "SELECT id, tenant_id, source_id, source_type, content, intent, status FROM triage_items WHERE tenant_id = ? AND status = 'PENDING'"
            )
            .bind(&tenant_id)
            .fetch_all(pool)
            .await.map(|rows| {
                rows.into_iter().map(|row| {
                    use sqlx::Row;
                    serde_json::json!({
                        "id": row.get::<String, _>("id"),
                        "tenant_id": row.get::<String, _>("tenant_id"),
                        "source_id": row.get::<String, _>("source_id"),
                        "source_type": row.get::<String, _>("source_type"),
                        "content": row.get::<String, _>("content"),
                        "intent": row.get::<String, _>("intent"),
                        "status": row.get::<String, _>("status"),
                    })
                }).collect::<Vec<_>>()
            })
        }
    };

    match items {
        Ok(items) => {
            (StatusCode::OK, Json(items)).into_response()
        },
        Err(e) => {
            error!("Failed to fetch triage items: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to fetch feed"}))).into_response()
        }
    }
}

pub async fn approve_action(
    State(state): State<AppState>,
    Path(item_id): Path<String>,
) -> impl IntoResponse {
    let result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "UPDATE triage_items SET status = 'APPROVED' WHERE id = $1 RETURNING id"
            )
            .bind(&item_id)
            .fetch_optional(&state.db.pool)
            .await.map(|o| o.is_some())
        },
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query(
                "UPDATE triage_items SET status = 'APPROVED' WHERE id = ? RETURNING id"
            )
            .bind(&item_id)
            .fetch_optional(pool)
            .await.map(|o| o.is_some())
        }
    };

    match result {
        Ok(true) => (StatusCode::OK, Json(serde_json::json!({"status": "approved"}))).into_response(),
        Ok(false) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Item not found"}))).into_response(),
        Err(e) => {
            error!("Failed to approve triage item: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to approve action"}))).into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use axum::body::Body;
    use tower::ServiceExt;
    use serde_json::json;
    use crate::db::DB;

    #[tokio::test]
    async fn test_webhook_enqueues_job() {
        let pool = crate::db::create_sqlite_pool_for_test().await;
        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://").unwrap(), // Mocked
            store: crate::db::DbStore::Sqlite(pool.clone()),
        });

        sqlx::query("CREATE TABLE ohc_job_queue (id TEXT PRIMARY KEY, tenant_id TEXT, job_type TEXT, payload TEXT, status TEXT, result TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, updated_at DATETIME DEFAULT CURRENT_TIMESTAMP, next_retry_at DATETIME DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool).await.unwrap();

        let app = router(db);

        let response = app
            .oneshot(
                Request::builder()
                    .method(axum::http::Method::POST)
                    .uri("/webhooks/omnichannel")
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({
                            "tenant_id": "tenant1",
                            "source": "ig_dm",
                            "identifier": "user123",
                            "message": "Hello world"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ohc_job_queue WHERE tenant_id = 'tenant1' AND status = 'PENDING'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_get_triage_feed() {
        let pool = crate::db::create_sqlite_pool_for_test().await;
        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://").unwrap(), // Mocked
            store: crate::db::DbStore::Sqlite(pool.clone()),
        });

        sqlx::query("CREATE TABLE triage_items (id TEXT PRIMARY KEY, tenant_id TEXT, source_id TEXT, source_type TEXT, content TEXT, intent TEXT, status TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool).await.unwrap();

        sqlx::query("INSERT INTO triage_items (id, tenant_id, source_id, source_type, content, intent, status) VALUES ('item1', 'tenant1', 'msg1', 'ig_dm', 'Need help', 'SUPPORT', 'PENDING')")
            .execute(&pool).await.unwrap();

        let app = router(db);

        let response = app
            .oneshot(
                Request::builder()
                    .method(axum::http::Method::GET)
                    .uri("/api/v1/triage/feed?tenant_id=tenant1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert!(body_json.is_array());
        assert_eq!(body_json.as_array().unwrap().len(), 1);
        assert_eq!(body_json[0]["id"], "item1");
    }

    #[tokio::test]
    async fn test_approve_action() {
        let pool = crate::db::create_sqlite_pool_for_test().await;
        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://").unwrap(), // Mocked
            store: crate::db::DbStore::Sqlite(pool.clone()),
        });

        sqlx::query("CREATE TABLE triage_items (id TEXT PRIMARY KEY, tenant_id TEXT, source_id TEXT, source_type TEXT, content TEXT, intent TEXT, status TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool).await.unwrap();

        sqlx::query("INSERT INTO triage_items (id, tenant_id, source_id, source_type, content, intent, status) VALUES ('item1', 'tenant1', 'msg1', 'ig_dm', 'Need help', 'SUPPORT', 'PENDING')")
            .execute(&pool).await.unwrap();

        let app = router(db);

        let response = app
            .oneshot(
                Request::builder()
                    .method(axum::http::Method::POST)
                    .uri("/api/v1/triage/feed/item1/approve")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let status: String = sqlx::query_scalar("SELECT status FROM triage_items WHERE id = 'item1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(status, "APPROVED");
    }
}
