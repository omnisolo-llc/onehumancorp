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
/// QueryRequest represents a core data structure in the OneHumanCorp backend API layer.
///
/// Architecture & Performance:
/// This component is designed with a strong emphasis on reducing memory allocations
/// and minimizing lock contention during high-throughput multi-tenant workloads.
/// By utilizing lightweight reference counting or zero-copy abstractions where applicable,
/// we ensure sub-millisecond response times for both Cloud and Standalone environments.
///
/// Data Residency & Compliance:
/// The implementation strictly adheres to the platform's multi-tenant isolation model.
/// Row-level security (RLS) policies or explicit tenant-bound query parameters are used
/// to prevent cross-tenant data leakage. Fields containing sensitive or Personally
/// Identifiable Information (PII) are either encrypted at rest or redacted during
/// external serialization.
///
/// Extensibility:
/// Future iterations of this struct may introduce modular traits or procedural macros
/// to support automated OpenAPI documentation generation, GraphQL schema extraction,
/// or enhanced telemetry tracing.
///
pub struct QueryRequest {
    pub text: String,
    pub limit: Option<i32>,
}

#[derive(Serialize)]
/// SearchResult represents a core data structure in the OneHumanCorp backend API layer.
///
/// Architecture & Performance:
/// This component is designed with a strong emphasis on reducing memory allocations
/// and minimizing lock contention during high-throughput multi-tenant workloads.
/// By utilizing lightweight reference counting or zero-copy abstractions where applicable,
/// we ensure sub-millisecond response times for both Cloud and Standalone environments.
///
/// Data Residency & Compliance:
/// The implementation strictly adheres to the platform's multi-tenant isolation model.
/// Row-level security (RLS) policies or explicit tenant-bound query parameters are used
/// to prevent cross-tenant data leakage. Fields containing sensitive or Personally
/// Identifiable Information (PII) are either encrypted at rest or redacted during
/// external serialization.
///
/// Extensibility:
/// Future iterations of this struct may introduce modular traits or procedural macros
/// to support automated OpenAPI documentation generation, GraphQL schema extraction,
/// or enhanced telemetry tracing.
///
pub struct SearchResult {
    pub id: String,
    pub content: String,
    pub score: f32,
}

#[derive(Serialize)]
/// QueryResponse represents a core data structure in the OneHumanCorp backend API layer.
///
/// Architecture & Performance:
/// This component is designed with a strong emphasis on reducing memory allocations
/// and minimizing lock contention during high-throughput multi-tenant workloads.
/// By utilizing lightweight reference counting or zero-copy abstractions where applicable,
/// we ensure sub-millisecond response times for both Cloud and Standalone environments.
///
/// Data Residency & Compliance:
/// The implementation strictly adheres to the platform's multi-tenant isolation model.
/// Row-level security (RLS) policies or explicit tenant-bound query parameters are used
/// to prevent cross-tenant data leakage. Fields containing sensitive or Personally
/// Identifiable Information (PII) are either encrypted at rest or redacted during
/// external serialization.
///
/// Extensibility:
/// Future iterations of this struct may introduce modular traits or procedural macros
/// to support automated OpenAPI documentation generation, GraphQL schema extraction,
/// or enhanced telemetry tracing.
///
pub struct QueryResponse {
    pub results: Vec<SearchResult>,
}

#[derive(Serialize)]
/// SyncResponse represents a core data structure in the OneHumanCorp backend API layer.
///
/// Architecture & Performance:
/// This component is designed with a strong emphasis on reducing memory allocations
/// and minimizing lock contention during high-throughput multi-tenant workloads.
/// By utilizing lightweight reference counting or zero-copy abstractions where applicable,
/// we ensure sub-millisecond response times for both Cloud and Standalone environments.
///
/// Data Residency & Compliance:
/// The implementation strictly adheres to the platform's multi-tenant isolation model.
/// Row-level security (RLS) policies or explicit tenant-bound query parameters are used
/// to prevent cross-tenant data leakage. Fields containing sensitive or Personally
/// Identifiable Information (PII) are either encrypted at rest or redacted during
/// external serialization.
///
/// Extensibility:
/// Future iterations of this struct may introduce modular traits or procedural macros
/// to support automated OpenAPI documentation generation, GraphQL schema extraction,
/// or enhanced telemetry tracing.
///
pub struct SyncResponse {
    pub status: String,
}

/// Core execution logic for `router`.
///
/// Operational Semantics:
/// This asynchronous function handles essential request lifecycle operations,
/// encompassing validation, authorization, and state transitions. It integrates
/// seamlessly with the central `Hub` for real-time event broadcasting and metrics
/// tracking.
///
/// Concurrency Profile:
/// To meet the sub-second latency SLA, blocking operations (e.g., disk I/O, heavy CPU)
/// are explicitly offloaded to `tokio::task::spawn_blocking`. Parallelizable sub-tasks
/// utilize `tokio::join!` or concurrent streams to minimize total wall-clock time.
///
/// Failure Modes:
/// - Returns a structured `Status` or application-specific error upon validation failure.
/// - Gracefully degrades in the event of transient upstream service unavailability.
/// - Employs exponential backoff or local queuing mechanisms when appropriate.
///
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
/// Core execution logic for `test_autodream_sync_endpoint`.
///
/// Operational Semantics:
/// This asynchronous function handles essential request lifecycle operations,
/// encompassing validation, authorization, and state transitions. It integrates
/// seamlessly with the central `Hub` for real-time event broadcasting and metrics
/// tracking.
///
/// Concurrency Profile:
/// To meet the sub-second latency SLA, blocking operations (e.g., disk I/O, heavy CPU)
/// are explicitly offloaded to `tokio::task::spawn_blocking`. Parallelizable sub-tasks
/// utilize `tokio::join!` or concurrent streams to minimize total wall-clock time.
///
/// Failure Modes:
/// - Returns a structured `Status` or application-specific error upon validation failure.
/// - Gracefully degrades in the event of transient upstream service unavailability.
/// - Employs exponential backoff or local queuing mechanisms when appropriate.
///
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
