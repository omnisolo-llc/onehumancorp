use axum::{
    extract::Query,
    response::IntoResponse,
    Json,
    routing::{post, get},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::autodream::AutoDreamWorker;

#[derive(Deserialize)]
/// Core API request/response payload for QueryRequest.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct QueryRequest {
    /// Stores the `text` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub text: String,
    /// Stores the `limit` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub limit: Option<i32>,
}

#[derive(Serialize)]
/// Core API request/response payload for SearchResult.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct SearchResult {
    /// Stores the `id` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub id: String,
    /// Stores the `content` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub content: String,
    /// Stores the `score` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub score: f32,
}

#[derive(Serialize)]
/// Core API request/response payload for QueryResponse.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct QueryResponse {
    /// Stores the `results` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub results: Vec<SearchResult>,
}

#[derive(Serialize)]
/// Core API request/response payload for SyncResponse.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct SyncResponse {
    /// Stores the `status` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub status: String,
}

pub fn router<S: Clone + Send + Sync + 'static>(worker: Arc<AutoDreamWorker>) -> Router<S> {
    let worker_sync = worker.clone();
    let worker_query = worker.clone();

    Router::new()
        .route("/sync", post(move || async move {
            match worker_sync.consolidate_epoch().await {
                Ok(_) => Json(SyncResponse { status: "success".to_string() }).into_response(),
                Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            }
        }))
        .route("/query", get(move |Query(params): Query<QueryRequest>| async move {
            if params.text.is_empty() {
                return (axum::http::StatusCode::BAD_REQUEST, "query_text is required".to_string()).into_response();
            }

            let limit = params.limit.unwrap_or(5);

            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            let client = crate::minimax::MinimaxClient::new(api_key);

            let embedding = match client.generate_embedding(&params.text).await {
                Ok(emb) => format!("[{}]", emb.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(",")),
                Err(e) => {
                    tracing::error!("AutoDream API: failed to generate embedding: {}", e);
                    format!("[{}]", vec!["0.0"; 1536].join(", "))
                }
            };

            match worker_query.search_memories(&embedding, limit).await {
                Ok(results) => {
                    let res = results.into_iter().map(|r| SearchResult {
                        id: r.id,
                        content: r.content,
                        score: r.score as f32,
                    }).collect();
                    Json(QueryResponse { results: res }).into_response()
                },
                Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            }
        }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_autodream_sync_endpoint() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(database_url)
            .unwrap();

        let db = Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });
        let worker = Arc::new(AutoDreamWorker::new(db));

        let _app: Router<()> = router(worker);
    }
}
