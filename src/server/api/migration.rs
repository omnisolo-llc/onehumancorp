use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;
use uuid::Uuid;
use sqlx::Row;

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    Router::new()
        .route("/", post(migration_handler))
        .route("/{id}/status", get(migration_status_handler))
        .with_state(db)
}

#[derive(Deserialize)]
pub struct MigrationRequest {
    pub url: String,
    pub platform: String,
}

#[derive(Serialize)]
pub struct MigrationResponse {
    pub id: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct MigrationStatusResponse {
    pub id: String,
    pub status: String,
    pub metrics: Option<serde_json::Value>,
}

async fn migration_handler(
    State(db): State<Arc<DB>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<MigrationRequest>,
) -> Result<Json<MigrationResponse>, axum::http::StatusCode> {
    let tenant_id = headers
        .get("X-Tenant-ID")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default_tenant");

    let id = Uuid::new_v4().to_string();

    match &db.store {
        crate::db::DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
            crate::common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

            sqlx::query("INSERT INTO platform_migrations (id, tenant_id, source_url, platform_type, status) VALUES ($1, $2, $3, $4, 'pending')")
                .bind(&id)
                .bind(tenant_id)
                .bind(&payload.url)
                .bind(&payload.platform)
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to insert migration: {}", e);
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR
                })?;
            tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        }
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("INSERT INTO platform_migrations (id, tenant_id, source_url, platform_type, status) VALUES (?, ?, ?, ?, 'pending')")
                .bind(&id)
                .bind(tenant_id)
                .bind(&payload.url)
                .bind(&payload.platform)
                .execute(pool)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to insert migration sqlite: {}", e);
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR
                })?;
        }
    }

    Ok(Json(MigrationResponse {
        id,
        status: "pending".to_string(),
    }))
}

async fn migration_status_handler(
    State(db): State<Arc<DB>>,
    Path(id): Path<String>,
) -> Result<Json<MigrationStatusResponse>, axum::http::StatusCode> {
    let (status, metrics_str) = match &db.store {
        crate::db::DbStore::Postgres => {
            let row = sqlx::query("SELECT id, status, metrics FROM platform_migrations WHERE id = $1")
                .bind(&id)
                .fetch_optional(&db.pool)
                .await
                .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
            if let Some(r) = row {
                let status: String = r.get("status");
                let metrics_str: Option<String> = r.try_get("metrics").ok();
                (status, metrics_str)
            } else {
                return Err(axum::http::StatusCode::NOT_FOUND);
            }
        }
        crate::db::DbStore::Sqlite(pool) => {
            let row = sqlx::query("SELECT id, status, metrics FROM platform_migrations WHERE id = ?")
                .bind(&id)
                .fetch_optional(pool)
                .await
                .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
            if let Some(r) = row {
                let status: String = r.get("status");
                let metrics_str: Option<String> = r.try_get("metrics").ok();
                (status, metrics_str)
            } else {
                return Err(axum::http::StatusCode::NOT_FOUND);
            }
        }
    };

    let metrics = metrics_str.and_then(|s| serde_json::from_str(&s).ok());

    Ok(Json(MigrationStatusResponse {
        id,
        status,
        metrics,
    }))
}
