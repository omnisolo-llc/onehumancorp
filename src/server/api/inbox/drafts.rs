use axum::{
    extract::{State, Extension, Query},
    response::IntoResponse,
    http::StatusCode,
    routing::get,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::db::{DB, DbStore};
use server_common::Claims;

#[derive(Clone)]
pub struct DraftsState {
    pub db: Arc<DB>,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

#[derive(Serialize)]
pub struct DraftsResponse {
    pub drafts: Vec<serde_json::Value>,
}

pub fn router<S>(state: DraftsState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/drafts", get(get_drafts))
        .with_state(state)
}

pub async fn get_drafts(
    State(state): State<DraftsState>,
    Query(query): Query<PaginationQuery>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, axum::Json(DraftsResponse { drafts: vec![] })).into_response(),
    };

    let limit = query.limit.unwrap_or(50);

    let drafts = match &state.db.store {
        DbStore::Postgres => {
            sqlx::query("SELECT id, tenant_id, source, original_content, content, draft_reply, status, sender_id, customer_id, created_at::text as created_at FROM inbox_messages WHERE tenant_id = $1 AND draft_reply IS NOT NULL AND draft_reply != '' AND status NOT IN ('auto_replied', 'resolved') ORDER BY created_at DESC LIMIT $2")
                .bind(&tenant_id)
                .bind(limit)
                .fetch_all(&state.db.pool)
                .await
                .map(|rows| {
                    rows.into_iter().map(|row| {
                        use sqlx::Row;
                        serde_json::json!({
                            "id": row.get::<String, _>("id"),
                            "tenant_id": row.get::<String, _>("tenant_id"),
                            "source": row.get::<Option<String>, _>("source").unwrap_or_default(),
                            "original_content": row.get::<Option<String>, _>("original_content").unwrap_or_default(),
                            "content": row.get::<Option<String>, _>("content").unwrap_or_default(),
                            "draft_reply": row.get::<Option<String>, _>("draft_reply").unwrap_or_default(),
                            "status": row.get::<Option<String>, _>("status").unwrap_or_default(),
                            "sender_id": row.get::<Option<String>, _>("sender_id").unwrap_or_default(),
                            "customer_id": row.get::<Option<String>, _>("customer_id").unwrap_or_default(),
                            "created_at": row.get::<Option<String>, _>("created_at").unwrap_or_default(),
                        })
                    }).collect()
                })
        }
        DbStore::Sqlite(pool) => {
            sqlx::query("SELECT id, tenant_id, source, original_content, content, draft_reply, status, sender_id, customer_id, CAST(created_at AS TEXT) as created_at FROM inbox_messages WHERE tenant_id = ? AND draft_reply IS NOT NULL AND draft_reply != '' AND status NOT IN ('auto_replied', 'resolved') ORDER BY created_at DESC LIMIT ?")
                .bind(&tenant_id)
                .bind(limit)
                .fetch_all(pool)
                .await
                .map(|rows| {
                    rows.into_iter().map(|row| {
                        use sqlx::Row;
                        serde_json::json!({
                            "id": row.get::<String, _>("id"),
                            "tenant_id": row.get::<String, _>("tenant_id"),
                            "source": row.get::<Option<String>, _>("source").unwrap_or_default(),
                            "original_content": row.get::<Option<String>, _>("original_content").unwrap_or_default(),
                            "content": row.get::<Option<String>, _>("content").unwrap_or_default(),
                            "draft_reply": row.get::<Option<String>, _>("draft_reply").unwrap_or_default(),
                            "status": row.get::<Option<String>, _>("status").unwrap_or_default(),
                            "sender_id": row.get::<Option<String>, _>("sender_id").unwrap_or_default(),
                            "customer_id": row.get::<Option<String>, _>("customer_id").unwrap_or_default(),
                            "created_at": row.get::<Option<String>, _>("created_at").unwrap_or_default(),
                        })
                    }).collect()
                })
        }
    };

    match drafts {
        Ok(drafts) => (StatusCode::OK, axum::Json(DraftsResponse { drafts })).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch drafts: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(DraftsResponse { drafts: vec![] })).into_response()
        }
    }
}
